pub const STATEMENTS: &[crate::LoggingStatement] = &[

	crate :: LoggingStatement { level : crate :: LogLevel :: Debug , path : "/lib/Operation.php" , line : 104usize , placeholders : & ["$e->getMessage()"] , exception : None , pattern : "\0\u{1}\u{1}" , has_meaningful_message : false , },
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/Operation.php" , line : 106usize , placeholders : & [] , exception : Some ("OCP\\Files\\ForbiddenException") , pattern : "Access denied\u{1}\u{1}" , has_meaningful_message : true , },
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/Operation.php" , line : 203usize , placeholders : & [] , exception : Some ("UnexpectedValueException") , pattern : "No rule given\u{1}\u{1}" , has_meaningful_message : true , },
];

