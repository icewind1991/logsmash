pub const STATEMENTS: &[crate::LoggingStatement] = &[

	crate :: LoggingStatement { level : crate :: LogLevel :: Debug , path : "/lib/Operation.php" , line : 104usize , placeholders : & ["$e->getMessage()"] , exception : None , pattern : "\0" , },
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/Operation.php" , line : 106usize , placeholders : & [] , exception : Some ("OCP\\Files\\ForbiddenException") , pattern : "Access denied" , },
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/Operation.php" , line : 203usize , placeholders : & ["$this->l->t('No rule given')"] , exception : Some ("UnexpectedValueException") , pattern : "\0" , },
];

