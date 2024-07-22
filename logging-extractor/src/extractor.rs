use crate::messagebuilder::MessageBuilder;
use crate::{LogLevel, LoggingStatement};
use tree_sitter::{Language, Node, Parser, Query, QueryCursor};

pub struct LogExtractor {
    language: Language,
    method_query: Query,
    throw_query: Query,
}

impl LogExtractor {
    pub fn new() -> Self {
        let language = tree_sitter_php::language_php();
        let method_query = Query::new(
            &language,
            r#"(member_call_expression
                name: (name)@name
                arguments: (arguments) @args
            )"#,
        )
        .expect("invalid query");
        let throw_query = Query::new(
            &language,
            r#"(throw_expression
                (object_creation_expression
                    [(name) (qualified_name)] @name
                    (arguments) @args
                )
            )"#,
        )
        .expect("invalid query");
        LogExtractor {
            language,
            method_query,
            throw_query,
        }
    }

    pub fn extract<'a>(
        &self,
        path: &'a str,
        code: &'a str,
    ) -> impl Iterator<Item = LoggingStatement<'a>> + 'a {
        let mut parser = Parser::new();

        parser
            .set_language(&self.language)
            .expect("Error loading PHP grammar");
        parser.set_timeout_micros(10 * 1000 * 1000);

        let tree = parser.parse(code, None).expect("parse timeout or canceled");

        let mut log_call_cursor = QueryCursor::new();
        let mut throw_call_cursor = QueryCursor::new();
        let log_calls = self.get_log_calls(&mut log_call_cursor, code, tree.root_node());
        let throw_calls = self.get_throw_calls(&mut throw_call_cursor, code, tree.root_node());
        let mut all = log_calls
            .chain(throw_calls)
            .filter_map(|call| {
                let mut message_builder = MessageBuilder::with_capacity(16);

                if let Some(argument) = call.arguments {
                    let argument = argument.child(0)?;
                    message_builder.push_node(argument, code);
                }

                Some(LoggingStatement {
                    level: call.level,
                    line: call.line + 1,
                    path,
                    has_meaningful_message: message_builder.is_meaningful(),
                    exception: call.exception,
                    message_parts: message_builder.into(),
                })
            })
            .collect::<Vec<_>>();

        all.sort_by_key(|statement| statement.line);
        all.into_iter()
    }

    fn get_log_calls<'a>(
        &'a self,
        cursor: &'a mut QueryCursor,
        code: &'a str,
        node: Node<'a>,
    ) -> impl Iterator<Item = LogCall> + 'a {
        let method_calls = cursor.matches(&self.method_query, node, code.as_bytes());

        method_calls.filter_map(|method_call| {
            let name = method_call.captures[0]
                .node
                .utf8_text(code.as_bytes())
                .unwrap_or("malformed utf8");
            let level = LogLevel::parse(name)?;
            let line = method_call.captures[0].node.start_position().row;

            let arguments = method_call.captures[1].node;
            Some(LogCall {
                level,
                line,
                arguments: arguments.named_child(0),
                exception: None,
            })
        })
    }

    fn get_throw_calls<'a>(
        &'a self,
        cursor: &'a mut QueryCursor,
        code: &'a str,
        node: Node<'a>,
    ) -> impl Iterator<Item = LogCall> + 'a {
        let throws = cursor.matches(&self.throw_query, node, code.as_bytes());

        throws.map(|method_call| {
            let level = LogLevel::Exception;
            let arguments = method_call.captures[1].node;
            let line = arguments.start_position().row;
            LogCall {
                level,
                line,
                arguments: arguments.named_child(0),
                exception: Some(
                    method_call.captures[0]
                        .node
                        .utf8_text(code.as_bytes())
                        .unwrap()
                        .into(),
                ),
            }
        })
    }
}

impl Default for LogExtractor {
    fn default() -> Self {
        Self::new()
    }
}

struct LogCall<'tree> {
    level: LogLevel,
    line: usize,
    exception: Option<String>,
    arguments: Option<Node<'tree>>,
}

#[test]
fn test_extract_logging() {
    use crate::MessagePart;

    let code = r#"<?php
      function test() {
        $this->logger->warning("failed to find trash item for $rootTrashedItemName deleted at $rootTrashedItemDate in folder $groupFolderId", ['app' => 'groupfolders']);
        $logger->info('foobar');
        throw new FooException("foo \"bar\" \' {$this->blarg}");
        throw new BarException();
        $this->logger->error('Share notification mail could not be sent to: ' . implode(', ', $failedRecipients));
      }
    ?>
    "#;
    let extractor = LogExtractor::new();
    let logs = extractor.extract("foo.php", code).collect::<Vec<_>>();
    assert_eq!(
        logs[0],
        LoggingStatement {
            path: "foo.php",
            line: 3,
            level: LogLevel::Warn,
            has_meaningful_message: true,
            exception: None,
            message_parts: vec![
                MessagePart::Literal("failed to find trash item for ".into()),
                MessagePart::PlaceHolder("$rootTrashedItemName".into()),
                MessagePart::Literal(" deleted at ".into()),
                MessagePart::PlaceHolder("$rootTrashedItemDate".into()),
                MessagePart::Literal(" in folder ".into()),
                MessagePart::PlaceHolder("$groupFolderId".into()),
            ]
        }
    );
    assert_eq!(
        logs[1],
        LoggingStatement {
            path: "foo.php",
            line: 4,
            level: LogLevel::Info,
            has_meaningful_message: true,
            exception: None,
            message_parts: vec![MessagePart::Literal("foobar".into())]
        }
    );
    assert_eq!(
        logs[2],
        LoggingStatement {
            path: "foo.php",
            line: 5,
            level: LogLevel::Exception,
            has_meaningful_message: true,
            exception: Some("FooException".into()),
            message_parts: vec![
                MessagePart::Literal(r#"foo "bar" \' "#.into()),
                MessagePart::PlaceHolder("$this->blarg".into())
            ]
        }
    );
    assert_eq!(
        logs[3],
        LoggingStatement {
            path: "foo.php",
            line: 6,
            level: LogLevel::Exception,
            has_meaningful_message: false,
            exception: Some("BarException".into()),
            message_parts: vec![]
        }
    );
    assert_eq!(
        logs[4],
        LoggingStatement {
            path: "foo.php",
            line: 7,
            level: LogLevel::Error,
            has_meaningful_message: true,
            exception: None,
            message_parts: vec![
                MessagePart::Literal("Share notification mail could not be sent to: ".into()),
                MessagePart::PlaceHolder("implode(', ', $failedRecipients)".into())
            ]
        }
    );
}
