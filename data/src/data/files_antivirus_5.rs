pub const STATEMENTS: &[crate::LoggingStatement] = &[

	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/Activity/Provider.php" , line : 53usize , placeholders : & [] , exception : Some ("InvalidArgumentException") , pattern : "\u{1}\u{1}" , },
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/AppConfig.php" , line : 173usize , placeholders : & ["$key"] , exception : Some ("BadFunctionCallException") , pattern : "\0 is not a valid key\u{1}\u{1}" , },
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/AppConfig.php" , line : 189usize , placeholders : & ["$key"] , exception : Some ("BadFunctionCallException") , pattern : "\0 is not a valid key\u{1}\u{1}" , },
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/AppConfig.php" , line : 242usize , placeholders : & ["$methodName"] , exception : Some ("BadFunctionCallException") , pattern : "\0 does not exist\u{1}\u{1}" , },
	crate :: LoggingStatement { level : crate :: LogLevel :: Error , path : "/lib/AvirWrapper.php" , line : 114usize , placeholders : & ["$e->getMessage()"] , exception : None , pattern : "\0\u{1}\u{1}" , },
	crate :: LoggingStatement { level : crate :: LogLevel :: Warn , path : "/lib/AvirWrapper.php" , line : 173usize , placeholders : & ["$status->getDetails()" , "$owner" , "$path"] , exception : None , pattern : "Infected file deleted. \0 Account: \0 Path: \0\u{1}\u{1}" , },
	crate :: LoggingStatement { level : crate :: LogLevel :: Error , path : "/lib/AvirWrapper.php" , line : 188usize , placeholders : & ["$status->getDetails()" , "$path" , "$owner"] , exception : None , pattern : "Infected file deleted. \0 File: \0 Account: \0\u{1}\u{1}" , },
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/AvirWrapper.php" , line : 191usize , placeholders : & ["$status->getDetails()"] , exception : Some ("OCP\\Files\\InvalidContentException") , pattern : "Virus \0 is detected in the file. Upload cannot be completed.\u{1}\u{1}" , },
	crate :: LoggingStatement { level : crate :: LogLevel :: Debug , path : "/lib/BackgroundJob/BackgroundScanner.php" , line : 77usize , placeholders : & [] , exception : None , pattern : "Antivirus background scan disabled, skipping\u{1}\u{1}" , },
	crate :: LoggingStatement { level : crate :: LogLevel :: Error , path : "/lib/BackgroundJob/BackgroundScanner.php" , line : 90usize , placeholders : & ["$e->getMessage()"] , exception : None , pattern : "\0\u{1}\u{1}" , },
	crate :: LoggingStatement { level : crate :: LogLevel :: Debug , path : "/lib/BackgroundJob/BackgroundScanner.php" , line : 95usize , placeholders : & [] , exception : None , pattern : "Start background scan\u{1}\u{1}" , },
	crate :: LoggingStatement { level : crate :: LogLevel :: Error , path : "/lib/BackgroundJob/BackgroundScanner.php" , line : 109usize , placeholders : & ["$e->getMessage()"] , exception : None , pattern : "\0\u{1}\u{1}" , },
	crate :: LoggingStatement { level : crate :: LogLevel :: Error , path : "/lib/BackgroundJob/BackgroundScanner.php" , line : 125usize , placeholders : & ["$e->getMessage()"] , exception : None , pattern : "\0\u{1}\u{1}" , },
	crate :: LoggingStatement { level : crate :: LogLevel :: Info , path : "/lib/BackgroundJob/BackgroundScanner.php" , line : 149usize , placeholders : & [] , exception : None , pattern : "Tried to scan non file\u{1}\u{1}" , },
	crate :: LoggingStatement { level : crate :: LogLevel :: Error , path : "/lib/BackgroundJob/BackgroundScanner.php" , line : 153usize , placeholders : & ["__METHOD__" , "$e->getMessage()"] , exception : None , pattern : "\0, exception: \0\u{1}\u{1}" , },
	crate :: LoggingStatement { level : crate :: LogLevel :: Debug , path : "/lib/BackgroundJob/BackgroundScanner.php" , line : 180usize , placeholders : & ["$batchSize"] , exception : None , pattern : "Batch size is: \0\u{1}\u{1}" , },
	crate :: LoggingStatement { level : crate :: LogLevel :: Debug , path : "/lib/BackgroundJob/BackgroundScanner.php" , line : 288usize , placeholders : & ["$file->getId()"] , exception : None , pattern : "Scanning file with fileid: \0\u{1}\u{1}" , },
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/Db/ItemMapper.php" , line : 40usize , placeholders : & [] , exception : Some ("InvalidArgumentException") , pattern : "\u{1}\u{1}" , },
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/ICAP/ICAPClient.php" , line : 73usize , placeholders : & ["$this->host" , "$this->port" , "$errorMessage" , "$errorCode"] , exception : Some ("RuntimeException") , pattern : "Cannot connect to \"tcp://\0:\0\": \0 (code \0)\u{1}\u{1}" , },
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/ICAP/ICAPTlsClient.php" , line : 63usize , placeholders : & ["$this->host" , "$this->port" , "$errorMessage" , "$errorCode"] , exception : Some ("RuntimeException") , pattern : "Cannot connect to \"tls://\0:\0\": \0 (code \0)\u{1}\u{1}" , },
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/ICAP/ResponseParser.php" , line : 58usize , placeholders : & [] , exception : Some ("RuntimeException") , pattern : "Empty ICAP response\u{1}\u{1}" , },
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/ICAP/ResponseParser.php" , line : 63usize , placeholders : & ["$icapHeader"] , exception : Some ("RuntimeException") , pattern : "Unknown ICAP response: \"\0\"\u{1}\u{1}" , },
	crate :: LoggingStatement { level : crate :: LogLevel :: Error , path : "/lib/Item.php" , line : 160usize , placeholders : & ["__METHOD__" , "$e->getMessage()"] , exception : None , pattern : "\0, exception: \0\u{1}\u{1}" , },
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/Item.php" , line : 200usize , placeholders : & [] , exception : Some ("RuntimeException") , pattern : "\u{1}\u{1}" , },
	crate :: LoggingStatement { level : crate :: LogLevel :: Debug , path : "/lib/Item.php" , line : 245usize , placeholders : & ["$message" , "$this->generateExtraInfo()"] , exception : None , pattern : "\0\0\u{1}\u{1}" , },
	crate :: LoggingStatement { level : crate :: LogLevel :: Error , path : "/lib/Item.php" , line : 252usize , placeholders : & ["$message" , "$this->generateExtraInfo()"] , exception : None , pattern : "\0\0\u{1}\u{1}" , },
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/Scanner/ExternalClam.php" , line : 41usize , placeholders : & ["$avSocket" , "$errstr" , "$errno"] , exception : Some ("RuntimeException") , pattern : "Cannot connect to \"\0\": \0 (code \0)\u{1}\u{1}" , },
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/Scanner/ExternalClam.php" , line : 47usize , placeholders : & [] , exception : Some ("RuntimeException") , pattern : "The ClamAV port and host are not set up.\u{1}\u{1}" , },
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/Scanner/ExternalClam.php" , line : 51usize , placeholders : & ["$avHost" , "$avPort"] , exception : Some ("RuntimeException") , pattern : "Could not connect to ClamAV via \0:\0. Please check that ClamAV is running and reachable.\u{1}\u{1}" , },
	crate :: LoggingStatement { level : crate :: LogLevel :: Debug , path : "/lib/Scanner/ExternalClam.php" , line : 65usize , placeholders : & ["$response"] , exception : None , pattern : "Response :: \0\u{1}\u{1}" , },
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/Scanner/ExternalKaspersky.php" , line : 58usize , placeholders : & [] , exception : Some ("RuntimeException") , pattern : "The Kaspersky port and host are not set up.\u{1}\u{1}" , },
	crate :: LoggingStatement { level : crate :: LogLevel :: Debug , path : "/lib/Scanner/ExternalKaspersky.php" , line : 90usize , placeholders : & ["$response"] , exception : None , pattern : "Response :: \0\u{1}\u{1}" , },
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/Scanner/ICAP.php" , line : 63usize , placeholders : & [] , exception : Some ("RuntimeException") , pattern : "The ICAP port and host are not set up.\u{1}\u{1}" , },
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/Scanner/ICAP.php" , line : 139usize , placeholders : & [] , exception : Some ("RuntimeException") , pattern : "Invalid response from ICAP server\u{1}\u{1}" , },
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/Scanner/LocalClam.php" , line : 41usize , placeholders : & ["$this->avPath"] , exception : Some ("RuntimeException") , pattern : "The antivirus executable could not be found at \0\u{1}\u{1}" , },
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/Scanner/LocalClam.php" , line : 60usize , placeholders : & [] , exception : Some ("RuntimeException") , pattern : "Error starting process\u{1}\u{1}" , },
	crate :: LoggingStatement { level : crate :: LogLevel :: Debug , path : "/lib/Scanner/LocalClam.php" , line : 74usize , placeholders : & ["$result" , "$output"] , exception : None , pattern : "Exit code :: \0 Response :: \0\u{1}\u{1}" , },
	crate :: LoggingStatement { level : crate :: LogLevel :: Debug , path : "/lib/Scanner/ScannerBase.php" , line : 178usize , placeholders : & [] , exception : None , pattern : "reinit scanner\u{1}\u{1}" , },
	crate :: LoggingStatement { level : crate :: LogLevel :: Warn , path : "/lib/Scanner/ScannerBase.php" , line : 191usize , placeholders : & [] , exception : None , pattern : "Failed to write a chunk. Check if Stream Length matches StreamMaxLength in anti virus daemon settings\u{1}\u{1}" , },
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/Scanner/ScannerFactory.php" , line : 52usize , placeholders : & ["$avMode"] , exception : Some ("InvalidArgumentException") , pattern : "Application is misconfigured. Please check the settings at the admin page. Invalid mode: \0\u{1}\u{1}" , },
	crate :: LoggingStatement { level : crate :: LogLevel :: Error , path : "/lib/Status.php" , line : 86usize , placeholders : & ["__METHOD__" , "$e->getMessage()"] , exception : None , pattern : "\0, exception: \0\u{1}\u{1}" , },
];

