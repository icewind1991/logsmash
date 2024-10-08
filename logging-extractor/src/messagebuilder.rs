use crate::string::{unescape, DoubleQuoteString, SingleQuoteString};
use crate::MessagePart;
use regex::Regex;
use sprintf::parser::{parse_format_string, FormatElement};
use std::collections::HashMap;
use tree_sitter::{Node, TreeCursor};

pub struct MessageBuilder {
    pub parts: Vec<MessagePart>,
    placeholder_regex: Regex,
}

impl MessageBuilder {
    pub fn with_capacity(cap: usize) -> Self {
        MessageBuilder {
            parts: Vec::with_capacity(cap),
            placeholder_regex: Regex::new("\\{([a-zA-Z0-9]+)}").unwrap(),
        }
    }

    pub fn push_literal(&mut self, content: &str) {
        if self.placeholder_regex.is_match(content) {
            let mut start = 0;
            for placeholder in self.placeholder_regex.find_iter(content) {
                if placeholder.start() > start {
                    Self::push_literal_inner(&mut self.parts, &content[start..placeholder.start()]);
                    Self::push_placeholder_inner(&mut self.parts, placeholder.as_str());
                }
                start = placeholder.end();
            }
            if start < content.len() {
                Self::push_literal_inner(&mut self.parts, &content[start..])
            }
        } else {
            Self::push_literal_inner(&mut self.parts, content)
        }
    }

    fn push_literal_inner(parts: &mut Vec<MessagePart>, content: &str) {
        if let Some(MessagePart::Literal(last_part)) = parts.last_mut() {
            last_part.push_str(content);
        } else {
            parts.push(MessagePart::Literal(content.into()))
        }
    }

    pub fn push_placeholder(&mut self, placeholder: &str) {
        Self::push_placeholder_inner(&mut self.parts, placeholder);
    }

    pub fn push_printf<'a, Args: Iterator<Item = &'a str>>(
        &mut self,
        string: &str,
        placeholders: &mut Args,
    ) {
        if let Ok(format_elements) = parse_format_string(string) {
            for element in format_elements {
                match element {
                    FormatElement::Verbatim(str) => Self::push_literal_inner(&mut self.parts, &str),
                    FormatElement::Format(_) => Self::push_placeholder_inner(
                        &mut self.parts,
                        placeholders.next().unwrap_or_default(),
                    ),
                }
            }
        } else {
            Self::push_placeholder_inner(&mut self.parts, string);
        }
    }

    fn push_placeholder_inner(parts: &mut Vec<MessagePart>, placeholder: &str) {
        let placeholder = placeholder.replace(['\n', '\r', '\t'], "");
        parts.push(MessagePart::PlaceHolder(placeholder));
    }

    fn extend<I: Iterator<Item = MessagePart>>(&mut self, parts: I) {
        for part in parts {
            match part {
                MessagePart::Literal(lit) => self.push_literal(&lit),
                MessagePart::PlaceHolder(placeholder) => self.push_placeholder(&placeholder),
            }
        }
    }

    pub fn push_node(&mut self, node: Node, code: &str, context: &mut HashMap<&str, Node>) {
        let mut cursor = node.walk();
        match node.grammar_name() {
            "string" | "encapsed_string" => {
                self.extend(string_parts(node, code, &mut cursor).into_iter());
            }
            "binary_expression" => {
                let start = node.named_child(0).unwrap().range().end_byte;
                let end = node.named_child(1).unwrap().range().start_byte;
                let operator = &code[start..end];
                if operator.trim() == "." {
                    for part in node.named_children(&mut cursor) {
                        self.push_node(part, code, context);
                    }
                }
            }
            "variable_name" => {
                let name = node
                    .child(1)
                    .map(|c| c.utf8_text(code.as_bytes()).unwrap())
                    .unwrap_or_default();
                if let Some(replacement) = context.remove(name) {
                    if has_literal(replacement, code, context) {
                        self.push_node(replacement, code, context);
                        return;
                    }
                }
                let placeholder = node.utf8_text(code.as_bytes()).unwrap();
                self.push_placeholder(placeholder);
            }
            "member_call_expression" | "function_call_expression" => {
                match node
                    .child_by_field_name("name")
                    .or_else(|| node.child_by_field_name("function"))
                    .and_then(|name| name.utf8_text(code.as_bytes()).ok())
                {
                    Some("t") | Some("sprintf") => {
                        let arguments =
                            node.child_by_field_name("arguments").expect("no arguments");
                        let mut arguments = arguments.children(&mut cursor).skip(1); // opening bracket
                        let mut cursor = node.walk();
                        let fmt = string_parts(
                            arguments.next().unwrap().child(0).unwrap(),
                            code,
                            &mut cursor,
                        );
                        let mut arguments = arguments
                            .filter(|arg| arg.grammar_name() != ",")
                            .map(|arg| arg.utf8_text(code.as_bytes()).unwrap());
                        for part in fmt {
                            match part {
                                MessagePart::Literal(lit) => self.push_printf(&lit, &mut arguments),
                                MessagePart::PlaceHolder(placeholder) => {
                                    self.push_placeholder(&placeholder)
                                }
                            }
                        }
                    }
                    _ => {
                        let placeholder = node.utf8_text(code.as_bytes()).unwrap();
                        self.push_placeholder(placeholder);
                    }
                }
            }
            _ => {
                let placeholder = node.utf8_text(code.as_bytes()).unwrap();
                self.push_placeholder(placeholder);
            }
        }
    }

    /// Ensure there is at least some text to match
    pub fn is_meaningful(&self) -> bool {
        self.parts.iter().any(|part| matches!(part, MessagePart::Literal(part) if part.contains(|c: char| c.is_ascii_alphanumeric())))
    }
}

fn has_literal(node: Node, code: &str, context: &HashMap<&str, Node>) -> bool {
    let mut replacement_builder = MessageBuilder::with_capacity(4);
    replacement_builder.push_node(node.clone(), code, &mut context.clone());
    replacement_builder
        .parts
        .iter()
        .any(|part| matches!(part, MessagePart::Literal(_)))
}

impl From<MessageBuilder> for Vec<MessagePart> {
    fn from(value: MessageBuilder) -> Self {
        value.parts
    }
}

fn string_parts<'cursor, 'node: 'cursor>(
    node: Node<'node>,
    code: &str,
    cursor: &mut TreeCursor<'cursor>,
) -> Vec<MessagePart> {
    let mut argument_string_parts = node.children(cursor);
    let is_double_quote = argument_string_parts
        .next()
        .map(|child| child.grammar_name())
        .unwrap_or_default()
        == r#"""#;

    argument_string_parts
        .filter_map(move |string_part| match string_part.grammar_name() {
            "string_content" => {
                let content = string_part.utf8_text(code.as_bytes()).unwrap();
                Some(MessagePart::Literal(content.into()))
            }
            "escape_sequence" => {
                let raw = string_part.utf8_text(code.as_bytes()).unwrap();
                let content = if is_double_quote {
                    unescape::<DoubleQuoteString>(raw)
                } else {
                    unescape::<SingleQuoteString>(raw)
                }
                .unwrap();
                Some(MessagePart::Literal(content))
            }
            r#"'"# | r#"""# | r#"{"# | r#"}"# => None,
            _ => {
                let placeholder = string_part.utf8_text(code.as_bytes()).unwrap();
                Some(MessagePart::PlaceHolder(placeholder.into()))
            }
        })
        .collect()
}

#[test]
fn test_printf() {
    let mut builder = MessageBuilder::with_capacity(4);
    builder.push_printf("test %s foo", &mut ["$name"].into_iter());
    assert_eq!(
        vec![
            MessagePart::Literal("test ".into()),
            MessagePart::PlaceHolder("$name".into()),
            MessagePart::Literal(" foo".into())
        ],
        builder.parts
    )
}

#[test]
fn test_push_pattern_literal() {
    fn push_str(str: &str) -> Vec<MessagePart> {
        let mut builder = MessageBuilder::with_capacity(4);
        builder.push_literal(str);
        builder.parts
    }
    assert_eq!(
        vec![
            MessagePart::Literal(r#"Trusted domain error. ""#.into()),
            MessagePart::PlaceHolder(r#"{remoteAddress}"#.into()),
            MessagePart::Literal(r#"" tried to access using ""#.into()),
            MessagePart::PlaceHolder(r#"{host}"#.into()),
            MessagePart::Literal(r#"" as host."#.into()),
        ],
        push_str(
            r#"Trusted domain error. "{remoteAddress}" tried to access using "{host}" as host."#
        )
    )
}
