use crate::string::{unescape, DoubleQuoteString, SingleQuoteString};
use crate::{LogLevel, LoggingStatement};
use std::borrow::Cow;
use tree_sitter::{Language, Node, Parser, Query, QueryCursor};

pub struct LogExtractor {
    language: Language,
    method_query: Query,
    throw_query: Query,
    string_query: Query,
}

impl LogExtractor {
    pub fn new() -> Self {
        let language = tree_sitter_php::language_php();
        let method_query = Query::new(
            &language,
            r#"(member_call_expression
                name: (name)@name
                arguments: (arguments ((argument)+ @arg))
            )"#,
        )
        .expect("invalid query");
        let throw_query = Query::new(
            &language,
            r#"(throw_expression
                (object_creation_expression
                    (arguments ((argument)+ @arg))
                )
            )"#,
        )
        .expect("invalid query");
        let string_query = Query::new(&language, r#"[(string_content)(escape_sequence)]@string"#)
            .expect("invalid query");
        LogExtractor {
            language,
            method_query,
            throw_query,
            string_query,
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
            .map(|call| {
                let mut string_cursor = QueryCursor::new();
                let message_parts = string_cursor
                    .matches(&self.string_query, call.arguments, code.as_bytes())
                    .map(|result| {
                        let node = result.captures[0].node;
                        let raw = node.utf8_text(code.as_bytes()).unwrap_or("malformed utf8");

                        if raw.contains('\\') {
                            let start_char =
                                code.as_bytes()[node.parent().unwrap().byte_range().start];
                            Cow::Owned(
                                if start_char == b'"' {
                                    unescape::<DoubleQuoteString>(raw)
                                } else {
                                    unescape::<SingleQuoteString>(raw)
                                }
                                .unwrap(),
                            )
                        } else {
                            Cow::Borrowed(raw)
                        }
                    })
                    .collect();

                LoggingStatement {
                    level: call.level,
                    line: call.line + 1,
                    path,
                    message_parts,
                }
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
                arguments,
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

        throws.filter_map(|method_call| {
            let level = LogLevel::Exception;
            let arguments = method_call.captures[0].node;
            let line = arguments.start_position().row;
            Some(LogCall {
                level,
                line,
                arguments,
            })
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
    arguments: Node<'tree>,
}

#[test]
fn test_extract_logging() {
    let code = r#"<?php
      function test() {
        $this->logger->warning("failed to find trash item for $rootTrashedItemName deleted at $rootTrashedItemDate in folder $groupFolderId", ['app' => 'groupfolders']);
        $logger->info("foobar");
        throw new FooException("foo \"bar\" \' {$blarg}");
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
            message_parts: vec![
                "failed to find trash item for ".into(),
                " deleted at ".into(),
                " in folder ".into()
            ]
        }
    );
    assert_eq!(
        logs[1],
        LoggingStatement {
            path: "foo.php",
            line: 4,
            level: LogLevel::Info,
            message_parts: vec!["foobar".into()]
        }
    );
    assert_eq!(
        logs[2],
        LoggingStatement {
            path: "foo.php",
            line: 5,
            level: LogLevel::Exception,
            message_parts: vec![
                "foo ".into(),
                "\"".into(),
                "bar".into(),
                "\"".into(),
                " \\' ".into()
            ]
        }
    );
}
