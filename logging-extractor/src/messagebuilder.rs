use crate::string::{unescape, DoubleQuoteString, SingleQuoteString};
use crate::MessagePart;
use regex::Regex;
use tree_sitter::Node;

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
                    self.parts
                        .push(MessagePart::PlaceHolder(placeholder.as_str().into()));
                }
                start = placeholder.end();
            }
            if start < content.len() {
                Self::push_literal_inner(&mut self.parts, content)
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
        self.parts
            .push(MessagePart::PlaceHolder(placeholder.into()));
    }

    pub fn push_node(&mut self, node: Node, code: &str) {
        let mut cursor = node.walk();
        match node.grammar_name() {
            "string" | "encapsed_string" => {
                let mut argument_string_parts = node.children(&mut cursor);
                let is_double_quote = argument_string_parts
                    .next()
                    .map(|child| child.grammar_name())
                    .unwrap_or_default()
                    == r#"""#;

                for string_part in argument_string_parts {
                    match string_part.grammar_name() {
                        "string_content" => {
                            let content = string_part.utf8_text(code.as_bytes()).unwrap();
                            self.push_literal(content);
                        }
                        "escape_sequence" => {
                            let raw = string_part.utf8_text(code.as_bytes()).unwrap();
                            let content = if is_double_quote {
                                unescape::<DoubleQuoteString>(raw)
                            } else {
                                unescape::<SingleQuoteString>(raw)
                            }
                            .unwrap();
                            self.push_literal(&content);
                        }
                        r#"'"# | r#"""# | r#"{"# | r#"}"# => {}
                        _ => {
                            let placeholder = string_part.utf8_text(code.as_bytes()).unwrap();
                            self.push_placeholder(placeholder);
                        }
                    }
                }
            }
            "binary_expression" => {
                let start = node.named_child(0).unwrap().range().end_byte;
                let end = node.named_child(1).unwrap().range().start_byte;
                let operator = &code[start..end];
                if operator.trim() == "." {
                    for part in node.named_children(&mut cursor) {
                        self.push_node(part, code);
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

impl From<MessageBuilder> for Vec<MessagePart> {
    fn from(value: MessageBuilder) -> Self {
        value.parts
    }
}
