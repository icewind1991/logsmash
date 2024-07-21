pub const STATEMENTS: &[crate::LoggingStatement] = &[

	crate :: LoggingStatement { level : crate :: LogLevel :: Debug , path : "/apps/support/lib/Sections/ServerSection.php" , line : 153usize , placeholders : & [] , regex : "^Unable to determine database version$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Warn , path : "/apps/support/lib/Controller/ApiController.php" , line : 105usize , placeholders : & [] , regex : "^Could not create folder \"System information\" to store generated report\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Info , path : "/apps/support/lib/Repair/SwitchUpdaterServer.php" , line : 49usize , placeholders : & [] , regex : "^Repair step already executed$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Warn , path : "/apps/support/lib/Service/SubscriptionService.php" , line : 132usize , placeholders : & ["$userCount" , "$disabledUsersCount"] , regex : "^Total user count was negative \\(users: (.*), disabled: (.*)\\)$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Info , path : "/apps/support/lib/Service/SubscriptionService.php" , line : 191usize , placeholders : & [] , regex : "^Subscription info successfully fetched$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Info , path : "/apps/support/lib/Service/SubscriptionService.php" , line : 235usize , placeholders : & [] , regex : "^Subscription key invalid$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Critical , path : "/apps/support/lib/AppInfo/Application.php" , line : 67usize , placeholders : & [] , regex : "^Multiple subscription adapters are registered\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/support/lib/Notification/Notifier.php" , line : 66usize , placeholders : & [] , regex : "^Unknown app id$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Error , path : "/apps/federation/lib/SyncJob.php" , line : 47usize , placeholders : & ["$url"] , regex : "^Error while syncing (.*)\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Debug , path : "/apps/federation/lib/SyncFederationAddressBooks.php" , line : 67usize , placeholders : & ["$url"] , regex : "^Shared secret for (.*) is null$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Debug , path : "/apps/federation/lib/SyncFederationAddressBooks.php" , line : 80usize , placeholders : & ["$url"] , regex : "^Sync Token for (.*) unchanged from previous sync$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Error , path : "/apps/federation/lib/SyncFederationAddressBooks.php" , line : 85usize , placeholders : & ["$url"] , regex : "^Server sync for (.*) failed because of revoked access\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Error , path : "/apps/federation/lib/SyncFederationAddressBooks.php" , line : 90usize , placeholders : & ["$url"] , regex : "^Server sync for (.*) failed\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Error , path : "/apps/federation/lib/TrustedServers.php" , line : 179usize , placeholders : & [] , regex : "^No Nextcloud server\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/federation/lib/TrustedServers.php" , line : 197usize , placeholders : & [] , regex : "^Remote server version is too low\\. 9\\.0 is required\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/federation/composer/composer/ClassLoader.php" , line : 256usize , placeholders : & [] , regex : "^A non\\-empty PSR\\-4 prefix must end with a namespace separator\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/federation/composer/composer/ClassLoader.php" , line : 311usize , placeholders : & [] , regex : "^A non\\-empty PSR\\-4 prefix must end with a namespace separator\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Warn , path : "/apps/password_policy/lib/Compliance/HistoryCompliance.php" , line : 123usize , placeholders : & [] , regex : "^Received password history of \\{uid\\} had the unexpected value of \\{history\\}, resetting\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/password_policy/lib/Generator.php" , line : 111usize , placeholders : & [] , regex : "^Could not generate a valid password$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/text/lib/TextFile.php" , line : 64usize , placeholders : & [] , regex : "^File not compatible with text because it could not be encoded to UTF\\-8\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Error , path : "/apps/text/lib/Controller/AttachmentController.php" , line : 121usize , placeholders : & [] , regex : "^File insertion error$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/text/lib/Controller/AttachmentController.php" , line : 137usize , placeholders : & [] , regex : "^Could not read file$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Error , path : "/apps/text/lib/Controller/AttachmentController.php" , line : 154usize , placeholders : & [] , regex : "^Upload error$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Error , path : "/apps/text/lib/Controller/AttachmentController.php" , line : 211usize , placeholders : & [] , regex : "^getImageFile error$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Error , path : "/apps/text/lib/Controller/AttachmentController.php" , line : 241usize , placeholders : & [] , regex : "^getMediaFile error$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Error , path : "/apps/text/lib/Controller/AttachmentController.php" , line : 277usize , placeholders : & [] , regex : "^getMediaFilePreview error$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Error , path : "/apps/text/lib/Controller/WorkspaceController.php" , line : 128usize , placeholders : & [] , regex : "^Failed to get workspace file$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Error , path : "/apps/text/lib/Controller/WorkspaceController.php" , line : 175usize , placeholders : & [] , regex : "^Failed to get public workspace file$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Error , path : "/apps/text/lib/Controller/WorkspaceController.php" , line : 201usize , placeholders : & [] , regex : "^Exception when creating a new file through direct editing$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Debug , path : "/apps/text/lib/Cron/Cleanup.php" , line : 55usize , placeholders : & [] , regex : "^Run cleanup job for text documents$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Debug , path : "/apps/text/lib/Cron/Cleanup.php" , line : 69usize , placeholders : & [] , regex : "^Run cleanup job for text sessions$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Debug , path : "/apps/text/lib/Cron/Cleanup.php" , line : 73usize , placeholders : & [] , regex : "^Run cleanup job for obsolete documents folders$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/text/lib/Db/SessionMapper.php" , line : 69usize , placeholders : & [] , regex : "^Session is invalid$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/text/lib/Db/Step.php" , line : 64usize , placeholders : & [] , regex : "^Failed to parse step data$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/text/lib/Db/DocumentMapper.php" , line : 54usize , placeholders : & [] , regex : "^Document doesn't exist$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Error , path : "/apps/text/lib/Service/ApiService.php" , line : 96usize , placeholders : & [] , regex : "^No permission to access this file$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/text/lib/Service/SessionService.php" , line : 229usize , placeholders : & [] , regex : "^Logged in users cannot set a guest name$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Debug , path : "/apps/text/lib/Service/DocumentService.php" , line : 146usize , placeholders : & [] , regex : "^Unsaved steps, continue collaborative editing$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/text/lib/Service/DocumentService.php" , line : 156usize , placeholders : & [] , regex : "^No app data folder present for text documents$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/text/lib/Service/DocumentService.php" , line : 187usize , placeholders : & [] , regex : "^No app data folder present for text documents$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Error , path : "/apps/text/lib/Service/DocumentService.php" , line : 212usize , placeholders : & [] , regex : "^Failed to create document state file$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/text/lib/Service/DocumentService.php" , line : 250usize , placeholders : & [] , regex : "^Read\\-only client tries to push steps with changes$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/text/lib/Service/DocumentService.php" , line : 283usize , placeholders : & [] , regex : "^Failed to encode steps$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Error , path : "/apps/text/lib/Service/DocumentService.php" , line : 299usize , placeholders : & [] , regex : "^This should never happen\\. An error occurred when storing the version, trying to recover the last stable one$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/text/lib/Service/DocumentService.php" , line : 337usize , placeholders : & [] , regex : "^File changed in the meantime from outside$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Warn , path : "/apps/text/lib/Service/DocumentService.php" , line : 359usize , placeholders : & [] , regex : "^Saving empty document$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/text/lib/Service/DocumentService.php" , line : 422usize , placeholders : & [] , regex : "^Did not reset document, as it has unsaved changes$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/text/lib/Service/DocumentService.php" , line : 451usize , placeholders : & [] , regex : "^No proper share data$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/text/lib/Service/DocumentService.php" , line : 467usize , placeholders : & [] , regex : "^No proper share data$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/text/lib/Service/DocumentService.php" , line : 482usize , placeholders : & [] , regex : "^Could not fallback to file from mounts$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/text/lib/Service/DocumentService.php" , line : 536usize , placeholders : & [] , regex : "^No proper share data$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/text/lib/Service/DocumentService.php" , line : 652usize , placeholders : & [] , regex : "^Folder not found$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/text/lib/Service/AttachmentService.php" , line : 222usize , placeholders : & [] , regex : "^Unable to read document$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/text/lib/Service/AttachmentService.php" , line : 284usize , placeholders : & [] , regex : "^No write permissions$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/text/lib/Service/AttachmentService.php" , line : 313usize , placeholders : & [] , regex : "^No write permissions$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/text/lib/Service/AttachmentService.php" , line : 343usize , placeholders : & [] , regex : "^No write permissions$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/text/composer/composer/ClassLoader.php" , line : 250usize , placeholders : & [] , regex : "^A non\\-empty PSR\\-4 prefix must end with a namespace separator\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/text/composer/composer/ClassLoader.php" , line : 305usize , placeholders : & [] , regex : "^A non\\-empty PSR\\-4 prefix must end with a namespace separator\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/viewer/composer/composer/ClassLoader.php" , line : 256usize , placeholders : & [] , regex : "^A non\\-empty PSR\\-4 prefix must end with a namespace separator\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/viewer/composer/composer/ClassLoader.php" , line : 311usize , placeholders : & [] , regex : "^A non\\-empty PSR\\-4 prefix must end with a namespace separator\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/user_ldap/composer/composer/ClassLoader.php" , line : 256usize , placeholders : & [] , regex : "^A non\\-empty PSR\\-4 prefix must end with a namespace separator\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/user_ldap/composer/composer/ClassLoader.php" , line : 311usize , placeholders : & [] , regex : "^A non\\-empty PSR\\-4 prefix must end with a namespace separator\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/user_ldap/lib/Command/ResetGroup.php" , line : 76usize , placeholders : & [] , regex : "^Group not found$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/user_ldap/lib/Command/ResetGroup.php" , line : 80usize , placeholders : & [] , regex : "^The given group is not a recognized LDAP group\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/user_ldap/lib/Command/ResetGroup.php" , line : 89usize , placeholders : & [] , regex : "^Reset cancelled by operator$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/user_ldap/lib/Command/CheckUser.php" , line : 109usize , placeholders : & [] , regex : "^The given user is not a recognized LDAP user\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/user_ldap/lib/Command/Search.php" , line : 96usize , placeholders : & [] , regex : "^limit must be  0 or greater$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/user_ldap/lib/Command/Search.php" , line : 99usize , placeholders : & [] , regex : "^offset must be 0 or greater$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/user_ldap/lib/Command/Search.php" , line : 102usize , placeholders : & [] , regex : "^offset must be 0 if limit is also set to 0$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/user_ldap/lib/Command/Search.php" , line : 105usize , placeholders : & [] , regex : "^offset must be a multiple of limit$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/user_ldap/lib/Command/ResetUser.php" , line : 79usize , placeholders : & [] , regex : "^User not found$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/user_ldap/lib/Command/ResetUser.php" , line : 83usize , placeholders : & [] , regex : "^The given user is not a recognized LDAP user\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/user_ldap/lib/Command/ResetUser.php" , line : 92usize , placeholders : & [] , regex : "^Reset cancelled by operator$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Info , path : "/apps/user_ldap/lib/Command/UpdateUUID.php" , line : 340usize , placeholders : & [] , regex : "^UUID of \\{id\\} was updated from \\{from\\} to \\{to\\}$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/user_ldap/lib/Mapping/AbstractMapping.php" , line : 93usize , placeholders : & [] , regex : "^Invalid Column Name$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Error , path : "/apps/user_ldap/lib/Mapping/AbstractMapping.php" , line : 354usize , placeholders : & [] , regex : "^Cannot map, because the DN exceeds 4000 characters: \\{dn\\}$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/user_ldap/lib/Helper.php" , line : 293usize , placeholders : & [] , regex : "^key uid is expected to be set in \\$param$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/user_ldap/lib/Wizard.php" , line : 107usize , placeholders : & [] , regex : "^Requirements not met$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/user_ldap/lib/Wizard.php" , line : 119usize , placeholders : & [] , regex : "^Internal error: Invalid object type$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/user_ldap/lib/Wizard.php" , line : 324usize , placeholders : & [] , regex : "^Failed to determine user attributes$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/user_ldap/lib/Wizard.php" , line : 355usize , placeholders : & [] , regex : "^Could not connect to LDAP$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/user_ldap/lib/Wizard.php" , line : 412usize , placeholders : & [] , regex : "^Could not connect to LDAP$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/user_ldap/lib/Wizard.php" , line : 421usize , placeholders : & [] , regex : "^memberOf is not supported by the server$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/user_ldap/lib/Wizard.php" , line : 516usize , placeholders : & [] , regex : "^Could not connect to LDAP$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/user_ldap/lib/Wizard.php" , line : 544usize , placeholders : & [] , regex : "^Could not connect to LDAP$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/user_ldap/lib/Wizard.php" , line : 606usize , placeholders : & [] , regex : "^Cannot create filter$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/user_ldap/lib/Wizard.php" , line : 628usize , placeholders : & [] , regex : "^Cannot create filter$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/user_ldap/lib/Wizard.php" , line : 650usize , placeholders : & [] , regex : "^connection error$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/user_ldap/lib/Wizard.php" , line : 656usize , placeholders : & [] , regex : "^missing placeholder$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/user_ldap/lib/Wizard.php" , line : 815usize , placeholders : & [] , regex : "^Could not connect to LDAP$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/user_ldap/lib/Wizard.php" , line : 854usize , placeholders : & [] , regex : "^Could not connect to LDAP$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/user_ldap/lib/Wizard.php" , line : 886usize , placeholders : & [] , regex : "^Could not connect to LDAP$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/user_ldap/lib/Wizard.php" , line : 924usize , placeholders : & [] , regex : "^Could not connect to LDAP$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/user_ldap/lib/Wizard.php" , line : 993usize , placeholders : & [] , regex : "^Failed to get user attributes$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Debug , path : "/apps/user_ldap/lib/Wizard.php" , line : 1068usize , placeholders : & [] , regex : "^Wiz: Attempting to connect$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Debug , path : "/apps/user_ldap/lib/Wizard.php" , line : 1091usize , placeholders : & [] , regex : "^Wiz: Attempting to Bind$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/user_ldap/lib/Wizard.php" , line : 1241usize , placeholders : & [] , regex : "^Could not connect to LDAP$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/user_ldap/lib/Access.php" , line : 131usize , placeholders : & [] , regex : "^UserMapper was not assigned to this Access instance\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/user_ldap/lib/Access.php" , line : 150usize , placeholders : & [] , regex : "^GroupMapper was not assigned to this Access instance\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Warn , path : "/apps/user_ldap/lib/Access.php" , line : 184usize , placeholders : & [] , regex : "^No LDAP Connector assigned, access impossible for readAttribute\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Debug , path : "/apps/user_ldap/lib/Access.php" , line : 193usize , placeholders : & [] , regex : "^LDAP resource not available\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/user_ldap/lib/Access.php" , line : 346usize , placeholders : & [] , regex : "^LDAP password changes are disabled\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Debug , path : "/apps/user_ldap/lib/Access.php" , line : 351usize , placeholders : & [] , regex : "^LDAP resource not available\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/user_ldap/lib/Access.php" , line : 359usize , placeholders : & [] , regex : "^Password change rejected\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Warn , path : "/apps/user_ldap/lib/Access.php" , line : 594usize , placeholders : & [] , regex : "^Mapped \\{fdn\\} as \\{altName\\} because of a name collision on \\{intName\\}\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Debug , path : "/apps/user_ldap/lib/Access.php" , line : 920usize , placeholders : & ["$ocName"] , regex : "^The ldap user manager returned null for (.*)$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/user_ldap/lib/Access.php" , line : 1060usize , placeholders : & [] , regex : "^Invoker does not support controlPagedResultResponse, call LDAP Wrapper directly instead\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Debug , path : "/apps/user_ldap/lib/Access.php" , line : 1076usize , placeholders : & ["$command"] , regex : "^Connection lost on (.*), attempting to reestablish\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Debug , path : "/apps/user_ldap/lib/Access.php" , line : 1082usize , placeholders : & ["$command"] , regex : "^Could not (.*), because resource is missing\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Debug , path : "/apps/user_ldap/lib/Access.php" , line : 1116usize , placeholders : & [] , regex : "^Could not search, because resource is missing\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Debug , path : "/apps/user_ldap/lib/Access.php" , line : 1176usize , placeholders : & [] , regex : "^Paged search was not available$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Debug , path : "/apps/user_ldap/lib/Access.php" , line : 1212usize , placeholders : & [] , regex : "^Count filter: \\{filter\\}$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/user_ldap/lib/Access.php" , line : 1397usize , placeholders : & [] , regex : "^provided name template for username does not contain any allowed characters$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/user_ldap/lib/Access.php" , line : 1409usize , placeholders : & [] , regex : "^provided name template for username does not contain any allowed characters$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/user_ldap/lib/Access.php" , line : 1507usize , placeholders : & [] , regex : "^searchAttributes must be an array with at least two string$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/user_ldap/lib/Access.php" , line : 1642usize , placeholders : & [] , regex : "^Cannot determine UUID attribute$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/user_ldap/lib/Access.php" , line : 1649usize , placeholders : & [] , regex : "^Cannot determine UUID attribute$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/user_ldap/lib/Access.php" , line : 1666usize , placeholders : & [] , regex : "^Cannot determine UUID attribute$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Debug , path : "/apps/user_ldap/lib/Access.php" , line : 1714usize , placeholders : & [] , regex : "^Setting \\{attribute\\} as \\{subject\\}$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Debug , path : "/apps/user_ldap/lib/Access.php" , line : 1727usize , placeholders : & [] , regex : "^Could not autodetect the UUID attribute$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Debug , path : "/apps/user_ldap/lib/Access.php" , line : 1984usize , placeholders : & [] , regex : "^initializing paged search for filter \\{filter\\}, base \\{base\\}, attr \\{attr\\}, pageSize \\{pageSize\\}, offset \\{offset\\}$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Debug , path : "/apps/user_ldap/lib/Access.php" , line : 2012usize , placeholders : & [] , regex : "^Ready for a paged search$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/user_ldap/lib/UserPluginManager.php" , line : 97usize , placeholders : & [] , regex : "^No plugin implements createUser in this LDAP Backend\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/user_ldap/lib/UserPluginManager.php" , line : 113usize , placeholders : & [] , regex : "^No plugin implements setPassword in this LDAP Backend\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/user_ldap/lib/UserPluginManager.php" , line : 128usize , placeholders : & [] , regex : "^No plugin implements canChangeAvatar in this LDAP Backend\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/user_ldap/lib/UserPluginManager.php" , line : 143usize , placeholders : & [] , regex : "^No plugin implements getHome in this LDAP Backend\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/user_ldap/lib/UserPluginManager.php" , line : 158usize , placeholders : & [] , regex : "^No plugin implements getDisplayName in this LDAP Backend\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/user_ldap/lib/UserPluginManager.php" , line : 174usize , placeholders : & [] , regex : "^No plugin implements setDisplayName in this LDAP Backend\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/user_ldap/lib/UserPluginManager.php" , line : 188usize , placeholders : & [] , regex : "^No plugin implements countUsers in this LDAP Backend\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/user_ldap/lib/UserPluginManager.php" , line : 211usize , placeholders : & [] , regex : "^No plugin implements deleteUser in this LDAP Backend\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/user_ldap/lib/LDAPProvider.php" , line : 77usize , placeholders : & [] , regex : "^To use the LDAPProvider, user_ldap app must be enabled$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/user_ldap/lib/LDAPProvider.php" , line : 89usize , placeholders : & [] , regex : "^User id not found in LDAP$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/user_ldap/lib/LDAPProvider.php" , line : 93usize , placeholders : & [] , regex : "^Translation to LDAP DN unsuccessful$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/user_ldap/lib/LDAPProvider.php" , line : 106usize , placeholders : & [] , regex : "^Group id not found in LDAP$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/user_ldap/lib/LDAPProvider.php" , line : 110usize , placeholders : & [] , regex : "^Translation to LDAP DN unsuccessful$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/user_ldap/lib/LDAPProvider.php" , line : 125usize , placeholders : & [] , regex : "^Translation to internal user name unsuccessful$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/user_ldap/lib/LDAPProvider.php" , line : 157usize , placeholders : & [] , regex : "^User id not found in LDAP$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/user_ldap/lib/LDAPProvider.php" , line : 171usize , placeholders : & [] , regex : "^Group id not found in LDAP$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/user_ldap/lib/LDAPProvider.php" , line : 184usize , placeholders : & [] , regex : "^User id not found in LDAP$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Info , path : "/apps/user_ldap/lib/LDAPProvider.php" , line : 195usize , placeholders : & [] , regex : "^No matching user base found for user \\{dn\\}, available: \\{bases\\}\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/user_ldap/lib/LDAPProvider.php" , line : 214usize , placeholders : & [] , regex : "^User id not found in LDAP$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/user_ldap/lib/LDAPProvider.php" , line : 227usize , placeholders : & [] , regex : "^User id not found in LDAP$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/user_ldap/lib/LDAPProvider.php" , line : 240usize , placeholders : & [] , regex : "^Group id not found in LDAP$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/user_ldap/lib/LDAPProvider.php" , line : 279usize , placeholders : & [] , regex : "^User id not found in LDAP$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/user_ldap/lib/LDAPProvider.php" , line : 292usize , placeholders : & [] , regex : "^User id not found in LDAP$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/user_ldap/lib/LDAPProvider.php" , line : 305usize , placeholders : & [] , regex : "^Group id not found in LDAP$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/user_ldap/lib/LDAPProvider.php" , line : 330usize , placeholders : & [] , regex : "^User id not found in LDAP$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/user_ldap/lib/Configuration.php" , line : 580usize , placeholders : & [] , regex : "^Invalid rule$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Warn , path : "/apps/user_ldap/lib/Configuration.php" , line : 598usize , placeholders : & [] , regex : "^Invalid config value to ldapUserAvatarRule; falling back to default\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/user_ldap/lib/PagedResults/TLinkId.php" , line : 42usize , placeholders : & [] , regex : "^No resource provided$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/user_ldap/lib/Controller/ConfigAPIController.php" , line : 125usize , placeholders : & [] , regex : "^An issue occurred when creating the new config\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/user_ldap/lib/Controller/ConfigAPIController.php" , line : 157usize , placeholders : & [] , regex : "^Could not delete configuration$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/user_ldap/lib/Controller/ConfigAPIController.php" , line : 163usize , placeholders : & [] , regex : "^An issue occurred when deleting the config\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/user_ldap/lib/Controller/ConfigAPIController.php" , line : 198usize , placeholders : & [] , regex : "^configData is not properly set$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/user_ldap/lib/Controller/ConfigAPIController.php" , line : 216usize , placeholders : & [] , regex : "^An issue occurred when modifying the config\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/user_ldap/lib/Controller/ConfigAPIController.php" , line : 312usize , placeholders : & [] , regex : "^An issue occurred when modifying the config\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/user_ldap/lib/Controller/ConfigAPIController.php" , line : 328usize , placeholders : & [] , regex : "^Config ID not found$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Warn , path : "/apps/user_ldap/lib/User_LDAP.php" , line : 656usize , placeholders : & [] , regex : "^Failed to map created LDAP user with userid \\{userid\\}, because UUID could not be determined$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/user_ldap/lib/User_LDAP.php" , line : 665usize , placeholders : & [] , regex : "^LDAP Plugin: Method createUser changed to return the user DN instead of boolean\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Debug , path : "/apps/user_ldap/lib/LDAP.php" , line : 329usize , placeholders : & [] , regex : "^Calling LDAP function \\{func\\} with parameters \\{args\\}$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Debug , path : "/apps/user_ldap/lib/LDAP.php" , line : 369usize , placeholders : & [] , regex : "^LDAP error \\{message\\} \\(\\{code\\}\\) after calling \\{func\\}$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/user_ldap/lib/LDAP.php" , line : 382usize , placeholders : & [] , regex : "^Lost connection to LDAP server\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/user_ldap/lib/LDAP.php" , line : 384usize , placeholders : & [] , regex : "^LDAP server is shutting down\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/user_ldap/lib/LDAP.php" , line : 386usize , placeholders : & [] , regex : "^LDAP authentication method rejected$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/user_ldap/lib/LDAP.php" , line : 388usize , placeholders : & [] , regex : "^LDAP Operations error$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/user_ldap/lib/Connection.php" , line : 257usize , placeholders : & [] , regex : "^Connection to LDAP server could not be established$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Info , path : "/apps/user_ldap/lib/Connection.php" , line : 456usize , placeholders : & [] , regex : "^LDAPS \\(already using secure connection\\) and TLS do not work together\\. Switched off TLS\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Warn , path : "/apps/user_ldap/lib/Connection.php" , line : 586usize , placeholders : & [] , regex : "^Configuration is invalid, cannot connect$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Error , path : "/apps/user_ldap/lib/Connection.php" , line : 595usize , placeholders : & [] , regex : "^function ldap_connect is not available\\. Make sure that the PHP ldap module is installed\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Debug , path : "/apps/user_ldap/lib/Connection.php" , line : 604usize , placeholders : & [] , regex : "^Turned off SSL certificate validation successfully\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Warn , path : "/apps/user_ldap/lib/Connection.php" , line : 609usize , placeholders : & [] , regex : "^Could not turn off SSL certificate validation\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Warn , path : "/apps/user_ldap/lib/Connection.php" , line : 637usize , placeholders : & [] , regex : "^Main LDAP not reachable, connecting to backup$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/user_ldap/lib/Connection.php" , line : 676usize , placeholders : & [] , regex : "^Could not set required LDAP Protocol version\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/user_ldap/lib/Connection.php" , line : 680usize , placeholders : & [] , regex : "^Could not disable LDAP referrals\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/user_ldap/lib/Connection.php" , line : 684usize , placeholders : & [] , regex : "^Could not set network timeout$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Debug , path : "/apps/user_ldap/lib/Group_LDAP.php" , line : 243usize , placeholders : & [] , regex : "^No search filter found on member url of group \\{dn\\}$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/user_ldap/lib/Group_LDAP.php" , line : 471usize , placeholders : & [] , regex : "^Not a valid group$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/user_ldap/lib/Group_LDAP.php" , line : 595usize , placeholders : & [] , regex : "^Not a valid group$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Debug , path : "/apps/user_ldap/lib/Group_LDAP.php" , line : 759usize , placeholders : & [] , regex : "^No search filter found on member url of group \\{dn\\}$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Debug , path : "/apps/user_ldap/lib/Group_LDAP.php" , line : 797usize , placeholders : & [] , regex : "^No uid attribute found for DN \\{dn\\} on \\{host\\}$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/user_ldap/lib/Group_LDAP.php" , line : 1270usize , placeholders : & [] , regex : "^Could not create group in LDAP backend\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/user_ldap/lib/Group_LDAP.php" , line : 1321usize , placeholders : & [] , regex : "^Could not add user to group in LDAP backend\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/user_ldap/lib/Group_LDAP.php" , line : 1340usize , placeholders : & [] , regex : "^Could not remove user from group in LDAP backend\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/user_ldap/lib/Group_LDAP.php" , line : 1354usize , placeholders : & [] , regex : "^Could not get group details in LDAP backend\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/user_ldap/lib/User/Manager.php" , line : 135usize , placeholders : & [] , regex : "^LDAP Access instance must be set first$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Error , path : "/apps/user_ldap/lib/User/User.php" , line : 127usize , placeholders : & ["$dn"] , regex : "^uid for '(.*)' must not be null!$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/user_ldap/lib/User/User.php" , line : 128usize , placeholders : & [] , regex : "^uid must not be null!$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Error , path : "/apps/user_ldap/lib/User/User.php" , line : 130usize , placeholders : & ["$dn"] , regex : "^uid for '(.*)' must not be an empty string$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/user_ldap/lib/User/User.php" , line : 131usize , placeholders : & [] , regex : "^uid must not be an empty string!$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Info , path : "/apps/user_ldap/lib/User/User.php" , line : 338usize , placeholders : & ["$username"] , regex : "^updated profile uid=(.*)$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Debug , path : "/apps/user_ldap/lib/User/User.php" , line : 341usize , placeholders : & [] , regex : "^profile data from LDAP unchanged$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Debug , path : "/apps/user_ldap/lib/User/User.php" , line : 346usize , placeholders : & [] , regex : "^skipping profile check, while cached data exist$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/user_ldap/lib/GroupPluginManager.php" , line : 87usize , placeholders : & [] , regex : "^No plugin implements createGroup in this LDAP Backend\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/user_ldap/lib/GroupPluginManager.php" , line : 117usize , placeholders : & [] , regex : "^No plugin implements deleteGroup in this LDAP Backend\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/user_ldap/lib/GroupPluginManager.php" , line : 135usize , placeholders : & [] , regex : "^No plugin implements addToGroup in this LDAP Backend\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/user_ldap/lib/GroupPluginManager.php" , line : 153usize , placeholders : & [] , regex : "^No plugin implements removeFromGroup in this LDAP Backend\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/user_ldap/lib/GroupPluginManager.php" , line : 169usize , placeholders : & [] , regex : "^No plugin implements countUsersInGroup in this LDAP Backend\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/user_ldap/lib/GroupPluginManager.php" , line : 184usize , placeholders : & [] , regex : "^No plugin implements getGroupDetails in this LDAP Backend\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Error , path : "/apps/user_ldap/lib/Migration/Version1120Date20210917155206.php" , line : 113usize , placeholders : & [] , regex : "^Failed to shorten owncloud_name \"\\{oldId\\}\" to \"\\{newId\\}\" \\(UUID: \"\\{uuid\\}\" of \\{table\\}\\)$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Error , path : "/apps/user_ldap/lib/Migration/Version1130Date20211102154716.php" , line : 167usize , placeholders : & [] , regex : "^Failed to add hash \"\\{dnHash\\}\" \\(\"\\{name\\}\" of \\{table\\}\\)$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Warn , path : "/apps/user_ldap/lib/Migration/Version1130Date20211102154716.php" , line : 230usize , placeholders : & [] , regex : "^LDAP user or group with ID \\{nid\\} has a duplicated UUID value which therefore was invalidated\\. You may double\\-check your LDAP configuration and trigger an update of the UUID\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Debug , path : "/apps/user_ldap/lib/Jobs/UpdateGroups.php" , line : 88usize , placeholders : & [] , regex : "^Run background job \"updateGroups\"$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Info , path : "/apps/user_ldap/lib/Jobs/UpdateGroups.php" , line : 98usize , placeholders : & [] , regex : "^bgJ \"updateGroups\" – groups do not seem to be configured properly, aborting\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Debug , path : "/apps/user_ldap/lib/Jobs/UpdateGroups.php" , line : 109usize , placeholders : & [] , regex : "^bgJ \"updateGroups\" – Finished\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Debug , path : "/apps/user_ldap/lib/Jobs/UpdateGroups.php" , line : 144usize , placeholders : & [] , regex : "^bgJ \"updateGroups\" – Dealing with known Groups\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Error , path : "/apps/user_ldap/lib/Jobs/UpdateGroups.php" , line : 162usize , placeholders : & [] , regex : "^bgJ \"updateGroups\" – Failed to get group \\{group\\} for update$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Info , path : "/apps/user_ldap/lib/Jobs/UpdateGroups.php" , line : 176usize , placeholders : & [] , regex : "^bgJ \"updateGroups\" – \\{user\\} removed from \\{group\\}$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Info , path : "/apps/user_ldap/lib/Jobs/UpdateGroups.php" , line : 191usize , placeholders : & [] , regex : "^bgJ \"updateGroups\" – \\{user\\} added to \\{group\\}$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Debug , path : "/apps/user_ldap/lib/Jobs/UpdateGroups.php" , line : 209usize , placeholders : & [] , regex : "^bgJ \"updateGroups\" – FINISHED dealing with known Groups\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Debug , path : "/apps/user_ldap/lib/Jobs/UpdateGroups.php" , line : 220usize , placeholders : & [] , regex : "^bgJ \"updateGroups\" – dealing with created Groups\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Debug , path : "/apps/user_ldap/lib/Jobs/UpdateGroups.php" , line : 241usize , placeholders : & [] , regex : "^bgJ \"updateGroups\" – FINISHED dealing with created Groups\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Debug , path : "/apps/user_ldap/lib/Jobs/UpdateGroups.php" , line : 252usize , placeholders : & [] , regex : "^bgJ \"updateGroups\" – dealing with removed groups\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Info , path : "/apps/user_ldap/lib/Jobs/UpdateGroups.php" , line : 262usize , placeholders : & [] , regex : "^bgJ \"updateGroups\" – groups \\{removedGroups\\} were removed\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Debug , path : "/apps/user_ldap/lib/Jobs/UpdateGroups.php" , line : 273usize , placeholders : & [] , regex : "^bgJ \"updateGroups\" – FINISHED dealing with removed groups\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/photos/vendor/hexogen/kdtree/src/ItemList.php" , line : 25usize , placeholders : & [] , regex : "^\\$dimensions should be bigger than 0$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/photos/vendor/hexogen/kdtree/src/Point.php" , line : 50usize , placeholders : & [] , regex : "^\\$dValues should be not empty$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/photos/vendor/hexogen/kdtree/src/Point.php" , line : 55usize , placeholders : & [] , regex : "^\\$dValues is not a simple array list$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/photos/vendor/composer/ClassLoader.php" , line : 250usize , placeholders : & [] , regex : "^A non\\-empty PSR\\-4 prefix must end with a namespace separator\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/photos/vendor/composer/ClassLoader.php" , line : 305usize , placeholders : & [] , regex : "^A non\\-empty PSR\\-4 prefix must end with a namespace separator\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/photos/lib/Command/MapMediaToPlaceCommand.php" , line : 61usize , placeholders : & [] , regex : "^File metadata is not enabled\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/photos/lib/Sabre/PhotosHome.php" , line : 70usize , placeholders : & [] , regex : "^Permission denied to rename this folder$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/photos/lib/Sabre/PhotosHome.php" , line : 74usize , placeholders : & [] , regex : "^Not allowed to create files in this folder$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/photos/lib/Sabre/PhotosHome.php" , line : 81usize , placeholders : & [] , regex : "^Permission denied to create folders in this folder$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/photos/lib/Sabre/Album/PublicAlbumRoot.php" , line : 36usize , placeholders : & [] , regex : "^Not allowed to delete a public album$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/photos/lib/Sabre/Album/PublicAlbumRoot.php" , line : 43usize , placeholders : & [] , regex : "^Not allowed to rename a public album$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/photos/lib/Sabre/Album/PublicAlbumRoot.php" , line : 47usize , placeholders : & [] , regex : "^Not allowed to copy into a public album$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/photos/lib/Sabre/Album/PublicAlbumRoot.php" , line : 58usize , placeholders : & [] , regex : "^Not allowed to create a file in a public album$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/photos/lib/Sabre/Album/PublicAlbumRoot.php" , line : 62usize , placeholders : & [] , regex : "^Not allowed to add a file to a public album$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/photos/lib/Sabre/Album/PublicAlbumRoot.php" , line : 72usize , placeholders : & [] , regex : "^Not allowed to collaborators a public album$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/photos/lib/Sabre/Album/PublicAlbumRoot.php" , line : 89usize , placeholders : & ["$name"] , regex : "^(.*) not found$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/photos/lib/Sabre/Album/AlbumPhoto.php" , line : 63usize , placeholders : & [] , regex : "^Photo not found for user$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/photos/lib/Sabre/Album/AlbumPhoto.php" , line : 72usize , placeholders : & [] , regex : "^Photo is a folder$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/photos/lib/Sabre/Album/SharedAlbumRoot.php" , line : 68usize , placeholders : & [] , regex : "^Not allowed to rename a shared album$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/photos/lib/Sabre/Album/SharedAlbumRoot.php" , line : 73usize , placeholders : & ["$sourceId"] , regex : "^File (.*) is already in the folder$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/photos/lib/Sabre/Album/SharedAlbumRoot.php" , line : 98usize , placeholders : & [] , regex : "^Not allowed to collaborators to a public album$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/photos/lib/Sabre/Album/PublicAlbumPhoto.php" , line : 32usize , placeholders : & [] , regex : "^Deleting photos from a public album is not allowed\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/photos/lib/Sabre/Album/PublicAlbumPhoto.php" , line : 37usize , placeholders : & [] , regex : "^Changing a photo from a public album is not allowed\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/photos/lib/Sabre/Album/AlbumsHome.php" , line : 78usize , placeholders : & [] , regex : "^Permission denied to rename this folder$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/photos/lib/Sabre/Album/AlbumsHome.php" , line : 82usize , placeholders : & [] , regex : "^Not allowed to create files in this folder$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/photos/lib/Sabre/Album/AlbumRoot.php" , line : 106usize , placeholders : & [] , regex : "^The destination exists and is not a folder$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/photos/lib/Sabre/Album/AlbumRoot.php" , line : 119usize , placeholders : & [] , regex : "^Could not create file$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/photos/lib/Sabre/Album/AlbumRoot.php" , line : 127usize , placeholders : & [] , regex : "^Not allowed to create directories in this folder$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/photos/lib/Sabre/Album/AlbumRoot.php" , line : 142usize , placeholders : & ["$name"] , regex : "^(.*) not found$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/photos/lib/Sabre/Album/AlbumRoot.php" , line : 160usize , placeholders : & [] , regex : "^The source is not a file$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/photos/lib/Sabre/Album/AlbumRoot.php" , line : 167usize , placeholders : & ["$uid"] , regex : "^Can't add file to album, only files from (.*) can be added$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/photos/lib/Sabre/Album/AlbumRoot.php" , line : 175usize , placeholders : & ["$sourceId"] , regex : "^File (.*) is already in the folder$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/photos/lib/Sabre/Album/SharedAlbumsHome.php" , line : 65usize , placeholders : & [] , regex : "^Not allowed to create folders in this folder$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/photos/lib/Sabre/PublicRootCollection.php" , line : 94usize , placeholders : & [] , regex : "^Unable to find public album$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/photos/lib/Sabre/Place/PlacePhoto.php" , line : 53usize , placeholders : & [] , regex : "^Cannot remove from a place$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/photos/lib/Sabre/Place/PlacePhoto.php" , line : 66usize , placeholders : & [] , regex : "^Photo not found for user$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/photos/lib/Sabre/Place/PlacePhoto.php" , line : 76usize , placeholders : & [] , regex : "^Photo is a folder$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/photos/lib/Sabre/Place/PlaceRoot.php" , line : 55usize , placeholders : & [] , regex : "^Not allowed to delete a place collection$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/photos/lib/Sabre/Place/PlaceRoot.php" , line : 66usize , placeholders : & [] , regex : "^Cannot change the place collection name$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/photos/lib/Sabre/Place/PlaceRoot.php" , line : 75usize , placeholders : & [] , regex : "^Cannot create a file in a place collection$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/photos/lib/Sabre/Place/PlaceRoot.php" , line : 82usize , placeholders : & [] , regex : "^Not allowed to create directories in this folder$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/photos/lib/Sabre/Place/PlaceRoot.php" , line : 105usize , placeholders : & ["$name"] , regex : "^File (.*) not found$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/photos/lib/Sabre/Place/PlaceRoot.php" , line : 125usize , placeholders : & [] , regex : "^No children found for place$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/photos/lib/Sabre/Place/PlacesHome.php" , line : 68usize , placeholders : & [] , regex : "^Permission denied to rename this folder$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/photos/lib/Sabre/Place/PlacesHome.php" , line : 72usize , placeholders : & [] , regex : "^Not allowed to create files in this folder$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/photos/lib/Sabre/Place/PlacesHome.php" , line : 76usize , placeholders : & [] , regex : "^Not allowed to create folder in this folder$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/photos/lib/Sabre/Place/PlacesHome.php" , line : 84usize , placeholders : & ["$name"] , regex : "^Place (.*) does not exist$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/photos/lib/Sabre/CollectionPhoto.php" , line : 49usize , placeholders : & [] , regex : "^Can't rename photos trough this api$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/photos/lib/Sabre/CollectionPhoto.php" , line : 64usize , placeholders : & [] , regex : "^Photo is a folder$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/photos/lib/Sabre/CollectionPhoto.php" , line : 67usize , placeholders : & [] , regex : "^Photo not found for user$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/photos/lib/Album/AlbumMapper.php" , line : 250usize , placeholders : & [] , regex : "^File already in album$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/photos/lib/Service/ReverseGeoCoderService.php" , line : 92usize , placeholders : & ["$res"] , regex : "^Fail to unzip place file: (.*)$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/photos/lib/Service/ReverseGeoCoderService.php" , line : 111usize , placeholders : & [] , regex : "^Failed to write csv line to tmp stream$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/photos/lib/Service/UserConfigService.php" , line : 57usize , placeholders : & [] , regex : "^Unknown user config key$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/systemtags/composer/composer/ClassLoader.php" , line : 256usize , placeholders : & [] , regex : "^A non\\-empty PSR\\-4 prefix must end with a namespace separator\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/systemtags/composer/composer/ClassLoader.php" , line : 311usize , placeholders : & [] , regex : "^A non\\-empty PSR\\-4 prefix must end with a namespace separator\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/user_status/composer/composer/ClassLoader.php" , line : 256usize , placeholders : & [] , regex : "^A non\\-empty PSR\\-4 prefix must end with a namespace separator\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/user_status/composer/composer/ClassLoader.php" , line : 311usize , placeholders : & [] , regex : "^A non\\-empty PSR\\-4 prefix must end with a namespace separator\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/user_status/lib/Service/StatusService.php" , line : 233usize , placeholders : & [] , regex : "^ClearAt is in the past$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/user_status/lib/Service/StatusService.php" , line : 328usize , placeholders : & [] , regex : "^Status\\-Icon is longer than one character$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/user_status/lib/Service/StatusService.php" , line : 336usize , placeholders : & [] , regex : "^ClearAt is in the past$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/user_status/lib/Controller/StatusesController.php" , line : 83usize , placeholders : & [] , regex : "^No status for the requested userId$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/user_status/lib/Controller/UserStatusController.php" , line : 86usize , placeholders : & [] , regex : "^No status for the current user$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/files_trashbin/lib/UserMigration/TrashbinMigrator.php" , line : 134usize , placeholders : & [] , regex : "^Could not import trashbin\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Error , path : "/apps/files_trashbin/lib/Trashbin.php" , line : 225usize , placeholders : & [] , regex : "^trash bin database couldn't be updated for the files owner$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Error , path : "/apps/files_trashbin/lib/Trashbin.php" , line : 355usize , placeholders : & [] , regex : "^trash bin database couldn't be updated$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/files_trashbin/lib/Trashbin.php" , line : 508usize , placeholders : & [] , regex : "^Can't restore trash item because the target folder is not writable$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/files_trashbin/lib/Sabre/TrashRoot.php" , line : 63usize , placeholders : & [] , regex : "^Permission denied to rename this trashbin$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/files_trashbin/lib/Sabre/TrashRoot.php" , line : 67usize , placeholders : & [] , regex : "^Not allowed to create files in the trashbin$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/files_trashbin/lib/Sabre/TrashRoot.php" , line : 71usize , placeholders : & [] , regex : "^Not allowed to create folders in the trashbin$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/files_trashbin/lib/Sabre/TrashHome.php" , line : 65usize , placeholders : & [] , regex : "^Permission denied to rename this trashbin$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/files_trashbin/lib/Sabre/TrashHome.php" , line : 69usize , placeholders : & [] , regex : "^Not allowed to create files in the trashbin$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/files_trashbin/lib/Sabre/TrashHome.php" , line : 73usize , placeholders : & [] , regex : "^Not allowed to create folders in the trashbin$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/files_trashbin/lib/Command/CleanUp.php" , line : 83usize , placeholders : & [] , regex : "^Either specify a user_id or \\-\\-all\\-users$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/files_trashbin/lib/Command/CleanUp.php" , line : 114usize , placeholders : & [] , regex : "^Either specify a user_id or \\-\\-all\\-users$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/files_trashbin/lib/Command/RestoreAllFiles.php" , line : 124usize , placeholders : & [] , regex : "^Either specify a user_id or \\-\\-all\\-users$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/files_trashbin/lib/Command/RestoreAllFiles.php" , line : 154usize , placeholders : & [] , regex : "^Either specify a user_id or \\-\\-all\\-users$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/files_trashbin/lib/Command/RestoreAllFiles.php" , line : 226usize , placeholders : & [] , regex : "^since must be before until$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/files_trashbin/lib/Command/RestoreAllFiles.php" , line : 242usize , placeholders : & ["$scope"] , regex : "^Invalid scope '(.*)'$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/files_trashbin/lib/Command/RestoreAllFiles.php" , line : 251usize , placeholders : & ["$timestamp"] , regex : "^Invalid timestamp '(.*)'$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/files_trashbin/lib/Helper.php" , line : 55usize , placeholders : & [] , regex : "^Directory does not exists$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/files_trashbin/lib/Trash/TrashManager.php" , line : 89usize , placeholders : & ["$fullType"] , regex : "^Trash backend for (.*) not found$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/files_trashbin/composer/composer/ClassLoader.php" , line : 256usize , placeholders : & [] , regex : "^A non\\-empty PSR\\-4 prefix must end with a namespace separator\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/files_trashbin/composer/composer/ClassLoader.php" , line : 311usize , placeholders : & [] , regex : "^A non\\-empty PSR\\-4 prefix must end with a namespace separator\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/comments/lib/Notification/Notifier.php" , line : 95usize , placeholders : & [] , regex : "^Comment not found$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/comments/lib/Notification/Notifier.php" , line : 111usize , placeholders : & [] , regex : "^Unsupported comment object$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/comments/lib/Notification/Notifier.php" , line : 159usize , placeholders : & [] , regex : "^Invalid subject$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/comments/lib/Search/Result.php" , line : 94usize , placeholders : & [] , regex : "^Path not inside visible section$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/comments/lib/Search/Result.php" , line : 109usize , placeholders : & [] , regex : "^Comment section not found$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/comments/lib/Search/LegacyProvider.php" , line : 108usize , placeholders : & [] , regex : "^File not found$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/comments/composer/composer/ClassLoader.php" , line : 256usize , placeholders : & [] , regex : "^A non\\-empty PSR\\-4 prefix must end with a namespace separator\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/comments/composer/composer/ClassLoader.php" , line : 311usize , placeholders : & [] , regex : "^A non\\-empty PSR\\-4 prefix must end with a namespace separator\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/activity/lib/ViewInfoCache.php" , line : 80usize , placeholders : & [] , regex : "^No entries returned$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/activity/lib/ViewInfoCache.php" , line : 97usize , placeholders : & [] , regex : "^No entries returned$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/activity/lib/Controller/APIv2Controller.php" , line : 134usize , placeholders : & [] , regex : "^Invalid filter$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/activity/lib/Controller/APIv2Controller.php" , line : 154usize , placeholders : & [] , regex : "^Not logged in$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Debug , path : "/apps/activity/lib/MailQueueHandler.php" , line : 154usize , placeholders : & [] , regex : "^Couldn't send notification email to user '\\{user\\}' \\(email address isn't set for that user\\)$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Warn , path : "/apps/activity/lib/MailQueueHandler.php" , line : 165usize , placeholders : & [] , regex : "^Failed sending activity email to user '\\{user\\}'\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Warn , path : "/apps/activity/lib/MailQueueHandler.php" , line : 320usize , placeholders : & [] , regex : "^Notification for user \"\\{user\\}\" not sent because the email address \"\\{email\\}\" is invalid\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Error , path : "/apps/activity/lib/GroupHelper.php" , line : 95usize , placeholders : & [] , regex : "^Error while parsing activity event$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Error , path : "/apps/activity/lib/DigestSender.php" , line : 115usize , placeholders : & [] , regex : "^Exception occurred while sending user digest email$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/activity/lib/Data.php" , line : 188usize , placeholders : & [] , regex : "^Invalid user$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/activity/lib/Data.php" , line : 294usize , placeholders : & [] , regex : "^Invalid since$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Debug , path : "/apps/activity/lib/Data.php" , line : 379usize , placeholders : & [] , regex : "^Choosing chunked activity delete for MySQL/MariaDB$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Debug , path : "/apps/activity/lib/Data.php" , line : 383usize , placeholders : & [] , regex : "^Choosing regular activity delete$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/activity/lib/AppInfo/Application.php" , line : 80usize , placeholders : & [] , regex : "^Invalid database type$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/workflowengine/lib/Manager.php" , line : 419usize , placeholders : & [] , regex : "^Target operation not within scope$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/workflowengine/lib/Manager.php" , line : 460usize , placeholders : & [] , regex : "^Target operation not within scope$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/workflowengine/lib/Controller/UserWorkflowsController.php" , line : 119usize , placeholders : & [] , regex : "^User not logged in$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Error , path : "/apps/workflowengine/lib/Controller/AWorkflowController.php" , line : 126usize , placeholders : & [] , regex : "^Error when inserting flow$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/workflowengine/lib/Controller/AWorkflowController.php" , line : 127usize , placeholders : & [] , regex : "^An internal error occurred$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/workflowengine/lib/Controller/AWorkflowController.php" , line : 156usize , placeholders : & [] , regex : "^An internal error occurred$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/workflowengine/lib/Controller/AWorkflowController.php" , line : 176usize , placeholders : & [] , regex : "^An internal error occurred$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/workflowengine/lib/Service/RuleMatcher.php" , line : 98usize , placeholders : & [] , regex : "^This method must not be called more than once$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/workflowengine/lib/Service/RuleMatcher.php" , line : 105usize , placeholders : & [] , regex : "^This method must not be called more than once$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/workflowengine/lib/Service/RuleMatcher.php" , line : 112usize , placeholders : & [] , regex : "^This method must not be called more than once$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/workflowengine/lib/Service/RuleMatcher.php" , line : 119usize , placeholders : & [] , regex : "^Entity was not set yet$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/workflowengine/lib/Service/RuleMatcher.php" , line : 126usize , placeholders : & [] , regex : "^Operation is not set$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/workflowengine/lib/Service/RuleMatcher.php" , line : 233usize , placeholders : & [] , regex : "^Must set file info before running the check$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/workflowengine/lib/Helper/ScopeContext.php" , line : 48usize , placeholders : & [] , regex : "^Invalid scope$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/workflowengine/lib/Helper/ScopeContext.php" , line : 54usize , placeholders : & [] , regex : "^user scope requires a user id$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Debug , path : "/apps/workflowengine/lib/AppInfo/Application.php" , line : 106usize , placeholders : & [] , regex : "^Cannot handle event \\{name\\} of \\{event\\} against entity \\{entity\\} and operation \\{operation\\}$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/workflowengine/lib/Check/TFileCheck.php" , line : 64usize , placeholders : & [] , regex : "^Expected Node subject for File entity, got \\{class\\}$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/workflowengine/lib/Check/FileSystemTags.php" , line : 166usize , placeholders : & [] , regex : "^Should not happen: Storage is instance of GroupFolderStorage but no group folder storage found while unwrapping\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/workflowengine/composer/composer/ClassLoader.php" , line : 256usize , placeholders : & [] , regex : "^A non\\-empty PSR\\-4 prefix must end with a namespace separator\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/workflowengine/composer/composer/ClassLoader.php" , line : 311usize , placeholders : & [] , regex : "^A non\\-empty PSR\\-4 prefix must end with a namespace separator\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/notifications/lib/FakeUser.php" , line : 42usize , placeholders : & [] , regex : "^Not implemented$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/notifications/lib/FakeUser.php" , line : 46usize , placeholders : & [] , regex : "^Not implemented$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/notifications/lib/FakeUser.php" , line : 50usize , placeholders : & [] , regex : "^Not implemented$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/notifications/lib/FakeUser.php" , line : 54usize , placeholders : & [] , regex : "^Not implemented$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/notifications/lib/FakeUser.php" , line : 58usize , placeholders : & [] , regex : "^Not implemented$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/notifications/lib/FakeUser.php" , line : 62usize , placeholders : & [] , regex : "^Not implemented$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/notifications/lib/FakeUser.php" , line : 66usize , placeholders : & [] , regex : "^Not implemented$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/notifications/lib/FakeUser.php" , line : 70usize , placeholders : & [] , regex : "^Not implemented$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/notifications/lib/FakeUser.php" , line : 74usize , placeholders : & [] , regex : "^Not implemented$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/notifications/lib/FakeUser.php" , line : 78usize , placeholders : & [] , regex : "^Not implemented$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/notifications/lib/FakeUser.php" , line : 82usize , placeholders : & [] , regex : "^Not implemented$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/notifications/lib/FakeUser.php" , line : 86usize , placeholders : & [] , regex : "^Not implemented$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/notifications/lib/FakeUser.php" , line : 90usize , placeholders : & [] , regex : "^Not implemented$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/notifications/lib/FakeUser.php" , line : 94usize , placeholders : & [] , regex : "^Not implemented$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/notifications/lib/FakeUser.php" , line : 98usize , placeholders : & [] , regex : "^Not implemented$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/notifications/lib/FakeUser.php" , line : 102usize , placeholders : & [] , regex : "^Not implemented$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/notifications/lib/FakeUser.php" , line : 106usize , placeholders : & [] , regex : "^Not implemented$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/notifications/lib/FakeUser.php" , line : 110usize , placeholders : & [] , regex : "^Not implemented$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/notifications/lib/FakeUser.php" , line : 114usize , placeholders : & [] , regex : "^Not implemented$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/notifications/lib/FakeUser.php" , line : 118usize , placeholders : & [] , regex : "^Not implemented$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/notifications/lib/FakeUser.php" , line : 122usize , placeholders : & [] , regex : "^Not implemented$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/notifications/lib/FakeUser.php" , line : 126usize , placeholders : & [] , regex : "^Not implemented$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/notifications/lib/FakeUser.php" , line : 130usize , placeholders : & [] , regex : "^Not implemented$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/notifications/lib/FakeUser.php" , line : 134usize , placeholders : & [] , regex : "^Not implemented$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/notifications/lib/FakeUser.php" , line : 138usize , placeholders : & [] , regex : "^Not implemented$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/notifications/lib/FakeUser.php" , line : 142usize , placeholders : & [] , regex : "^Not implemented$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/notifications/lib/Notifier/AdminNotifications.php" , line : 90usize , placeholders : & [] , regex : "^Unknown app$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/notifications/lib/Notifier/AdminNotifications.php" , line : 219usize , placeholders : & [] , regex : "^Unknown subject$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Debug , path : "/apps/notifications/lib/Push.php" , line : 512usize , placeholders : & [] , regex : "^Could not send notification to push server \\[\\{url\\}\\]: \\{error\\}$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Warn , path : "/apps/notifications/lib/Push.php" , line : 547usize , placeholders : & [] , regex : "^Could not send notification to push server \\[\\{url\\}\\]: \\{error\\}$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Info , path : "/apps/notifications/lib/Push.php" , line : 555usize , placeholders : & [] , regex : "^Push notification sent but response was not parsable, using an outdated push proxy\\? \\[\\{url\\}\\]: \\{error\\}$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/notifications/lib/Push.php" , line : 635usize , placeholders : & [] , regex : "^Failed to encrypt message for device$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/notifications/lib/Push.php" , line : 686usize , placeholders : & [] , regex : "^Failed to encrypt message for device$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/notifications/lib/App.php" , line : 60usize , placeholders : & [] , regex : "^Error while preparing push notification$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/notifications/lib/Handler.php" , line : 212usize , placeholders : & [] , regex : "^No entry returned from database$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/notifications/lib/Handler.php" , line : 218usize , placeholders : & [] , regex : "^Could not create notification from database row$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/files_versions/composer/composer/ClassLoader.php" , line : 256usize , placeholders : & [] , regex : "^A non\\-empty PSR\\-4 prefix must end with a namespace separator\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/files_versions/composer/composer/ClassLoader.php" , line : 311usize , placeholders : & [] , regex : "^A non\\-empty PSR\\-4 prefix must end with a namespace separator\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/files_versions/lib/Versions/VersionManager.php" , line : 85usize , placeholders : & ["$fullType"] , regex : "^Version backend for (.*) not found$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/files_versions/lib/Versions/LegacyVersionsBackend.php" , line : 83usize , placeholders : & ["$fileId"] , regex : "^File not found \\((.*)\\)$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/files_versions/lib/Versions/LegacyVersionsBackend.php" , line : 87usize , placeholders : & ["$owner" , "$fileId"] , regex : "^User (.*) not found for (.*)$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/files_versions/lib/Versions/LegacyVersionsBackend.php" , line : 96usize , placeholders : & [] , regex : "^version file not found for share owner$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/files_versions/lib/Versions/LegacyVersionsBackend.php" , line : 104usize , placeholders : & ["$fileId"] , regex : "^File not found \\((.*)\\)$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/files_versions/lib/Versions/LegacyVersionsBackend.php" , line : 190usize , placeholders : & [] , regex : "^You cannot restore this version because you do not have update permissions on the source file\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/files_versions/lib/Versions/LegacyVersionsBackend.php" , line : 240usize , placeholders : & [] , regex : "^You cannot label this version because you do not have update permissions on the source file\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/files_versions/lib/Versions/LegacyVersionsBackend.php" , line : 256usize , placeholders : & [] , regex : "^You cannot delete this version because you do not have delete permissions on the source file\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/files_versions/lib/Versions/LegacyVersionsBackend.php" , line : 304usize , placeholders : & [] , regex : "^No user logged in$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/files_versions/lib/Versions/LegacyVersionsBackend.php" , line : 311usize , placeholders : & [] , regex : "^Version file not accessible by current user$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Error , path : "/apps/files_versions/lib/Storage.php" , line : 508usize , placeholders : & [] , regex : "^Version file \\{path\\} has incorrect name format$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/related_resources/vendor/composer/ClassLoader.php" , line : 250usize , placeholders : & [] , regex : "^A non\\-empty PSR\\-4 prefix must end with a namespace separator\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/related_resources/vendor/composer/ClassLoader.php" , line : 305usize , placeholders : & [] , regex : "^A non\\-empty PSR\\-4 prefix must end with a namespace separator\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Info , path : "/apps/related_resources/lib/RelatedResourceProviders/TalkRelatedResourceProvider.php" , line : 119usize , placeholders : & [] , regex : "^session restarted$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/related_resources/lib/Command/Test.php" , line : 118usize , placeholders : & [] , regex : "^must specify a valid local user$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/related_resources/lib/Command/Test.php" , line : 127usize , placeholders : & [] , regex : "^Circles needs to be enabled$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Info , path : "/apps/related_resources/lib/Controller/ApiController.php" , line : 93usize , placeholders : & [] , regex : "^RelatedResources require Circles$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Debug , path : "/apps/related_resources/lib/Service/RelatedService.php" , line : 600usize , placeholders : & [] , regex : "^flush cache$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/circles/vendor/composer/ClassLoader.php" , line : 250usize , placeholders : & [] , regex : "^A non\\-empty PSR\\-4 prefix must end with a namespace separator\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/circles/vendor/composer/ClassLoader.php" , line : 305usize , placeholders : & [] , regex : "^A non\\-empty PSR\\-4 prefix must end with a namespace separator\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/circles/lib/Model/GlobalScale/GSEvent.php" , line : 407usize , placeholders : & [] , regex : "^invalid JSON$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/circles/lib/Model/GlobalScale/GSEvent.php" , line : 439usize , placeholders : & [] , regex : "^invalid GSEvent$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/circles/lib/Model/Circle.php" , line : 635usize , placeholders : & [] , regex : "^circle has no owner$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/circles/lib/Model/FederatedLink.php" , line : 310usize , placeholders : & [] , regex : "^The status could not be updated$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/circles/lib/Model/Mount.php" , line : 432usize , placeholders : & [] , regex : "^ShareWrapper has no Circle$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/circles/lib/Model/Federated/RemoteInstance.php" , line : 471usize , placeholders : & [] , regex : "^identity not authed$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/circles/lib/Model/Helpers/MemberHelper.php" , line : 91usize , placeholders : & [] , regex : "^unknown method call$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/circles/lib/Model/Helpers/MemberHelper.php" , line : 140usize , placeholders : & [] , regex : "^Insufficient rights$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/circles/lib/Model/Helpers/MemberHelper.php" , line : 152usize , placeholders : & [] , regex : "^Insufficient rights$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/circles/lib/Model/Helpers/MemberHelper.php" , line : 164usize , placeholders : & [] , regex : "^Insufficient rights$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/circles/lib/Model/DeprecatedMember.php" , line : 42usize , placeholders : & [] , regex : "^Invalid circle type$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/circles/lib/Db/RemoteRequestBuilder.php" , line : 106usize , placeholders : & [] , regex : "^Unknown remote instance$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/circles/lib/Db/MountRequestBuilder.php" , line : 101usize , placeholders : & [] , regex : "^Mount not found$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/circles/lib/Db/CircleRequest.php" , line : 398usize , placeholders : & [] , regex : "^singleId not found$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/circles/lib/Db/TokensRequest.php" , line : 56usize , placeholders : & [] , regex : "^Unknown share token$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/circles/lib/Db/TokensRequest.php" , line : 81usize , placeholders : & [] , regex : "^Unknown share token$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/circles/lib/Db/CircleRequestBuilder.php" , line : 112usize , placeholders : & [] , regex : "^Circle not found$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/circles/lib/Command/CirclesEdit.php" , line : 151usize , placeholders : & [] , regex : "^edit can only be 'displayName' or 'description'$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/circles/lib/Command/CirclesCheck.php" , line : 177usize , placeholders : & [] , regex : "^Please specify a \\-\\-type for the test$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/circles/lib/Command/CirclesCheck.php" , line : 264usize , placeholders : & [] , regex : "^Your Circles App is not fully configured\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/circles/lib/Command/SharesFiles.php" , line : 317usize , placeholders : & [] , regex : "^Specify a FileId or an option: \\-\\-with \\(USER\\), \\-\\-by \\(USER\\), \\-\\-to \\(CIRCLE\\)$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/circles/lib/Command/CirclesReport.php" , line : 136usize , placeholders : & [] , regex : "^not available$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/circles/lib/Command/CirclesSetting.php" , line : 158usize , placeholders : & [] , regex : "^you need to specify a value$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/circles/lib/Command/CirclesDetails.php" , line : 174usize , placeholders : & [] , regex : "^unknown circle, use \\-\\-instance to retrieve the data from a remote instance$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/circles/lib/Command/CirclesMemberships.php" , line : 203usize , placeholders : & [] , regex : "^Not enough arguments \\(missing: \"userId\"\\)\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/circles/lib/Command/CirclesTest.php" , line : 431usize , placeholders : & [] , regex : "^local not defined$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/circles/lib/Command/CirclesTest.php" , line : 534usize , placeholders : & [] , regex : "^no$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/circles/lib/Command/CirclesTest.php" , line : 715usize , placeholders : & [] , regex : "^could not parse circle\\.name$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/circles/lib/Command/CirclesTest.php" , line : 751usize , placeholders : & [] , regex : "^empty id or owner\\.member_id$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/circles/lib/Command/CirclesTest.php" , line : 973usize , placeholders : & [] , regex : "^empty owner$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/circles/lib/Command/CirclesTest.php" , line : 985usize , placeholders : & [] , regex : "^empty initiator$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/circles/lib/Controller/AdminController.php" , line : 201usize , placeholders : & [] , regex : "^works only from local instance$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/circles/lib/Controller/LocalController.php" , line : 243usize , placeholders : & [] , regex : "^works only from local instance$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/circles/lib/Controller/LocalController.php" , line : 592usize , placeholders : & [] , regex : "^frontend disabled$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/circles/lib/Controller/RemoteController.php" , line : 367usize , placeholders : & [] , regex : "^Entity not found$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Debug , path : "/apps/circles/lib/Controller/RemoteController.php" , line : 504usize , placeholders : & [] , regex : "^Signatory is not a known RemoteInstance$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/circles/lib/Controller/RemoteController.php" , line : 505usize , placeholders : & [] , regex : "^Could not confirm identity$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Debug , path : "/apps/circles/lib/Controller/RemoteController.php" , line : 510usize , placeholders : & [] , regex : "^Could not confirm identity$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/circles/lib/Controller/RemoteController.php" , line : 511usize , placeholders : & [] , regex : "^Could not confirm identity$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/circles/lib/Tools/Traits/TNCLocalSignatory.php" , line : 69usize , placeholders : & [] , regex : "^signatory not found$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Debug , path : "/apps/circles/lib/Tools/Traits/TNCRequest.php" , line : 83usize , placeholders : & [] , regex : "^doRequest initiated$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Debug , path : "/apps/circles/lib/Tools/Traits/TNCRequest.php" , line : 97usize , placeholders : & [] , regex : "^doRequest done$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Debug , path : "/apps/circles/lib/Tools/Traits/TNCSignatory.php" , line : 94usize , placeholders : & [] , regex : "^network issue while downloading Signatory$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Debug , path : "/apps/circles/lib/Tools/Traits/TNCSignatory.php" , line : 120usize , placeholders : & [] , regex : "^invalid format$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/circles/lib/Tools/Traits/TNCSignatory.php" , line : 121usize , placeholders : & [] , regex : "^invalid format$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/circles/lib/Tools/Traits/TNCSignatory.php" , line : 124usize , placeholders : & [] , regex : "^invalid origin$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/circles/lib/Tools/Traits/TNCSignatory.php" , line : 183usize , placeholders : & [] , regex : "^empty private key$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/circles/lib/Tools/Traits/TNCSignatory.php" , line : 204usize , placeholders : & [] , regex : "^signature issue$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Debug , path : "/apps/circles/lib/Tools/ActivityPub/NCSignature.php" , line : 102usize , placeholders : & [] , regex : "^\\[<<\\] incoming$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/circles/lib/Tools/ActivityPub/NCSignature.php" , line : 152usize , placeholders : & [] , regex : "^datetime exception$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/circles/lib/Tools/ActivityPub/NCSignature.php" , line : 156usize , placeholders : & [] , regex : "^object is too old$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/circles/lib/Tools/ActivityPub/NCSignature.php" , line : 170usize , placeholders : & [] , regex : "^issue with content\\-length$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/circles/lib/Tools/ActivityPub/NCSignature.php" , line : 175usize , placeholders : & [] , regex : "^issue with digest$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/circles/lib/Tools/ActivityPub/NCSignature.php" , line : 216usize , placeholders : & [] , regex : "^missing elements in 'headers'$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/circles/lib/Tools/ActivityPub/NCSignature.php" , line : 228usize , placeholders : & [] , regex : "^empty elements in 'headers'$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/circles/lib/Tools/ActivityPub/NCSignature.php" , line : 279usize , placeholders : & [] , regex : "^empty public key$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Debug , path : "/apps/circles/lib/Tools/ActivityPub/NCSignature.php" , line : 290usize , placeholders : & [] , regex : "^signature issue$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/circles/lib/Service/MiscService.php" , line : 109usize , placeholders : & [] , regex : "^missing_key_in_array$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/circles/lib/Service/ContactService.php" , line : 134usize , placeholders : & [] , regex : "^issue with contact format USERID/ADDRESSBOOK/CONTACTID$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/circles/lib/Service/CirclesService.php" , line : 246usize , placeholders : & [] , regex : "^UserID cannot be null$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/circles/lib/Service/CirclesService.php" , line : 535usize , placeholders : & [] , regex : "^This circle already reach its limit on the number of members$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/circles/lib/Service/FederatedEventService.php" , line : 228usize , placeholders : & [] , regex : "^Initiator does not exist$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/circles/lib/Service/FederatedEventService.php" , line : 234usize , placeholders : & [] , regex : "^Initiator is not from the instance at the origin of the request$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/circles/lib/Service/FederatedEventService.php" , line : 240usize , placeholders : & [] , regex : "^Initiator must belong to the instance at the origin of the request$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/circles/lib/Service/FederatedEventService.php" , line : 247usize , placeholders : & [] , regex : "^Initiator must be a member of the Circle$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/circles/lib/Service/FederatedEventService.php" , line : 330usize , placeholders : & [] , regex : "^FederatedEvent has no Circle linked$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/circles/lib/Service/FederatedEventService.php" , line : 337usize , placeholders : & [] , regex : "^FederatedEvent has no Member linked$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/circles/lib/Service/FederatedEventService.php" , line : 350usize , placeholders : & [] , regex : "^FederatedItem must be executed locally$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/circles/lib/Service/FederatedEventService.php" , line : 370usize , placeholders : & [] , regex : "^FederatedItem must contains ItemId$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/circles/lib/Service/FederatedEventService.php" , line : 376usize , placeholders : & [] , regex : "^ShareLock belongs to another instance$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/circles/lib/Service/CircleService.php" , line : 199usize , placeholders : & [] , regex : "^owner not defined$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/circles/lib/Service/CircleService.php" , line : 208usize , placeholders : & [] , regex : "^Circle name is too short$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/circles/lib/Service/GSUpstreamService.php" , line : 390usize , placeholders : & [] , regex : "^result status is not good$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/circles/lib/Service/InterfaceService.php" , line : 125usize , placeholders : & [] , regex : "^interface not initialized$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/circles/lib/Service/InterfaceService.php" , line : 272usize , placeholders : & [] , regex : "^unknown interface$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/circles/lib/Service/InterfaceService.php" , line : 360usize , placeholders : & [] , regex : "^unknown configured interface$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/circles/lib/Service/InterfaceService.php" , line : 409usize , placeholders : & [] , regex : "^misconfigured scheme$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/circles/lib/Service/ShareWrapperService.php" , line : 131usize , placeholders : & [] , regex : "^\\$initiator cannot be empty$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/circles/lib/Service/FederatedUserService.php" , line : 342usize , placeholders : & [] , regex : "^Invalid initiator$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/circles/lib/Service/FederatedUserService.php" , line : 368usize , placeholders : & [] , regex : "^Must initialise Super Session$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/circles/lib/Service/FederatedUserService.php" , line : 638usize , placeholders : & [] , regex : "^This Circle is not managed from this instance, please use \\-\\-initiator$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/circles/lib/Service/FederatedUserService.php" , line : 903usize , placeholders : & [] , regex : "^remote group not supported yet\\. Use singleId$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/circles/lib/Service/FederatedUserService.php" , line : 1019usize , placeholders : & [] , regex : "^FederatedUser must be local$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Debug , path : "/apps/circles/lib/Service/FederatedUserService.php" , line : 1101usize , placeholders : & [] , regex : "^FederatedUser is not empty$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/circles/lib/Service/FederatedUserService.php" , line : 1102usize , placeholders : & [] , regex : "^FederatedUser is not complete$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Debug , path : "/apps/circles/lib/Service/FederatedUserService.php" , line : 1121usize , placeholders : & [] , regex : "^uniqueness of SingleId could not be confirmed$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/circles/lib/Service/FederatedUserService.php" , line : 1125usize , placeholders : & [] , regex : "^uniqueness of SingleId could not be confirmed$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/circles/lib/Service/FederatedUserService.php" , line : 1243usize , placeholders : & [] , regex : "^group not found$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/circles/lib/Service/DavService.php" , line : 526usize , placeholders : & [] , regex : "^Circles needs to be set as Contacts App Backend first$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Unknown , path : "/apps/circles/lib/Service/DavService.php" , line : 565usize , placeholders : & [] , regex : "^Deprecated Contacts managed$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Unknown , path : "/apps/circles/lib/Service/DavService.php" , line : 618usize , placeholders : & [] , regex : "^Deprecated Circles managed$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/circles/lib/Service/RemoteDownstreamService.php" , line : 144usize , placeholders : & [] , regex : "^Circle is not from this instance$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/circles/lib/Service/RemoteDownstreamService.php" , line : 150usize , placeholders : & [] , regex : "^Instance have no members in this Circle$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/circles/lib/Service/RemoteDownstreamService.php" , line : 225usize , placeholders : & [] , regex : "^Could not verify Circle$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Debug , path : "/apps/circles/lib/Service/RemoteDownstreamService.php" , line : 266usize , placeholders : & [] , regex : "^failed to compare Circles$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/circles/lib/Service/RemoteDownstreamService.php" , line : 291usize , placeholders : & [] , regex : "^Could not verify Member$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Debug , path : "/apps/circles/lib/Service/RemoteDownstreamService.php" , line : 301usize , placeholders : & [] , regex : "^verifyMember\\(\\)$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Debug , path : "/apps/circles/lib/Service/RemoteDownstreamService.php" , line : 307usize , placeholders : & [] , regex : "^Member not found$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Debug , path : "/apps/circles/lib/Service/RemoteDownstreamService.php" , line : 313usize , placeholders : & [] , regex : "^failed to compare Members$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Debug , path : "/apps/circles/lib/Service/RemoteDownstreamService.php" , line : 333usize , placeholders : & [] , regex : "^invalid origin$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/circles/lib/Service/RemoteDownstreamService.php" , line : 334usize , placeholders : & [] , regex : "^invalid origin$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/circles/lib/Service/RemoteStreamService.php" , line : 418usize , placeholders : & [] , regex : "^instance is local$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/circles/lib/Service/RemoteStreamService.php" , line : 434usize , placeholders : & [] , regex : "^instance is already known$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/circles/lib/Service/RemoteStreamService.php" , line : 490usize , placeholders : & [] , regex : "^invalid auth\\-signed$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/circles/lib/Service/RemoteStreamService.php" , line : 501usize , placeholders : & [] , regex : "^auth not confirmed$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/circles/lib/Service/MaintenanceService.php" , line : 217usize , placeholders : & [] , regex : "^maintenance already running$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/circles/lib/Service/MigrationService.php" , line : 185usize , placeholders : & [] , regex : "^A migration process is already running$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/circles/lib/Service/RemoteService.php" , line : 410usize , placeholders : & [] , regex : "^incorrect federated user returned from instance$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/circles/lib/Service/RemoteService.php" , line : 413usize , placeholders : & [] , regex : "^incorrect instance on returned federated user$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/circles/lib/CirclesQueryHelper.php" , line : 102usize , placeholders : & [] , regex : "^session not initiated$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/circles/lib/GlobalScale/AGlobalScaleEvent.php" , line : 214usize , placeholders : & [] , regex : "^GSEvent cannot be checked$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/circles/lib/GlobalScale/AGlobalScaleEvent.php" , line : 229usize , placeholders : & [] , regex : "^Viewer seems DSync$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Debug , path : "/apps/circles/lib/ShareByCircleProvider.php" , line : 305usize , placeholders : & [] , regex : "^Shares::move\\(\\)$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Debug , path : "/apps/circles/lib/ShareByCircleProvider.php" , line : 334usize , placeholders : & [] , regex : "^Shares::move\\(\\)$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Debug , path : "/apps/circles/lib/ShareByCircleProvider.php" , line : 358usize , placeholders : & [] , regex : "^Shares::restore\\(\\)$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Debug , path : "/apps/circles/lib/ShareByCircleProvider.php" , line : 408usize , placeholders : & [] , regex : "^shared document is not available anymore$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Notice , path : "/apps/circles/lib/ShareByCircleProvider.php" , line : 410usize , placeholders : & [] , regex : "^share is not available while path is empty\\. might comes from an unsupported storage\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Warn , path : "/apps/circles/lib/ShareByCircleProvider.php" , line : 615usize , placeholders : & [] , regex : "^Circle not found while probeCircle$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/circles/lib/FederatedItems/CircleConfig.php" , line : 158usize , placeholders : & [] , regex : "^Configuration value is not valid$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/circles/lib/FederatedItems/CircleEdit.php" , line : 106usize , placeholders : & [] , regex : "^Circle name is too short$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/circles/lib/FederatedItems/CircleCreate.php" , line : 127usize , placeholders : & [] , regex : "^Circle already exist$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/circles/lib/FederatedItems/CircleCreate.php" , line : 135usize , placeholders : & [] , regex : "^Owner already exist$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/circles/lib/FederatedItems/MemberLevel.php" , line : 111usize , placeholders : & [] , regex : "^This level cannot be edited$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/circles/lib/FederatedItems/MemberLevel.php" , line : 116usize , placeholders : & [] , regex : "^invalid level$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/circles/lib/FederatedItems/MemberLevel.php" , line : 120usize , placeholders : & [] , regex : "^This member already have the selected level$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/circles/lib/FederatedItems/CircleJoin.php" , line : 290usize , placeholders : & [] , regex : "^Blocked$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Warn , path : "/apps/circles/lib/Migration/Version0028Date20230705222601.php" , line : 57usize , placeholders : & [] , regex : "^Could not find circles_circle$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Info , path : "/apps/circles/lib/Migration/ImportOwncloudCustomGroups.php" , line : 87usize , placeholders : & [] , regex : "^Creating circles$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Info , path : "/apps/circles/lib/Migration/ImportOwncloudCustomGroups.php" , line : 124usize , placeholders : & [] , regex : "^Creating memberships$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Info , path : "/apps/circles/lib/Migration/ImportOwncloudCustomGroups.php" , line : 181usize , placeholders : & [] , regex : "^Update shares from custom groups to circles$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/circles/lib/CirclesManager.php" , line : 410usize , placeholders : & [] , regex : "^This Circle is not managed from this instance$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/sharebymail/composer/composer/ClassLoader.php" , line : 256usize , placeholders : & [] , regex : "^A non\\-empty PSR\\-4 prefix must end with a namespace separator\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/sharebymail/composer/composer/ClassLoader.php" , line : 311usize , placeholders : & [] , regex : "^A non\\-empty PSR\\-4 prefix must end with a namespace separator\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/sharebymail/lib/ShareByMailProvider.php" , line : 383usize , placeholders : & [] , regex : "^Failed to send share by mail$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/sharebymail/lib/ShareByMailProvider.php" , line : 828usize , placeholders : & [] , regex : "^not implemented$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/sharebymail/lib/ShareByMailProvider.php" , line : 998usize , placeholders : & [] , regex : "^Share not found$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/sharebymail/lib/ShareByMailProvider.php" , line : 1004usize , placeholders : & [] , regex : "^Share not found$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/lookup_server_connector/composer/composer/ClassLoader.php" , line : 256usize , placeholders : & [] , regex : "^A non\\-empty PSR\\-4 prefix must end with a namespace separator\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/lookup_server_connector/composer/composer/ClassLoader.php" , line : 311usize , placeholders : & [] , regex : "^A non\\-empty PSR\\-4 prefix must end with a namespace separator\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/twofactor_totp/vendor/composer/ClassLoader.php" , line : 250usize , placeholders : & [] , regex : "^A non\\-empty PSR\\-4 prefix must end with a namespace separator\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/twofactor_totp/vendor/composer/ClassLoader.php" , line : 305usize , placeholders : & [] , regex : "^A non\\-empty PSR\\-4 prefix must end with a namespace separator\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/twofactor_totp/lib/Controller/SettingsController.php" , line : 61usize , placeholders : & [] , regex : "^user not available$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/twofactor_totp/lib/Controller/SettingsController.php" , line : 78usize , placeholders : & [] , regex : "^user not available$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/twofactor_totp/lib/Controller/SettingsController.php" , line : 99usize , placeholders : & [] , regex : "^code is missing$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/twofactor_totp/lib/Controller/SettingsController.php" , line : 106usize , placeholders : & [] , regex : "^Invalid TOTP state$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/twofactor_totp/lib/Controller/SettingsController.php" , line : 119usize , placeholders : & [] , regex : "^No user in this context$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/twofactor_totp/lib/Db/TotpSecretMapper.php" , line : 61usize , placeholders : & [] , regex : "^Secret does not exist$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/contactsinteraction/composer/composer/ClassLoader.php" , line : 256usize , placeholders : & [] , regex : "^A non\\-empty PSR\\-4 prefix must end with a namespace separator\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/contactsinteraction/composer/composer/ClassLoader.php" , line : 311usize , placeholders : & [] , regex : "^A non\\-empty PSR\\-4 prefix must end with a namespace separator\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/contactsinteraction/lib/AddressBook.php" , line : 65usize , placeholders : & [] , regex : "^This addressbook is immutable$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/contactsinteraction/lib/AddressBook.php" , line : 72usize , placeholders : & [] , regex : "^This addressbook is immutable$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/contactsinteraction/lib/AddressBook.php" , line : 136usize , placeholders : & [] , regex : "^This addressbook is immutable$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Warn , path : "/apps/contactsinteraction/lib/Listeners/ContactInteractionListener.php" , line : 79usize , placeholders : & [] , regex : "^Contact interaction event has no user identifier set$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Info , path : "/apps/contactsinteraction/lib/Listeners/ContactInteractionListener.php" , line : 84usize , placeholders : & [] , regex : "^Ignoring contact interaction with self$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/federatedfilesharing/composer/composer/ClassLoader.php" , line : 256usize , placeholders : & [] , regex : "^A non\\-empty PSR\\-4 prefix must end with a namespace separator\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/federatedfilesharing/composer/composer/ClassLoader.php" , line : 311usize , placeholders : & [] , regex : "^A non\\-empty PSR\\-4 prefix must end with a namespace separator\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/federatedfilesharing/lib/FederatedShareProvider.php" , line : 370usize , placeholders : & [] , regex : "^share not found in share_external table$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/federatedfilesharing/lib/FederatedShareProvider.php" , line : 638usize , placeholders : & [] , regex : "^not implemented$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/federatedfilesharing/lib/FederatedShareProvider.php" , line : 856usize , placeholders : & [] , regex : "^Share not found$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/federatedfilesharing/lib/FederatedShareProvider.php" , line : 862usize , placeholders : & [] , regex : "^Share not found$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/federatedfilesharing/lib/AddressHandler.php" , line : 79usize , placeholders : & [] , regex : "^Invalid Federated Cloud ID$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/federatedfilesharing/lib/OCM/CloudFederationProviderFiles.php" , line : 194usize , placeholders : & [] , regex : "^Server does not support federated cloud sharing$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/federatedfilesharing/lib/OCM/CloudFederationProviderFiles.php" , line : 199usize , placeholders : & [] , regex : "^Unsupported protocol for data exchange\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/federatedfilesharing/lib/OCM/CloudFederationProviderFiles.php" , line : 228usize , placeholders : & [] , regex : "^The mountpoint name contains invalid characters\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/federatedfilesharing/lib/OCM/CloudFederationProviderFiles.php" , line : 242usize , placeholders : & [] , regex : "^User does not exists$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/federatedfilesharing/lib/OCM/CloudFederationProviderFiles.php" , line : 249usize , placeholders : & [] , regex : "^Group does not exists$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/federatedfilesharing/lib/OCM/CloudFederationProviderFiles.php" , line : 292usize , placeholders : & [] , regex : "^server can not add remote share, missing parameter$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/federatedfilesharing/lib/OCM/CloudFederationProviderFiles.php" , line : 378usize , placeholders : & [] , regex : "^Server does not support federated cloud sharing$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/federatedfilesharing/lib/OCM/CloudFederationProviderFiles.php" , line : 448usize , placeholders : & [] , regex : "^Server does not support federated cloud sharing$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/federatedfilesharing/lib/OCM/CloudFederationProviderFiles.php" , line : 542usize , placeholders : & [] , regex : "^incoming shares disabled!$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/federatedfilesharing/lib/OCM/CloudFederationProviderFiles.php" , line : 692usize , placeholders : & [] , regex : "^Updating reshares not allowed$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/federatedfilesharing/lib/Controller/RequestHandlerController.php" , line : 173usize , placeholders : & [] , regex : "^Server does not support federated cloud sharing$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/federatedfilesharing/lib/Controller/RequestHandlerController.php" , line : 225usize , placeholders : & [] , regex : "^Server does not support federated cloud sharing$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/federatedfilesharing/lib/Controller/RequestHandlerController.php" , line : 261usize , placeholders : & [] , regex : "^Server does not support federated cloud sharing$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/federatedfilesharing/lib/Controller/RequestHandlerController.php" , line : 294usize , placeholders : & [] , regex : "^Server does not support federated cloud sharing$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/federatedfilesharing/lib/Controller/RequestHandlerController.php" , line : 316usize , placeholders : & [] , regex : "^Server does not support federated cloud sharing$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/federatedfilesharing/lib/Controller/RequestHandlerController.php" , line : 447usize , placeholders : & [] , regex : "^Server does not support federated cloud sharing$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/federatedfilesharing/lib/Controller/RequestHandlerController.php" , line : 467usize , placeholders : & [] , regex : "^Share not found or token invalid$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Info , path : "/apps/federatedfilesharing/lib/Notifications.php" , line : 133usize , placeholders : & ["$name" , "$shareWith"] , regex : "^failed sharing (.*) with (.*)$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Info , path : "/apps/federatedfilesharing/lib/Notifications.php" , line : 139usize , placeholders : & ["$name" , "$shareWith"] , regex : "^could not share (.*), invalid contact (.*)$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Info , path : "/apps/federatedfilesharing/lib/Notifications.php" , line : 194usize , placeholders : & ["$filename" , "$remote"] , regex : "^invalid or missing token requesting re\\-share for (.*) to (.*)$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Info , path : "/apps/federatedfilesharing/lib/Notifications.php" , line : 199usize , placeholders : & ["$filename" , "$remote"] , regex : "^missing remote id requesting re\\-share for (.*) to (.*)$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Info , path : "/apps/federatedfilesharing/lib/Notifications.php" , line : 204usize , placeholders : & ["$filename" , "$remote"] , regex : "^failed requesting re\\-share for (.*) to (.*)$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/federatedfilesharing/lib/Notifier.php" , line : 248usize , placeholders : & [] , regex : "^No contact found for federated cloud id$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/federatedfilesharing/lib/Notifier.php" , line : 270usize , placeholders : & [] , regex : "^No contact found for federated cloud id$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/weather_status/composer/composer/ClassLoader.php" , line : 256usize , placeholders : & [] , regex : "^A non\\-empty PSR\\-4 prefix must end with a namespace separator\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/weather_status/composer/composer/ClassLoader.php" , line : 311usize , placeholders : & [] , regex : "^A non\\-empty PSR\\-4 prefix must end with a namespace separator\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Warn , path : "/apps/theming/lib/Jobs/MigrateBackgroundImages.php" , line : 178usize , placeholders : & [] , regex : "^Lacking permissions to create \\{file\\}$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Info , path : "/apps/theming/lib/Jobs/MigrateBackgroundImages.php" , line : 198usize , placeholders : & [] , regex : "^Could not delete \\{file\\} due to permissions\\. It is safe to delete manually inside data \\-> appdata \\-> theming \\-> global\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/theming/lib/Controller/UserThemeController.php" , line : 137usize , placeholders : & [] , regex : "^Theme switching is disabled$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/theming/lib/ImageManager.php" , line : 243usize , placeholders : & [] , regex : "^Unsupported image type$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/theming/lib/ImageManager.php" , line : 252usize , placeholders : & [] , regex : "^Could not read background image, possibly corrupted\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/theming/lib/ImageManager.php" , line : 263usize , placeholders : & [] , regex : "^Could not scale uploaded background image\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/theming/lib/ImageManager.php" , line : 271usize , placeholders : & [] , regex : "^Could not recompress background image as JPEG$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/theming/lib/ImageManager.php" , line : 275usize , placeholders : & [] , regex : "^Could not recompress background image as PNG$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/theming/lib/Service/BackgroundService.php" , line : 205usize , placeholders : & [] , regex : "^Invalid image file$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/theming/lib/Service/BackgroundService.php" , line : 213usize , placeholders : & [] , regex : "^The given file name is invalid$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/theming/lib/Service/BackgroundService.php" , line : 221usize , placeholders : & [] , regex : "^The given color is invalid$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/theming/composer/composer/ClassLoader.php" , line : 256usize , placeholders : & [] , regex : "^A non\\-empty PSR\\-4 prefix must end with a namespace separator\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/theming/composer/composer/ClassLoader.php" , line : 311usize , placeholders : & [] , regex : "^A non\\-empty PSR\\-4 prefix must end with a namespace separator\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Warn , path : "/apps/twofactor_backupcodes/lib/Listener/ActivityPublisher.php" , line : 63usize , placeholders : & [] , regex : "^could not publish backup code creation activity$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/twofactor_backupcodes/composer/composer/ClassLoader.php" , line : 256usize , placeholders : & [] , regex : "^A non\\-empty PSR\\-4 prefix must end with a namespace separator\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/twofactor_backupcodes/composer/composer/ClassLoader.php" , line : 311usize , placeholders : & [] , regex : "^A non\\-empty PSR\\-4 prefix must end with a namespace separator\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/logreader/lib/Log/LogIteratorFactory.php" , line : 63usize , placeholders : & [] , regex : "^Can't find log class$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/logreader/lib/Log/Console.php" , line : 84usize , placeholders : & ["$level"] , regex : "^Unknown log level (.*)$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/dav/composer/composer/ClassLoader.php" , line : 256usize , placeholders : & [] , regex : "^A non\\-empty PSR\\-4 prefix must end with a namespace separator\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/dav/composer/composer/ClassLoader.php" , line : 311usize , placeholders : & [] , regex : "^A non\\-empty PSR\\-4 prefix must end with a namespace separator\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/dav/lib/SystemTag/SystemTagsObjectMappingCollection.php" , line : 81usize , placeholders : & [] , regex : "^Permission denied to create collections$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/dav/lib/SystemTag/SystemTagsObjectMappingCollection.php" , line : 98usize , placeholders : & [] , regex : "^Invalid tag id$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/dav/lib/SystemTag/SystemTagsObjectMappingCollection.php" , line : 140usize , placeholders : & [] , regex : "^Invalid tag id$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/dav/lib/SystemTag/SystemTagsObjectMappingCollection.php" , line : 150usize , placeholders : & [] , regex : "^Permission denied to delete this collection$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/dav/lib/SystemTag/SystemTagsObjectMappingCollection.php" , line : 161usize , placeholders : & [] , regex : "^Permission denied to rename this collection$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/dav/lib/SystemTag/SystemTagsInUseCollection.php" , line : 67usize , placeholders : & [] , regex : "^Permission denied to rename this collection$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/dav/lib/SystemTag/SystemTagsInUseCollection.php" , line : 72usize , placeholders : & [] , regex : "^Invalid media type$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/dav/lib/SystemTag/SystemTagsInUseCollection.php" , line : 93usize , placeholders : & [] , regex : "^Permission denied to read this collection$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/dav/lib/SystemTag/SystemTagsObjectTypeCollection.php" , line : 61usize , placeholders : & [] , regex : "^Permission denied to create nodes$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/dav/lib/SystemTag/SystemTagsObjectTypeCollection.php" , line : 72usize , placeholders : & [] , regex : "^Permission denied to create collections$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/dav/lib/SystemTag/SystemTagsObjectTypeCollection.php" , line : 84usize , placeholders : & [] , regex : "^Entity does not exist or is not available$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/dav/lib/SystemTag/SystemTagsObjectTypeCollection.php" , line : 118usize , placeholders : & [] , regex : "^Permission denied to delete this collection$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/dav/lib/SystemTag/SystemTagsObjectTypeCollection.php" , line : 133usize , placeholders : & [] , regex : "^Permission denied to rename this collection$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/dav/lib/SystemTag/SystemTagPlugin.php" , line : 186usize , placeholders : & [] , regex : "^Missing \"name\" attribute$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/dav/lib/SystemTag/SystemTagPlugin.php" , line : 211usize , placeholders : & [] , regex : "^Not sufficient permissions$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/dav/lib/SystemTag/SystemTagPlugin.php" , line : 222usize , placeholders : & [] , regex : "^Tag already exists$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/dav/lib/SystemTag/SystemTagsRelationsCollection.php" , line : 103usize , placeholders : & [] , regex : "^Permission denied to rename this collection$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/dav/lib/SystemTag/SystemTagsByIdCollection.php" , line : 93usize , placeholders : & [] , regex : "^Cannot create tags by id$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/dav/lib/SystemTag/SystemTagsByIdCollection.php" , line : 102usize , placeholders : & [] , regex : "^Permission denied to create collections$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/dav/lib/SystemTag/SystemTagsByIdCollection.php" , line : 119usize , placeholders : & [] , regex : "^Invalid tag id$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/dav/lib/SystemTag/SystemTagsByIdCollection.php" , line : 154usize , placeholders : & [] , regex : "^Invalid tag id$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/dav/lib/SystemTag/SystemTagsByIdCollection.php" , line : 164usize , placeholders : & [] , regex : "^Permission denied to delete this collection$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/dav/lib/SystemTag/SystemTagsByIdCollection.php" , line : 180usize , placeholders : & [] , regex : "^Permission denied to rename this collection$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/dav/lib/Direct/DirectHome.php" , line : 103usize , placeholders : & [] , regex : "^Listing members of this collection is disabled$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/dav/lib/BulkUpload/MultipartRequestParser.php" , line : 50usize , placeholders : & [] , regex : "^Body should be of type resource$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/dav/lib/BulkUpload/MultipartRequestParser.php" , line : 54usize , placeholders : & [] , regex : "^Content\\-Type can not be null$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/dav/lib/BulkUpload/MultipartRequestParser.php" , line : 75usize , placeholders : & [] , regex : "^Error while parsing boundary in Content\\-Type header\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/dav/lib/BulkUpload/MultipartRequestParser.php" , line : 86usize , placeholders : & [] , regex : "^Content\\-Type must be multipart/related$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/dav/lib/BulkUpload/MultipartRequestParser.php" , line : 90usize , placeholders : & [] , regex : "^Boundary is invalid$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/dav/lib/BulkUpload/MultipartRequestParser.php" , line : 106usize , placeholders : & [] , regex : "^An error occurred while checking content$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/dav/lib/BulkUpload/MultipartRequestParser.php" , line : 111usize , placeholders : & [] , regex : "^Unknown error while seeking content$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/dav/lib/BulkUpload/MultipartRequestParser.php" , line : 161usize , placeholders : & [] , regex : "^Boundary not found where it should be\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/dav/lib/BulkUpload/MultipartRequestParser.php" , line : 179usize , placeholders : & [] , regex : "^An error occurred while reading headers of a part$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/dav/lib/BulkUpload/MultipartRequestParser.php" , line : 186usize , placeholders : & [] , regex : "^An error occurred while parsing headers of a part$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/dav/lib/BulkUpload/MultipartRequestParser.php" , line : 191usize , placeholders : & [] , regex : "^The Content\\-Length header must not be null\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/dav/lib/BulkUpload/MultipartRequestParser.php" , line : 195usize , placeholders : & [] , regex : "^The X\\-File\\-MD5 header must not be null\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/dav/lib/BulkUpload/MultipartRequestParser.php" , line : 211usize , placeholders : & [] , regex : "^Computed md5 hash is incorrect\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/dav/lib/BulkUpload/MultipartRequestParser.php" , line : 221usize , placeholders : & [] , regex : "^Fail to read part's content\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/dav/lib/BulkUpload/MultipartRequestParser.php" , line : 225usize , placeholders : & [] , regex : "^Unexpected EOF while reading stream\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/dav/lib/Avatars/AvatarHome.php" , line : 53usize , placeholders : & [] , regex : "^Permission denied to create a file$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/dav/lib/Avatars/AvatarHome.php" , line : 57usize , placeholders : & [] , regex : "^Permission denied to create a folder$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/dav/lib/Avatars/AvatarHome.php" , line : 65usize , placeholders : & [] , regex : "^File format not allowed$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/dav/lib/Avatars/AvatarHome.php" , line : 68usize , placeholders : & [] , regex : "^Invalid image size$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/dav/lib/Avatars/AvatarHome.php" , line : 99usize , placeholders : & [] , regex : "^Permission denied to delete this folder$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/dav/lib/Avatars/AvatarHome.php" , line : 108usize , placeholders : & [] , regex : "^Permission denied to rename this folder$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/dav/lib/Upload/AssemblyStream.php" , line : 261usize , placeholders : & [] , regex : "^Invalid context, nodes not set$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/dav/lib/Upload/FutureFile.php" , line : 57usize , placeholders : & [] , regex : "^Permission denied to put into this file$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/dav/lib/Upload/FutureFile.php" , line : 117usize , placeholders : & [] , regex : "^Permission denied to rename this file$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/dav/lib/Upload/ChunkingV2Plugin.php" , line : 161usize , placeholders : & [] , regex : "^Invalid chunk name, must be numeric between 1 and 10000$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/dav/lib/Upload/ChunkingV2Plugin.php" , line : 177usize , placeholders : & ["$this->uploadPath"] , regex : "^Insufficient space in (.*)$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/dav/lib/Upload/ChunkingV2Plugin.php" , line : 233usize , placeholders : & ["$this->uploadPath"] , regex : "^Insufficient space in (.*)$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/dav/lib/Upload/ChunkingV2Plugin.php" , line : 281usize , placeholders : & [] , regex : "^Skipping chunking v2 since no proper distributed cache is available$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/dav/lib/Upload/ChunkingV2Plugin.php" , line : 284usize , placeholders : & [] , regex : "^Skipping chunked file writing as the destination header was not passed$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/dav/lib/Upload/ChunkingV2Plugin.php" , line : 287usize , placeholders : & [] , regex : "^Storage does not support chunked file writing$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/dav/lib/Upload/ChunkingV2Plugin.php" , line : 290usize , placeholders : & [] , regex : "^Storage does not support multi part uploads$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/dav/lib/Upload/ChunkingV2Plugin.php" , line : 295usize , placeholders : & [] , regex : "^Missing metadata for chunked upload\\. The distributed cache does not hold the information of previous requests\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/dav/lib/Upload/ChunkingV2Plugin.php" , line : 311usize , placeholders : & [] , regex : "^X\\-OC\\-MTime header must be an integer \\(unix timestamp\\)\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/dav/lib/Upload/UploadHome.php" , line : 80usize , placeholders : & [] , regex : "^Permission denied to rename this folder$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/dav/lib/Upload/UploadFolder.php" , line : 113usize , placeholders : & [] , regex : "^Permission denied to rename this folder$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/dav/lib/Upload/ChunkingPlugin.php" , line : 69usize , placeholders : & ["$destination"] , regex : "^The given destination (.*) is a directory\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/dav/lib/Upload/ChunkingPlugin.php" , line : 128usize , placeholders : & ["$expectedSize" , "$actualSize"] , regex : "^Chunks on server do not sum up to (.*) but to (.*) bytes$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/dav/lib/Upload/PartFile.php" , line : 49usize , placeholders : & [] , regex : "^Permission denied to put into this file$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/dav/lib/Upload/PartFile.php" , line : 56usize , placeholders : & [] , regex : "^Permission denied to get this file$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/dav/lib/Upload/PartFile.php" , line : 102usize , placeholders : & [] , regex : "^Permission denied to rename this file$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/dav/lib/Command/MoveCalendar.php" , line : 98usize , placeholders : & ["$userOrigin"] , regex : "^User <(.*)> is unknown\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/dav/lib/Command/MoveCalendar.php" , line : 102usize , placeholders : & ["$userDestination"] , regex : "^User <(.*)> is unknown\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/dav/lib/Command/MoveCalendar.php" , line : 111usize , placeholders : & ["$userOrigin" , "$name"] , regex : "^User <(.*)> has no calendar named <(.*)>\\. You can run occ dav:list\\-calendars to list calendars URIs for this user\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/dav/lib/Command/MoveCalendar.php" , line : 122usize , placeholders : & ["$userDestination" , "$name"] , regex : "^Unable to find a suitable calendar name for <(.*)> with initial name <(.*)>\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/dav/lib/Command/MoveCalendar.php" , line : 125usize , placeholders : & ["$userDestination" , "$name"] , regex : "^User <(.*)> already has a calendar named <(.*)>\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/dav/lib/Command/CreateAddressBook.php" , line : 69usize , placeholders : & ["$user"] , regex : "^User <(.*)> in unknown\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/dav/lib/Command/SyncBirthdayCalendar.php" , line : 80usize , placeholders : & ["$user"] , regex : "^User <(.*)> in unknown\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/dav/lib/Command/SyncBirthdayCalendar.php" , line : 119usize , placeholders : & [] , regex : "^Birthday calendars are disabled$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/dav/lib/Command/ListCalendars.php" , line : 67usize , placeholders : & ["$user"] , regex : "^User <(.*)> is unknown\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/dav/lib/Command/CreateCalendar.php" , line : 82usize , placeholders : & ["$user"] , regex : "^User <(.*)> in unknown\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/dav/lib/Command/DeleteCalendar.php" , line : 116usize , placeholders : & [] , regex : "^Please specify a calendar name or \\-\\-birthday$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/dav/lib/CardDAV/CardDavBackend.php" , line : 394usize , placeholders : & [] , regex : "^URI too long\\. Address book not created$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/dav/lib/CardDAV/CardDavBackend.php" , line : 659usize , placeholders : & [] , regex : "^VCard object with uid already exists in this addressbook collection\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/dav/lib/CardDAV/CardDavBackend.php" , line : 1467usize , placeholders : & [] , regex : "^vCards on CardDAV servers MUST have a UID property$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/dav/lib/CardDAV/CardDavBackend.php" , line : 1470usize , placeholders : & [] , regex : "^vCard can not be empty$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/dav/lib/CardDAV/Security/CardDavRateLimitingPlugin.php" , line : 68usize , placeholders : & [] , regex : "^Too many addressbooks created$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Warn , path : "/apps/dav/lib/CardDAV/Security/CardDavRateLimitingPlugin.php" , line : 78usize , placeholders : & [] , regex : "^Maximum number of address books reached$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/dav/lib/CardDAV/Security/CardDavRateLimitingPlugin.php" , line : 82usize , placeholders : & [] , regex : "^AddressBook limit reached$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/dav/lib/CardDAV/SystemAddressbook.php" , line : 234usize , placeholders : & [] , regex : "^Card not found$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/dav/lib/CardDAV/Integration/ExternalAddressBook.php" , line : 76usize , placeholders : & [] , regex : "^Renaming address books is not yet supported$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/dav/lib/CardDAV/Integration/ExternalAddressBook.php" , line : 83usize , placeholders : & [] , regex : "^Creating collections in address book objects is not allowed$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/dav/lib/CardDAV/Integration/ExternalAddressBook.php" , line : 109usize , placeholders : & [] , regex : "^Provided address book uri was not app\\-generated$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/dav/lib/CardDAV/UserAddressBooks.php" , line : 140usize , placeholders : & [] , regex : "^The resource you tried to create has a reserved name$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Error , path : "/apps/dav/lib/CardDAV/SyncService.php" , line : 90usize , placeholders : & [] , regex : "^Client exception:$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/dav/lib/CardDAV/PhotoCache.php" , line : 190usize , placeholders : & [] , regex : "^Avatar not found$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Error , path : "/apps/dav/lib/CardDAV/PhotoCache.php" , line : 202usize , placeholders : & [] , regex : "^Exception during vcard photo parsing$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Error , path : "/apps/dav/lib/CardDAV/PhotoCache.php" , line : 247usize , placeholders : & [] , regex : "^Exception during vcard photo parsing$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/dav/lib/CardDAV/AddressBook.php" , line : 172usize , placeholders : & [] , regex : "^Card not found$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/dav/lib/Files/RootCollection.php" , line : 56usize , placeholders : & [] , regex : "^Home does not exist$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/dav/lib/Files/Sharing/FilesDropPlugin.php" , line : 75usize , placeholders : & [] , regex : "^Only PUT is allowed on files drop$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/dav/lib/Files/FilesHome.php" , line : 52usize , placeholders : & [] , regex : "^Permission denied to delete home folder$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/dav/lib/Files/FilesHome.php" , line : 61usize , placeholders : & [] , regex : "^Permission denied to rename this folder$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/dav/lib/Files/FileSearchBackend.php" , line : 176usize , placeholders : & [] , regex : "^Searching more than one folder is not supported$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/dav/lib/Files/FileSearchBackend.php" , line : 181usize , placeholders : & [] , regex : "^Using uri's as scope is not supported, please use a path relative to the search arbiter instead$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/dav/lib/Files/FileSearchBackend.php" , line : 185usize , placeholders : & [] , regex : "^Search is only supported on directories$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/dav/lib/Files/FileSearchBackend.php" , line : 316usize , placeholders : & [] , regex : "^Invalid search value for '\\{http://owncloud\\.org/ns\\}owner\\-id', only the current user id is allowed$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/dav/lib/Files/FileSearchBackend.php" , line : 474usize , placeholders : & ["$propertyName"] , regex : "^searching by '(.*)' is only allowed with a literal value$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/dav/lib/Files/FileSearchBackend.php" , line : 477usize , placeholders : & ["$propertyName"] , regex : "^searching by '(.*)' is not allowed inside a '\\{DAV:\\}or' or '\\{DAV:\\}not'$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/dav/lib/Files/FileSearchBackend.php" , line : 480usize , placeholders : & ["$propertyName" , "$comparison"] , regex : "^searching by '(.*)' is only allowed inside a '(.*)'$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/dav/lib/Controller/DirectController.php" , line : 107usize , placeholders : & [] , regex : "^Direct download only works for files$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/dav/lib/Controller/DirectController.php" , line : 114usize , placeholders : & [] , regex : "^Permission denied to download file$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/dav/lib/DAV/SystemPrincipalBackend.php" , line : 151usize , placeholders : & [] , regex : "^Principal not found$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/dav/lib/DAV/SystemPrincipalBackend.php" , line : 169usize , placeholders : & [] , regex : "^Principal not found$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/dav/lib/DAV/SystemPrincipalBackend.php" , line : 187usize , placeholders : & [] , regex : "^Setting members of the group is not supported yet$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/dav/lib/DAV/ViewOnlyPlugin.php" , line : 88usize , placeholders : & [] , regex : "^Version file not accessible by current user$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/dav/lib/DAV/ViewOnlyPlugin.php" , line : 112usize , placeholders : & [] , regex : "^Access to this resource has been denied because it is in view\\-only mode\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/dav/lib/DAV/GroupPrincipalBackend.php" , line : 172usize , placeholders : & [] , regex : "^Setting members of the group is not supported yet$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/dav/lib/Connector/Sabre/QuotaPlugin.php" , line : 210usize , placeholders : & ["$path" , "$length" , "$freeSpace"] , regex : "^Insufficient space in (.*), (.*) required, (.*) available$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Debug , path : "/apps/dav/lib/Connector/Sabre/FilesPlugin.php" , line : 448usize , placeholders : & [] , regex : "^Inefficient fetching of metadata$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/dav/lib/Connector/Sabre/Auth.php" , line : 146usize , placeholders : & ["$class" , "$msg"] , regex : "^(.*): (.*)$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/dav/lib/Connector/Sabre/Auth.php" , line : 201usize , placeholders : & [] , regex : "^CSRF check not passed\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/dav/lib/Connector/Sabre/Auth.php" , line : 209usize , placeholders : & [] , regex : "^2FA challenge not passed\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/dav/lib/Connector/Sabre/Auth.php" , line : 229usize , placeholders : & [] , regex : "^Cannot authenticate over ajax calls$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/dav/lib/Connector/Sabre/Directory.php" , line : 257usize , placeholders : & [] , regex : "^No read permissions\\. This might be caused by files_accesscontrol, check your configured rules$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/dav/lib/Connector/Sabre/Directory.php" , line : 259usize , placeholders : & [] , regex : "^No read permissions$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Warn , path : "/apps/dav/lib/Connector/Sabre/Directory.php" , line : 333usize , placeholders : & [] , regex : "^error while getting quota as the relative path cannot be found$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Warn , path : "/apps/dav/lib/Connector/Sabre/Directory.php" , line : 350usize , placeholders : & [] , regex : "^error while getting quota into$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Warn , path : "/apps/dav/lib/Connector/Sabre/Directory.php" , line : 353usize , placeholders : & [] , regex : "^error while getting quota into$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Warn , path : "/apps/dav/lib/Connector/Sabre/Directory.php" , line : 356usize , placeholders : & [] , regex : "^error while getting quota into$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/dav/lib/Connector/Sabre/Directory.php" , line : 393usize , placeholders : & [] , regex : "^Incompatible node types$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/dav/lib/Connector/Sabre/Directory.php" , line : 397usize , placeholders : & [] , regex : "^filesystem not setup$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/dav/lib/Connector/Sabre/Directory.php" , line : 454usize , placeholders : & [] , regex : "^$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/dav/lib/Connector/Sabre/MtimeSanitizer.php" , line : 32usize , placeholders : & [] , regex : "^X\\-OC\\-MTime header must be an integer \\(unix timestamp\\)\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/dav/lib/Connector/Sabre/MtimeSanitizer.php" , line : 37usize , placeholders : & [] , regex : "^X\\-OC\\-MTime header must be a valid positive integer$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/dav/lib/Connector/Sabre/FilesReportPlugin.php" , line : 217usize , placeholders : & [] , regex : "^Missing filter\\-rule block in request$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/dav/lib/Connector/Sabre/FilesReportPlugin.php" , line : 229usize , placeholders : & [] , regex : "^Cannot filter by non\\-existing tag$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/dav/lib/Connector/Sabre/ObjectTree.php" , line : 111usize , placeholders : & [] , regex : "^filesystem not setup$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/dav/lib/Connector/Sabre/ObjectTree.php" , line : 161usize , placeholders : & [] , regex : "^Storage is temporarily not available$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/dav/lib/Connector/Sabre/ObjectTree.php" , line : 205usize , placeholders : & [] , regex : "^filesystem not setup$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/dav/lib/Connector/Sabre/ObjectTree.php" , line : 216usize , placeholders : & [] , regex : "^No permissions to copy object\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/dav/lib/Connector/Sabre/BlockLegacyClientPlugin.php" , line : 74usize , placeholders : & [] , regex : "^Unsupported client version\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/dav/lib/Connector/Sabre/Principal.php" , line : 237usize , placeholders : & [] , regex : "^Principal not found$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/dav/lib/Connector/Sabre/Principal.php" , line : 600usize , placeholders : & [] , regex : "^Principal not found$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Error , path : "/apps/dav/lib/Connector/Sabre/File.php" , line : 265usize , placeholders : & [] , regex : "^\\\\OC\\\\Files\\\\Filesystem::fopen\\(\\) failed$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Error , path : "/apps/dav/lib/Connector/Sabre/File.php" , line : 653usize , placeholders : & [] , regex : "^\\\\OC\\\\Files\\\\Filesystem::rename\\(\\) failed$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/dav/lib/Connector/PublicAuth.php" , line : 107usize , placeholders : & [] , regex : "^Cannot authenticate over ajax calls$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/dav/lib/Traits/PrincipalProxyTrait.php" , line : 51usize , placeholders : & [] , regex : "^Principal not found$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/dav/lib/Traits/PrincipalProxyTrait.php" , line : 86usize , placeholders : & [] , regex : "^Principal not found$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/dav/lib/Traits/PrincipalProxyTrait.php" , line : 117usize , placeholders : & [] , regex : "^Setting members of the group is not supported yet$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/dav/lib/Traits/PrincipalProxyTrait.php" , line : 122usize , placeholders : & [] , regex : "^Principal not found$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/dav/lib/Traits/PrincipalProxyTrait.php" , line : 142usize , placeholders : & [] , regex : "^Principal not found$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Info , path : "/apps/dav/lib/Migration/CalDAVRemoveEmptyValue.php" , line : 73usize , placeholders : & [] , regex : "^Calendar object for calendar \\{cal\\} with uri \\{uri\\} still invalid$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Info , path : "/apps/dav/lib/Migration/RegisterBuildReminderIndexBackgroundJob.php" , line : 83usize , placeholders : & [] , regex : "^Repair step already executed$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Info , path : "/apps/dav/lib/Migration/RegisterBuildReminderIndexBackgroundJob.php" , line : 94usize , placeholders : & [] , regex : "^Add background job$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Info , path : "/apps/dav/lib/Migration/BuildSocialSearchIndex.php" , line : 69usize , placeholders : & [] , regex : "^Repair step already executed$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Info , path : "/apps/dav/lib/Migration/BuildSocialSearchIndex.php" , line : 83usize , placeholders : & [] , regex : "^Add background job$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Info , path : "/apps/dav/lib/Migration/FixBirthdayCalendarComponent.php" , line : 60usize , placeholders : & ["$updated"] , regex : "^(.*) birthday calendars updated\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Info , path : "/apps/dav/lib/Migration/RegenerateBirthdayCalendars.php" , line : 63usize , placeholders : & [] , regex : "^Repair step already executed$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Info , path : "/apps/dav/lib/Migration/RegenerateBirthdayCalendars.php" , line : 67usize , placeholders : & [] , regex : "^Adding background jobs to regenerate birthday calendar$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Info , path : "/apps/dav/lib/Migration/Version1027Date20230504122946.php" , line : 54usize , placeholders : & [] , regex : "^Could not sync system address books during update \\- too many user records have been found\\. Please call occ dav:sync\\-system\\-addressbook manually\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Error , path : "/apps/dav/lib/Migration/Version1027Date20230504122946.php" , line : 63usize , placeholders : & [] , regex : "^Could not sync system address books during update$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Warn , path : "/apps/dav/lib/Migration/Version1027Date20230504122946.php" , line : 66usize , placeholders : & [] , regex : "^System address book sync failed\\. See logs for details$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Info , path : "/apps/dav/lib/Migration/DeleteSchedulingObjects.php" , line : 30usize , placeholders : & [] , regex : "^Cleaning up old scheduling events$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Info , path : "/apps/dav/lib/Migration/DeleteSchedulingObjects.php" , line : 34usize , placeholders : & [] , regex : "^Adding background job to delete old scheduling objects$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Info , path : "/apps/dav/lib/Migration/ChunkCleanup.php" , line : 67usize , placeholders : & [] , regex : "^Cleanup not required$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Info , path : "/apps/dav/lib/Migration/BuildCalendarSearchIndex.php" , line : 72usize , placeholders : & [] , regex : "^Repair step already executed$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Info , path : "/apps/dav/lib/Migration/BuildCalendarSearchIndex.php" , line : 83usize , placeholders : & [] , regex : "^Add background job$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Info , path : "/apps/dav/lib/Migration/RefreshWebcalJobRegistrar.php" , line : 83usize , placeholders : & ["$count"] , regex : "^Added (.*) background jobs to update webcal calendars$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Info , path : "/apps/dav/lib/Migration/RemoveObjectProperties.php" , line : 63usize , placeholders : & ["$updated"] , regex : "^(.*) invalid object properties removed\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Info , path : "/apps/dav/lib/Migration/BuildSocialSearchIndexBackgroundJob.php" , line : 77usize , placeholders : & [] , regex : "^All contacts with social profiles indexed$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Info , path : "/apps/dav/lib/Migration/RemoveClassifiedEventActivity.php" , line : 61usize , placeholders : & ["$deletedEvents"] , regex : "^Removed (.*) activity entries$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Info , path : "/apps/dav/lib/Migration/BuildCalendarSearchIndexBackgroundJob.php" , line : 84usize , placeholders : & [] , regex : "^Building calendar index done$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/dav/lib/Comments/EntityTypeCollection.php" , line : 72usize , placeholders : & [] , regex : "^\"name\" parameter must be non\\-empty string$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/dav/lib/Comments/EntityTypeCollection.php" , line : 94usize , placeholders : & [] , regex : "^Entity does not exist or is not available$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/dav/lib/Comments/EntityTypeCollection.php" , line : 113usize , placeholders : & [] , regex : "^No permission to list folder contents$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/dav/lib/Comments/CommentsPlugin.php" , line : 247usize , placeholders : & [] , regex : "^Invalid input values$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/dav/lib/Comments/CommentNode.php" , line : 131usize , placeholders : & [] , regex : "^Only authors are allowed to edit their comment\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/dav/lib/Comments/RootCollection.php" , line : 114usize , placeholders : & [] , regex : "^Cannot create comments by id$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/dav/lib/Comments/RootCollection.php" , line : 124usize , placeholders : & [] , regex : "^Permission denied to create collections$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/dav/lib/Comments/RootCollection.php" , line : 172usize , placeholders : & [] , regex : "^Permission denied to delete this collection$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/dav/lib/Comments/RootCollection.php" , line : 193usize , placeholders : & [] , regex : "^Permission denied to rename this collection$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/dav/lib/CalDAV/Calendar.php" , line : 280usize , placeholders : & [] , regex : "^Calendar object not found$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/dav/lib/CalDAV/Calendar.php" , line : 284usize , placeholders : & [] , regex : "^Calendar object not found$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/dav/lib/CalDAV/Integration/ExternalCalendar.php" , line : 84usize , placeholders : & [] , regex : "^Renaming calendars is not yet supported$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/dav/lib/CalDAV/Integration/ExternalCalendar.php" , line : 91usize , placeholders : & [] , regex : "^Creating collections in calendar objects is not allowed$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/dav/lib/CalDAV/Integration/ExternalCalendar.php" , line : 115usize , placeholders : & [] , regex : "^Provided calendar uri was not app\\-generated$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/dav/lib/CalDAV/Reminder/Notifier.php" , line : 107usize , placeholders : & [] , regex : "^Notification not from this app$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/dav/lib/CalDAV/Reminder/Notifier.php" , line : 119usize , placeholders : & [] , regex : "^Unknown subject$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Error , path : "/apps/dav/lib/CalDAV/Reminder/NotificationProvider/EmailProvider.php" , line : 123usize , placeholders : & [] , regex : "^Email address \\{address\\} for reminder notification is incorrect$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Error , path : "/apps/dav/lib/CalDAV/Reminder/NotificationProvider/EmailProvider.php" , line : 139usize , placeholders : & [] , regex : "^Unable to deliver message to \\{failed\\}$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/dav/lib/CalDAV/Reminder/NotificationProviderManager.php" , line : 79usize , placeholders : & [] , regex : "^Invalid notification provider registered$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Debug , path : "/apps/dav/lib/CalDAV/Reminder/ReminderService.php" , line : 128usize , placeholders : & [] , regex : "^\\{numReminders\\} reminders to process$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Debug , path : "/apps/dav/lib/CalDAV/Reminder/ReminderService.php" , line : 143usize , placeholders : & [] , regex : "^Reminder \\{id\\} does not belong to a valid calendar$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Debug , path : "/apps/dav/lib/CalDAV/Reminder/ReminderService.php" , line : 153usize , placeholders : & [] , regex : "^Recurrence with too many instances detected, skipping VEVENT$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Debug , path : "/apps/dav/lib/CalDAV/Reminder/ReminderService.php" , line : 159usize , placeholders : & [] , regex : "^Reminder \\{id\\} does not belong to a valid event$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Debug , path : "/apps/dav/lib/CalDAV/Reminder/ReminderService.php" , line : 167usize , placeholders : & [] , regex : "^Reminder \\{id\\} belongs to a cancelled event$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Debug , path : "/apps/dav/lib/CalDAV/Reminder/ReminderService.php" , line : 175usize , placeholders : & [] , regex : "^Reminder \\{id\\} does not belong to a valid notification provider$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Debug , path : "/apps/dav/lib/CalDAV/Reminder/ReminderService.php" , line : 199usize , placeholders : & [] , regex : "^Reminder \\{id\\} will be sent to \\{numUsers\\} users$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Debug , path : "/apps/dav/lib/CalDAV/Reminder/ReminderService.php" , line : 563usize , placeholders : & [] , regex : "^Recurrence with too many instances detected, skipping VEVENT$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/dav/lib/CalDAV/Reminder/ReminderService.php" , line : 830usize , placeholders : & [] , regex : "^Multiple master objects$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/dav/lib/CalDAV/CalendarImpl.php" , line : 156usize , placeholders : & [] , regex : "^Could not write to calendar as URI parameter is missing$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/dav/lib/CalDAV/CalendarImpl.php" , line : 194usize , placeholders : & [] , regex : "^Could not write to calendar as URI parameter is missing$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/dav/lib/CalDAV/CalendarImpl.php" , line : 207usize , placeholders : & [] , regex : "^No Method provided for scheduling data\\. Could not process message$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/dav/lib/CalDAV/CalendarImpl.php" , line : 211usize , placeholders : & [] , regex : "^Could not process scheduling data, neccessary data missing from ICAL$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/dav/lib/CalDAV/CachedSubscription.php" , line : 138usize , placeholders : & [] , regex : "^Calendar object not found$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/dav/lib/CalDAV/CachedSubscription.php" , line : 181usize , placeholders : & [] , regex : "^Creating objects in cached subscription is not allowed$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Debug , path : "/apps/dav/lib/CalDAV/ICSExportPlugin/ICSExportPlugin.php" , line : 75usize , placeholders : & [] , regex : "^Invalid refresh interval for exported calendar, falling back to default value \\.\\.\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/dav/lib/CalDAV/Trashbin/DeletedCalendarObjectsCollection.php" , line : 79usize , placeholders : & [] , regex : "^The calendar object you're trying to restore is not marked as deleted$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/dav/lib/CalDAV/Trashbin/TrashbinHome.php" , line : 63usize , placeholders : & [] , regex : "^Permission denied to create files in the trashbin$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/dav/lib/CalDAV/Trashbin/TrashbinHome.php" , line : 67usize , placeholders : & [] , regex : "^Permission denied to create a directory in the trashbin$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/dav/lib/CalDAV/Trashbin/TrashbinHome.php" , line : 102usize , placeholders : & [] , regex : "^Permission denied to delete the trashbin$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/dav/lib/CalDAV/Trashbin/TrashbinHome.php" , line : 110usize , placeholders : & [] , regex : "^Permission denied to rename the trashbin$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/dav/lib/CalDAV/Trashbin/TrashbinHome.php" , line : 118usize , placeholders : & [] , regex : "^not implemented$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/dav/lib/CalDAV/Security/RateLimitingPlugin.php" , line : 90usize , placeholders : & [] , regex : "^Too many calendars created$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Warn , path : "/apps/dav/lib/CalDAV/Security/RateLimitingPlugin.php" , line : 101usize , placeholders : & [] , regex : "^Maximum number of calendars/subscriptions reached$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/dav/lib/CalDAV/Security/RateLimitingPlugin.php" , line : 106usize , placeholders : & [] , regex : "^Calendar limit reached$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/dav/lib/CalDAV/CachedSubscriptionObject.php" , line : 57usize , placeholders : & [] , regex : "^Creating objects in a cached subscription is not allowed$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/dav/lib/CalDAV/CachedSubscriptionObject.php" , line : 64usize , placeholders : & [] , regex : "^Deleting objects in a cached subscription is not allowed$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Error , path : "/apps/dav/lib/CalDAV/WebcalCaching/RefreshWebcalService.php" , line : 131usize , placeholders : & [] , regex : "^Unable to create calendar object from subscription \\{subscriptionId\\}$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Error , path : "/apps/dav/lib/CalDAV/WebcalCaching/RefreshWebcalService.php" , line : 142usize , placeholders : & [] , regex : "^Subscription \\{subscriptionId\\} could not be refreshed due to a parsing error$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Warn , path : "/apps/dav/lib/CalDAV/WebcalCaching/RefreshWebcalService.php" , line : 229usize , placeholders : & ["$subscriptionId"] , regex : "^Subscription (.*) could not be parsed$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Warn , path : "/apps/dav/lib/CalDAV/WebcalCaching/RefreshWebcalService.php" , line : 239usize , placeholders : & ["$subscriptionId"] , regex : "^Subscription (.*) could not be parsed$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Warn , path : "/apps/dav/lib/CalDAV/WebcalCaching/RefreshWebcalService.php" , line : 250usize , placeholders : & ["$subscriptionId"] , regex : "^Subscription (.*) could not be parsed$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Warn , path : "/apps/dav/lib/CalDAV/WebcalCaching/RefreshWebcalService.php" , line : 256usize , placeholders : & ["$subscriptionId"] , regex : "^Subscription (.*) was not refreshed because it violates local access rules$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Warn , path : "/apps/dav/lib/CalDAV/WebcalCaching/RefreshWebcalService.php" , line : 262usize , placeholders : & ["$subscriptionId"] , regex : "^Subscription (.*) could not be refreshed due to a network error$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Debug , path : "/apps/dav/lib/CalDAV/Schedule/Plugin.php" , line : 228usize , placeholders : & [] , regex : "^Calendar user type is room or resource, not processing further$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Debug , path : "/apps/dav/lib/CalDAV/Schedule/Plugin.php" , line : 234usize , placeholders : & [] , regex : "^No attendee set for scheduling message$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Debug , path : "/apps/dav/lib/CalDAV/Schedule/Plugin.php" , line : 246usize , placeholders : & [] , regex : "^No VEVENT set to process on scheduling message$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Debug , path : "/apps/dav/lib/CalDAV/Schedule/Plugin.php" , line : 252usize , placeholders : & [] , regex : "^VEVENT is a recurring event, autoresponding not supported$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Warn , path : "/apps/dav/lib/CalDAV/Schedule/IMipPlugin.php" , line : 181usize , placeholders : & [] , regex : "^iTip message said the change was significant but comparison did not detect any updated VEvents$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Debug , path : "/apps/dav/lib/CalDAV/Schedule/IMipPlugin.php" , line : 200usize , placeholders : & [] , regex : "^No invitation sent as recipient is room or resource$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Error , path : "/apps/dav/lib/CalDAV/Schedule/IMipPlugin.php" , line : 317usize , placeholders : & [] , regex : "^Unable to deliver message to \\{failed\\}$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/dav/lib/CalDAV/PublicCalendar.php" , line : 40usize , placeholders : & [] , regex : "^Calendar object not found$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/dav/lib/CalDAV/PublicCalendar.php" , line : 43usize , placeholders : & [] , regex : "^Calendar object not found$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/dav/lib/CalDAV/CalendarHome.php" , line : 93usize , placeholders : & [] , regex : "^The resource you tried to create has a reserved name$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/dav/lib/CalDAV/CalDavBackend.php" , line : 790usize , placeholders : & [] , regex : "^URI too long\\. Calendar not created$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/dav/lib/CalDAV/CalDavBackend.php" , line : 976usize , placeholders : & [] , regex : "^Calendar data that was just written can't be read back\\. Check your database configuration\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/dav/lib/CalDAV/CalDavBackend.php" , line : 1268usize , placeholders : & [] , regex : "^Calendar object with uid already exists in this calendar collection\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/dav/lib/CalDAV/CalDavBackend.php" , line : 1518usize , placeholders : & ["$newUri" , "$calendarId"] , regex : "^A calendar object with URI (.*) already exists in calendar (.*), therefore this object can't be moved into the trashbin$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/dav/lib/CalDAV/CalDavBackend.php" , line : 1564usize , placeholders : & ["$id" , "$restoreUri"] , regex : "^Can not restore calendar (.*) because a calendar object with the URI (.*) already exists$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/dav/lib/CalDAV/CalDavBackend.php" , line : 1591usize , placeholders : & [] , regex : "^Calendar object data that was just written can't be read back\\. Check your database configuration\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/dav/lib/CalDAV/CalDavBackend.php" , line : 2578usize , placeholders : & [] , regex : "^The \\{http://calendarserver\\.org/ns/\\}source property is required when creating subscriptions$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Info , path : "/apps/dav/lib/CalDAV/CalDavBackend.php" , line : 2828usize , placeholders : & ["$limit"] , regex : "^Deleted (.*) scheduling objects, continuing with next batch$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/dav/lib/CalDAV/CalDavBackend.php" , line : 2984usize , placeholders : & [] , regex : "^Calendar objects must have a VJOURNAL, VEVENT or VTODO component$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/dav/lib/CalDAV/AppCalendar/CalendarObject.php" , line : 76usize , placeholders : & [] , regex : "^Setting ACL is not supported on this node$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/dav/lib/CalDAV/AppCalendar/CalendarObject.php" , line : 90usize , placeholders : & [] , regex : "^This calendar\\-object is read\\-only$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/dav/lib/CalDAV/AppCalendar/CalendarObject.php" , line : 123usize , placeholders : & [] , regex : "^This calendar\\-object is read\\-only$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/dav/lib/CalDAV/AppCalendar/CalendarObject.php" , line : 132usize , placeholders : & [] , regex : "^Invalid node$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/dav/lib/CalDAV/AppCalendar/CalendarObject.php" , line : 141usize , placeholders : & [] , regex : "^This calendar\\-object is read\\-only$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/dav/lib/CalDAV/AppCalendar/AppCalendar.php" , line : 96usize , placeholders : & [] , regex : "^Setting ACL is not supported on this node$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/dav/lib/CalDAV/AppCalendar/AppCalendar.php" , line : 111usize , placeholders : & [] , regex : "^Deleting an entry is not implemented$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/dav/lib/CalDAV/AppCalendar/AppCalendar.php" , line : 122usize , placeholders : & [] , regex : "^Creating a new entry is not allowed$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/dav/lib/CalDAV/AppCalendar/AppCalendar.php" , line : 182usize , placeholders : & [] , regex : "^Node not found$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Debug , path : "/apps/dav/lib/BackgroundJob/UserStatusAutomation.php" , line : 160usize , placeholders : & [] , regex : "^User is currently available, reverting DND status if applicable$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Debug , path : "/apps/dav/lib/BackgroundJob/UserStatusAutomation.php" , line : 163usize , placeholders : & [] , regex : "^User is currently NOT available, reverting call status if applicable and then setting DND$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Debug , path : "/apps/dav/lib/BackgroundJob/UserStatusAutomation.php" , line : 168usize , placeholders : & [] , regex : "^User status automation ran$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Info , path : "/apps/dav/lib/BackgroundJob/PruneOutdatedSyncTokensJob.php" , line : 60usize , placeholders : & [] , regex : "^Pruned \\{calendarSyncTokensNumber\\} calendar sync tokens and \\{addressBooksSyncTokensNumber\\} address book sync tokens$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Info , path : "/apps/dav/lib/BackgroundJob/BuildReminderIndexBackgroundJob.php" , line : 83usize , placeholders : & [] , regex : "^Building calendar reminder index done$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Warn , path : "/apps/dav/lib/BackgroundJob/RefreshWebcalJob.php" , line : 99usize , placeholders : & ["$subscriptionId"] , regex : "^Subscription (.*) could not be refreshed, refreshrate in database is invalid$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Info , path : "/apps/dav/lib/BackgroundJob/DeleteOutdatedSchedulingObjects.php" , line : 33usize , placeholders : & [] , regex : "^Removed outdated scheduling objects$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/dav/lib/UserMigration/ContactsMigrator.php" , line : 173usize , placeholders : & [] , regex : "^Failed to get unique address book URI$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/dav/lib/UserMigration/ContactsMigrator.php" , line : 254usize , placeholders : & [] , regex : "^Could not export address book$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/dav/lib/UserMigration/CalendarMigrator.php" , line : 193usize , placeholders : & [] , regex : "^Failed to get unique calendar URI$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/dav/lib/UserMigration/CalendarMigrator.php" , line : 261usize , placeholders : & [] , regex : "^Could not export calendars$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/dav/lib/UserMigration/CalendarMigrator.php" , line : 460usize , placeholders : & ["$importPath"] , regex : "^Failed to read file \"(.*)\"$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/dav/lib/UserMigration/CalendarMigrator.php" , line : 465usize , placeholders : & ["$importPath"] , regex : "^Invalid calendar data contained in \"(.*)\"$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Debug , path : "/apps/dav/lib/Listener/CalendarDeletionDefaultUpdaterListener.php" , line : 65usize , placeholders : & [] , regex : "^Default calendar needs no update because the deleted calendar does not belong to a user principal$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Debug , path : "/apps/dav/lib/Listener/CalendarDeletionDefaultUpdaterListener.php" , line : 72usize , placeholders : & [] , regex : "^Default calendar needs no update because the deleted calendar is no the user's default one$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Debug , path : "/apps/dav/lib/Listener/CalendarDeletionDefaultUpdaterListener.php" , line : 78usize , placeholders : & [] , regex : "^Default user calendar reset$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Debug , path : "/apps/dav/lib/Listener/CalendarShareUpdateListener.php" , line : 53usize , placeholders : & [] , regex : "^Creating activity for Calendar having its shares updated$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Debug , path : "/apps/dav/lib/Listener/CalendarPublicationListener.php" , line : 50usize , placeholders : & [] , regex : "^Creating activity for Calendar being published$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Debug , path : "/apps/dav/lib/Listener/CalendarPublicationListener.php" , line : 57usize , placeholders : & [] , regex : "^Creating activity for Calendar being unpublished$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/provisioning_api/composer/composer/ClassLoader.php" , line : 256usize , placeholders : & [] , regex : "^A non\\-empty PSR\\-4 prefix must end with a namespace separator\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/provisioning_api/composer/composer/ClassLoader.php" , line : 311usize , placeholders : & [] , regex : "^A non\\-empty PSR\\-4 prefix must end with a namespace separator\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/provisioning_api/lib/Controller/AUserData.php" , line : 116usize , placeholders : & [] , regex : "^User does not exist$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/provisioning_api/lib/Controller/AUserData.php" , line : 235usize , placeholders : & [] , regex : "^User does not exist$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/provisioning_api/lib/Controller/AUserData.php" , line : 269usize , placeholders : & [] , regex : "^User does not exist$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Error , path : "/apps/provisioning_api/lib/Controller/AUserData.php" , line : 280usize , placeholders : & [] , regex : "^Could not load storage info for \\{user\\}$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/provisioning_api/lib/Controller/AppsController.php" , line : 75usize , placeholders : & [] , regex : "^$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/provisioning_api/lib/Controller/AppsController.php" , line : 93usize , placeholders : & [] , regex : "^The request app was not found$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/provisioning_api/lib/Controller/AppsController.php" , line : 106usize , placeholders : & [] , regex : "^The request app was not found$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/provisioning_api/lib/Controller/UsersController.php" , line : 315usize , placeholders : & [] , regex : "^Could not create non\\-existing user id$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Error , path : "/apps/provisioning_api/lib/Controller/UsersController.php" , line : 353usize , placeholders : & [] , regex : "^Failed addUser attempt: User already exists\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/provisioning_api/lib/Controller/UsersController.php" , line : 368usize , placeholders : & [] , regex : "^no group specified \\(required for subadmins\\)$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/provisioning_api/lib/Controller/UsersController.php" , line : 378usize , placeholders : & [] , regex : "^Subadmin group does not exist$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/provisioning_api/lib/Controller/UsersController.php" , line : 382usize , placeholders : & [] , regex : "^Cannot create subadmins for admin group$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/provisioning_api/lib/Controller/UsersController.php" , line : 386usize , placeholders : & [] , regex : "^No permissions to promote subadmins$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/provisioning_api/lib/Controller/UsersController.php" , line : 394usize , placeholders : & [] , regex : "^Invalid password value$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/provisioning_api/lib/Controller/UsersController.php" , line : 398usize , placeholders : & [] , regex : "^To send a password link to the user an email address is required\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/provisioning_api/lib/Controller/UsersController.php" , line : 417usize , placeholders : & [] , regex : "^Required email address was not provided$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Error , path : "/apps/provisioning_api/lib/Controller/UsersController.php" , line : 470usize , placeholders : & ["$email"] , regex : "^Unable to send the invitation mail to (.*)$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Warn , path : "/apps/provisioning_api/lib/Controller/UsersController.php" , line : 483usize , placeholders : & [] , regex : "^Failed addUser attempt with hint exception\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Warn , path : "/apps/provisioning_api/lib/Controller/UsersController.php" , line : 492usize , placeholders : & [] , regex : "^Failed addUser attempt with ocs exception\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Error , path : "/apps/provisioning_api/lib/Controller/UsersController.php" , line : 501usize , placeholders : & [] , regex : "^Failed addUser attempt with invalid argument exception\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Error , path : "/apps/provisioning_api/lib/Controller/UsersController.php" , line : 510usize , placeholders : & [] , regex : "^Failed addUser attempt with exception\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/provisioning_api/lib/Controller/UsersController.php" , line : 517usize , placeholders : & [] , regex : "^Bad request$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/provisioning_api/lib/Controller/UsersController.php" , line : 541usize , placeholders : & [] , regex : "^$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/provisioning_api/lib/Controller/UsersController.php" , line : 562usize , placeholders : & [] , regex : "^$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/provisioning_api/lib/Controller/UsersController.php" , line : 575usize , placeholders : & [] , regex : "^$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/provisioning_api/lib/Controller/UsersController.php" , line : 592usize , placeholders : & [] , regex : "^$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/provisioning_api/lib/Controller/UsersController.php" , line : 600usize , placeholders : & [] , regex : "^$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/provisioning_api/lib/Controller/UsersController.php" , line : 608usize , placeholders : & [] , regex : "^$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/provisioning_api/lib/Controller/UsersController.php" , line : 656usize , placeholders : & [] , regex : "^$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/provisioning_api/lib/Controller/UsersController.php" , line : 661usize , placeholders : & [] , regex : "^$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/provisioning_api/lib/Controller/UsersController.php" , line : 680usize , placeholders : & [] , regex : "^$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/provisioning_api/lib/Controller/UsersController.php" , line : 686usize , placeholders : & [] , regex : "^$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/provisioning_api/lib/Controller/UsersController.php" , line : 720usize , placeholders : & [] , regex : "^$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/provisioning_api/lib/Controller/UsersController.php" , line : 723usize , placeholders : & [] , regex : "^$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/provisioning_api/lib/Controller/UsersController.php" , line : 728usize , placeholders : & [] , regex : "^$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/provisioning_api/lib/Controller/UsersController.php" , line : 752usize , placeholders : & [] , regex : "^$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/provisioning_api/lib/Controller/UsersController.php" , line : 854usize , placeholders : & [] , regex : "^$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/provisioning_api/lib/Controller/UsersController.php" , line : 859usize , placeholders : & [] , regex : "^$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/provisioning_api/lib/Controller/UsersController.php" , line : 896usize , placeholders : & [] , regex : "^Unlimited quota is forbidden on this instance$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/provisioning_api/lib/Controller/UsersController.php" , line : 907usize , placeholders : & [] , regex : "^Invalid password value$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/provisioning_api/lib/Controller/UsersController.php" , line : 910usize , placeholders : & [] , regex : "^Setting the password is not supported by the users backend$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/provisioning_api/lib/Controller/UsersController.php" , line : 920usize , placeholders : & [] , regex : "^Invalid language$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/provisioning_api/lib/Controller/UsersController.php" , line : 926usize , placeholders : & [] , regex : "^Invalid locale$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Info , path : "/apps/provisioning_api/lib/Controller/UsersController.php" , line : 937usize , placeholders : & [] , regex : "^Cannot set primary email, because provided address is not verified$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/provisioning_api/lib/Controller/UsersController.php" , line : 947usize , placeholders : & [] , regex : "^$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/provisioning_api/lib/Controller/UsersController.php" , line : 954usize , placeholders : & [] , regex : "^$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/provisioning_api/lib/Controller/UsersController.php" , line : 963usize , placeholders : & [] , regex : "^$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/provisioning_api/lib/Controller/UsersController.php" , line : 969usize , placeholders : & [] , regex : "^$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/provisioning_api/lib/Controller/UsersController.php" , line : 1041usize , placeholders : & [] , regex : "^$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/provisioning_api/lib/Controller/UsersController.php" , line : 1063usize , placeholders : & [] , regex : "^$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/provisioning_api/lib/Controller/UsersController.php" , line : 1067usize , placeholders : & [] , regex : "^$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/provisioning_api/lib/Controller/UsersController.php" , line : 1073usize , placeholders : & [] , regex : "^$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/provisioning_api/lib/Controller/UsersController.php" , line : 1095usize , placeholders : & [] , regex : "^$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/provisioning_api/lib/Controller/UsersController.php" , line : 1099usize , placeholders : & [] , regex : "^$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/provisioning_api/lib/Controller/UsersController.php" , line : 1105usize , placeholders : & [] , regex : "^$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/provisioning_api/lib/Controller/UsersController.php" , line : 1112usize , placeholders : & [] , regex : "^$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/provisioning_api/lib/Controller/UsersController.php" , line : 1153usize , placeholders : & [] , regex : "^$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/provisioning_api/lib/Controller/UsersController.php" , line : 1159usize , placeholders : & [] , regex : "^$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/provisioning_api/lib/Controller/UsersController.php" , line : 1180usize , placeholders : & [] , regex : "^$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/provisioning_api/lib/Controller/UsersController.php" , line : 1206usize , placeholders : & [] , regex : "^$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/provisioning_api/lib/Controller/UsersController.php" , line : 1222usize , placeholders : & [] , regex : "^$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/provisioning_api/lib/Controller/UsersController.php" , line : 1228usize , placeholders : & [] , regex : "^$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/provisioning_api/lib/Controller/UsersController.php" , line : 1231usize , placeholders : & [] , regex : "^$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/provisioning_api/lib/Controller/UsersController.php" , line : 1238usize , placeholders : & [] , regex : "^$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/provisioning_api/lib/Controller/UsersController.php" , line : 1259usize , placeholders : & [] , regex : "^$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/provisioning_api/lib/Controller/UsersController.php" , line : 1264usize , placeholders : & [] , regex : "^$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/provisioning_api/lib/Controller/UsersController.php" , line : 1269usize , placeholders : & [] , regex : "^$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/provisioning_api/lib/Controller/UsersController.php" , line : 1275usize , placeholders : & [] , regex : "^$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/provisioning_api/lib/Controller/UsersController.php" , line : 1282usize , placeholders : & [] , regex : "^Cannot remove yourself from the admin group$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/provisioning_api/lib/Controller/UsersController.php" , line : 1286usize , placeholders : & [] , regex : "^Cannot remove yourself from this group as you are a SubAdmin$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/provisioning_api/lib/Controller/UsersController.php" , line : 1299usize , placeholders : & [] , regex : "^Not viable to remove user from the last group you are SubAdmin of$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/provisioning_api/lib/Controller/UsersController.php" , line : 1324usize , placeholders : & [] , regex : "^User does not exist$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/provisioning_api/lib/Controller/UsersController.php" , line : 1328usize , placeholders : & [] , regex : "^Group does not exist$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/provisioning_api/lib/Controller/UsersController.php" , line : 1332usize , placeholders : & [] , regex : "^Cannot create subadmins for admin group$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/provisioning_api/lib/Controller/UsersController.php" , line : 1363usize , placeholders : & [] , regex : "^User does not exist$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/provisioning_api/lib/Controller/UsersController.php" , line : 1367usize , placeholders : & [] , regex : "^Group does not exist$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/provisioning_api/lib/Controller/UsersController.php" , line : 1371usize , placeholders : & [] , regex : "^User is not a subadmin of this group$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/provisioning_api/lib/Controller/UsersController.php" , line : 1406usize , placeholders : & [] , regex : "^$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/provisioning_api/lib/Controller/UsersController.php" , line : 1416usize , placeholders : & [] , regex : "^$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/provisioning_api/lib/Controller/UsersController.php" , line : 1421usize , placeholders : & [] , regex : "^Email address not available$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Error , path : "/apps/provisioning_api/lib/Controller/UsersController.php" , line : 1428usize , placeholders : & ["$email"] , regex : "^Can't send new user mail to (.*)$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/provisioning_api/lib/Controller/UsersController.php" , line : 1435usize , placeholders : & [] , regex : "^Sending email failed$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/provisioning_api/lib/Controller/GroupsController.php" , line : 159usize , placeholders : & [] , regex : "^The requested group could not be found$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/provisioning_api/lib/Controller/GroupsController.php" , line : 198usize , placeholders : & [] , regex : "^The requested group could not be found$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/provisioning_api/lib/Controller/GroupsController.php" , line : 227usize , placeholders : & [] , regex : "^The requested group could not be found$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Error , path : "/apps/provisioning_api/lib/Controller/GroupsController.php" , line : 243usize , placeholders : & [] , regex : "^Group name not supplied$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/provisioning_api/lib/Controller/GroupsController.php" , line : 244usize , placeholders : & [] , regex : "^Invalid group name$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/provisioning_api/lib/Controller/GroupsController.php" , line : 248usize , placeholders : & [] , regex : "^group exists$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/provisioning_api/lib/Controller/GroupsController.php" , line : 252usize , placeholders : & [] , regex : "^Not supported by backend$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/provisioning_api/lib/Controller/GroupsController.php" , line : 275usize , placeholders : & [] , regex : "^Group does not exist$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/provisioning_api/lib/Controller/GroupsController.php" , line : 281usize , placeholders : & [] , regex : "^Not supported by backend$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/provisioning_api/lib/Controller/GroupsController.php" , line : 283usize , placeholders : & [] , regex : "^$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/provisioning_api/lib/Controller/GroupsController.php" , line : 299usize , placeholders : & [] , regex : "^$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/provisioning_api/lib/Controller/GroupsController.php" , line : 302usize , placeholders : & [] , regex : "^$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/provisioning_api/lib/Controller/GroupsController.php" , line : 317usize , placeholders : & [] , regex : "^Group does not exist$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/provisioning_api/lib/Controller/VerificationController.php" , line : 83usize , placeholders : & [] , regex : "^Logged in user is not mail address owner$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/provisioning_api/lib/Controller/VerificationController.php" , line : 104usize , placeholders : & [] , regex : "^Logged in user is not mail address owner$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/provisioning_api/lib/Controller/AppConfigController.php" , line : 139usize , placeholders : & [] , regex : "^User is not logged in\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/provisioning_api/lib/Controller/AppConfigController.php" , line : 181usize , placeholders : & [] , regex : "^Invalid app id given$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/provisioning_api/lib/Controller/AppConfigController.php" , line : 193usize , placeholders : & [] , regex : "^The given key can not be set$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/provisioning_api/lib/Controller/AppConfigController.php" , line : 197usize , placeholders : & [] , regex : "^The given key can not be set$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/provisioning_api/lib/Controller/AppConfigController.php" , line : 201usize , placeholders : & [] , regex : "^The given key can not be set$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/provisioning_api/lib/Controller/AppConfigController.php" , line : 208usize , placeholders : & [] , regex : "^The given key can not be set, unlimited quota is forbidden on this instance$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/updatenotification/lib/Notification/Notifier.php" , line : 113usize , placeholders : & [] , regex : "^Unknown app id$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/updatenotification/lib/Notification/Notifier.php" , line : 121usize , placeholders : & [] , regex : "^Update checked worked again$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/updatenotification/composer/composer/ClassLoader.php" , line : 256usize , placeholders : & [] , regex : "^A non\\-empty PSR\\-4 prefix must end with a namespace separator\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/updatenotification/composer/composer/ClassLoader.php" , line : 311usize , placeholders : & [] , regex : "^A non\\-empty PSR\\-4 prefix must end with a namespace separator\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/suspicious_login/vendor/composer/ClassLoader.php" , line : 250usize , placeholders : & [] , regex : "^A non\\-empty PSR\\-4 prefix must end with a namespace separator\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/suspicious_login/vendor/composer/ClassLoader.php" , line : 305usize , placeholders : & [] , regex : "^A non\\-empty PSR\\-4 prefix must end with a namespace separator\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/suspicious_login/vendor/league/flysystem/src/ProxyArrayAccessToProperties.php" , line : 51usize , placeholders : & [] , regex : "^Properties can not be manipulated$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/suspicious_login/vendor/league/flysystem/src/ProxyArrayAccessToProperties.php" , line : 60usize , placeholders : & [] , regex : "^Properties can not be manipulated$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Unknown , path : "/apps/suspicious_login/vendor/psr/log/Psr/Log/Test/LoggerInterfaceTest.php" , line : 74usize , placeholders : & [] , regex : "^invalid level$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Info , path : "/apps/suspicious_login/vendor/psr/log/Psr/Log/Test/LoggerInterfaceTest.php" , line : 80usize , placeholders : & [] , regex : "^\\{Message \\{nothing\\} \\{user\\} \\{foo\\.bar\\} a\\}$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Warn , path : "/apps/suspicious_login/vendor/psr/log/Psr/Log/Test/LoggerInterfaceTest.php" , line : 120usize , placeholders : & [] , regex : "^Crazy context data$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Warn , path : "/apps/suspicious_login/vendor/psr/log/Psr/Log/Test/LoggerInterfaceTest.php" , line : 129usize , placeholders : & [] , regex : "^Random message$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Critical , path : "/apps/suspicious_login/vendor/psr/log/Psr/Log/Test/LoggerInterfaceTest.php" , line : 130usize , placeholders : & [] , regex : "^Uncaught Exception!$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/suspicious_login/vendor/rubix/ml/src/Classifiers/KDNeighbors.php" , line : 190usize , placeholders : & [] , regex : "^Estimator has not been trained\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/suspicious_login/vendor/rubix/ml/src/Classifiers/KDNeighbors.php" , line : 233usize , placeholders : & [] , regex : "^Estimator has not been trained\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/suspicious_login/vendor/rubix/ml/src/Classifiers/SVC.php" , line : 232usize , placeholders : & [] , regex : "^Estimator has not been trained\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/suspicious_login/vendor/rubix/ml/src/Classifiers/SVC.php" , line : 249usize , placeholders : & [] , regex : "^Learner must be trained before saving\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/suspicious_login/vendor/rubix/ml/src/Classifiers/ClassificationTree.php" , line : 155usize , placeholders : & [] , regex : "^Estimator has not been trained\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/suspicious_login/vendor/rubix/ml/src/Classifiers/ClassificationTree.php" , line : 189usize , placeholders : & [] , regex : "^Estimator has not been trained\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Info , path : "/apps/suspicious_login/vendor/rubix/ml/src/Classifiers/LogisticRegression.php" , line : 310usize , placeholders : & ["$this"] , regex : "^(.*) initialized$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Info , path : "/apps/suspicious_login/vendor/rubix/ml/src/Classifiers/LogisticRegression.php" , line : 329usize , placeholders : & [] , regex : "^Numerical instability detected$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Info , path : "/apps/suspicious_login/vendor/rubix/ml/src/Classifiers/LogisticRegression.php" , line : 340usize , placeholders : & ["$epoch" , "$this->costFn" , "$loss"] , regex : "^Epoch (.*) \\- (.*): (.*)$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Info , path : "/apps/suspicious_login/vendor/rubix/ml/src/Classifiers/LogisticRegression.php" , line : 367usize , placeholders : & [] , regex : "^Training complete$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/suspicious_login/vendor/rubix/ml/src/Classifiers/LogisticRegression.php" , line : 392usize , placeholders : & [] , regex : "^Estimator has not been trained\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/suspicious_login/vendor/rubix/ml/src/Classifiers/LogisticRegression.php" , line : 422usize , placeholders : & [] , regex : "^Estimator has not been trained\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/suspicious_login/vendor/rubix/ml/src/Classifiers/LogisticRegression.php" , line : 428usize , placeholders : & [] , regex : "^Weight layer not found\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Info , path : "/apps/suspicious_login/vendor/rubix/ml/src/Classifiers/AdaBoost.php" , line : 289usize , placeholders : & ["$this"] , regex : "^(.*) initialized$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Info , path : "/apps/suspicious_login/vendor/rubix/ml/src/Classifiers/AdaBoost.php" , line : 330usize , placeholders : & [] , regex : "^Numerical instability detected$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Info , path : "/apps/suspicious_login/vendor/rubix/ml/src/Classifiers/AdaBoost.php" , line : 343usize , placeholders : & ["$epoch" , "$loss"] , regex : "^Epoch (.*) \\- Exp Loss: (.*)$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Info , path : "/apps/suspicious_login/vendor/rubix/ml/src/Classifiers/AdaBoost.php" , line : 396usize , placeholders : & [] , regex : "^Training complete$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/suspicious_login/vendor/rubix/ml/src/Classifiers/AdaBoost.php" , line : 446usize , placeholders : & [] , regex : "^Estimator has not been trained\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/suspicious_login/vendor/rubix/ml/src/Classifiers/ExtraTreeClassifier.php" , line : 156usize , placeholders : & [] , regex : "^Estimator has not been trained\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/suspicious_login/vendor/rubix/ml/src/Classifiers/ExtraTreeClassifier.php" , line : 190usize , placeholders : & [] , regex : "^Estimator has not been trained\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/suspicious_login/vendor/rubix/ml/src/Classifiers/GaussianNB.php" , line : 294usize , placeholders : & [] , regex : "^Estimator has not been trained\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/suspicious_login/vendor/rubix/ml/src/Classifiers/GaussianNB.php" , line : 325usize , placeholders : & [] , regex : "^Estimator has not been trained\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/suspicious_login/vendor/rubix/ml/src/Classifiers/DummyClassifier.php" , line : 131usize , placeholders : & [] , regex : "^Estimator has not been trained\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/suspicious_login/vendor/rubix/ml/src/Classifiers/RadiusNeighbors.php" , line : 219usize , placeholders : & [] , regex : "^Estimator has not been trained\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/suspicious_login/vendor/rubix/ml/src/Classifiers/RadiusNeighbors.php" , line : 266usize , placeholders : & [] , regex : "^Estimator has not been trained\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/suspicious_login/vendor/rubix/ml/src/Classifiers/KNearestNeighbors.php" , line : 208usize , placeholders : & [] , regex : "^Estimator has not been trained\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/suspicious_login/vendor/rubix/ml/src/Classifiers/KNearestNeighbors.php" , line : 251usize , placeholders : & [] , regex : "^Estimator has not been trained\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Info , path : "/apps/suspicious_login/vendor/rubix/ml/src/Classifiers/SoftmaxClassifier.php" , line : 306usize , placeholders : & ["$this"] , regex : "^(.*) initialized$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Info , path : "/apps/suspicious_login/vendor/rubix/ml/src/Classifiers/SoftmaxClassifier.php" , line : 325usize , placeholders : & [] , regex : "^Numerical instability detected$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Info , path : "/apps/suspicious_login/vendor/rubix/ml/src/Classifiers/SoftmaxClassifier.php" , line : 336usize , placeholders : & ["$epoch" , "$this->costFn" , "$loss"] , regex : "^Epoch (.*) \\- (.*): (.*)$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Info , path : "/apps/suspicious_login/vendor/rubix/ml/src/Classifiers/SoftmaxClassifier.php" , line : 363usize , placeholders : & [] , regex : "^Training complete$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/suspicious_login/vendor/rubix/ml/src/Classifiers/SoftmaxClassifier.php" , line : 388usize , placeholders : & [] , regex : "^Estimator has not been trained\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/suspicious_login/vendor/rubix/ml/src/Classifiers/RandomForest.php" , line : 266usize , placeholders : & [] , regex : "^Estimator has not been trained\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/suspicious_login/vendor/rubix/ml/src/Classifiers/RandomForest.php" , line : 298usize , placeholders : & [] , regex : "^Estimator has not been trained\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/suspicious_login/vendor/rubix/ml/src/Classifiers/RandomForest.php" , line : 340usize , placeholders : & [] , regex : "^Estimator has not been trained\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/suspicious_login/vendor/rubix/ml/src/Classifiers/NaiveBayes.php" , line : 295usize , placeholders : & [] , regex : "^Estimator has not been trained\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/suspicious_login/vendor/rubix/ml/src/Classifiers/NaiveBayes.php" , line : 326usize , placeholders : & [] , regex : "^Estimator has not been trained\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Info , path : "/apps/suspicious_login/vendor/rubix/ml/src/Classifiers/MultilayerPerceptron.php" , line : 391usize , placeholders : & ["$this"] , regex : "^(.*) initialized$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Info , path : "/apps/suspicious_login/vendor/rubix/ml/src/Classifiers/MultilayerPerceptron.php" , line : 417usize , placeholders : & [] , regex : "^Numerical instability detected$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Info , path : "/apps/suspicious_login/vendor/rubix/ml/src/Classifiers/MultilayerPerceptron.php" , line : 476usize , placeholders : & ["$bestEpoch"] , regex : "^Network restored from snapshot at epoch (.*)$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Info , path : "/apps/suspicious_login/vendor/rubix/ml/src/Classifiers/MultilayerPerceptron.php" , line : 481usize , placeholders : & [] , regex : "^Training complete$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/suspicious_login/vendor/rubix/ml/src/Classifiers/MultilayerPerceptron.php" , line : 506usize , placeholders : & [] , regex : "^Estimator has not been trained\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/suspicious_login/vendor/rubix/ml/src/Encoding.php" , line : 59usize , placeholders : & [] , regex : "^Folder does not exist or is not writable$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/suspicious_login/vendor/rubix/ml/src/Encoding.php" , line : 63usize , placeholders : & ["$path"] , regex : "^File (.*) is not writable\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/suspicious_login/vendor/rubix/ml/src/Encoding.php" , line : 69usize , placeholders : & [] , regex : "^Failed to write to the filesystem\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/suspicious_login/vendor/rubix/ml/src/AnomalyDetectors/Loda.php" , line : 345usize , placeholders : & [] , regex : "^Estimator has not been trained\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/suspicious_login/vendor/rubix/ml/src/AnomalyDetectors/GaussianMLE.php" , line : 260usize , placeholders : & [] , regex : "^Estimator has not been trained\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/suspicious_login/vendor/rubix/ml/src/AnomalyDetectors/GaussianMLE.php" , line : 291usize , placeholders : & [] , regex : "^Estimator has not been trained\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/suspicious_login/vendor/rubix/ml/src/AnomalyDetectors/IsolationForest.php" , line : 261usize , placeholders : & [] , regex : "^Estimator has not been trained\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/suspicious_login/vendor/rubix/ml/src/AnomalyDetectors/IsolationForest.php" , line : 292usize , placeholders : & [] , regex : "^Estimator has not been trained\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/suspicious_login/vendor/rubix/ml/src/AnomalyDetectors/LocalOutlierFactor.php" , line : 246usize , placeholders : & [] , regex : "^Estimator has not been trained\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/suspicious_login/vendor/rubix/ml/src/AnomalyDetectors/LocalOutlierFactor.php" , line : 277usize , placeholders : & [] , regex : "^Estimator has not been trained\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/suspicious_login/vendor/rubix/ml/src/AnomalyDetectors/RobustZScore.php" , line : 213usize , placeholders : & [] , regex : "^Estimator has not been trained\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/suspicious_login/vendor/rubix/ml/src/AnomalyDetectors/RobustZScore.php" , line : 242usize , placeholders : & [] , regex : "^Estimator has not been trained\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/suspicious_login/vendor/rubix/ml/src/AnomalyDetectors/OneClassSVM.php" , line : 207usize , placeholders : & [] , regex : "^Estimator has not been trained\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/suspicious_login/vendor/rubix/ml/src/AnomalyDetectors/OneClassSVM.php" , line : 222usize , placeholders : & [] , regex : "^Learner must be trained before saving\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/suspicious_login/vendor/rubix/ml/src/NeuralNet/Layers/Dense.php" , line : 142usize , placeholders : & [] , regex : "^Layer is not initialized$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/suspicious_login/vendor/rubix/ml/src/NeuralNet/Layers/Dense.php" , line : 186usize , placeholders : & [] , regex : "^Layer is not initialized$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/suspicious_login/vendor/rubix/ml/src/NeuralNet/Layers/Dense.php" , line : 212usize , placeholders : & [] , regex : "^Layer is not initialized$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/suspicious_login/vendor/rubix/ml/src/NeuralNet/Layers/Dense.php" , line : 237usize , placeholders : & [] , regex : "^Layer has not been initialized\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/suspicious_login/vendor/rubix/ml/src/NeuralNet/Layers/Dense.php" , line : 293usize , placeholders : & [] , regex : "^Layer has not been initialized\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/suspicious_login/vendor/rubix/ml/src/NeuralNet/Layers/PReLU.php" , line : 77usize , placeholders : & [] , regex : "^Layer has not been initialized\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/suspicious_login/vendor/rubix/ml/src/NeuralNet/Layers/PReLU.php" , line : 146usize , placeholders : & [] , regex : "^Layer has not been initialized\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/suspicious_login/vendor/rubix/ml/src/NeuralNet/Layers/PReLU.php" , line : 194usize , placeholders : & [] , regex : "^Layer has not been initialized\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/suspicious_login/vendor/rubix/ml/src/NeuralNet/Layers/PReLU.php" , line : 222usize , placeholders : & [] , regex : "^Layer has not been initialized\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/suspicious_login/vendor/rubix/ml/src/NeuralNet/Layers/PReLU.php" , line : 256usize , placeholders : & [] , regex : "^Layer has not been initialized\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/suspicious_login/vendor/rubix/ml/src/NeuralNet/Layers/Activation.php" , line : 70usize , placeholders : & [] , regex : "^Layer has not been initialized\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/suspicious_login/vendor/rubix/ml/src/NeuralNet/Layers/Noise.php" , line : 67usize , placeholders : & [] , regex : "^Layer has not been initialized\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/suspicious_login/vendor/rubix/ml/src/NeuralNet/Layers/Dropout.php" , line : 83usize , placeholders : & [] , regex : "^Layer has not been initialized\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/suspicious_login/vendor/rubix/ml/src/NeuralNet/Layers/Dropout.php" , line : 150usize , placeholders : & [] , regex : "^Must perform forward pass before backpropagating\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/suspicious_login/vendor/rubix/ml/src/NeuralNet/Layers/BatchNorm.php" , line : 139usize , placeholders : & [] , regex : "^Layer has not been initialized\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/suspicious_login/vendor/rubix/ml/src/NeuralNet/Layers/BatchNorm.php" , line : 181usize , placeholders : & [] , regex : "^Layer has not been initialized\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/suspicious_login/vendor/rubix/ml/src/NeuralNet/Layers/BatchNorm.php" , line : 220usize , placeholders : & [] , regex : "^Layer has not been initialized\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/suspicious_login/vendor/rubix/ml/src/NeuralNet/Layers/BatchNorm.php" , line : 243usize , placeholders : & [] , regex : "^Layer has not been initialized\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/suspicious_login/vendor/rubix/ml/src/NeuralNet/Layers/BatchNorm.php" , line : 308usize , placeholders : & [] , regex : "^Layer has not been initialized\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/suspicious_login/vendor/rubix/ml/src/Other/Tokenizers/NGram.php" , line : 66usize , placeholders : & [] , regex : "^Minimum cannot be less than 1\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/suspicious_login/vendor/rubix/ml/src/Other/Tokenizers/NGram.php" , line : 70usize , placeholders : & [] , regex : "^Maximum cannot be less than minimum\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/suspicious_login/vendor/rubix/ml/src/Other/Tokenizers/KSkipNGram.php" , line : 78usize , placeholders : & [] , regex : "^Minimum cannot be less than 1\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/suspicious_login/vendor/rubix/ml/src/Other/Tokenizers/KSkipNGram.php" , line : 82usize , placeholders : & [] , regex : "^Maximum cannot be less than minimum\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/suspicious_login/vendor/rubix/ml/src/Other/Strategies/Percentile.php" , line : 102usize , placeholders : & [] , regex : "^Strategy has not been fitted\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/suspicious_login/vendor/rubix/ml/src/Other/Strategies/WildGuess.php" , line : 88usize , placeholders : & [] , regex : "^Strategy has not been fitted\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/suspicious_login/vendor/rubix/ml/src/Other/Strategies/KMostFrequent.php" , line : 110usize , placeholders : & [] , regex : "^Strategy has not been fitted\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/suspicious_login/vendor/rubix/ml/src/Other/Strategies/Mean.php" , line : 78usize , placeholders : & [] , regex : "^Strategy has not been fitted\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/suspicious_login/vendor/rubix/ml/src/Other/Strategies/Prior.php" , line : 88usize , placeholders : & [] , regex : "^Strategy has not been fitted\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/suspicious_login/vendor/rubix/ml/src/Other/Helpers/Stats.php" , line : 34usize , placeholders : & [] , regex : "^Mean is undefined for empty set\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/suspicious_login/vendor/rubix/ml/src/Other/Helpers/Stats.php" , line : 50usize , placeholders : & [] , regex : "^Weighted mean is undefined for empty set\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/suspicious_login/vendor/rubix/ml/src/Other/Helpers/Stats.php" , line : 61usize , placeholders : & [] , regex : "^Total weight cannot equal 0\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/suspicious_login/vendor/rubix/ml/src/Other/Helpers/Stats.php" , line : 96usize , placeholders : & [] , regex : "^Variance is undefined for empty set\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/suspicious_login/vendor/rubix/ml/src/Other/Helpers/Stats.php" , line : 120usize , placeholders : & [] , regex : "^Median is undefined for empty set\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/suspicious_login/vendor/rubix/ml/src/Other/Helpers/Stats.php" , line : 162usize , placeholders : & [] , regex : "^Quantile is undefined for empty set\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/suspicious_login/vendor/rubix/ml/src/Other/Helpers/Stats.php" , line : 261usize , placeholders : & [] , regex : "^Skewness is undefined for empty set\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/suspicious_login/vendor/rubix/ml/src/Other/Helpers/Stats.php" , line : 283usize , placeholders : & [] , regex : "^Kurtosis is undefined for empty set\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/suspicious_login/vendor/rubix/ml/src/Other/Helpers/Stats.php" , line : 306usize , placeholders : & [] , regex : "^Central moment is undefined for empty set\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/suspicious_login/vendor/rubix/ml/src/Other/Helpers/Stats.php" , line : 310usize , placeholders : & [] , regex : "^Moment cannot be less than 1\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/suspicious_login/vendor/rubix/ml/src/Other/Helpers/Stats.php" , line : 335usize , placeholders : & [] , regex : "^Range is undefined for empty set\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/suspicious_login/vendor/rubix/ml/src/Regressors/KDNeighborsRegressor.php" , line : 175usize , placeholders : & [] , regex : "^Estimator has not been trained\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/suspicious_login/vendor/rubix/ml/src/Regressors/KNNRegressor.php" , line : 193usize , placeholders : & [] , regex : "^Estimator has not been trained\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/suspicious_login/vendor/rubix/ml/src/Regressors/RadiusNeighborsRegressor.php" , line : 185usize , placeholders : & [] , regex : "^Estimator has not been trained\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Info , path : "/apps/suspicious_login/vendor/rubix/ml/src/Regressors/MLPRegressor.php" , line : 376usize , placeholders : & ["$this"] , regex : "^(.*) initialized$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Info , path : "/apps/suspicious_login/vendor/rubix/ml/src/Regressors/MLPRegressor.php" , line : 402usize , placeholders : & [] , regex : "^Numerical instability detected$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Info , path : "/apps/suspicious_login/vendor/rubix/ml/src/Regressors/MLPRegressor.php" , line : 461usize , placeholders : & ["$bestEpoch"] , regex : "^Network restored from snapshot at epoch (.*)$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Info , path : "/apps/suspicious_login/vendor/rubix/ml/src/Regressors/MLPRegressor.php" , line : 466usize , placeholders : & [] , regex : "^Training complete$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/suspicious_login/vendor/rubix/ml/src/Regressors/MLPRegressor.php" , line : 481usize , placeholders : & [] , regex : "^Estimator has not been trained\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Info , path : "/apps/suspicious_login/vendor/rubix/ml/src/Regressors/Adaline.php" , line : 295usize , placeholders : & ["$this"] , regex : "^(.*) initialized$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Info , path : "/apps/suspicious_login/vendor/rubix/ml/src/Regressors/Adaline.php" , line : 314usize , placeholders : & [] , regex : "^Numerical under/overflow detected$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Info , path : "/apps/suspicious_login/vendor/rubix/ml/src/Regressors/Adaline.php" , line : 325usize , placeholders : & ["$epoch" , "$this->costFn" , "$loss"] , regex : "^Epoch (.*) \\- (.*): (.*)$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Info , path : "/apps/suspicious_login/vendor/rubix/ml/src/Regressors/Adaline.php" , line : 352usize , placeholders : & [] , regex : "^Training complete$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/suspicious_login/vendor/rubix/ml/src/Regressors/Adaline.php" , line : 366usize , placeholders : & [] , regex : "^Estimator has not been trained\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/suspicious_login/vendor/rubix/ml/src/Regressors/Adaline.php" , line : 383usize , placeholders : & [] , regex : "^Estimator has not been trained\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/suspicious_login/vendor/rubix/ml/src/Regressors/Adaline.php" , line : 389usize , placeholders : & [] , regex : "^Weight layer is missing\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/suspicious_login/vendor/rubix/ml/src/Regressors/RegressionTree.php" , line : 142usize , placeholders : & [] , regex : "^Estimator has not been trained\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/suspicious_login/vendor/rubix/ml/src/Regressors/ExtraTreeRegressor.php" , line : 142usize , placeholders : & [] , regex : "^Estimator has not been trained\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/suspicious_login/vendor/rubix/ml/src/Regressors/Ridge.php" , line : 195usize , placeholders : & [] , regex : "^Estimator has not been trained\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/suspicious_login/vendor/rubix/ml/src/Regressors/Ridge.php" , line : 215usize , placeholders : & [] , regex : "^Learner has not been trained\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/suspicious_login/vendor/rubix/ml/src/Regressors/SVR.php" , line : 232usize , placeholders : & [] , regex : "^Estimator has not been trained\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Info , path : "/apps/suspicious_login/vendor/rubix/ml/src/Regressors/GradientBoost.php" , line : 349usize , placeholders : & ["$this"] , regex : "^(.*) initialized$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Info , path : "/apps/suspicious_login/vendor/rubix/ml/src/Regressors/GradientBoost.php" , line : 359usize , placeholders : & ["$this->base"] , regex : "^Training (.*)$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Info , path : "/apps/suspicious_login/vendor/rubix/ml/src/Regressors/GradientBoost.php" , line : 410usize , placeholders : & [] , regex : "^Numerical instability detected$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Info , path : "/apps/suspicious_login/vendor/rubix/ml/src/Regressors/GradientBoost.php" , line : 471usize , placeholders : & ["$bestEpoch"] , regex : "^Restoring ensemble state to epoch (.*)$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Info , path : "/apps/suspicious_login/vendor/rubix/ml/src/Regressors/GradientBoost.php" , line : 478usize , placeholders : & [] , regex : "^Training complete$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/suspicious_login/vendor/rubix/ml/src/Regressors/GradientBoost.php" , line : 492usize , placeholders : & [] , regex : "^Estimator has not been trained\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/suspicious_login/vendor/rubix/ml/src/Regressors/GradientBoost.php" , line : 519usize , placeholders : & [] , regex : "^Estimator has not been trained\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/suspicious_login/vendor/rubix/ml/src/Regressors/DummyRegressor.php" , line : 132usize , placeholders : & [] , regex : "^Estimator has not been trained\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Info , path : "/apps/suspicious_login/vendor/rubix/ml/src/Clusterers/KMeans.php" , line : 307usize , placeholders : & ["$this"] , regex : "^(.*) initialized$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Info , path : "/apps/suspicious_login/vendor/rubix/ml/src/Clusterers/KMeans.php" , line : 346usize , placeholders : & [] , regex : "^Numerical instability detected$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Info , path : "/apps/suspicious_login/vendor/rubix/ml/src/Clusterers/KMeans.php" , line : 370usize , placeholders : & ["$epoch" , "$loss"] , regex : "^Epoch (.*) \\- Inertia: (.*)$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Info , path : "/apps/suspicious_login/vendor/rubix/ml/src/Clusterers/KMeans.php" , line : 399usize , placeholders : & [] , regex : "^Training complete$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/suspicious_login/vendor/rubix/ml/src/Clusterers/KMeans.php" , line : 413usize , placeholders : & [] , regex : "^Estimator has not been trained\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/suspicious_login/vendor/rubix/ml/src/Clusterers/KMeans.php" , line : 456usize , placeholders : & [] , regex : "^Estimator has not been trained\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Info , path : "/apps/suspicious_login/vendor/rubix/ml/src/Clusterers/FuzzyCMeans.php" , line : 246usize , placeholders : & ["$this"] , regex : "^(.*) initialized$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Info , path : "/apps/suspicious_login/vendor/rubix/ml/src/Clusterers/FuzzyCMeans.php" , line : 264usize , placeholders : & [] , regex : "^Numerical instability detected$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Info , path : "/apps/suspicious_login/vendor/rubix/ml/src/Clusterers/FuzzyCMeans.php" , line : 275usize , placeholders : & ["$epoch" , "$loss"] , regex : "^Epoch (.*) \\- Inertia: (.*)$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Info , path : "/apps/suspicious_login/vendor/rubix/ml/src/Clusterers/FuzzyCMeans.php" , line : 309usize , placeholders : & [] , regex : "^Training complete$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/suspicious_login/vendor/rubix/ml/src/Clusterers/FuzzyCMeans.php" , line : 323usize , placeholders : & [] , regex : "^Estimator has not been trained\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/suspicious_login/vendor/rubix/ml/src/Clusterers/FuzzyCMeans.php" , line : 354usize , placeholders : & [] , regex : "^Estimator has not been trained\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Info , path : "/apps/suspicious_login/vendor/rubix/ml/src/Clusterers/GaussianMixture.php" , line : 256usize , placeholders : & ["$this"] , regex : "^(.*) initialized$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Info , path : "/apps/suspicious_login/vendor/rubix/ml/src/Clusterers/GaussianMixture.php" , line : 294usize , placeholders : & [] , regex : "^Numerical instability detected$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Info , path : "/apps/suspicious_login/vendor/rubix/ml/src/Clusterers/GaussianMixture.php" , line : 305usize , placeholders : & ["$epoch" , "$loss"] , regex : "^Epoch (.*) \\- loss: (.*)$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Info , path : "/apps/suspicious_login/vendor/rubix/ml/src/Clusterers/GaussianMixture.php" , line : 353usize , placeholders : & [] , regex : "^Training complete$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/suspicious_login/vendor/rubix/ml/src/Clusterers/GaussianMixture.php" , line : 367usize , placeholders : & [] , regex : "^Estimator has not been trained\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/suspicious_login/vendor/rubix/ml/src/Clusterers/GaussianMixture.php" , line : 398usize , placeholders : & [] , regex : "^Estimator has not been trained\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Info , path : "/apps/suspicious_login/vendor/rubix/ml/src/Clusterers/MeanShift.php" , line : 296usize , placeholders : & ["$this"] , regex : "^(.*) initialized$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Info , path : "/apps/suspicious_login/vendor/rubix/ml/src/Clusterers/MeanShift.php" , line : 344usize , placeholders : & [] , regex : "^Numerical instability detected$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Info , path : "/apps/suspicious_login/vendor/rubix/ml/src/Clusterers/MeanShift.php" , line : 355usize , placeholders : & ["$epoch" , "$loss"] , regex : "^Epoch (.*) \\- loss: (.*)$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Info , path : "/apps/suspicious_login/vendor/rubix/ml/src/Clusterers/MeanShift.php" , line : 370usize , placeholders : & [] , regex : "^Training complete$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/suspicious_login/vendor/rubix/ml/src/Clusterers/MeanShift.php" , line : 384usize , placeholders : & [] , regex : "^Estimator has not been trained\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/suspicious_login/vendor/rubix/ml/src/Clusterers/MeanShift.php" , line : 427usize , placeholders : & [] , regex : "^Estimator has not been trained\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/suspicious_login/vendor/rubix/ml/src/GridSearch.php" , line : 133usize , placeholders : & ["$base"] , regex : "^Class (.*) does not exist\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Info , path : "/apps/suspicious_login/vendor/rubix/ml/src/GridSearch.php" , line : 287usize , placeholders : & ["$this"] , regex : "^(.*) initialized$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Info , path : "/apps/suspicious_login/vendor/rubix/ml/src/GridSearch.php" , line : 323usize , placeholders : & [] , regex : "^Training with best hyper\\-parameters$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Info , path : "/apps/suspicious_login/vendor/rubix/ml/src/GridSearch.php" , line : 331usize , placeholders : & [] , regex : "^Training complete$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/suspicious_login/vendor/rubix/ml/src/CommitteeMachine.php" , line : 96usize , placeholders : & [] , regex : "^Committee must contain at least 1 expert\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/suspicious_login/vendor/rubix/ml/src/CommitteeMachine.php" , line : 105usize , placeholders : & [] , regex : "^Expert must implement the Learner interface\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Info , path : "/apps/suspicious_login/vendor/rubix/ml/src/CommitteeMachine.php" , line : 254usize , placeholders : & ["$this"] , regex : "^(.*) initialized$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Info , path : "/apps/suspicious_login/vendor/rubix/ml/src/CommitteeMachine.php" , line : 283usize , placeholders : & [] , regex : "^Training complete$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/suspicious_login/vendor/rubix/ml/src/CommitteeMachine.php" , line : 298usize , placeholders : & ["$estimator"] , regex : "^There was a problem training (.*)\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Info , path : "/apps/suspicious_login/vendor/rubix/ml/src/CommitteeMachine.php" , line : 302usize , placeholders : & ["$estimator"] , regex : "^(.*) finished training$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/suspicious_login/vendor/rubix/ml/src/CommitteeMachine.php" , line : 315usize , placeholders : & [] , regex : "^Estimator has not been trained\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/suspicious_login/vendor/rubix/ml/src/Transformers/LinearDiscriminantAnalysis.php" , line : 219usize , placeholders : & [] , regex : "^Transformer has not been fitted\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/suspicious_login/vendor/rubix/ml/src/Transformers/KBestFeatureSelector.php" , line : 156usize , placeholders : & [] , regex : "^Transformer has not been fitted\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/suspicious_login/vendor/rubix/ml/src/Transformers/IntervalDiscretizer.php" , line : 155usize , placeholders : & [] , regex : "^Transformer has not been fitted\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/suspicious_login/vendor/rubix/ml/src/Transformers/KNNImputer.php" , line : 164usize , placeholders : & [] , regex : "^No complete donors found in dataset\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/suspicious_login/vendor/rubix/ml/src/Transformers/KNNImputer.php" , line : 183usize , placeholders : & [] , regex : "^Transformer has not been fitted\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/suspicious_login/vendor/rubix/ml/src/Transformers/ZScaleStandardizer.php" , line : 208usize , placeholders : & [] , regex : "^Transformer has not been fitted\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/suspicious_login/vendor/rubix/ml/src/Transformers/MissingDataImputer.php" , line : 181usize , placeholders : & [] , regex : "^Transformer has not been fitted\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/suspicious_login/vendor/rubix/ml/src/Transformers/TruncatedSVD.php" , line : 147usize , placeholders : & [] , regex : "^Transformer has not been fitted\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/suspicious_login/vendor/rubix/ml/src/Transformers/TfIdfTransformer.php" , line : 174usize , placeholders : & [] , regex : "^Transformer has not been fitted\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/suspicious_login/vendor/rubix/ml/src/Transformers/OneHotEncoder.php" , line : 104usize , placeholders : & [] , regex : "^Transformer has not been fitted\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/suspicious_login/vendor/rubix/ml/src/Transformers/ImageResizer.php" , line : 110usize , placeholders : & [] , regex : "^Could not create placeholder image\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/suspicious_login/vendor/rubix/ml/src/Transformers/ImageResizer.php" , line : 116usize , placeholders : & [] , regex : "^Failed to resize image\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/suspicious_login/vendor/rubix/ml/src/Transformers/MaxAbsoluteScaler.php" , line : 120usize , placeholders : & [] , regex : "^Transformer has not been fitted\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/suspicious_login/vendor/rubix/ml/src/Transformers/MinMaxNormalizer.php" , line : 197usize , placeholders : & [] , regex : "^Transformer has not been fitted\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/suspicious_login/vendor/rubix/ml/src/Transformers/VarianceThresholdFilter.php" , line : 127usize , placeholders : & [] , regex : "^Transformer has not been fitted\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/suspicious_login/vendor/rubix/ml/src/Transformers/ImageVectorizer.php" , line : 110usize , placeholders : & [] , regex : "^Transformer has not been fitted\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/suspicious_login/vendor/rubix/ml/src/Transformers/PrincipalComponentAnalysis.php" , line : 208usize , placeholders : & [] , regex : "^Transformer has not been fitted\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/suspicious_login/vendor/rubix/ml/src/Transformers/WordCountVectorizer.php" , line : 201usize , placeholders : & [] , regex : "^Transformer has not been fitted\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/suspicious_login/vendor/rubix/ml/src/Transformers/RobustStandardizer.php" , line : 141usize , placeholders : & [] , regex : "^Transformer has not been fitted\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/suspicious_login/vendor/rubix/ml/src/Transformers/GaussianRandomProjector.php" , line : 132usize , placeholders : & [] , regex : "^Transformer has not been fitted\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Info , path : "/apps/suspicious_login/vendor/rubix/ml/src/Transformers/RecursiveFeatureEliminator.php" , line : 181usize , placeholders : & ["$this"] , regex : "^(.*) initialized$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Info , path : "/apps/suspicious_login/vendor/rubix/ml/src/Transformers/RecursiveFeatureEliminator.php" , line : 242usize , placeholders : & [] , regex : "^Fitting complete$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/suspicious_login/vendor/rubix/ml/src/Transformers/RecursiveFeatureEliminator.php" , line : 255usize , placeholders : & [] , regex : "^Transformer has not been fitted\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/suspicious_login/vendor/rubix/ml/src/BootstrapAggregator.php" , line : 222usize , placeholders : & [] , regex : "^Estimator has not been trained\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/suspicious_login/vendor/rubix/ml/src/Report.php" , line : 72usize , placeholders : & [] , regex : "^Reports cannot be mutated directly\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/suspicious_login/vendor/rubix/ml/src/Report.php" , line : 99usize , placeholders : & ["$key"] , regex : "^Attribute with key (.*) not found\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/suspicious_login/vendor/rubix/ml/src/Report.php" , line : 108usize , placeholders : & [] , regex : "^Reports cannot be mutated directly\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/suspicious_login/vendor/rubix/ml/src/Persisters/RedisDB.php" , line : 71usize , placeholders : & [] , regex : "^Key cannot be empty\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/suspicious_login/vendor/rubix/ml/src/Persisters/RedisDB.php" , line : 88usize , placeholders : & [] , regex : "^Password is invalid\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/suspicious_login/vendor/rubix/ml/src/Persisters/RedisDB.php" , line : 93usize , placeholders : & ["$database"] , regex : "^Failed to select database (.*)\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/suspicious_login/vendor/rubix/ml/src/Persisters/Serializers/Native.php" , line : 52usize , placeholders : & [] , regex : "^Unserialized data must be an object\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/suspicious_login/vendor/rubix/ml/src/Persisters/Serializers/Native.php" , line : 56usize , placeholders : & [] , regex : "^Missing class for object data\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/suspicious_login/vendor/rubix/ml/src/Persisters/Serializers/Gzip.php" , line : 72usize , placeholders : & [] , regex : "^Failed to compress data\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/suspicious_login/vendor/rubix/ml/src/Persisters/Serializers/Gzip.php" , line : 90usize , placeholders : & [] , regex : "^Failed to decompress data\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/suspicious_login/vendor/rubix/ml/src/Persisters/Serializers/Igbinary.php" , line : 48usize , placeholders : & [] , regex : "^Could not serialize data\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/suspicious_login/vendor/rubix/ml/src/Persisters/Serializers/Igbinary.php" , line : 68usize , placeholders : & [] , regex : "^Unserialized data must be an object\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/suspicious_login/vendor/rubix/ml/src/Persisters/Serializers/Igbinary.php" , line : 72usize , placeholders : & [] , regex : "^Missing class for object data\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/suspicious_login/vendor/rubix/ml/src/Persisters/Serializers/RBX.php" , line : 130usize , placeholders : & [] , regex : "^Unrecognized message format\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/suspicious_login/vendor/rubix/ml/src/Persisters/Serializers/RBX.php" , line : 138usize , placeholders : & [] , regex : "^Invalid message format\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/suspicious_login/vendor/rubix/ml/src/Persisters/Serializers/RBX.php" , line : 144usize , placeholders : & [] , regex : "^Header checksum verification failed\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/suspicious_login/vendor/rubix/ml/src/Persisters/Serializers/RBX.php" , line : 150usize , placeholders : & [] , regex : "^Data is corrupted\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/suspicious_login/vendor/rubix/ml/src/Persisters/Serializers/RBX.php" , line : 156usize , placeholders : & [] , regex : "^Data checksum verification failed\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/suspicious_login/vendor/rubix/ml/src/Persisters/Serializers/RBX.php" , line : 162usize , placeholders : & [] , regex : "^Class name mismatch\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/suspicious_login/vendor/rubix/ml/src/Persisters/Flysystem.php" , line : 106usize , placeholders : & ["$filename"] , regex : "^Failed to create history file '(.*)'\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/suspicious_login/vendor/rubix/ml/src/Persisters/Flysystem.php" , line : 115usize , placeholders : & [] , regex : "^Could not write to filesystem\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/suspicious_login/vendor/rubix/ml/src/Persisters/Flysystem.php" , line : 128usize , placeholders : & ["$this->path"] , regex : "^File does not exist at (.*)\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/suspicious_login/vendor/rubix/ml/src/Persisters/Flysystem.php" , line : 134usize , placeholders : & ["$this->path"] , regex : "^Error reading data from (.*)\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/suspicious_login/vendor/rubix/ml/src/Persisters/Flysystem.php" , line : 140usize , placeholders : & ["$this->path"] , regex : "^File at (.*) does not contain any data\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/suspicious_login/vendor/rubix/ml/src/EstimatorType.php" , line : 139usize , placeholders : & [] , regex : "^Invalid type code\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/suspicious_login/vendor/rubix/ml/src/Graph/Trees/CART.php" , line : 312usize , placeholders : & [] , regex : "^Tree has not been constructed\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/suspicious_login/vendor/rubix/ml/src/Graph/Trees/CART.php" , line : 342usize , placeholders : & [] , regex : "^Tree has not been constructed\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/suspicious_login/vendor/rubix/ml/src/Graph/Trees/CART.php" , line : 450usize , placeholders : & [] , regex : "^Could not split dataset\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/suspicious_login/vendor/rubix/ml/src/Graph/Trees/ExtraTree.php" , line : 78usize , placeholders : & [] , regex : "^Could not split dataset\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/suspicious_login/vendor/rubix/ml/src/Graph/Trees/KDTree.php" , line : 140usize , placeholders : & [] , regex : "^KD Tree only works with continuous features\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/suspicious_login/vendor/rubix/ml/src/DataType.php" , line : 187usize , placeholders : & [] , regex : "^Invalid type code\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/suspicious_login/vendor/rubix/ml/src/Extractors/JSON.php" , line : 40usize , placeholders : & ["$path"] , regex : "^Path (.*) does not exist\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/suspicious_login/vendor/rubix/ml/src/Extractors/JSON.php" , line : 44usize , placeholders : & ["$path"] , regex : "^Path (.*) is not readable\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/suspicious_login/vendor/rubix/ml/src/Extractors/JSON.php" , line : 61usize , placeholders : & ["$this->path"] , regex : "^Could not open (.*)\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/suspicious_login/vendor/rubix/ml/src/Extractors/CSV.php" , line : 74usize , placeholders : & ["$path"] , regex : "^Path (.*) does not exist\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/suspicious_login/vendor/rubix/ml/src/Extractors/CSV.php" , line : 78usize , placeholders : & ["$path"] , regex : "^Path (.*) is not readable\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/suspicious_login/vendor/rubix/ml/src/Extractors/CSV.php" , line : 94usize , placeholders : & ["$path"] , regex : "^Could not open (.*)\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/suspicious_login/vendor/rubix/ml/src/Extractors/CSV.php" , line : 127usize , placeholders : & [] , regex : "^Header not found on the first line\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/suspicious_login/vendor/rubix/ml/src/Extractors/CSV.php" , line : 147usize , placeholders : & ["$line"] , regex : "^Malformed record on line (.*)\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/suspicious_login/vendor/rubix/ml/src/Extractors/NDJSON.php" , line : 40usize , placeholders : & ["$path"] , regex : "^Path (.*) does not exist\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/suspicious_login/vendor/rubix/ml/src/Extractors/NDJSON.php" , line : 44usize , placeholders : & ["$path"] , regex : "^Path (.*) is not readable\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/suspicious_login/vendor/rubix/ml/src/Extractors/NDJSON.php" , line : 50usize , placeholders : & ["$path"] , regex : "^Could not open (.*)\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/suspicious_login/vendor/rubix/ml/src/Extractors/NDJSON.php" , line : 89usize , placeholders : & ["$line" , "$e->getMessage()"] , regex : "^JSON Error on line (.*): (.*)$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/suspicious_login/vendor/rubix/ml/src/Datasets/Dataset.php" , line : 136usize , placeholders : & ["$offset"] , regex : "^Sample at offset (.*) not found\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/suspicious_login/vendor/rubix/ml/src/Datasets/Dataset.php" , line : 765usize , placeholders : & [] , regex : "^Datasets cannot be mutated directly\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/suspicious_login/vendor/rubix/ml/src/Datasets/Dataset.php" , line : 785usize , placeholders : & [] , regex : "^Datasets cannot be mutated directly\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/suspicious_login/vendor/rubix/ml/src/Datasets/Unlabeled.php" , line : 602usize , placeholders : & ["$offset"] , regex : "^Row at offset (.*) not found\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/suspicious_login/vendor/rubix/ml/src/Datasets/Labeled.php" , line : 193usize , placeholders : & ["$offset"] , regex : "^Row at offset (.*) not found\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/suspicious_login/vendor/rubix/ml/src/Datasets/Labeled.php" , line : 208usize , placeholders : & [] , regex : "^Dataset is empty\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/suspicious_login/vendor/rubix/ml/src/Datasets/Labeled.php" , line : 1024usize , placeholders : & ["$offset"] , regex : "^Row at offset (.*) not found\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/suspicious_login/vendor/rubix/ml/src/Datasets/Labeled.php" , line : 1057usize , placeholders : & [] , regex : "^Label must be a string or integer type\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Info , path : "/apps/suspicious_login/vendor/rubix/ml/src/Pipeline.php" , line : 176usize , placeholders : & ["$transformer"] , regex : "^Fitted (.*)$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Info , path : "/apps/suspicious_login/vendor/rubix/ml/src/Pipeline.php" , line : 201usize , placeholders : & ["$transformer"] , regex : "^Updated (.*)$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Info , path : "/apps/suspicious_login/vendor/rubix/ml/src/Embedders/TSNE.php" , line : 333usize , placeholders : & ["$this"] , regex : "^(.*) initialized$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Info , path : "/apps/suspicious_login/vendor/rubix/ml/src/Embedders/TSNE.php" , line : 335usize , placeholders : & [] , regex : "^Computing high\\-dimensional affinities$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Info , path : "/apps/suspicious_login/vendor/rubix/ml/src/Embedders/TSNE.php" , line : 381usize , placeholders : & [] , regex : "^Numerical instability detected$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Info , path : "/apps/suspicious_login/vendor/rubix/ml/src/Embedders/TSNE.php" , line : 390usize , placeholders : & ["$epoch" , "$loss"] , regex : "^Epoch (.*) \\- Gradient: (.*)$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Info , path : "/apps/suspicious_login/vendor/rubix/ml/src/Embedders/TSNE.php" , line : 415usize , placeholders : & [] , regex : "^Early exaggeration exhausted$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Info , path : "/apps/suspicious_login/vendor/rubix/ml/src/Embedders/TSNE.php" , line : 421usize , placeholders : & [] , regex : "^Embedding complete$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/suspicious_login/vendor/rubix/tensor/src/Vector.php" , line : 323usize , placeholders : & [] , regex : "^Vector must contain at least one element\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/suspicious_login/vendor/rubix/tensor/src/Vector.php" , line : 688usize , placeholders : & ["$p"] , regex : "^P must be greater than 0, (.*) given\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/suspicious_login/vendor/rubix/tensor/src/Vector.php" , line : 2369usize , placeholders : & [] , regex : "^Vector cannot be mutated directly\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/suspicious_login/vendor/rubix/tensor/src/Vector.php" , line : 2389usize , placeholders : & [] , regex : "^Vector cannot be mutated directly\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/suspicious_login/vendor/rubix/tensor/src/Matrix.php" , line : 448usize , placeholders : & [] , regex : "^Vectors must be the same size\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/suspicious_login/vendor/rubix/tensor/src/Matrix.php" , line : 810usize , placeholders : & [] , regex : "^Pseudoinverse is not implemented in Tensor PHP\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/suspicious_login/vendor/rubix/tensor/src/Matrix.php" , line : 1859usize , placeholders : & [] , regex : "^Mean must be a Column Vector\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/suspicious_login/vendor/rubix/tensor/src/Matrix.php" , line : 2022usize , placeholders : & [] , regex : "^Minimum cannot be greater than maximum\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/suspicious_login/vendor/rubix/tensor/src/Matrix.php" , line : 3768usize , placeholders : & [] , regex : "^Matrix cannot be mutated directly\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/suspicious_login/vendor/rubix/tensor/src/Matrix.php" , line : 3788usize , placeholders : & [] , regex : "^Matrix cannot be mutated directly\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/suspicious_login/vendor/rubix/tensor/src/Decompositions/SVD.php" , line : 46usize , placeholders : & [] , regex : "^SVD is not implemented in Tensor PHP\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/suspicious_login/vendor/rubix/tensor/optimizers/TensorArgminOptimizer.php" , line : 40usize , placeholders : & [] , regex : "^Return value must only be assigned to a dynamic variable\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/suspicious_login/vendor/rubix/tensor/optimizers/TensorLessEqualScalarOptimizer.php" , line : 40usize , placeholders : & [] , regex : "^Return value must only be assigned to a dynamic variable\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/suspicious_login/vendor/rubix/tensor/optimizers/TensorLessEqualOptimizer.php" , line : 40usize , placeholders : & [] , regex : "^Return value must only be assigned to a dynamic variable\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/suspicious_login/vendor/rubix/tensor/optimizers/TensorAddOptimizer.php" , line : 40usize , placeholders : & [] , regex : "^Return value must only be assigned to a dynamic variable\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/suspicious_login/vendor/rubix/tensor/optimizers/TensorRefOptimizer.php" , line : 40usize , placeholders : & [] , regex : "^Return value must only be assigned to a dynamic variable\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/suspicious_login/vendor/rubix/tensor/optimizers/TensorGreaterScalarOptimizer.php" , line : 40usize , placeholders : & [] , regex : "^Return value must only be assigned to a dynamic variable\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/suspicious_login/vendor/rubix/tensor/optimizers/TensorModScalarOptimizer.php" , line : 40usize , placeholders : & [] , regex : "^Return value must only be assigned to a dynamic variable\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/suspicious_login/vendor/rubix/tensor/optimizers/TensorGreaterEqualScalarOptimizer.php" , line : 40usize , placeholders : & [] , regex : "^Return value must only be assigned to a dynamic variable\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/suspicious_login/vendor/rubix/tensor/optimizers/TensorSetNumThreadsOptimizer.php" , line : 40usize , placeholders : & [] , regex : "^Return value must only be assigned to a dynamic variable\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/suspicious_login/vendor/rubix/tensor/optimizers/TensorDivideOptimizer.php" , line : 40usize , placeholders : & [] , regex : "^Return value must only be assigned to a dynamic variable\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/suspicious_login/vendor/rubix/tensor/optimizers/TensorNotEqualScalarOptimizer.php" , line : 40usize , placeholders : & [] , regex : "^Return value must only be assigned to a dynamic variable\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/suspicious_login/vendor/rubix/tensor/optimizers/TensorGreaterEqualOptimizer.php" , line : 40usize , placeholders : & [] , regex : "^Return value must only be assigned to a dynamic variable\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/suspicious_login/vendor/rubix/tensor/optimizers/TensorConvolve1dOptimizer.php" , line : 40usize , placeholders : & [] , regex : "^Return value must only be assigned to a dynamic variable\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/suspicious_login/vendor/rubix/tensor/optimizers/TensorDotOptimizer.php" , line : 40usize , placeholders : & [] , regex : "^Return value must only be assigned to a dynamic variable\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/suspicious_login/vendor/rubix/tensor/optimizers/TensorLessScalarOptimizer.php" , line : 40usize , placeholders : & [] , regex : "^Return value must only be assigned to a dynamic variable\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/suspicious_login/vendor/rubix/tensor/optimizers/TensorPowScalarOptimizer.php" , line : 40usize , placeholders : & [] , regex : "^Return value must only be assigned to a dynamic variable\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/suspicious_login/vendor/rubix/tensor/optimizers/TensorDivideScalarOptimizer.php" , line : 40usize , placeholders : & [] , regex : "^Return value must only be assigned to a dynamic variable\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/suspicious_login/vendor/rubix/tensor/optimizers/TensorEqualScalarOptimizer.php" , line : 40usize , placeholders : & [] , regex : "^Return value must only be assigned to a dynamic variable\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/suspicious_login/vendor/rubix/tensor/optimizers/TensorSubtractScalarOptimizer.php" , line : 40usize , placeholders : & [] , regex : "^Return value must only be assigned to a dynamic variable\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/suspicious_login/vendor/rubix/tensor/optimizers/TensorMatmulOptimizer.php" , line : 40usize , placeholders : & [] , regex : "^Return value must only be assigned to a dynamic variable\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/suspicious_login/vendor/rubix/tensor/optimizers/TensorPseudoinverseOptimizer.php" , line : 40usize , placeholders : & [] , regex : "^Return value must only be assigned to a dynamic variable\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/suspicious_login/vendor/rubix/tensor/optimizers/TensorModOptimizer.php" , line : 40usize , placeholders : & [] , regex : "^Return value must only be assigned to a dynamic variable\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/suspicious_login/vendor/rubix/tensor/optimizers/TensorCholeskyOptimizer.php" , line : 40usize , placeholders : & [] , regex : "^Return value must only be assigned to a dynamic variable\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/suspicious_login/vendor/rubix/tensor/optimizers/TensorEigSymmetricOptimizer.php" , line : 40usize , placeholders : & [] , regex : "^Return value must only be assigned to a dynamic variable\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/suspicious_login/vendor/rubix/tensor/optimizers/TensorSubtractOptimizer.php" , line : 40usize , placeholders : & [] , regex : "^Return value must only be assigned to a dynamic variable\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/suspicious_login/vendor/rubix/tensor/optimizers/TensorNotEqualOptimizer.php" , line : 40usize , placeholders : & [] , regex : "^Return value must only be assigned to a dynamic variable\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/suspicious_login/vendor/rubix/tensor/optimizers/TensorGetNumThreadsOptimizer.php" , line : 29usize , placeholders : & [] , regex : "^Return value must only be assigned to a dynamic variable\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/suspicious_login/vendor/rubix/tensor/optimizers/TensorLessOptimizer.php" , line : 40usize , placeholders : & [] , regex : "^Return value must only be assigned to a dynamic variable\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/suspicious_login/vendor/rubix/tensor/optimizers/TensorLuOptimizer.php" , line : 40usize , placeholders : & [] , regex : "^Return value must only be assigned to a dynamic variable\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/suspicious_login/vendor/rubix/tensor/optimizers/TensorInverseOptimizer.php" , line : 40usize , placeholders : & [] , regex : "^Return value must only be assigned to a dynamic variable\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/suspicious_login/vendor/rubix/tensor/optimizers/TensorSvdOptimizer.php" , line : 40usize , placeholders : & [] , regex : "^Return value must only be assigned to a dynamic variable\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/suspicious_login/vendor/rubix/tensor/optimizers/TensorArgmaxOptimizer.php" , line : 40usize , placeholders : & [] , regex : "^Return value must only be assigned to a dynamic variable\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/suspicious_login/vendor/rubix/tensor/optimizers/TensorEqualOptimizer.php" , line : 40usize , placeholders : & [] , regex : "^Return value must only be assigned to a dynamic variable\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/suspicious_login/vendor/rubix/tensor/optimizers/TensorEigOptimizer.php" , line : 40usize , placeholders : & [] , regex : "^Return value must only be assigned to a dynamic variable\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/suspicious_login/vendor/rubix/tensor/optimizers/TensorMultiplyScalarOptimizer.php" , line : 40usize , placeholders : & [] , regex : "^Return value must only be assigned to a dynamic variable\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/suspicious_login/vendor/rubix/tensor/optimizers/TensorAddScalarOptimizer.php" , line : 40usize , placeholders : & [] , regex : "^Return value must only be assigned to a dynamic variable\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/suspicious_login/vendor/rubix/tensor/optimizers/TensorMultiplyOptimizer.php" , line : 40usize , placeholders : & [] , regex : "^Return value must only be assigned to a dynamic variable\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/suspicious_login/vendor/rubix/tensor/optimizers/TensorConvolve2dOptimizer.php" , line : 40usize , placeholders : & [] , regex : "^Return value must only be assigned to a dynamic variable\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/suspicious_login/vendor/rubix/tensor/optimizers/TensorPowOptimizer.php" , line : 40usize , placeholders : & [] , regex : "^Return value must only be assigned to a dynamic variable\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/suspicious_login/vendor/rubix/tensor/optimizers/TensorGreaterOptimizer.php" , line : 40usize , placeholders : & [] , regex : "^Return value must only be assigned to a dynamic variable\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/suspicious_login/vendor/symfony/polyfill-mbstring/Mbstring.php" , line : 557usize , placeholders : & [] , regex : "^Argument \\#2 \\(\\$length\\) must be greater than 0$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/suspicious_login/vendor/symfony/polyfill-mbstring/Mbstring.php" , line : 610usize , placeholders : & [] , regex : "^Argument \\#1 \\(\\$substitute_character\\) must be \"none\", \"long\", \"entity\" or a valid codepoint$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/suspicious_login/vendor/amphp/process/lib/Process.php" , line : 57usize , placeholders : & [] , regex : "^\\$env cannot accept array values$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/suspicious_login/vendor/amphp/process/lib/Process.php" , line : 91usize , placeholders : & [] , regex : "^Cloning is not allowed!$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/suspicious_login/vendor/amphp/process/lib/Process.php" , line : 104usize , placeholders : & [] , regex : "^Process has already been started\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/suspicious_login/vendor/amphp/process/lib/Process.php" , line : 123usize , placeholders : & [] , regex : "^Process has not been started\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/suspicious_login/vendor/amphp/process/lib/Process.php" , line : 138usize , placeholders : & [] , regex : "^Process is not running\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/suspicious_login/vendor/amphp/process/lib/Process.php" , line : 155usize , placeholders : & [] , regex : "^Process is not running\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/suspicious_login/vendor/amphp/process/lib/Process.php" , line : 171usize , placeholders : & [] , regex : "^Process has not been started or has not completed starting\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/suspicious_login/vendor/amphp/process/lib/Process.php" , line : 239usize , placeholders : & [] , regex : "^Process has not been started or has not completed starting\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/suspicious_login/vendor/amphp/process/lib/Process.php" , line : 253usize , placeholders : & [] , regex : "^Process has not been started or has not completed starting\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/suspicious_login/vendor/amphp/process/lib/Process.php" , line : 267usize , placeholders : & [] , regex : "^Process has not been started or has not completed starting\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/suspicious_login/vendor/amphp/process/lib/Internal/Posix/Runner.php" , line : 104usize , placeholders : & [] , regex : "^Could not get process status$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/suspicious_login/vendor/amphp/process/lib/Internal/Posix/Runner.php" , line : 146usize , placeholders : & [] , regex : "^Unable to list open file descriptors$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/suspicious_login/vendor/amphp/process/lib/Internal/Posix/Runner.php" , line : 188usize , placeholders : & [] , regex : "^Terminating process failed$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/suspicious_login/vendor/amphp/process/lib/Internal/Windows/Runner.php" , line : 82usize , placeholders : & [] , regex : "^Can't execute commands that contain null bytes\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/suspicious_login/vendor/amphp/process/lib/Internal/Windows/Runner.php" , line : 109usize , placeholders : & [] , regex : "^Could not get process status$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/suspicious_login/vendor/amphp/process/lib/Internal/Windows/Runner.php" , line : 122usize , placeholders : & [] , regex : "^Could not send security tokens / command to process wrapper$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/suspicious_login/vendor/amphp/process/lib/Internal/Windows/Runner.php" , line : 192usize , placeholders : & [] , regex : "^Signals are not supported on Windows$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/suspicious_login/vendor/amphp/process/lib/Internal/Windows/SocketConnector.php" , line : 42usize , placeholders : & ["$errNo" , "$errStr"] , regex : "^Failed to create TCP server socket for process wrapper: (.*): (.*)$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/suspicious_login/vendor/amphp/process/lib/Internal/Windows/SocketConnector.php" , line : 46usize , placeholders : & [] , regex : "^Failed to set server socket to non\\-blocking mode$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/suspicious_login/vendor/amphp/process/lib/Internal/Windows/SocketConnector.php" , line : 335usize , placeholders : & [] , regex : "^Failed to set client socket to non\\-blocking mode$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/suspicious_login/vendor/amphp/process/lib/ProcessOutputStream.php" , line : 71usize , placeholders : & [] , regex : "^Stream has already been closed\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/suspicious_login/vendor/amphp/process/lib/ProcessOutputStream.php" , line : 92usize , placeholders : & [] , regex : "^Stream has already been closed\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/suspicious_login/vendor/amphp/byte-stream/lib/OutputBuffer.php" , line : 28usize , placeholders : & [] , regex : "^The stream has already been closed\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/suspicious_login/vendor/amphp/byte-stream/lib/OutputBuffer.php" , line : 39usize , placeholders : & [] , regex : "^The stream has already been closed\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/suspicious_login/vendor/amphp/byte-stream/lib/functions.php" , line : 14usize , placeholders : & [] , regex : "^The mbstring\\.func_overload ini setting is enabled\\. It must be disabled to use the stream package\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/suspicious_login/vendor/amphp/byte-stream/lib/ZlibOutputStream.php" , line : 38usize , placeholders : & [] , regex : "^Failed initializing deflate context$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/suspicious_login/vendor/amphp/byte-stream/lib/ZlibOutputStream.php" , line : 46usize , placeholders : & [] , regex : "^The stream has already been closed$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/suspicious_login/vendor/amphp/byte-stream/lib/ZlibOutputStream.php" , line : 54usize , placeholders : & [] , regex : "^Failed adding data to deflate context$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/suspicious_login/vendor/amphp/byte-stream/lib/ZlibOutputStream.php" , line : 71usize , placeholders : & [] , regex : "^The stream has already been closed$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/suspicious_login/vendor/amphp/byte-stream/lib/ZlibOutputStream.php" , line : 79usize , placeholders : & [] , regex : "^Failed adding data to deflate context$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/suspicious_login/vendor/amphp/byte-stream/lib/Base64/Base64DecodingInputStream.php" , line : 27usize , placeholders : & [] , regex : "^Failed to read stream chunk due to invalid base64 data$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/suspicious_login/vendor/amphp/byte-stream/lib/Base64/Base64DecodingInputStream.php" , line : 41usize , placeholders : & [] , regex : "^Failed to read stream chunk due to invalid base64 data$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/suspicious_login/vendor/amphp/byte-stream/lib/Base64/Base64DecodingInputStream.php" , line : 57usize , placeholders : & [] , regex : "^Failed to read stream chunk due to invalid base64 data$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/suspicious_login/vendor/amphp/byte-stream/lib/ZlibInputStream.php" , line : 40usize , placeholders : & [] , regex : "^Failed initializing deflate context$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/suspicious_login/vendor/amphp/byte-stream/lib/ZlibInputStream.php" , line : 66usize , placeholders : & [] , regex : "^Failed adding data to deflate context$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/suspicious_login/vendor/amphp/byte-stream/lib/ZlibInputStream.php" , line : 77usize , placeholders : & [] , regex : "^Failed adding data to deflate context$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/suspicious_login/vendor/amphp/byte-stream/lib/ResourceInputStream.php" , line : 50usize , placeholders : & [] , regex : "^Expected a valid stream$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/suspicious_login/vendor/amphp/byte-stream/lib/ResourceInputStream.php" , line : 58usize , placeholders : & [] , regex : "^Expected a readable stream$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/suspicious_login/vendor/amphp/byte-stream/lib/ResourceInputStream.php" , line : 234usize , placeholders : & [] , regex : "^Resource has already been freed$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/suspicious_login/vendor/amphp/byte-stream/lib/ResourceInputStream.php" , line : 250usize , placeholders : & [] , regex : "^Resource has already been freed$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/suspicious_login/vendor/amphp/byte-stream/lib/Payload.php" , line : 63usize , placeholders : & [] , regex : "^Cannot stream message data once a buffered message has been requested$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/suspicious_login/vendor/amphp/byte-stream/lib/ResourceOutputStream.php" , line : 41usize , placeholders : & [] , regex : "^Expected a valid stream$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/suspicious_login/vendor/amphp/byte-stream/lib/ResourceOutputStream.php" , line : 47usize , placeholders : & [] , regex : "^Expected a writable stream$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/suspicious_login/vendor/amphp/byte-stream/lib/ResourceOutputStream.php" , line : 75usize , placeholders : & [] , regex : "^The stream was closed by the peer$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/suspicious_login/vendor/amphp/sync/src/PosixSemaphore.php" , line : 46usize , placeholders : & [] , regex : "^Number of locks must be greater than 0$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/suspicious_login/vendor/amphp/sync/src/PosixSemaphore.php" , line : 93usize , placeholders : & [] , regex : "^A semaphore cannot be serialized!$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/suspicious_login/vendor/amphp/sync/src/PosixSemaphore.php" , line : 104usize , placeholders : & [] , regex : "^No semaphore with that ID found$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/suspicious_login/vendor/amphp/sync/src/PosixSemaphore.php" , line : 110usize , placeholders : & [] , regex : "^Failed to open the semaphore\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/suspicious_login/vendor/amphp/sync/src/PosixSemaphore.php" , line : 123usize , placeholders : & [] , regex : "^A semaphore with that ID already exists$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/suspicious_login/vendor/amphp/sync/src/PosixSemaphore.php" , line : 128usize , placeholders : & [] , regex : "^Failed to create the semaphore\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/suspicious_login/vendor/amphp/sync/src/PosixSemaphore.php" , line : 162usize , placeholders : & [] , regex : "^Failed to change the semaphore permissions\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/suspicious_login/vendor/amphp/sync/src/PosixSemaphore.php" , line : 232usize , placeholders : & [] , regex : "^The semaphore size is larger than the system allows\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/suspicious_login/vendor/amphp/sync/src/PosixSemaphore.php" , line : 235usize , placeholders : & [] , regex : "^Failed to release the lock\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/suspicious_login/vendor/amphp/sync/src/Barrier.php" , line : 73usize , placeholders : & [] , regex : "^Can't increase count, because the barrier already broke$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/suspicious_login/vendor/amphp/sync/src/FileMutex.php" , line : 80usize , placeholders : & [] , regex : "^Failed to unlock the mutex file\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/suspicious_login/vendor/amphp/sync/src/LocalSemaphore.php" , line : 23usize , placeholders : & [] , regex : "^The number of locks must be greater than 0$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/suspicious_login/vendor/amphp/sync/src/ThreadedSemaphore.php" , line : 30usize , placeholders : & [] , regex : "^The number of locks should be a positive integer$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/suspicious_login/vendor/amphp/sync/src/SemaphoreMutex.php" , line : 29usize , placeholders : & [] , regex : "^Cannot use a semaphore with more than a single lock$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/suspicious_login/vendor/amphp/parser/src/Parser.php" , line : 69usize , placeholders : & [] , regex : "^The parser is no longer writable$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/suspicious_login/vendor/amphp/parallel/lib/Context/Parallel.php" , line : 92usize , placeholders : & [] , regex : "^The parallel extension is required to create parallel threads\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/suspicious_login/vendor/amphp/parallel/lib/Context/Parallel.php" , line : 122usize , placeholders : & [] , regex : "^Could not locate autoload\\.php$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/suspicious_login/vendor/amphp/parallel/lib/Context/Parallel.php" , line : 173usize , placeholders : & [] , regex : "^The thread has already been started\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/suspicious_login/vendor/amphp/parallel/lib/Context/Parallel.php" , line : 262usize , placeholders : & [] , regex : "^Starting the parallel runtime failed$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/suspicious_login/vendor/amphp/parallel/lib/Context/Parallel.php" , line : 318usize , placeholders : & [] , regex : "^The thread has not been started or has already finished\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/suspicious_login/vendor/amphp/parallel/lib/Context/Parallel.php" , line : 327usize , placeholders : & [] , regex : "^Failed to receive result from thread$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/suspicious_login/vendor/amphp/parallel/lib/Context/Parallel.php" , line : 332usize , placeholders : & [] , regex : "^Did not receive an exit result from thread\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/suspicious_login/vendor/amphp/parallel/lib/Context/Parallel.php" , line : 345usize , placeholders : & [] , regex : "^The thread has not been started\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/suspicious_login/vendor/amphp/parallel/lib/Context/Parallel.php" , line : 352usize , placeholders : & [] , regex : "^The thread stopped responding, potentially due to a fatal error or calling exit$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/suspicious_login/vendor/amphp/parallel/lib/Context/Parallel.php" , line : 373usize , placeholders : & [] , regex : "^The thread has not been started or has already finished\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/suspicious_login/vendor/amphp/parallel/lib/Context/Parallel.php" , line : 377usize , placeholders : & [] , regex : "^Cannot send exit result objects\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/suspicious_login/vendor/amphp/parallel/lib/Context/Parallel.php" , line : 385usize , placeholders : & [] , regex : "^The thread stopped responding, potentially due to a fatal error or calling exit$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/suspicious_login/vendor/amphp/parallel/lib/Context/Parallel.php" , line : 392usize , placeholders : & [] , regex : "^The thread stopped responding, potentially due to a fatal error or calling exit$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/suspicious_login/vendor/amphp/parallel/lib/Context/Parallel.php" , line : 413usize , placeholders : & [] , regex : "^The thread has not been started$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/suspicious_login/vendor/amphp/parallel/lib/Context/Internal/Thread.php" , line : 85usize , placeholders : & [] , regex : "^Could not locate autoload\\.php$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/suspicious_login/vendor/amphp/parallel/lib/Context/Internal/ProcessHub.php" , line : 141usize , placeholders : & [] , regex : "^Starting the process timed out$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/suspicious_login/vendor/amphp/parallel/lib/Context/Internal/process-runner.php" , line : 87usize , placeholders : & [] , regex : "^No script path given$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/suspicious_login/vendor/amphp/parallel/lib/Context/Process.php" , line : 167usize , placeholders : & [] , regex : "^Could not locate PHP executable binary$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/suspicious_login/vendor/amphp/parallel/lib/Context/Process.php" , line : 206usize , placeholders : & [] , regex : "^Starting the process failed$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/suspicious_login/vendor/amphp/parallel/lib/Context/Process.php" , line : 225usize , placeholders : & [] , regex : "^The process has not been started$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/suspicious_login/vendor/amphp/parallel/lib/Context/Process.php" , line : 232usize , placeholders : & [] , regex : "^The process stopped responding, potentially due to a fatal error or calling exit$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/suspicious_login/vendor/amphp/parallel/lib/Context/Process.php" , line : 253usize , placeholders : & [] , regex : "^The process has not been started$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/suspicious_login/vendor/amphp/parallel/lib/Context/Process.php" , line : 257usize , placeholders : & [] , regex : "^Cannot send exit result objects$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/suspicious_login/vendor/amphp/parallel/lib/Context/Process.php" , line : 265usize , placeholders : & [] , regex : "^The process stopped responding, potentially due to a fatal error or calling exit$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/suspicious_login/vendor/amphp/parallel/lib/Context/Process.php" , line : 274usize , placeholders : & [] , regex : "^The process stopped responding, potentially due to a fatal error or calling exit$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/suspicious_login/vendor/amphp/parallel/lib/Context/Process.php" , line : 291usize , placeholders : & [] , regex : "^The process has not been started$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/suspicious_login/vendor/amphp/parallel/lib/Context/Process.php" , line : 301usize , placeholders : & [] , regex : "^Failed to receive result from process$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/suspicious_login/vendor/amphp/parallel/lib/Context/Process.php" , line : 308usize , placeholders : & [] , regex : "^Did not receive an exit result from process$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/suspicious_login/vendor/amphp/parallel/lib/Context/Thread.php" , line : 94usize , placeholders : & [] , regex : "^The pthreads extension is required to create threads\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/suspicious_login/vendor/amphp/parallel/lib/Context/Thread.php" , line : 146usize , placeholders : & [] , regex : "^The thread has already been started\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/suspicious_login/vendor/amphp/parallel/lib/Context/Thread.php" , line : 200usize , placeholders : & [] , regex : "^Could not kill thread\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/suspicious_login/vendor/amphp/parallel/lib/Context/Thread.php" , line : 234usize , placeholders : & [] , regex : "^The thread has not been started or has already finished\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/suspicious_login/vendor/amphp/parallel/lib/Context/Thread.php" , line : 244usize , placeholders : & [] , regex : "^Failed to receive result from thread$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/suspicious_login/vendor/amphp/parallel/lib/Context/Thread.php" , line : 252usize , placeholders : & [] , regex : "^Did not receive an exit result from thread\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/suspicious_login/vendor/amphp/parallel/lib/Context/Thread.php" , line : 265usize , placeholders : & [] , regex : "^The process has not been started\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/suspicious_login/vendor/amphp/parallel/lib/Context/Thread.php" , line : 295usize , placeholders : & [] , regex : "^The thread has not been started or has already finished\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/suspicious_login/vendor/amphp/parallel/lib/Context/Thread.php" , line : 299usize , placeholders : & [] , regex : "^Cannot send exit result objects\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/suspicious_login/vendor/amphp/parallel/lib/Context/Thread.php" , line : 325usize , placeholders : & [] , regex : "^The thread has not been started$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/suspicious_login/vendor/amphp/parallel/lib/Worker/TaskWorker.php" , line : 40usize , placeholders : & [] , regex : "^The context was already running$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/suspicious_login/vendor/amphp/parallel/lib/Worker/TaskWorker.php" , line : 93usize , placeholders : & [] , regex : "^The worker has been shut down$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/suspicious_login/vendor/amphp/parallel/lib/Worker/TaskWorker.php" , line : 106usize , placeholders : & [] , regex : "^The worker was shutdown$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/suspicious_login/vendor/amphp/parallel/lib/Worker/TaskWorker.php" , line : 111usize , placeholders : & [] , regex : "^The worker crashed$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/suspicious_login/vendor/amphp/parallel/lib/Worker/TaskWorker.php" , line : 128usize , placeholders : & [] , regex : "^The worker failed unexpectedly$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/suspicious_login/vendor/amphp/parallel/lib/Worker/TaskWorker.php" , line : 131usize , placeholders : & [] , regex : "^The worker exited unexpectedly$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/suspicious_login/vendor/amphp/parallel/lib/Worker/TaskWorker.php" , line : 136usize , placeholders : & [] , regex : "^Context did not return a task result$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/suspicious_login/vendor/amphp/parallel/lib/Worker/TaskWorker.php" , line : 141usize , placeholders : & [] , regex : "^Task results returned out of order$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/suspicious_login/vendor/amphp/parallel/lib/Worker/TaskWorker.php" , line : 181usize , placeholders : & [] , regex : "^Failed to gracefully shutdown worker$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/suspicious_login/vendor/amphp/parallel/lib/Worker/BasicEnvironment.php" , line : 117usize , placeholders : & [] , regex : "^The time\\-to\\-live must be a positive integer or null$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/suspicious_login/vendor/amphp/parallel/lib/Worker/CallableTask.php" , line : 29usize , placeholders : & [] , regex : "^When using a class instance as a callable, the class must be autoloadable$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/suspicious_login/vendor/amphp/parallel/lib/Worker/CallableTask.php" , line : 33usize , placeholders : & [] , regex : "^When using a class instance method as a callable, the class must be autoloadable$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/suspicious_login/vendor/amphp/parallel/lib/Worker/DefaultPool.php" , line : 55usize , placeholders : & [] , regex : "^Maximum size must be a non\\-negative integer$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/suspicious_login/vendor/amphp/parallel/lib/Worker/DefaultPool.php" , line : 217usize , placeholders : & [] , regex : "^The pool was shutdown$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/suspicious_login/vendor/amphp/parallel/lib/Worker/DefaultPool.php" , line : 229usize , placeholders : & [] , regex : "^Worker factory did not create a viable worker$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/suspicious_login/vendor/amphp/parallel/lib/Worker/Internal/worker-process.php" , line : 26usize , placeholders : & [] , regex : "^No environment class name provided$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/suspicious_login/vendor/amphp/parallel/lib/Sync/SharedMemoryParcel.php" , line : 132usize , placeholders : & [] , regex : "^The memory size must be greater than 0$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/suspicious_login/vendor/amphp/parallel/lib/Sync/SharedMemoryParcel.php" , line : 136usize , placeholders : & [] , regex : "^Invalid permissions$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/suspicious_login/vendor/amphp/parallel/lib/Sync/SharedMemoryParcel.php" , line : 200usize , placeholders : & [] , regex : "^The object has already been freed$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/suspicious_login/vendor/amphp/parallel/lib/Sync/SharedMemoryParcel.php" , line : 207usize , placeholders : & [] , regex : "^Shared object memory is corrupt$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/suspicious_login/vendor/amphp/parallel/lib/Sync/SharedMemoryParcel.php" , line : 226usize , placeholders : & [] , regex : "^The object has already been freed$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/suspicious_login/vendor/amphp/parallel/lib/Sync/SharedMemoryParcel.php" , line : 319usize , placeholders : & [] , regex : "^A shared memory parcel cannot be serialized!$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/suspicious_login/vendor/amphp/parallel/lib/Sync/ChannelledStream.php" , line : 55usize , placeholders : & [] , regex : "^Sending on the channel failed\\. Did the context die\\?$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/suspicious_login/vendor/amphp/parallel/lib/Sync/ChannelledStream.php" , line : 70usize , placeholders : & [] , regex : "^Reading from the channel failed\\. Did the context die\\?$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/suspicious_login/vendor/amphp/parallel/lib/Sync/ChannelledStream.php" , line : 74usize , placeholders : & [] , regex : "^The channel closed unexpectedly\\. Did the context die\\?$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/suspicious_login/vendor/amphp/serialization/src/PassthroughSerializer.php" , line : 10usize , placeholders : & [] , regex : "^Serializer implementation only allows strings$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/suspicious_login/vendor/amphp/serialization/src/NativeSerializer.php" , line : 42usize , placeholders : & [] , regex : "^Exception thrown when unserializing data$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/suspicious_login/vendor/amphp/amp/lib/functions.php" , line : 231usize , placeholders : & [] , regex : "^Loop exceptionally stopped without resolving the promise$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/suspicious_login/vendor/amphp/amp/lib/functions.php" , line : 235usize , placeholders : & [] , regex : "^Loop stopped without resolving the promise$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/suspicious_login/vendor/amphp/amp/lib/functions.php" , line : 333usize , placeholders : & [] , regex : "^Object must be provided$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/suspicious_login/vendor/amphp/amp/lib/functions.php" , line : 343usize , placeholders : & [] , regex : "^Object must have a 'then' or 'done' method$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/suspicious_login/vendor/amphp/amp/lib/functions.php" , line : 443usize , placeholders : & [] , regex : "^No promises provided$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/suspicious_login/vendor/amphp/amp/lib/functions.php" , line : 500usize , placeholders : & [] , regex : "^Number of promises required must be non\\-negative$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/suspicious_login/vendor/amphp/amp/lib/functions.php" , line : 506usize , placeholders : & [] , regex : "^Too few promises provided$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/suspicious_login/vendor/amphp/amp/lib/Success.php" , line : 29usize , placeholders : & [] , regex : "^Cannot use a promise as success value$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/suspicious_login/vendor/amphp/amp/lib/Producer.php" , line : 26usize , placeholders : & [] , regex : "^The callable did not return a Generator$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/suspicious_login/vendor/amphp/amp/lib/Internal/Producer.php" , line : 51usize , placeholders : & [] , regex : "^The prior promise returned must resolve before invoking this method again$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/suspicious_login/vendor/amphp/amp/lib/Internal/Producer.php" , line : 84usize , placeholders : & [] , regex : "^The iterator has completed$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/suspicious_login/vendor/amphp/amp/lib/Internal/Producer.php" , line : 88usize , placeholders : & [] , regex : "^Promise returned from advance\\(\\) must resolve before calling this method$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/suspicious_login/vendor/amphp/amp/lib/Internal/Producer.php" , line : 107usize , placeholders : & [] , regex : "^Iterators cannot emit values after calling complete$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/suspicious_login/vendor/amphp/amp/lib/Loop/NativeDriver.php" , line : 91usize , placeholders : & [] , regex : "^Signal handling requires the pcntl extension$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/suspicious_login/vendor/amphp/amp/lib/Loop/NativeDriver.php" , line : 214usize , placeholders : & [] , regex : "^Unknown watcher type$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/suspicious_login/vendor/amphp/amp/lib/Loop/NativeDriver.php" , line : 264usize , placeholders : & [] , regex : "^Unknown watcher type$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/suspicious_login/vendor/amphp/amp/lib/Loop/EvDriver.php" , line : 288usize , placeholders : & [] , regex : "^Unknown watcher type$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/suspicious_login/vendor/amphp/amp/lib/Loop/Driver.php" , line : 221usize , placeholders : & [] , regex : "^Delay must be greater than or equal to zero$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/suspicious_login/vendor/amphp/amp/lib/Loop/Driver.php" , line : 258usize , placeholders : & [] , regex : "^Interval must be greater than or equal to zero$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/suspicious_login/vendor/amphp/amp/lib/Loop/Driver.php" , line : 714usize , placeholders : & [] , regex : "^Unknown watcher type$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/suspicious_login/vendor/amphp/amp/lib/Loop/UvDriver.php" , line : 295usize , placeholders : & [] , regex : "^Unknown watcher type$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/suspicious_login/vendor/amphp/amp/lib/Loop/UvDriver.php" , line : 346usize , placeholders : & [] , regex : "^Unknown watcher type$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/suspicious_login/vendor/amphp/amp/lib/Loop/EventDriver.php" , line : 326usize , placeholders : & [] , regex : "^Unknown watcher type$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/suspicious_login/vendor/amphp/amp/lib/Loop.php" , line : 47usize , placeholders : & [] , regex : "^Can't activate watcher during garbage collection\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/suspicious_login/vendor/amphp/amp/lib/Loop.php" , line : 55usize , placeholders : & [] , regex : "^Can't dispatch during garbage collection\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Warn , path : "/apps/suspicious_login/lib/Listener/LoginMailListener.php" , line : 68usize , placeholders : & ["$uid"] , regex : "^not sending suspicious login email because user <(.*)> does not exist \\(anymore\\)$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Info , path : "/apps/suspicious_login/lib/Listener/LoginMailListener.php" , line : 72usize , placeholders : & ["$uid"] , regex : "^not sending a suspicous login email because user <(.*)> has no email set$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Critical , path : "/apps/suspicious_login/lib/Listener/LoginNotificationListener.php" , line : 69usize , placeholders : & [] , regex : "^could not send notification about a suspicious login$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Debug , path : "/apps/suspicious_login/lib/Service/ETLService.php" , line : 88usize , placeholders : & [] , regex : "^starting login data ETL process$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Debug , path : "/apps/suspicious_login/lib/Service/ETLService.php" , line : 147usize , placeholders : & [] , regex : "^finished login data ETL process, sending transaction commit$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Debug , path : "/apps/suspicious_login/lib/Service/ETLService.php" , line : 149usize , placeholders : & [] , regex : "^ETL finished$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/suspicious_login/lib/Service/IpV6Strategy.php" , line : 56usize , placeholders : & [] , regex : "^Invalid IPv6 address$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Debug , path : "/apps/suspicious_login/lib/Service/LoginClassifier.php" , line : 106usize , placeholders : & [] , regex : "^App password detected\\. No address classification is performed$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Debug , path : "/apps/suspicious_login/lib/Service/LoginClassifier.php" , line : 112usize , placeholders : & ["$ip" , "$uid"] , regex : "^Ip (.*) for user (.*) is not suspicious$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Critical , path : "/apps/suspicious_login/lib/Service/LoginClassifier.php" , line : 145usize , placeholders : & [] , regex : "^could not save the details of a suspicious login$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Debug , path : "/apps/suspicious_login/lib/Service/LoginClassifier.php" , line : 163usize , placeholders : & ["$uid" , "$ip"] , regex : "^Notification for (.*):(.*) already sent, waiting until the next training period$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Warn , path : "/apps/suspicious_login/lib/Service/LoginClassifier.php" , line : 171usize , placeholders : & ["$uid" , "$lastTwoDays"] , regex : "^Suspicious login peak detected: (.*) received (.*) alerts in the last two days$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Warn , path : "/apps/suspicious_login/lib/Service/LoginClassifier.php" , line : 179usize , placeholders : & ["$uid" , "$lastHour"] , regex : "^Suspicious login peak detected: (.*) received (.*) alerts in the last hour$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Debug , path : "/apps/suspicious_login/lib/Service/EstimatorService.php" , line : 58usize , placeholders : & [] , regex : "^loading latest model$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Debug , path : "/apps/suspicious_login/lib/Service/EstimatorService.php" , line : 62usize , placeholders : & ["$modelId"] , regex : "^loading model (.*)$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/suspicious_login/lib/Service/DataLoader.php" , line : 68usize , placeholders : & [] , regex : "^Not enough data for the specified maximum age$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/suspicious_login/lib/Service/DataLoader.php" , line : 76usize , placeholders : & [] , regex : "^No historic data available$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/suspicious_login/lib/Service/DataLoader.php" , line : 79usize , placeholders : & [] , regex : "^No recent data available$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Debug , path : "/apps/suspicious_login/lib/Service/ModelStore.php" , line : 92usize , placeholders : & [] , regex : "^No models found\\. Can't load latest$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/suspicious_login/lib/Service/ModelStore.php" , line : 93usize , placeholders : & [] , regex : "^No models found$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Debug , path : "/apps/suspicious_login/lib/Service/ModelStore.php" , line : 124usize , placeholders : & ["$id"] , regex : "^using cached model (.*)$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Debug , path : "/apps/suspicious_login/lib/Service/ModelStore.php" , line : 128usize , placeholders : & ["$id"] , regex : "^loading model (.*) from app data$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/suspicious_login/lib/Service/ModelStore.php" , line : 135usize , placeholders : & ["$id"] , regex : "^Could not load model (.*)$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/suspicious_login/lib/Service/ModelStore.php" , line : 160usize , placeholders : & [] , regex : "^Deserialized object is not an estimator$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/suspicious_login/lib/Service/ModelStore.php" , line : 172usize , placeholders : & [] , regex : "^Estimator is not persistable$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Info , path : "/apps/suspicious_login/lib/Service/ModelStore.php" , line : 182usize , placeholders : & [] , regex : "^App data models folder does not exist\\. Creating it$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Error , path : "/apps/suspicious_login/lib/Service/ModelStore.php" , line : 197usize , placeholders : & [] , regex : "^Could not save persisted estimator to storage, reverting$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Info , path : "/apps/survey_client/lib/BackgroundJobs/MonthlyReport.php" , line : 51usize , placeholders : & [] , regex : "^Error while sending usage statistic$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/cloud_federation_api/lib/Capabilities.php" , line : 69usize , placeholders : & [] , regex : "^generated route should contains a slash character$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/cloud_federation_api/composer/composer/ClassLoader.php" , line : 256usize , placeholders : & [] , regex : "^A non\\-empty PSR\\-4 prefix must end with a namespace separator\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/cloud_federation_api/composer/composer/ClassLoader.php" , line : 311usize , placeholders : & [] , regex : "^A non\\-empty PSR\\-4 prefix must end with a namespace separator\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Error , path : "/apps/files_reminders/lib/Command/ListCommand.php" , line : 74usize , placeholders : & ["$uid"] , regex : "^Unknown user <(.*)>$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/files_reminders/composer/composer/ClassLoader.php" , line : 250usize , placeholders : & [] , regex : "^A non\\-empty PSR\\-4 prefix must end with a namespace separator\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/files_reminders/composer/composer/ClassLoader.php" , line : 305usize , placeholders : & [] , regex : "^A non\\-empty PSR\\-4 prefix must end with a namespace separator\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Unknown , path : "/apps/admin_audit/lib/Actions/Sharing.php" , line : 48usize , placeholders : & [] , regex : "^The %s \"%s\" with ID \"%s\" has been shared via link with permissions \"%s\" \\(Share ID: %s\\)$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Unknown , path : "/apps/admin_audit/lib/Actions/Sharing.php" , line : 60usize , placeholders : & [] , regex : "^The %s \"%s\" with ID \"%s\" has been shared to the user \"%s\" with permissions \"%s\"  \\(Share ID: %s\\)$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Unknown , path : "/apps/admin_audit/lib/Actions/Sharing.php" , line : 73usize , placeholders : & [] , regex : "^The %s \"%s\" with ID \"%s\" has been shared to the group \"%s\" with permissions \"%s\"  \\(Share ID: %s\\)$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Unknown , path : "/apps/admin_audit/lib/Actions/Sharing.php" , line : 86usize , placeholders : & [] , regex : "^The %s \"%s\" with ID \"%s\" has been shared to the room \"%s\" with permissions \"%s\" \\(Share ID: %s\\)$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Unknown , path : "/apps/admin_audit/lib/Actions/Sharing.php" , line : 99usize , placeholders : & [] , regex : "^The %s \"%s\" with ID \"%s\" has been shared to the email recipient \"%s\" with permissions \"%s\" \\(Share ID: %s\\)$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Unknown , path : "/apps/admin_audit/lib/Actions/Sharing.php" , line : 112usize , placeholders : & [] , regex : "^The %s \"%s\" with ID \"%s\" has been shared to the circle \"%s\" with permissions \"%s\" \\(Share ID: %s\\)$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Unknown , path : "/apps/admin_audit/lib/Actions/Sharing.php" , line : 125usize , placeholders : & [] , regex : "^The %s \"%s\" with ID \"%s\" has been shared to the remote user \"%s\" with permissions \"%s\" \\(Share ID: %s\\)$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Unknown , path : "/apps/admin_audit/lib/Actions/Sharing.php" , line : 138usize , placeholders : & [] , regex : "^The %s \"%s\" with ID \"%s\" has been shared to the remote group \"%s\" with permissions \"%s\" \\(Share ID: %s\\)$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Unknown , path : "/apps/admin_audit/lib/Actions/Sharing.php" , line : 151usize , placeholders : & [] , regex : "^The %s \"%s\" with ID \"%s\" has been shared to the deck card \"%s\" with permissions \"%s\" \\(Share ID: %s\\)$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Unknown , path : "/apps/admin_audit/lib/Actions/Sharing.php" , line : 164usize , placeholders : & [] , regex : "^The %s \"%s\" with ID \"%s\" has been shared to the ScienceMesh user \"%s\" with permissions \"%s\" \\(Share ID: %s\\)$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Unknown , path : "/apps/admin_audit/lib/Actions/Sharing.php" , line : 186usize , placeholders : & [] , regex : "^The %s \"%s\" with ID \"%s\" has been unshared \\(Share ID: %s\\)$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Unknown , path : "/apps/admin_audit/lib/Actions/Sharing.php" , line : 197usize , placeholders : & [] , regex : "^The %s \"%s\" with ID \"%s\" has been unshared from the user \"%s\" \\(Share ID: %s\\)$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Unknown , path : "/apps/admin_audit/lib/Actions/Sharing.php" , line : 209usize , placeholders : & [] , regex : "^The %s \"%s\" with ID \"%s\" has been unshared from the group \"%s\" \\(Share ID: %s\\)$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Unknown , path : "/apps/admin_audit/lib/Actions/Sharing.php" , line : 221usize , placeholders : & [] , regex : "^The %s \"%s\" with ID \"%s\" has been unshared from the room \"%s\" \\(Share ID: %s\\)$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Unknown , path : "/apps/admin_audit/lib/Actions/Sharing.php" , line : 233usize , placeholders : & [] , regex : "^The %s \"%s\" with ID \"%s\" has been unshared from the email recipient \"%s\" \\(Share ID: %s\\)$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Unknown , path : "/apps/admin_audit/lib/Actions/Sharing.php" , line : 245usize , placeholders : & [] , regex : "^The %s \"%s\" with ID \"%s\" has been unshared from the circle \"%s\" \\(Share ID: %s\\)$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Unknown , path : "/apps/admin_audit/lib/Actions/Sharing.php" , line : 257usize , placeholders : & [] , regex : "^The %s \"%s\" with ID \"%s\" has been unshared from the remote user \"%s\" \\(Share ID: %s\\)$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Unknown , path : "/apps/admin_audit/lib/Actions/Sharing.php" , line : 269usize , placeholders : & [] , regex : "^The %s \"%s\" with ID \"%s\" has been unshared from the remote group \"%s\" \\(Share ID: %s\\)$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Unknown , path : "/apps/admin_audit/lib/Actions/Sharing.php" , line : 281usize , placeholders : & [] , regex : "^The %s \"%s\" with ID \"%s\" has been unshared from the deck card \"%s\" \\(Share ID: %s\\)$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Unknown , path : "/apps/admin_audit/lib/Actions/Sharing.php" , line : 293usize , placeholders : & [] , regex : "^The %s \"%s\" with ID \"%s\" has been unshared from the ScienceMesh user \"%s\" \\(Share ID: %s\\)$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Unknown , path : "/apps/admin_audit/lib/Actions/Sharing.php" , line : 313usize , placeholders : & [] , regex : "^The permissions of the shared %s \"%s\" with ID \"%s\" have been changed to \"%s\"$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Unknown , path : "/apps/admin_audit/lib/Actions/Sharing.php" , line : 331usize , placeholders : & [] , regex : "^The password of the publicly shared %s \"%s\" with ID \"%s\" has been changed$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Unknown , path : "/apps/admin_audit/lib/Actions/Sharing.php" , line : 349usize , placeholders : & [] , regex : "^The expiration date of the publicly shared %s with ID \"%s\" has been removed$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Unknown , path : "/apps/admin_audit/lib/Actions/Sharing.php" , line : 358usize , placeholders : & [] , regex : "^The expiration date of the publicly shared %s with ID \"%s\" has been changed to \"%s\"$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Unknown , path : "/apps/admin_audit/lib/Actions/Sharing.php" , line : 376usize , placeholders : & [] , regex : "^The shared %s with the token \"%s\" by \"%s\" has been accessed\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Unknown , path : "/apps/admin_audit/lib/Actions/UserManagement.php" , line : 48usize , placeholders : & [] , regex : "^User created: \"%s\"$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Unknown , path : "/apps/admin_audit/lib/Actions/UserManagement.php" , line : 63usize , placeholders : & [] , regex : "^UserID assigned: \"%s\"$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Unknown , path : "/apps/admin_audit/lib/Actions/UserManagement.php" , line : 76usize , placeholders : & [] , regex : "^User deleted: \"%s\"$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Unknown , path : "/apps/admin_audit/lib/Actions/UserManagement.php" , line : 91usize , placeholders : & [] , regex : "^UserID unassigned: \"%s\"$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Unknown , path : "/apps/admin_audit/lib/Actions/UserManagement.php" , line : 117usize , placeholders : & [] , regex : "^Email address changed for user %s$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Unknown , path : "/apps/admin_audit/lib/Actions/UserManagement.php" , line : 135usize , placeholders : & [] , regex : "^Password of user \"%s\" has been changed$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Unknown , path : "/apps/admin_audit/lib/Actions/Trashbin.php" , line : 32usize , placeholders : & [] , regex : "^File \"%s\" deleted from trash bin\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Unknown , path : "/apps/admin_audit/lib/Actions/Trashbin.php" , line : 38usize , placeholders : & [] , regex : "^File \"%s\" restored from trash bin\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Unknown , path : "/apps/admin_audit/lib/Actions/AppManagement.php" , line : 35usize , placeholders : & [] , regex : "^App \"%s\" enabled$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Unknown , path : "/apps/admin_audit/lib/Actions/AppManagement.php" , line : 46usize , placeholders : & [] , regex : "^App \"%1\\$s\" enabled for groups: %2\\$s$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Unknown , path : "/apps/admin_audit/lib/Actions/AppManagement.php" , line : 56usize , placeholders : & [] , regex : "^App \"%s\" disabled$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Unknown , path : "/apps/admin_audit/lib/Actions/Auth.php" , line : 37usize , placeholders : & [] , regex : "^Login attempt: \"%s\"$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Unknown , path : "/apps/admin_audit/lib/Actions/Auth.php" , line : 48usize , placeholders : & [] , regex : "^Login successful: \"%s\"$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Unknown , path : "/apps/admin_audit/lib/Actions/Auth.php" , line : 59usize , placeholders : & [] , regex : "^Logout occurred$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Unknown , path : "/apps/admin_audit/lib/Actions/GroupManagement.php" , line : 49usize , placeholders : & [] , regex : "^User \"%s\" added to group \"%s\"$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Unknown , path : "/apps/admin_audit/lib/Actions/GroupManagement.php" , line : 67usize , placeholders : & [] , regex : "^User \"%s\" removed from group \"%s\"$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Unknown , path : "/apps/admin_audit/lib/Actions/GroupManagement.php" , line : 84usize , placeholders : & [] , regex : "^Group created: \"%s\"$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Unknown , path : "/apps/admin_audit/lib/Actions/GroupManagement.php" , line : 100usize , placeholders : & [] , regex : "^Group deleted: \"%s\"$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Unknown , path : "/apps/admin_audit/lib/Actions/Console.php" , line : 43usize , placeholders : & [] , regex : "^Console command executed: %s$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Unknown , path : "/apps/admin_audit/lib/Actions/Versions.php" , line : 32usize , placeholders : & [] , regex : "^Version \"%s\" of \"%s\" was restored\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Unknown , path : "/apps/admin_audit/lib/Actions/Versions.php" , line : 42usize , placeholders : & [] , regex : "^Version \"%s\" was deleted\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Unknown , path : "/apps/admin_audit/lib/Actions/Files.php" , line : 42usize , placeholders : & [] , regex : "^File accessed: \"%s\"$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Unknown , path : "/apps/admin_audit/lib/Actions/Files.php" , line : 57usize , placeholders : & [] , regex : "^File renamed: \"%s\" to \"%s\"$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Unknown , path : "/apps/admin_audit/lib/Actions/Files.php" , line : 77usize , placeholders : & [] , regex : "^File created: \"%s\"$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Unknown , path : "/apps/admin_audit/lib/Actions/Files.php" , line : 92usize , placeholders : & [] , regex : "^File copied: \"%s\" to \"%s\"$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Unknown , path : "/apps/admin_audit/lib/Actions/Files.php" , line : 112usize , placeholders : & [] , regex : "^File written to: \"%s\"$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Unknown , path : "/apps/admin_audit/lib/Actions/Files.php" , line : 127usize , placeholders : & [] , regex : "^File updated: \"%s\"$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Unknown , path : "/apps/admin_audit/lib/Actions/Files.php" , line : 142usize , placeholders : & [] , regex : "^File deleted: \"%s\"$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Unknown , path : "/apps/admin_audit/lib/Actions/Files.php" , line : 157usize , placeholders : & [] , regex : "^Preview accessed: \"%s\" \\(width: \"%s\", height: \"%s\" crop: \"%s\", mode: \"%s\"\\)$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Unknown , path : "/apps/admin_audit/lib/Actions/Security.php" , line : 47usize , placeholders : & [] , regex : "^Failed two factor attempt by user %s \\(%s\\) with provider %s$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Unknown , path : "/apps/admin_audit/lib/Actions/Security.php" , line : 68usize , placeholders : & [] , regex : "^Successful two factor attempt by user %s \\(%s\\) with provider %s$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/admin_audit/composer/composer/ClassLoader.php" , line : 256usize , placeholders : & [] , regex : "^A non\\-empty PSR\\-4 prefix must end with a namespace separator\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/admin_audit/composer/composer/ClassLoader.php" , line : 311usize , placeholders : & [] , regex : "^A non\\-empty PSR\\-4 prefix must end with a namespace separator\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/settings/composer/composer/ClassLoader.php" , line : 256usize , placeholders : & [] , regex : "^A non\\-empty PSR\\-4 prefix must end with a namespace separator\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/settings/composer/composer/ClassLoader.php" , line : 311usize , placeholders : & [] , regex : "^A non\\-empty PSR\\-4 prefix must end with a namespace separator\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/settings/lib/Activity/Provider.php" , line : 88usize , placeholders : & [] , regex : "^Unknown app$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/settings/lib/Activity/Provider.php" , line : 128usize , placeholders : & [] , regex : "^Unknown subject$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/settings/lib/Activity/Provider.php" , line : 184usize , placeholders : & [] , regex : "^Unknown subject$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/settings/lib/UserMigration/AccountMigrator.php" , line : 113usize , placeholders : & [] , regex : "^Could not export account information$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/settings/lib/UserMigration/AccountMigrator.php" , line : 126usize , placeholders : & [] , regex : "^Could not export avatar$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/settings/lib/UserMigration/AccountMigrator.php" , line : 134usize , placeholders : & [] , regex : "^Could not export profile config$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/settings/lib/UserMigration/AccountMigrator.php" , line : 158usize , placeholders : & [] , regex : "^Failed to import account information$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/settings/lib/UserMigration/AccountMigrator.php" , line : 183usize , placeholders : & [] , regex : "^Avatar image must be square$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/settings/lib/UserMigration/AccountMigrator.php" , line : 185usize , placeholders : & [] , regex : "^Failed to import avatar$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/settings/lib/UserMigration/AccountMigrator.php" , line : 199usize , placeholders : & [] , regex : "^Failed to import profile config$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/settings/lib/Service/AuthorizedGroupService.php" , line : 67usize , placeholders : & [] , regex : "^AuthorizedGroup not found$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Warn , path : "/apps/settings/lib/Controller/CheckSetupController.php" , line : 330usize , placeholders : & [] , regex : "^error checking curl$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Warn , path : "/apps/settings/lib/Controller/AuthSettingsController.php" , line : 278usize , placeholders : & [] , regex : "^could not publish activity$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/settings/lib/Controller/AuthSettingsController.php" , line : 296usize , placeholders : & [] , regex : "^This token does not belong to you!$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Warn , path : "/apps/settings/lib/Controller/AppSettingsController.php" , line : 251usize , placeholders : & [] , regex : "^The value given for app_install_overwrite is not an array\\. Ignoring\\.\\.\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Error , path : "/apps/settings/lib/Controller/AppSettingsController.php" , line : 464usize , placeholders : & [] , regex : "^could not enable apps$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Error , path : "/apps/settings/lib/Controller/AppSettingsController.php" , line : 505usize , placeholders : & [] , regex : "^could not disable app$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/settings/lib/Controller/AdminSettingsController.php" , line : 81usize , placeholders : & [] , regex : "^Logged in user doesn't have permission to access these settings\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/settings/lib/Controller/LogSettingsController.php" , line : 53usize , placeholders : & [] , regex : "^Log file not available$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Debug , path : "/apps/settings/lib/Controller/WebAuthnController.php" , line : 74usize , placeholders : & [] , regex : "^Starting WebAuthn registration$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Debug , path : "/apps/settings/lib/Controller/WebAuthnController.php" , line : 91usize , placeholders : & [] , regex : "^Finishing WebAuthn registration$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Debug , path : "/apps/settings/lib/Controller/WebAuthnController.php" , line : 94usize , placeholders : & [] , regex : "^Trying to finish WebAuthn registration without session data$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Debug , path : "/apps/settings/lib/Controller/WebAuthnController.php" , line : 112usize , placeholders : & [] , regex : "^Finishing WebAuthn registration$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/files_external/lib/Config/ConfigAdapter.php" , line : 72usize , placeholders : & [] , regex : "^Invalid object store$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/files_external/lib/Config/SimpleSubstitutionTrait.php" , line : 72usize , placeholders : & [] , regex : "^Invalid empty placeholder$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/files_external/lib/Migration/Version1015Date20211104103506.php" , line : 58usize , placeholders : & [] , regex : "^Could not fetch existing mounts for migration$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/files_external/lib/Service/UserGlobalStoragesService.php" , line : 90usize , placeholders : & [] , regex : "^UserGlobalStoragesService writing disallowed$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/files_external/lib/Service/UserGlobalStoragesService.php" , line : 94usize , placeholders : & [] , regex : "^UserGlobalStoragesService writing disallowed$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/files_external/lib/Service/UserGlobalStoragesService.php" , line : 101usize , placeholders : & [] , regex : "^UserGlobalStoragesService writing disallowed$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/files_external/lib/Service/BackendService.php" , line : 331usize , placeholders : & [] , regex : "^Invalid empty placeholder$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/files_external/lib/Command/ListCommand.php" , line : 252usize , placeholders : & ["$userId"] , regex : "^user (.*) not found$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Warn , path : "/apps/files_external/lib/Command/Notify.php" , line : 224usize , placeholders : & [] , regex : "^Error while trying to find correct storage ids\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Warn , path : "/apps/files_external/lib/Command/Notify.php" , line : 310usize , placeholders : & [] , regex : "^Error while disconnecting from DB$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Warn , path : "/apps/files_external/lib/Command/Notify.php" , line : 318usize , placeholders : & [] , regex : "^Error while re\\-connecting to database$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/files_external/lib/Command/Import.php" , line : 199usize , placeholders : & ["$userId"] , regex : "^user (.*) not found$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/files_external/lib/Command/Create.php" , line : 202usize , placeholders : & ["$userId"] , regex : "^user (.*) not found$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/files_external/lib/Lib/Storage/SMB.php" , line : 103usize , placeholders : & [] , regex : "^Invalid configuration, no host provided$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/files_external/lib/Lib/Storage/SMB.php" , line : 112usize , placeholders : & [] , regex : "^Invalid configuration, no credentials provided$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/files_external/lib/Lib/Storage/SMB.php" , line : 528usize , placeholders : & [] , regex : "^not enough available space to create file$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/files_external/lib/Lib/Storage/SMB.php" , line : 571usize , placeholders : & [] , regex : "^not enough available space to create file$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/files_external/lib/Lib/Storage/FtpConnection.php" , line : 41usize , placeholders : & [] , regex : "^Failed to connect to ftp$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/files_external/lib/Lib/Storage/FtpConnection.php" , line : 45usize , placeholders : & [] , regex : "^Failed to connect to login to ftp$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/files_external/lib/Lib/Storage/FtpConnection.php" , line : 157usize , placeholders : & ["$item"] , regex : "^Metadata can't be parsed from item '(.*)' , not enough parts\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/files_external/lib/Lib/Storage/FtpConnection.php" , line : 215usize , placeholders : & ["$item"] , regex : "^Metadata can't be parsed from item '(.*)' , not enough parts\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/files_external/lib/Lib/Storage/FTP.php" , line : 84usize , placeholders : & [] , regex : "^Failed to create ftp connection$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/files_external/lib/Lib/Storage/FTP.php" , line : 88usize , placeholders : & [] , regex : "^Could not set UTF\\-8 mode$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Warn , path : "/apps/files_external/lib/Lib/Storage/FTP.php" , line : 119usize , placeholders : & ["$path"] , regex : "^Unable to get last modified date for ftp folder \\((.*)\\), failed to list folder contents$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/files_external/lib/Lib/Storage/FTP.php" , line : 129usize , placeholders : & ["$currentDir"] , regex : "^Invalid date format for directory: (.*)$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Warn , path : "/apps/files_external/lib/Lib/Storage/FTP.php" , line : 133usize , placeholders : & ["$path"] , regex : "^Unable to get last modified date for ftp folder \\((.*)\\), folder contents doesn't include current folder$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/files_external/lib/Lib/Storage/SFTPWriteStream.php" , line : 76usize , placeholders : & [] , regex : "^Invalid context, session not set$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/files_external/lib/Lib/Storage/SFTPReadStream.php" , line : 78usize , placeholders : & [] , regex : "^Invalid context, session not set$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/files_external/lib/Lib/Storage/SFTP.php" , line : 103usize , placeholders : & [] , regex : "^no authentication parameters specified$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/files_external/lib/Lib/Storage/SFTP.php" , line : 115usize , placeholders : & [] , regex : "^no authentication parameters specified$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/files_external/lib/Lib/Storage/SFTP.php" , line : 144usize , placeholders : & [] , regex : "^Host public key does not match known key$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/files_external/lib/Lib/Storage/SFTP.php" , line : 161usize , placeholders : & [] , regex : "^Login failed$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/files_external/lib/Lib/Storage/SFTP.php" , line : 510usize , placeholders : & [] , regex : "^Failed to wrap stream$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/files_external/lib/Lib/Storage/SFTP.php" , line : 519usize , placeholders : & [] , regex : "^Failed to write steam to sftp storage$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/files_external/lib/Lib/Storage/Swift.php" , line : 169usize , placeholders : & [] , regex : "^API Key or password, Username, Bucket and Region have to be configured\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/files_external/lib/Lib/Storage/Swift.php" , line : 560usize , placeholders : & [] , regex : "^failed to remove original$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/files_external/lib/Lib/Auth/PublicKey/RSAPrivateKey.php" , line : 64usize , placeholders : & [] , regex : "^unable to load private key$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/files_external/lib/Lib/Auth/PublicKey/RSA.php" , line : 66usize , placeholders : & [] , regex : "^unable to load private key$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/files_external/lib/Lib/Auth/Password/UserProvided.php" , line : 76usize , placeholders : & [] , regex : "^No credentials saved$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/files_external/lib/Lib/Auth/Password/UserProvided.php" , line : 82usize , placeholders : & [] , regex : "^No credentials saved$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/files_external/lib/Lib/Auth/Password/GlobalAuth.php" , line : 77usize , placeholders : & [] , regex : "^No credentials saved$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/files_external/lib/Lib/Auth/Password/SessionCredentials.php" , line : 58usize , placeholders : & [] , regex : "^No session credentials saved$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/files_external/lib/Lib/Auth/Password/SessionCredentials.php" , line : 62usize , placeholders : & [] , regex : "^Session credentials for storage owner not available$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/files_external/lib/Lib/Auth/Password/UserGlobalAuth.php" , line : 74usize , placeholders : & [] , regex : "^No credentials saved$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/files_external/lib/Lib/Auth/Password/LoginCredentials.php" , line : 108usize , placeholders : & [] , regex : "^No login credentials saved$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/files_external/lib/Lib/Auth/Password/LoginCredentials.php" , line : 117usize , placeholders : & [] , regex : "^No login credentials saved$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/files_external/lib/Lib/Backend/SMB.php" , line : 84usize , placeholders : & [] , regex : "^user or password is not set$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/files_external/lib/Lib/Backend/SMB.php" , line : 99usize , placeholders : & [] , regex : "^invalid authentication backend$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/files_external/lib/Lib/Backend/SMB.php" , line : 129usize , placeholders : & [] , regex : "^No session credentials saved$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/files_external/lib/Lib/Backend/SMB.php" , line : 135usize , placeholders : & [] , regex : "^unknown authentication backend$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/files_external/3rdparty/icewind/smb/src/KerberosApacheAuth.php" , line : 104usize , placeholders : & [] , regex : "^Ensure php\\-krb5 is installed\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/files_external/3rdparty/icewind/smb/src/KerberosApacheAuth.php" , line : 109usize , placeholders : & [] , regex : "^No kerberos ticket cache environment variable \\(KRB5CCNAME\\) found\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/files_external/3rdparty/icewind/smb/src/System.php" , line : 33usize , placeholders : & [] , regex : "^Cant find file descriptor path$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/files_external/3rdparty/icewind/smb/src/ServerFactory.php" , line : 83usize , placeholders : & [] , regex : "^No valid backend available, ensure smbclient is in the path or php\\-smbclient is installed$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/files_external/3rdparty/icewind/smb/src/Native/NativeShare.php" , line : 227usize , placeholders : & [] , regex : "^Invalid target path: Filename cannot be empty$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/files_external/3rdparty/icewind/smb/src/Native/NativeShare.php" , line : 321usize , placeholders : & [] , regex : "^Invalid value for attribute$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/files_external/3rdparty/icewind/smb/src/Native/NativeShare.php" , line : 350usize , placeholders : & [] , regex : "^smbclient not found in path for notify command$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/files_external/3rdparty/icewind/smb/src/Native/NativeStream.php" , line : 56usize , placeholders : & [] , regex : "^Failed to register stream wrapper$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/files_external/3rdparty/icewind/smb/src/Native/NativeStream.php" , line : 67usize , placeholders : & [] , regex : "^Failed to unregister stream wrapper$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/files_external/3rdparty/icewind/smb/src/Native/NativeStream.php" , line : 92usize , placeholders : & [] , regex : "^context not set$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/files_external/3rdparty/icewind/smb/src/Native/NativeStream.php" , line : 96usize , placeholders : & [] , regex : "^invalid context set$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/files_external/3rdparty/icewind/smb/src/Native/NativeStream.php" , line : 101usize , placeholders : & [] , regex : "^invalid context set$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/files_external/3rdparty/icewind/smb/src/Native/NativeStream.php" , line : 106usize , placeholders : & [] , regex : "^invalid context set$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/files_external/3rdparty/icewind/smb/src/Native/NativeFileInfo.php" , line : 55usize , placeholders : & [] , regex : "^Invalid attribute response$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/files_external/3rdparty/icewind/smb/src/Native/NativeFileInfo.php" , line : 58usize , placeholders : & [] , regex : "^Invalid attribute response$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/files_external/3rdparty/icewind/smb/src/Native/NativeFileInfo.php" , line : 61usize , placeholders : & [] , regex : "^Invalid attribute response$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/files_external/3rdparty/icewind/smb/src/Native/NativeState.php" , line : 373usize , placeholders : & [] , regex : "^Failed to free smb state$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/files_external/3rdparty/icewind/smb/src/AnonymousAuth.php" , line : 45usize , placeholders : & [] , regex : "^Failed to set smbclient options for anonymous auth$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/files_external/3rdparty/icewind/smb/src/Wrapped/Server.php" , line : 51usize , placeholders : & [] , regex : "^Backend not available$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/files_external/3rdparty/icewind/smb/src/Wrapped/Parser.php" , line : 107usize , placeholders : & [] , regex : "^Invalid login$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/files_external/3rdparty/icewind/smb/src/Wrapped/Parser.php" , line : 110usize , placeholders : & [] , regex : "^Invalid hostname$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/files_external/3rdparty/icewind/smb/src/Wrapped/Parser.php" , line : 113usize , placeholders : & [] , regex : "^Connection unsuccessful$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/files_external/3rdparty/icewind/smb/src/Wrapped/Parser.php" , line : 116usize , placeholders : & [] , regex : "^Connection refused$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/files_external/3rdparty/icewind/smb/src/Wrapped/Parser.php" , line : 119usize , placeholders : & [] , regex : "^No login server$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/files_external/3rdparty/icewind/smb/src/Wrapped/Parser.php" , line : 122usize , placeholders : & [] , regex : "^Access denied$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/files_external/3rdparty/icewind/smb/src/Wrapped/Parser.php" , line : 157usize , placeholders : & [] , regex : "^Malformed state response from server$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/files_external/3rdparty/icewind/smb/src/Wrapped/RawConnection.php" , line : 60usize , placeholders : & [] , regex : "^Authentication not set before connecting$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/files_external/3rdparty/icewind/smb/src/Wrapped/Share.php" , line : 93usize , placeholders : & [] , regex : "^Backend not available$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/files_external/3rdparty/icewind/smb/src/Wrapped/Share.php" , line : 357usize , placeholders : & [] , regex : "^Failed to wrap file output$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/files_external/3rdparty/icewind/smb/src/Wrapped/Share.php" , line : 404usize , placeholders : & [] , regex : "^php\\-libsmbclient is required for append$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/files_external/3rdparty/icewind/smb/src/Wrapped/Share.php" , line : 444usize , placeholders : & [] , regex : "^stdbuf is required for usage of the notify command$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/files_external/3rdparty/icewind/smb/src/Wrapped/Connection.php" , line : 75usize , placeholders : & [] , regex : "^Connection not valid$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/files_external/3rdparty/icewind/smb/src/Wrapped/Connection.php" , line : 104usize , placeholders : & [] , regex : "^Unknown error$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/files_external/3rdparty/icewind/smb/src/KerberosAuth.php" , line : 51usize , placeholders : & [] , regex : "^Failed to set smbclient options for kerberos auth$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/files_external/3rdparty/icewind/streams/src/CountWrapper.php" , line : 64usize , placeholders : & [] , regex : "^Invalid or missing callback$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/files_external/3rdparty/icewind/streams/src/IteratorDirectory.php" , line : 48usize , placeholders : & [] , regex : "^Invalid context, iterator or array not set$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/files_external/3rdparty/icewind/streams/src/IteratorDirectory.php" , line : 109usize , placeholders : & [] , regex : "^\\$source should be an Iterator or array$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/files_external/3rdparty/icewind/streams/src/UrlCallback.php" , line : 133usize , placeholders : & [] , regex : "^stat is not supported due to php bug 50526$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/files_external/3rdparty/icewind/streams/src/WrapperHandler.php" , line : 87usize , placeholders : & [] , regex : "^Invalid stream source$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/files_external/3rdparty/icewind/streams/src/Wrapper.php" , line : 40usize , placeholders : & [] , regex : "^Invalid context, source not set$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/files_external/3rdparty/composer/ClassLoader.php" , line : 256usize , placeholders : & [] , regex : "^A non\\-empty PSR\\-4 prefix must end with a namespace separator\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/files_external/3rdparty/composer/ClassLoader.php" , line : 311usize , placeholders : & [] , regex : "^A non\\-empty PSR\\-4 prefix must end with a namespace separator\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/files_external/composer/composer/ClassLoader.php" , line : 256usize , placeholders : & [] , regex : "^A non\\-empty PSR\\-4 prefix must end with a namespace separator\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/files_external/composer/composer/ClassLoader.php" , line : 311usize , placeholders : & [] , regex : "^A non\\-empty PSR\\-4 prefix must end with a namespace separator\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/dashboard/composer/composer/ClassLoader.php" , line : 256usize , placeholders : & [] , regex : "^A non\\-empty PSR\\-4 prefix must end with a namespace separator\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/dashboard/composer/composer/ClassLoader.php" , line : 311usize , placeholders : & [] , regex : "^A non\\-empty PSR\\-4 prefix must end with a namespace separator\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/files/composer/composer/ClassLoader.php" , line : 256usize , placeholders : & [] , regex : "^A non\\-empty PSR\\-4 prefix must end with a namespace separator\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/files/composer/composer/ClassLoader.php" , line : 311usize , placeholders : & [] , regex : "^A non\\-empty PSR\\-4 prefix must end with a namespace separator\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/files/lib/Helper.php" , line : 229usize , placeholders : & [] , regex : "^\\$tags must be an array$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/files/lib/Command/Object/ObjectUtil.php" , line : 70usize , placeholders : & [] , regex : "^no arguments configured for object store configuration$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/files/lib/Command/Object/ObjectUtil.php" , line : 73usize , placeholders : & [] , regex : "^no class configured for object store configuration$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/files/lib/Command/Object/ObjectUtil.php" , line : 85usize , placeholders : & [] , regex : "^configured object store class is not an object store implementation$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/files/lib/Collaboration/Resources/ResourceProvider.php" , line : 99usize , placeholders : & [] , regex : "^File not found$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/files/lib/Notification/Notifier.php" , line : 88usize , placeholders : & [] , regex : "^Unhandled app$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/files/lib/Notification/Notifier.php" , line : 107usize , placeholders : & [] , regex : "^Unhandled subject$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/files/lib/Notification/Notifier.php" , line : 263usize , placeholders : & [] , regex : "^Unhandled app$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/files/lib/Notification/Notifier.php" , line : 293usize , placeholders : & [] , regex : "^User not found$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/files/lib/Service/UserConfig.php" , line : 104usize , placeholders : & [] , regex : "^No user logged in$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/files/lib/Service/UserConfig.php" , line : 108usize , placeholders : & [] , regex : "^Unknown config key$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/files/lib/Service/UserConfig.php" , line : 112usize , placeholders : & [] , regex : "^Invalid config value$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/files/lib/Service/UserConfig.php" , line : 129usize , placeholders : & [] , regex : "^No user logged in$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/files/lib/Service/ViewConfig.php" , line : 113usize , placeholders : & [] , regex : "^No user logged in$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/files/lib/Service/ViewConfig.php" , line : 117usize , placeholders : & [] , regex : "^Unknown view$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/files/lib/Service/ViewConfig.php" , line : 121usize , placeholders : & [] , regex : "^Unknown config key$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/files/lib/Service/ViewConfig.php" , line : 126usize , placeholders : & [] , regex : "^Invalid config value$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/files/lib/Service/ViewConfig.php" , line : 147usize , placeholders : & [] , regex : "^No user logged in$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/files/lib/Service/ViewConfig.php" , line : 172usize , placeholders : & [] , regex : "^No user logged in$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/files/lib/Service/TagService.php" , line : 78usize , placeholders : & [] , regex : "^No tagger set$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/files/lib/Service/TagService.php" , line : 81usize , placeholders : & [] , regex : "^No homeFolder set$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/files/lib/Service/OwnershipTransferService.php" , line : 115usize , placeholders : & [] , regex : "^The target user is not ready to accept files\\. The user has at least to have logged in once\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/files/lib/Service/OwnershipTransferService.php" , line : 149usize , placeholders : & ["$path"] , regex : "^Unknown path provided: (.*)$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/files/lib/Service/OwnershipTransferService.php" , line : 158usize , placeholders : & [] , regex : "^Destination path does not exists or is not empty$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/files/lib/Service/OwnershipTransferService.php" , line : 250usize , placeholders : & [] , regex : "^Execution terminated\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/files/lib/Service/OwnershipTransferService.php" , line : 283usize , placeholders : & [] , regex : "^Execution terminated\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/files/lib/Service/OwnershipTransferService.php" , line : 401usize , placeholders : & [] , regex : "^Could not transfer files\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Warn , path : "/apps/files/lib/BackgroundJob/ScanFiles.php" , line : 120usize , placeholders : & ["$user"] , regex : "^User (.*) still has unscanned files after running background scan, background scan might be stopped prematurely$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Debug , path : "/apps/files/lib/BackgroundJob/DeleteOrphanedItems.php" , line : 118usize , placeholders : & ["$deletedEntries"] , regex : "^(.*) orphaned system tag relations deleted$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Debug , path : "/apps/files/lib/BackgroundJob/DeleteOrphanedItems.php" , line : 129usize , placeholders : & ["$deletedEntries"] , regex : "^(.*) orphaned user tag relations deleted$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Debug , path : "/apps/files/lib/BackgroundJob/DeleteOrphanedItems.php" , line : 140usize , placeholders : & ["$deletedEntries"] , regex : "^(.*) orphaned comments deleted$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Debug , path : "/apps/files/lib/BackgroundJob/DeleteOrphanedItems.php" , line : 151usize , placeholders : & ["$deletedEntries"] , regex : "^(.*) orphaned comment read marks deleted$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Alert , path : "/apps/files/lib/BackgroundJob/TransferOwnership.php" , line : 90usize , placeholders : & [] , regex : "^Could not transfer ownership: Node not found$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Alert , path : "/apps/files/lib/BackgroundJob/TransferOwnership.php" , line : 106usize , placeholders : & ["$destinationUser"] , regex : "^Unknown destination user (.*)$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/files/lib/Activity/Helper.php" , line : 54usize , placeholders : & [] , regex : "^No favorites$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/files/lib/Activity/Helper.php" , line : 56usize , placeholders : & [] , regex : "^Too many favorites$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/files/lib/Activity/Helper.php" , line : 77usize , placeholders : & [] , regex : "^No favorites$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/files/lib/Activity/Provider.php" , line : 380usize , placeholders : & [] , regex : "^Could not generate file parameter$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/files/lib/Activity/Provider.php" , line : 391usize , placeholders : & [] , regex : "^Path could not be split correctly$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/files/lib/Activity/Provider.php" , line : 474usize , placeholders : & [] , regex : "^Reached the root$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/oauth2/composer/composer/ClassLoader.php" , line : 256usize , placeholders : & [] , regex : "^A non\\-empty PSR\\-4 prefix must end with a namespace separator\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/oauth2/composer/composer/ClassLoader.php" , line : 311usize , placeholders : & [] , regex : "^A non\\-empty PSR\\-4 prefix must end with a namespace separator\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Warn , path : "/apps/oauth2/lib/BackgroundJob/CleanupExpiredAuthorizationCode.php" , line : 58usize , placeholders : & [] , regex : "^Failed to cleanup tokens with expired authorization code$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/oauth2/lib/Db/AccessTokenMapper.php" , line : 67usize , placeholders : & [] , regex : "^Could not find access token$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Error , path : "/apps/oauth2/lib/Settings/Admin.php" , line : 63usize , placeholders : & [] , regex : "^\\[Settings\\] OAuth client secret decryption error$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Error , path : "/apps/oauth2/lib/Controller/OauthApiController.php" , line : 154usize , placeholders : & [] , regex : "^OAuth client secret decryption error$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/recommendations/lib/Controller/RecommendationController.php" , line : 59usize , placeholders : & [] , regex : "^Not logged in$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/recommendations/lib/Controller/RecommendationController.php" , line : 78usize , placeholders : & [] , regex : "^Not logged in$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/recommendations/lib/Controller/SettingsController.php" , line : 59usize , placeholders : & [] , regex : "^Not logged in$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/recommendations/lib/Controller/SettingsController.php" , line : 74usize , placeholders : & [] , regex : "^Not logged in$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/bruteforcesettings/vendor/composer/ClassLoader.php" , line : 250usize , placeholders : & [] , regex : "^A non\\-empty PSR\\-4 prefix must end with a namespace separator\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/bruteforcesettings/vendor/composer/ClassLoader.php" , line : 305usize , placeholders : & [] , regex : "^A non\\-empty PSR\\-4 prefix must end with a namespace separator\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/serverinfo/lib/OperatingSystems/DefaultOs.php" , line : 277usize , placeholders : & [] , regex : "^Unable to get network interfaces$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/serverinfo/lib/OperatingSystems/FreeBSD.php" , line : 246usize , placeholders : & [] , regex : "^Unable to get network interfaces$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/files_sharing/composer/composer/ClassLoader.php" , line : 256usize , placeholders : & [] , regex : "^A non\\-empty PSR\\-4 prefix must end with a namespace separator\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/files_sharing/composer/composer/ClassLoader.php" , line : 311usize , placeholders : & [] , regex : "^A non\\-empty PSR\\-4 prefix must end with a namespace separator\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Error , path : "/apps/files_sharing/lib/ShareBackend/File.php" , line : 236usize , placeholders : & [] , regex : "^No owner found for reshare$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Debug , path : "/apps/files_sharing/lib/DeleteOrphanedSharesJob.php" , line : 110usize , placeholders : & [] , regex : "^\\{deleted\\} orphaned share\\(s\\) deleted$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Error , path : "/apps/files_sharing/lib/MountProvider.php" , line : 161usize , placeholders : & [] , regex : "^Error while trying to create shared mount$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/files_sharing/lib/Middleware/SharingCheckMiddleware.php" , line : 91usize , placeholders : & [] , regex : "^Sharing is disabled\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/files_sharing/lib/Middleware/SharingCheckMiddleware.php" , line : 96usize , placeholders : & [] , regex : "^Federated sharing not allowed$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Debug , path : "/apps/files_sharing/lib/Listener/ShareInteractionListener.php" , line : 71usize , placeholders : & [] , regex : "^Share type does not allow to emit interaction event$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Warn , path : "/apps/files_sharing/lib/Listener/ShareInteractionListener.php" , line : 77usize , placeholders : & [] , regex : "^Share was not created by a user, can't emit interaction event$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/files_sharing/lib/Notification/Notifier.php" , line : 109usize , placeholders : & [] , regex : "^Unhandled app or subject$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/files_sharing/lib/Notification/Notifier.php" , line : 168usize , placeholders : & [] , regex : "^Invalid share type$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/files_sharing/lib/Notification/Notifier.php" , line : 179usize , placeholders : & [] , regex : "^Temporary failure$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/files_sharing/lib/Notification/Notifier.php" , line : 215usize , placeholders : & [] , regex : "^Temporary failure$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/files_sharing/lib/Notification/Notifier.php" , line : 239usize , placeholders : & [] , regex : "^Invalid subject$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/files_sharing/lib/Activity/Providers/Base.php" , line : 146usize , placeholders : & [] , regex : "^Could not generate file parameter$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/files_sharing/lib/SharedMount.php" , line : 201usize , placeholders : & [] , regex : "^Path does not start with /user/files$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/files_sharing/lib/Controller/ShareesAPIController.php" , line : 142usize , placeholders : & [] , regex : "^Invalid perPage argument$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/files_sharing/lib/Controller/ShareesAPIController.php" , line : 145usize , placeholders : & [] , regex : "^Invalid page$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/files_sharing/lib/Controller/ShareesAPIController.php" , line : 153usize , placeholders : & [] , regex : "^Missing itemType$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/files_sharing/lib/Controller/ShareesAPIController.php" , line : 331usize , placeholders : & [] , regex : "^Missing itemType$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/files_sharing/lib/Controller/ShareController.php" , line : 540usize , placeholders : & [] , regex : "^Downloading more than 1 file$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/files_sharing/lib/Controller/ShareController.php" , line : 557usize , placeholders : & [] , regex : "^Downloading a folder$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/files_sharing/lib/Controller/ShareAPIController.php" , line : 1061usize , placeholders : & [] , regex : "^no sharing rights on this item$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/files_sharing/lib/Controller/ShareAPIController.php" , line : 1171usize , placeholders : & [] , regex : "^You are not allowed to edit incoming shares$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/files_sharing/lib/Controller/ShareAPIController.php" , line : 1212usize , placeholders : & [] , regex : "^You are not allowed to edit link shares that you don't own$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/files_sharing/lib/Controller/ShareAPIController.php" , line : 1284usize , placeholders : & [] , regex : "^Maximum label length is 255$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/files_sharing/lib/Controller/ShareAPIController.php" , line : 1627usize , placeholders : & [] , regex : "^Invalid date\\. Format must be YYYY\\-MM\\-DD$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/files_sharing/lib/Controller/ShareAPIController.php" , line : 1835usize , placeholders : & [] , regex : "^no sharing rights on this item$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/files_sharing/lib/Controller/ShareAPIController.php" , line : 2028usize , placeholders : & [] , regex : "^Should not happen, instanceOfStorage but getInstanceOfStorage return null$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/files_sharing/lib/Controller/ShareAPIController.php" , line : 2031usize , placeholders : & [] , regex : "^Should not happen, instanceOfStorage but not a wrapper$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/files_sharing/lib/Controller/DeletedShareAPIController.php" , line : 202usize , placeholders : & [] , regex : "^Share not found$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/files_sharing/lib/Controller/DeletedShareAPIController.php" , line : 206usize , placeholders : & [] , regex : "^No deleted share found$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/files_sharing/lib/Controller/DeletedShareAPIController.php" , line : 212usize , placeholders : & [] , regex : "^Something went wrong$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/files_sharing/lib/Controller/RemoteController.php" , line : 89usize , placeholders : & [] , regex : "^wrong share ID, share does not exist\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/files_sharing/lib/Controller/RemoteController.php" , line : 109usize , placeholders : & [] , regex : "^wrong share ID, share does not exist\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/files_sharing/lib/Controller/RemoteController.php" , line : 160usize , placeholders : & [] , regex : "^share does not exist$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/files_sharing/lib/Controller/RemoteController.php" , line : 181usize , placeholders : & [] , regex : "^Share does not exist$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/files_sharing/lib/Controller/RemoteController.php" , line : 189usize , placeholders : & [] , regex : "^Could not unshare$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Info , path : "/apps/files_sharing/lib/BackgroundJob/FederatedSharesDiscoverJob.php" , line : 63usize , placeholders : & [] , regex : "^exception while running files_sharing/lib/BackgroundJob/FederatedSharesDiscoverJob$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Warn , path : "/apps/files_sharing/lib/SharedStorage.php" , line : 155usize , placeholders : & [] , regex : "^Possible share setup recursion detected$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/files_sharing/lib/SharedStorage.php" , line : 168usize , placeholders : & [] , regex : "^Maximum share depth reached$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/files_sharing/lib/SharedStorage.php" , line : 184usize , placeholders : & [] , regex : "^recursive share detected$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Notice , path : "/apps/files_sharing/lib/External/Storage.php" , line : 88usize , placeholders : & [] , regex : "^exception while retrieving webdav endpoint$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/files_sharing/lib/External/Storage.php" , line : 241usize , placeholders : & [] , regex : "^Remote share not found$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/files_sharing/lib/External/Storage.php" , line : 244usize , placeholders : & [] , regex : "^No nextcloud instance found at remote$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/files_sharing/lib/External/Storage.php" , line : 250usize , placeholders : & [] , regex : "^Auth error when getting remote share$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/files_sharing/lib/External/Storage.php" , line : 252usize , placeholders : & [] , regex : "^Failed to connect to remote instance$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/files_sharing/lib/External/Storage.php" , line : 254usize , placeholders : & [] , regex : "^Error while sending request to remote instance$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Emergency , path : "/apps/files_sharing/lib/External/Manager.php" , line : 350usize , placeholders : & [] , regex : "^Could not update share$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Emergency , path : "/apps/files_sharing/lib/External/Manager.php" , line : 368usize , placeholders : & [] , regex : "^Could not create share$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Emergency , path : "/apps/files_sharing/lib/External/Manager.php" , line : 420usize , placeholders : & [] , regex : "^Could not update share$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Emergency , path : "/apps/files_sharing/lib/External/Manager.php" , line : 440usize , placeholders : & [] , regex : "^Could not create share$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Error , path : "/apps/files_sharing/lib/External/Manager.php" , line : 606usize , placeholders : & [] , regex : "^Mount point to remove share not found$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Error , path : "/apps/files_sharing/lib/External/Manager.php" , line : 610usize , placeholders : & [] , regex : "^Mount point to remove share is not an external share, share probably doesn't exist$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Emergency , path : "/apps/files_sharing/lib/External/Manager.php" , line : 646usize , placeholders : & [] , regex : "^Could not update share$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Emergency , path : "/apps/files_sharing/lib/External/Manager.php" , line : 714usize , placeholders : & [] , regex : "^Could not delete user shares$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Emergency , path : "/apps/files_sharing/lib/External/Manager.php" , line : 749usize , placeholders : & [] , regex : "^Could not delete user shares$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Emergency , path : "/apps/files_sharing/lib/External/Manager.php" , line : 827usize , placeholders : & [] , regex : "^Error when retrieving shares$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/nextcloud_announcements/lib/Cron/Crawler.php" , line : 83usize , placeholders : & [] , regex : "^Invalid XML feed$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/nextcloud_announcements/lib/Cron/Crawler.php" , line : 141usize , placeholders : & [] , regex : "^Invalid signature fetched from the server$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/nextcloud_announcements/lib/Cron/Crawler.php" , line : 153usize , placeholders : & [] , regex : "^Could not validate CRL signature$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/nextcloud_announcements/lib/Cron/Crawler.php" , line : 158usize , placeholders : & [] , regex : "^Certificate has been revoked$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/nextcloud_announcements/lib/Cron/Crawler.php" , line : 163usize , placeholders : & [] , regex : "^App with id nextcloud_announcements has a certificate not issued by a trusted Code Signing Authority$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/nextcloud_announcements/lib/Cron/Crawler.php" , line : 169usize , placeholders : & [] , regex : "^App with id nextcloud_announcements has a cert with no CN$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/nextcloud_announcements/lib/Cron/Crawler.php" , line : 188usize , placeholders : & [] , regex : "^Feed has an invalid signature$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/nextcloud_announcements/lib/Cron/Crawler.php" , line : 203usize , placeholders : & [] , regex : "^Could not load file$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/encryption/composer/composer/ClassLoader.php" , line : 256usize , placeholders : & [] , regex : "^A non\\-empty PSR\\-4 prefix must end with a namespace separator\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/encryption/composer/composer/ClassLoader.php" , line : 311usize , placeholders : & [] , regex : "^A non\\-empty PSR\\-4 prefix must end with a namespace separator\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/encryption/lib/Session.php" , line : 90usize , placeholders : & [] , regex : "^please try to log\\-out and log\\-in again$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/encryption/lib/Session.php" , line : 151usize , placeholders : & [] , regex : "^No uid found while in decrypt all mode$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/encryption/lib/Session.php" , line : 153usize , placeholders : & [] , regex : "^Please activate decrypt all mode first$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/encryption/lib/Session.php" , line : 168usize , placeholders : & [] , regex : "^No private key found while in decrypt all mode$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/encryption/lib/Session.php" , line : 170usize , placeholders : & [] , regex : "^Please activate decrypt all mode first$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Warn , path : "/apps/encryption/lib/Crypto/Encryption.php" , line : 286usize , placeholders : & [] , regex : "^no public key found for user \"\\{uid\\}\", user will not be able to read the file$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Warn , path : "/apps/encryption/lib/Crypto/Encryption.php" , line : 301usize , placeholders : & [] , regex : "^Failed to delete legacy filekey for \\{path\\}$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Debug , path : "/apps/encryption/lib/Crypto/Encryption.php" , line : 435usize , placeholders : & [] , regex : "^no file key found, we assume that the file \"\\{file\\}\" is not encrypted$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Error , path : "/apps/encryption/lib/Crypto/Crypt.php" , line : 134usize , placeholders : & ["$this->user"] , regex : "^Encryption Library couldn't generate users key\\-pair for (.*)$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Error , path : "/apps/encryption/lib/Crypto/Crypt.php" , line : 192usize , placeholders : & [] , regex : "^Encryption Library, symmetrical encryption failed no content given$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/encryption/lib/Crypto/Crypt.php" , line : 330usize , placeholders : & [] , regex : "^Legacy cipher is no longer supported!$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/encryption/lib/Crypto/Crypt.php" , line : 528usize , placeholders : & [] , regex : "^Bad Signature$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Info , path : "/apps/encryption/lib/Crypto/Crypt.php" , line : 530usize , placeholders : & [] , regex : "^Signature check skipped$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/encryption/lib/Crypto/Crypt.php" , line : 607usize , placeholders : & [] , regex : "^Missing Signature$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/encryption/lib/Crypto/Crypt.php" , line : 612usize , placeholders : & [] , regex : "^Missing Signature$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/encryption/lib/Crypto/Crypt.php" , line : 711usize , placeholders : & [] , regex : "^Cannot multikey decrypt empty plain content$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/encryption/lib/Crypto/Crypt.php" , line : 728usize , placeholders : & [] , regex : "^Cannot multikeyencrypt empty plain content$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/encryption/lib/Crypto/Crypt.php" , line : 775usize , placeholders : & [] , regex : "^Cannot multikeyencrypt empty plain content$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/encryption/lib/Command/FixKeyLocation.php" , line : 174usize , placeholders : & [] , regex : "^Read failed$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Error , path : "/apps/encryption/lib/Hooks/UserHooks.php" , line : 269usize , placeholders : & [] , regex : "^Encryption could not update users encryption password$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Error , path : "/apps/encryption/lib/Hooks/UserHooks.php" , line : 322usize , placeholders : & [] , regex : "^Encryption Could not update users encryption password$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Error , path : "/apps/encryption/lib/KeyManager.php" , line : 226usize , placeholders : & [] , regex : "^A private master key is available but the public key could not be found\\. This should never happen\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Error , path : "/apps/encryption/lib/KeyManager.php" , line : 229usize , placeholders : & [] , regex : "^A public master key is available but the private key could not be found\\. This should never happen\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/encryption/lib/KeyManager.php" , line : 746usize , placeholders : & [] , regex : "^Can not get secret from Nextcloud instance$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Debug , path : "/cron.php" , line : 46usize , placeholders : & [] , regex : "^Update required, skipping cron$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Debug , path : "/cron.php" , line : 50usize , placeholders : & [] , regex : "^We are in maintenance mode, skipping cron$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/remote.php" , line : 75usize , placeholders : & ["$class" , "$msg"] , regex : "^(.*): (.*)$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/remote.php" , line : 129usize , placeholders : & [] , regex : "^Service unavailable$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/remote.php" , line : 135usize , placeholders : & [] , regex : "^Path not found$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/remote.php" , line : 145usize , placeholders : & [] , regex : "^Path not found$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/guzzlehttp/promises/src/FulfilledPromise.php" , line : 19usize , placeholders : & [] , regex : "^You cannot create a FulfilledPromise with a promise\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/guzzlehttp/promises/src/FulfilledPromise.php" , line : 71usize , placeholders : & [] , regex : "^Cannot resolve a fulfilled promise$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/guzzlehttp/promises/src/FulfilledPromise.php" , line : 77usize , placeholders : & [] , regex : "^Cannot reject a fulfilled promise$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/guzzlehttp/promises/src/RejectedPromise.php" , line : 19usize , placeholders : & [] , regex : "^You cannot create a RejectedPromise with a promise\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/guzzlehttp/promises/src/RejectedPromise.php" , line : 77usize , placeholders : & [] , regex : "^Cannot resolve a rejected promise$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/guzzlehttp/promises/src/RejectedPromise.php" , line : 83usize , placeholders : & [] , regex : "^Cannot reject a rejected promise$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/guzzlehttp/promises/src/Utils.php" , line : 222usize , placeholders : & [] , regex : "^Not enough promises to fulfill count$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/guzzlehttp/promises/src/Promise.php" , line : 131usize , placeholders : & [] , regex : "^Cannot fulfill or reject a promise with itself$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/guzzlehttp/guzzle/src/Handler/MockHandler.php" , line : 83usize , placeholders : & [] , regex : "^Mock queue is empty$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/guzzlehttp/guzzle/src/Handler/MockHandler.php" , line : 96usize , placeholders : & [] , regex : "^on_headers must be callable$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/guzzlehttp/guzzle/src/Handler/HeaderProcessor.php" , line : 24usize , placeholders : & [] , regex : "^Expected a non\\-empty array of header data$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/guzzlehttp/guzzle/src/Handler/HeaderProcessor.php" , line : 31usize , placeholders : & [] , regex : "^HTTP version missing from header data$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/guzzlehttp/guzzle/src/Handler/HeaderProcessor.php" , line : 37usize , placeholders : & [] , regex : "^HTTP status code missing from header data$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/guzzlehttp/guzzle/src/Handler/CurlMultiHandler.php" , line : 95usize , placeholders : & [] , regex : "^Can not get other property as '_mh'\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/guzzlehttp/guzzle/src/Handler/CurlMultiHandler.php" , line : 101usize , placeholders : & [] , regex : "^Can not initialize curl multi handle\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/guzzlehttp/guzzle/src/Handler/StreamHandler.php" , line : 290usize , placeholders : & [] , regex : "^on_headers must be callable$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/guzzlehttp/guzzle/src/Handler/StreamHandler.php" , line : 304usize , placeholders : & [] , regex : "^stream_context must be an array$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/guzzlehttp/guzzle/src/Handler/StreamHandler.php" , line : 311usize , placeholders : & [] , regex : "^Microsoft NTLM authentication only supported with curl handler$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/guzzlehttp/guzzle/src/Handler/StreamHandler.php" , line : 490usize , placeholders : & ["$value"] , regex : "^SSL CA bundle not found: (.*)$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/guzzlehttp/guzzle/src/Handler/StreamHandler.php" , line : 493usize , placeholders : & [] , regex : "^Invalid verify request option$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/guzzlehttp/guzzle/src/Handler/StreamHandler.php" , line : 512usize , placeholders : & ["$value"] , regex : "^SSL certificate not found: (.*)$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/guzzlehttp/guzzle/src/Handler/CurlFactory.php" , line : 363usize , placeholders : & ["$options['verify']"] , regex : "^SSL CA bundle not found: (.*)$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/guzzlehttp/guzzle/src/Handler/CurlFactory.php" , line : 460usize , placeholders : & ["$cert"] , regex : "^SSL certificate not found: (.*)$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/guzzlehttp/guzzle/src/Handler/CurlFactory.php" , line : 483usize , placeholders : & ["$sslKey"] , regex : "^SSL private key not found: (.*)$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/guzzlehttp/guzzle/src/Handler/CurlFactory.php" , line : 491usize , placeholders : & [] , regex : "^progress client option must be callable$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/guzzlehttp/guzzle/src/Handler/CurlFactory.php" , line : 556usize , placeholders : & [] , regex : "^on_headers must be callable$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/guzzlehttp/guzzle/src/Client.php" , line : 62usize , placeholders : & [] , regex : "^handler must be a callable$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/guzzlehttp/guzzle/src/Client.php" , line : 84usize , placeholders : & [] , regex : "^Magic request methods require a URI and optional options array$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/guzzlehttp/guzzle/src/Client.php" , line : 299usize , placeholders : & [] , regex : "^headers must be an array$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/guzzlehttp/guzzle/src/Client.php" , line : 348usize , placeholders : & [] , regex : "^The headers array must have header name as keys\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/guzzlehttp/guzzle/src/Client.php" , line : 426usize , placeholders : & [] , regex : "^query must be a string or array$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/guzzlehttp/guzzle/src/Client.php" , line : 436usize , placeholders : & [] , regex : "^sink must not be a boolean$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/guzzlehttp/guzzle/src/Cookie/SessionCookieJar.php" , line : 74usize , placeholders : & [] , regex : "^Invalid cookie data$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/guzzlehttp/guzzle/src/Cookie/SetCookie.php" , line : 80usize , placeholders : & [] , regex : "^Unable to replace the default values for the Cookie\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/guzzlehttp/guzzle/src/Cookie/FileCookieJar.php" , line : 69usize , placeholders : & ["$filename"] , regex : "^Unable to save file (.*)$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/guzzlehttp/guzzle/src/Cookie/FileCookieJar.php" , line : 86usize , placeholders : & ["$filename"] , regex : "^Unable to load file (.*)$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/guzzlehttp/guzzle/src/Cookie/FileCookieJar.php" , line : 98usize , placeholders : & ["$filename"] , regex : "^Invalid cookie file: (.*)$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/guzzlehttp/guzzle/src/HandlerStack.php" , line : 206usize , placeholders : & [] , regex : "^No handler has been specified$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/guzzlehttp/guzzle/src/HandlerStack.php" , line : 228usize , placeholders : & ["$name"] , regex : "^Middleware not found: (.*)$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/guzzlehttp/guzzle/src/Middleware.php" , line : 33usize , placeholders : & [] , regex : "^cookies must be an instance of GuzzleHttp\\\\Cookie\\\\CookieJarInterface$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/guzzlehttp/guzzle/src/Middleware.php" , line : 88usize , placeholders : & [] , regex : "^history container must be an array or object implementing ArrayAccess$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/guzzlehttp/guzzle/src/Utils.php" , line : 105usize , placeholders : & [] , regex : "^GuzzleHttp requires cURL, the allow_url_fopen ini setting, or a custom HTTP handler\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/guzzlehttp/guzzle/src/Utils.php" , line : 227usize , placeholders : & [] , regex : "^Empty host provided$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/guzzlehttp/guzzle/src/Utils.php" , line : 383usize , placeholders : & [] , regex : "^ext\\-idn or symfony/polyfill\\-intl\\-idn not loaded or too old$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/guzzlehttp/guzzle/src/Pool.php" , line : 62usize , placeholders : & [] , regex : "^Each value yielded by the iterator must be a Psr7\\\\Http\\\\Message\\\\RequestInterface or a callable that returns a promise that fulfills with a Psr7\\\\Message\\\\Http\\\\ResponseInterface object\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/guzzlehttp/guzzle/src/RedirectMiddleware.php" , line : 61usize , placeholders : & [] , regex : "^allow_redirects must be true, false, or array$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/guzzlehttp/guzzle/src/RedirectMiddleware.php" , line : 155usize , placeholders : & ["$max"] , regex : "^Will not follow more than (.*) redirects$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/guzzlehttp/psr7/src/ServerRequest.php" , line : 100usize , placeholders : & [] , regex : "^Invalid value in files specification$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/guzzlehttp/psr7/src/AppendStream.php" , line : 63usize , placeholders : & [] , regex : "^Each stream must be readable$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/guzzlehttp/psr7/src/AppendStream.php" , line : 157usize , placeholders : & [] , regex : "^This AppendStream is not seekable$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/guzzlehttp/psr7/src/AppendStream.php" , line : 159usize , placeholders : & [] , regex : "^The AppendStream can only seek with SEEK_SET$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/guzzlehttp/psr7/src/AppendStream.php" , line : 236usize , placeholders : & [] , regex : "^Cannot write to an AppendStream$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/guzzlehttp/psr7/src/Query.php" , line : 86usize , placeholders : & [] , regex : "^Invalid type$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/guzzlehttp/psr7/src/BufferStream.php" , line : 89usize , placeholders : & [] , regex : "^Cannot seek a BufferStream$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/guzzlehttp/psr7/src/BufferStream.php" , line : 99usize , placeholders : & [] , regex : "^Cannot determine the position of a BufferStream$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/guzzlehttp/psr7/src/NoSeekStream.php" , line : 21usize , placeholders : & [] , regex : "^Cannot seek a NoSeekStream$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/guzzlehttp/psr7/src/LimitStream.php" , line : 119usize , placeholders : & ["$offset"] , regex : "^Could not seek to stream offset (.*)$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/guzzlehttp/psr7/src/Header.php" , line : 86usize , placeholders : & [] , regex : "^\\$header must either be a string or an array containing strings\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/guzzlehttp/psr7/src/StreamDecoratorTrait.php" , line : 37usize , placeholders : & ["$name"] , regex : "^(.*) not found on class$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/guzzlehttp/psr7/src/StreamDecoratorTrait.php" , line : 153usize , placeholders : & [] , regex : "^Not implemented$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/guzzlehttp/psr7/src/Stream.php" , line : 53usize , placeholders : & [] , regex : "^Stream must be a resource$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/guzzlehttp/psr7/src/Stream.php" , line : 96usize , placeholders : & [] , regex : "^Stream is detached$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/guzzlehttp/psr7/src/Stream.php" , line : 100usize , placeholders : & [] , regex : "^Cannot read from non\\-readable stream$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/guzzlehttp/psr7/src/Stream.php" , line : 172usize , placeholders : & [] , regex : "^Stream is detached$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/guzzlehttp/psr7/src/Stream.php" , line : 181usize , placeholders : & [] , regex : "^Stream is detached$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/guzzlehttp/psr7/src/Stream.php" , line : 187usize , placeholders : & [] , regex : "^Unable to determine stream position$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/guzzlehttp/psr7/src/Stream.php" , line : 203usize , placeholders : & [] , regex : "^Stream is detached$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/guzzlehttp/psr7/src/Stream.php" , line : 206usize , placeholders : & [] , regex : "^Stream is not seekable$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/guzzlehttp/psr7/src/Stream.php" , line : 217usize , placeholders : & [] , regex : "^Stream is detached$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/guzzlehttp/psr7/src/Stream.php" , line : 220usize , placeholders : & [] , regex : "^Cannot read from non\\-readable stream$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/guzzlehttp/psr7/src/Stream.php" , line : 223usize , placeholders : & [] , regex : "^Length parameter cannot be negative$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/guzzlehttp/psr7/src/Stream.php" , line : 233usize , placeholders : & [] , regex : "^Unable to read from stream$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/guzzlehttp/psr7/src/Stream.php" , line : 237usize , placeholders : & [] , regex : "^Unable to read from stream$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/guzzlehttp/psr7/src/Stream.php" , line : 246usize , placeholders : & [] , regex : "^Stream is detached$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/guzzlehttp/psr7/src/Stream.php" , line : 249usize , placeholders : & [] , regex : "^Cannot write to a non\\-writable stream$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/guzzlehttp/psr7/src/Stream.php" , line : 257usize , placeholders : & [] , regex : "^Unable to write to stream$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/guzzlehttp/psr7/src/CachingStream.php" , line : 71usize , placeholders : & [] , regex : "^Invalid whence$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/guzzlehttp/psr7/src/Uri.php" , line : 85usize , placeholders : & ["$uri"] , regex : "^Unable to parse URI: (.*)$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/guzzlehttp/psr7/src/Uri.php" , line : 580usize , placeholders : & [] , regex : "^Scheme must be a string$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/guzzlehttp/psr7/src/Uri.php" , line : 594usize , placeholders : & [] , regex : "^User info must be a string$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/guzzlehttp/psr7/src/Uri.php" , line : 612usize , placeholders : & [] , regex : "^Host must be a string$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/guzzlehttp/psr7/src/Uri.php" , line : 690usize , placeholders : & [] , regex : "^Path must be a string$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/guzzlehttp/psr7/src/Uri.php" , line : 710usize , placeholders : & [] , regex : "^Query and fragment must be a string$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/guzzlehttp/psr7/src/Uri.php" , line : 733usize , placeholders : & [] , regex : "^The path of a URI without an authority must not start with two slashes \"//\"$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/guzzlehttp/psr7/src/Uri.php" , line : 736usize , placeholders : & [] , regex : "^A relative URI must not have a path beginning with a segment containing a colon$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/guzzlehttp/psr7/src/MessageTrait.php" , line : 176usize , placeholders : & [] , regex : "^Header value can not be an empty array\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/guzzlehttp/psr7/src/HttpFactory.php" , line : 79usize , placeholders : & [] , regex : "^Cannot determine HTTP method$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/guzzlehttp/psr7/src/Utils.php" , line : 457usize , placeholders : & [] , regex : "^URI must be a string or UriInterface$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/guzzlehttp/psr7/src/UploadedFile.php" , line : 97usize , placeholders : & [] , regex : "^Invalid stream or file provided for UploadedFile$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/guzzlehttp/psr7/src/UploadedFile.php" , line : 109usize , placeholders : & [] , regex : "^Invalid error status for UploadedFile$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/guzzlehttp/psr7/src/UploadedFile.php" , line : 140usize , placeholders : & [] , regex : "^Cannot retrieve stream due to upload error$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/guzzlehttp/psr7/src/UploadedFile.php" , line : 144usize , placeholders : & [] , regex : "^Cannot retrieve stream after it has already been moved$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/guzzlehttp/psr7/src/UploadedFile.php" , line : 168usize , placeholders : & [] , regex : "^Invalid path provided for move operation; must be a non\\-empty string$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/guzzlehttp/psr7/src/Request.php" , line : 82usize , placeholders : & [] , regex : "^Invalid request target provided; cannot contain whitespace$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/guzzlehttp/psr7/src/Request.php" , line : 154usize , placeholders : & [] , regex : "^Method must be a non\\-empty string\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/guzzlehttp/psr7/src/Message.php" , line : 32usize , placeholders : & [] , regex : "^Unknown message type$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/guzzlehttp/psr7/src/Message.php" , line : 118usize , placeholders : & [] , regex : "^Invalid message$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/guzzlehttp/psr7/src/Message.php" , line : 126usize , placeholders : & [] , regex : "^Invalid message: Missing header delimiter$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/guzzlehttp/psr7/src/Message.php" , line : 134usize , placeholders : & [] , regex : "^Invalid message: Missing status line$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/guzzlehttp/psr7/src/Message.php" , line : 151usize , placeholders : & [] , regex : "^Invalid header syntax: Obsolete line folding$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/guzzlehttp/psr7/src/Message.php" , line : 154usize , placeholders : & [] , regex : "^Invalid header syntax$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/guzzlehttp/psr7/src/Message.php" , line : 206usize , placeholders : & [] , regex : "^Invalid request string$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/guzzlehttp/psr7/src/FnStream.php" , line : 68usize , placeholders : & [] , regex : "^FnStream should never be unserialized$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/guzzlehttp/psr7/src/MultipartStream.php" , line : 75usize , placeholders : & [] , regex : "^An array is expected$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/guzzlehttp/psr7/src/MultipartStream.php" , line : 90usize , placeholders : & ["$key"] , regex : "^A '(.*)' key is required$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/guzzlehttp/psr7/src/Response.php" , line : 150usize , placeholders : & [] , regex : "^Status code must be an integer value\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/guzzlehttp/psr7/src/Response.php" , line : 157usize , placeholders : & [] , regex : "^Status code must be an integer value between 1xx and 5xx\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/guzzlehttp/psr7/src/PumpStream.php" , line : 107usize , placeholders : & [] , regex : "^Cannot seek a PumpStream$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/guzzlehttp/psr7/src/PumpStream.php" , line : 117usize , placeholders : & [] , regex : "^Cannot write to a PumpStream$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/spomky-labs/base64url/src/Base64Url.php" , line : 51usize , placeholders : & [] , regex : "^Invalid data provided$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/spomky-labs/cbor-php/src/UnsignedIntegerObject.php" , line : 97usize , placeholders : & [] , regex : "^The value must be a positive integer\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/spomky-labs/cbor-php/src/UnsignedIntegerObject.php" , line : 118usize , placeholders : & [] , regex : "^Out of range\\. Please use PositiveBigIntegerTag tag with ByteStringObject object instead\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/spomky-labs/cbor-php/src/UnsignedIntegerObject.php" , line : 128usize , placeholders : & [] , regex : "^Unable to convert the data$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/spomky-labs/cbor-php/src/OtherObject/OtherObjectManager.php" , line : 31usize , placeholders : & [] , regex : "^Invalid additional information\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/spomky-labs/cbor-php/src/OtherObject/SimpleObject.php" , line : 53usize , placeholders : & [] , regex : "^The value is not a valid simple value$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/spomky-labs/cbor-php/src/OtherObject/SinglePrecisionFloatObject.php" , line : 39usize , placeholders : & [] , regex : "^The value is not a valid single precision floating point$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/spomky-labs/cbor-php/src/OtherObject/DoublePrecisionFloatObject.php" , line : 39usize , placeholders : & [] , regex : "^The value is not a valid double precision floating point$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/spomky-labs/cbor-php/src/OtherObject/HalfPrecisionFloatObject.php" , line : 39usize , placeholders : & [] , regex : "^The value is not a valid half precision floating point$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/spomky-labs/cbor-php/src/Utils.php" , line : 59usize , placeholders : & [] , regex : "^Invalid data provided$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/spomky-labs/cbor-php/src/StringStream.php" , line : 30usize , placeholders : & [] , regex : "^Unable to open the memory$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/spomky-labs/cbor-php/src/StringStream.php" , line : 34usize , placeholders : & [] , regex : "^Unable to write the memory$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/spomky-labs/cbor-php/src/StringStream.php" , line : 38usize , placeholders : & [] , regex : "^Unable to rewind the memory$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/spomky-labs/cbor-php/src/StringStream.php" , line : 50usize , placeholders : & [] , regex : "^Unable to read the memory$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/spomky-labs/cbor-php/src/InfiniteMapObject.php" , line : 47usize , placeholders : & [] , regex : "^Unable to convert the data$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/spomky-labs/cbor-php/src/MapObject.php" , line : 45usize , placeholders : & [] , regex : "^The list must contain only MapItem objects\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/spomky-labs/cbor-php/src/LengthCalculator.php" , line : 59usize , placeholders : & [] , regex : "^Unable to convert the data$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/spomky-labs/cbor-php/src/InfiniteListObject.php" , line : 46usize , placeholders : & [] , regex : "^Unable to convert the data$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/spomky-labs/cbor-php/src/Decoder.php" , line : 117usize , placeholders : & [] , regex : "^Unable to parse the data\\. Infinite Byte String object can only get Byte String objects\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/spomky-labs/cbor-php/src/Decoder.php" , line : 127usize , placeholders : & [] , regex : "^Unable to parse the data\\. Infinite Text String object can only get Text String objects\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/spomky-labs/cbor-php/src/Decoder.php" , line : 149usize , placeholders : & [] , regex : "^Cannot parse the data\\. No enclosing indefinite\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/spomky-labs/cbor-php/src/ByteStringWithChunkObject.php" , line : 41usize , placeholders : & [] , regex : "^Unable to convert the data$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/spomky-labs/cbor-php/src/Tag/TagObjectManager.php" , line : 32usize , placeholders : & [] , regex : "^Invalid tag ID\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/spomky-labs/cbor-php/src/Tag/TimestampTag.php" , line : 41usize , placeholders : & [] , regex : "^This tag only accepts a Byte String object\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/spomky-labs/cbor-php/src/Tag/Base16EncodingTag.php" , line : 39usize , placeholders : & [] , regex : "^This tag only accepts Byte String, Infinite Byte String, Text String or Infinite Text String objects\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/spomky-labs/cbor-php/src/Tag/DecimalFractionTag.php" , line : 31usize , placeholders : & [] , regex : "^The extension \"bcmath\" is required to use this tag$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/spomky-labs/cbor-php/src/Tag/DecimalFractionTag.php" , line : 34usize , placeholders : & [] , regex : "^This tag only accepts a ListObject object that contains an exponent and a mantissa\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/spomky-labs/cbor-php/src/Tag/DecimalFractionTag.php" , line : 38usize , placeholders : & [] , regex : "^The exponent must be a Signed Integer or an Unsigned Integer object\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/spomky-labs/cbor-php/src/Tag/DecimalFractionTag.php" , line : 42usize , placeholders : & [] , regex : "^The mantissa must be a Positive or Negative Signed Integer or an Unsigned Integer object\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/spomky-labs/cbor-php/src/Tag/Base64UrlEncodingTag.php" , line : 40usize , placeholders : & [] , regex : "^This tag only accepts Byte String, Infinite Byte String, Text String or Infinite Text String objects\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/spomky-labs/cbor-php/src/Tag/PositiveBigIntegerTag.php" , line : 37usize , placeholders : & [] , regex : "^This tag only accepts a Byte String object\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/spomky-labs/cbor-php/src/Tag/Base64EncodingTag.php" , line : 39usize , placeholders : & [] , regex : "^This tag only accepts Byte String, Infinite Byte String, Text String or Infinite Text String objects\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/spomky-labs/cbor-php/src/Tag/Base64EncodingTag.php" , line : 57usize , placeholders : & [] , regex : "^Unable to decode the data$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/spomky-labs/cbor-php/src/Tag/BigFloatTag.php" , line : 31usize , placeholders : & [] , regex : "^The extension \"bcmath\" is required to use this tag$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/spomky-labs/cbor-php/src/Tag/BigFloatTag.php" , line : 49usize , placeholders : & [] , regex : "^This tag only accepts a ListObject object that contains an exponent and a mantissa\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/spomky-labs/cbor-php/src/Tag/BigFloatTag.php" , line : 53usize , placeholders : & [] , regex : "^The exponent must be a Signed Integer or an Unsigned Integer object\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/spomky-labs/cbor-php/src/Tag/BigFloatTag.php" , line : 57usize , placeholders : & [] , regex : "^The mantissa must be a Positive or Negative Signed Integer or an Unsigned Integer object\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/spomky-labs/cbor-php/src/Tag/NegativeBigIntegerTag.php" , line : 37usize , placeholders : & [] , regex : "^This tag only accepts a Byte String object\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/spomky-labs/cbor-php/src/SignedIntegerObject.php" , line : 81usize , placeholders : & [] , regex : "^The value must be a negative integer\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/spomky-labs/cbor-php/src/SignedIntegerObject.php" , line : 105usize , placeholders : & [] , regex : "^Out of range\\. Please use NegativeBigIntegerTag tag with ByteStringObject object instead\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/spomky-labs/cbor-php/src/SignedIntegerObject.php" , line : 115usize , placeholders : & [] , regex : "^Unable to convert the data$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/spomky-labs/cbor-php/src/ListObject.php" , line : 46usize , placeholders : & [] , regex : "^The list must contain only CBORObject objects\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/spomky-labs/cbor-php/src/ListObject.php" , line : 77usize , placeholders : & [] , regex : "^Index not found\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/spomky-labs/cbor-php/src/TextStringWithChunkObject.php" , line : 41usize , placeholders : & [] , regex : "^Unable to convert the data$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/microsoft/azure-storage-common/src/Common/Internal/Validate.php" , line : 124usize , placeholders : & [] , regex : "^double$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/microsoft/azure-storage-common/src/Common/Internal/Validate.php" , line : 194usize , placeholders : & [] , regex : "^DateTimeInterface$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/microsoft/azure-storage-common/src/Common/Internal/Validate.php" , line : 337usize , placeholders : & [] , regex : "^object$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/fgrosse/phpasn1/lib/X509/SAN/SubjectAlternativeNames.php" , line : 76usize , placeholders : & [] , regex : "^Can not parse Subject Alternative Names: The Sequence length does not match the length of the surrounding octet string$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/fgrosse/phpasn1/lib/X509/SAN/SubjectAlternativeNames.php" , line : 89usize , placeholders : & [] , regex : "^Could not parse Subject Alternative Name: Only DNSName and IP SANs are currently supported$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/fgrosse/phpasn1/lib/X509/SAN/IPAddress.php" , line : 60usize , placeholders : & ["$contentLength"] , regex : "^A FG\\\\X509\\\\SAN\\\\IPAddress should have a content length of 4\\. Extracted length was (.*)$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/fgrosse/phpasn1/lib/X509/CertificateExtensions.php" , line : 74usize , placeholders : & [] , regex : "^Could not parse Certificate Extensions: Needs at least two child elements per extension sequence \\(object identifier and octet string\\)$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/fgrosse/phpasn1/lib/ASN1/Identifier.php" , line : 89usize , placeholders : & ["$isConstructed"] , regex : "^\\$isConstructed must be a boolean value \\((.*) given\\)$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/fgrosse/phpasn1/lib/ASN1/AbstractTime.php" , line : 35usize , placeholders : & [] , regex : "^Invalid first argument for some instance of AbstractTime constructor$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/fgrosse/phpasn1/lib/ASN1/ASNObject.php" , line : 207usize , placeholders : & [] , regex : "^Can not parse binary from data: Offset index larger than input size$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/fgrosse/phpasn1/lib/ASN1/ASNObject.php" , line : 293usize , placeholders : & [] , regex : "^Can not parse identifier from data: Offset index larger than input size$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/fgrosse/phpasn1/lib/ASN1/ASNObject.php" , line : 304usize , placeholders : & [] , regex : "^Can not parse identifier \\(long form\\) from data: Offset index larger than input size$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/fgrosse/phpasn1/lib/ASN1/ASNObject.php" , line : 321usize , placeholders : & [] , regex : "^Can not parse content length from data: Offset index larger than input size$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/fgrosse/phpasn1/lib/ASN1/ASNObject.php" , line : 331usize , placeholders : & [] , regex : "^Can not parse content length \\(long form\\) from data: Offset index larger than input size$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/fgrosse/phpasn1/lib/ASN1/ASNObject.php" , line : 337usize , placeholders : & [] , regex : "^Can not parse content length from data: length > maximum integer$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/fgrosse/phpasn1/lib/ASN1/ASNObject.php" , line : 350usize , placeholders : & ["$contentLength" , "$lenDataRemaining"] , regex : "^Content length (.*) exceeds remaining data length (.*)$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/fgrosse/phpasn1/lib/ASN1/Construct.php" , line : 173usize , placeholders : & [] , regex : "^Sequence length incorrect$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/fgrosse/phpasn1/lib/ASN1/ExplicitlyTaggedObject.php" , line : 124usize , placeholders : & ["$tag" , "$offsetIndexOfDecoratedObject" , "$totalContentLength" , "$remainingContentLength"] , regex : "^Context\\-Specific explicitly tagged object \\[(.*)\\] starting at offset (.*) specifies a length of (.*) octets but (.*) remain after parsing the content$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/fgrosse/phpasn1/lib/ASN1/AbstractString.php" , line : 106usize , placeholders : & ["$typeName" , "$this->value"] , regex : "^Could not create a (.*) from the character sequence '(.*)'\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/fgrosse/phpasn1/lib/ASN1/TemplateParser.php" , line : 67usize , placeholders : & ["$expectedTypeId" , "$actualType"] , regex : "^Expected type \\((.*)\\) does not match actual type \\((.*)$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/fgrosse/phpasn1/lib/ASN1/Universal/Boolean.php" , line : 64usize , placeholders : & ["$contentLength"] , regex : "^An ASN\\.1 Boolean should not have a length other than one\\. Extracted length was (.*)$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/fgrosse/phpasn1/lib/ASN1/Universal/RelativeObjectIdentifier.php" , line : 49usize , placeholders : & [] , regex : "^Malformed ASN\\.1 Relative Object Identifier$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/fgrosse/phpasn1/lib/ASN1/Universal/OctetString.php" , line : 32usize , placeholders : & [] , regex : "^OctetString: unrecognized input type!$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/fgrosse/phpasn1/lib/ASN1/Universal/Integer.php" , line : 33usize , placeholders : & ["$value"] , regex : "^Invalid VALUE \\[(.*)\\] for ASN1_INTEGER$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/fgrosse/phpasn1/lib/ASN1/Universal/Integer.php" , line : 101usize , placeholders : & [] , regex : "^Integer not minimally encoded$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/fgrosse/phpasn1/lib/ASN1/Universal/NullObject.php" , line : 46usize , placeholders : & ["$contentLength"] , regex : "^An ASN\\.1 Null should not have a length other than zero\\. Extracted length was (.*)$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/fgrosse/phpasn1/lib/ASN1/Universal/BitString.php" , line : 35usize , placeholders : & [] , regex : "^BitString: second parameter needs to be a positive number \\(or zero\\)!$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/fgrosse/phpasn1/lib/ASN1/Universal/BitString.php" , line : 78usize , placeholders : & [] , regex : "^Can not parse bit string with invalid padding$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/fgrosse/phpasn1/lib/ASN1/Universal/GeneralizedTime.php" , line : 124usize , placeholders : & [] , regex : "^Invalid ISO 8601 Time String$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/fgrosse/phpasn1/lib/ASN1/Universal/ObjectIdentifier.php" , line : 124usize , placeholders : & [] , regex : "^Malformed ASN\\.1 Object Identifier$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/fgrosse/phpasn1/lib/ASN1/Universal/UTCTime.php" , line : 69usize , placeholders : & [] , regex : "^Invalid UTC String$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/fgrosse/phpasn1/lib/Utility/BigIntegerGmp.php" , line : 104usize , placeholders : & [] , regex : "^Unable to raise to power greater than PHP_INT_MAX\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/fgrosse/phpasn1/lib/Utility/BigInteger.php" , line : 62usize , placeholders : & [] , regex : "^Requires GMP or bcmath extension\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/fgrosse/phpasn1/lib/Utility/BigInteger.php" , line : 75usize , placeholders : & [] , regex : "^Expects a string representation of an integer\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/beberlei/assert/lib/Assert/Assertion.php" , line : 2110usize , placeholders : & [] , regex : "^Missing the first argument\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/beberlei/assert/lib/Assert/Assertion.php" , line : 2124usize , placeholders : & [] , regex : "^Missing the first argument\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/beberlei/assert/lib/Assert/AssertionChain.php" , line : 237usize , placeholders : & [] , regex : "^Exception class name must be passed as a string$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/mexitek/phpcolors/src/Mexitek/PHPColors/Color.php" , line : 128usize , placeholders : & [] , regex : "^Param was not an HSL array$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/mexitek/phpcolors/src/Mexitek/PHPColors/Color.php" , line : 199usize , placeholders : & [] , regex : "^Param was not an RGB array$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/mexitek/phpcolors/src/Mexitek/PHPColors/Color.php" , line : 226usize , placeholders : & [] , regex : "^Param was not an RGB array$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/mexitek/phpcolors/src/Mexitek/PHPColors/Color.php" , line : 697usize , placeholders : & [] , regex : "^HEX color does not match format$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/mexitek/phpcolors/src/Mexitek/PHPColors/Color.php" , line : 704usize , placeholders : & [] , regex : "^HEX color needs to be 6 or 3 digits long$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/deepdiver/zipstreamer/src/ZipStreamer.php" , line : 310usize , placeholders : & [] , regex : "^unable to use compression method DEFLATE with level other than NONE \\(requires pecl_http >= 0\\.10\\)$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/php-opencloud/openstack/src/Common/Service/Builder.php" , line : 164usize , placeholders : & [] , regex : "^\"authUrl\" is a required option$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/php-opencloud/openstack/src/Compute/v2/Models/Server.php" , line : 137usize , placeholders : & [] , regex : "^imageId or blockDeviceMapping\\.uuid must be set\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/php-opencloud/openstack/src/Compute/v2/Models/Server.php" , line : 203usize , placeholders : & [] , regex : "^Reboot type must either be SOFT or HARD$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/php-opencloud/openstack/src/Compute/v2/Models/Server.php" , line : 426usize , placeholders : & [] , regex : "^networkId or portId must be set\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/php-opencloud/openstack/src/Identity/v3/Models/Token.php" , line : 114usize , placeholders : & [] , regex : "^Either a user or token must be provided\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/php-opencloud/openstack/src/Identity/v3/Models/Catalog.php" , line : 50usize , placeholders : & [] , regex : "^No services are defined$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/php-opencloud/openstack/src/OpenStack.php" , line : 50usize , placeholders : & [] , regex : "^'authUrl' is a required option$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/aws/aws-sdk-php/src/Glacier/GlacierClient.php" , line : 142usize , placeholders : & [] , regex : "^sha256$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/aws/aws-sdk-php/src/Sdk.php" , line : 678usize , placeholders : & ["$name"] , regex : "^Unknown method: (.*)\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/aws/aws-sdk-php/src/Waiter.php" , line : 78usize , placeholders : & [] , regex : "^The provided waiter configuration was incomplete\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/aws/aws-sdk-php/src/Waiter.php" , line : 84usize , placeholders : & [] , regex : "^The provided \"before\" callback is not callable\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/aws/aws-sdk-php/src/HandlerList.php" , line : 294usize , placeholders : & [] , regex : "^No handler has been specified$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/aws/aws-sdk-php/src/HandlerList.php" , line : 332usize , placeholders : & ["$findName"] , regex : "^(.*) not found$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/aws/aws-sdk-php/src/EndpointDiscovery/Configuration.php" , line : 14usize , placeholders : & [] , regex : "^'cache_limit' value must be a positive integer\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/aws/aws-sdk-php/src/CloudFront/UrlSigner.php" , line : 54usize , placeholders : & ["$url"] , regex : "^Invalid URL: (.*)$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/aws/aws-sdk-php/src/CloudFront/Signer.php" , line : 36usize , placeholders : & ["$privateKey"] , regex : "^PK file not found: (.*)$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/aws/aws-sdk-php/src/CloudFront/CloudFrontClient.php" , line : 238usize , placeholders : & ["$required"] , regex : "^(.*) is required$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/aws/aws-sdk-php/src/CloudFront/CloudFrontClient.php" , line : 281usize , placeholders : & ["$required"] , regex : "^(.*) is required$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/aws/aws-sdk-php/src/CloudFront/CookieSigner.php" , line : 62usize , placeholders : & [] , regex : "^Invalid or missing URI scheme$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/aws/aws-sdk-php/src/AwsClient.php" , line : 261usize , placeholders : & ["$name"] , regex : "^Operation not found: (.*)$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/aws/aws-sdk-php/src/Credentials/AssumeRoleWithWebIdentityCredentialProvider.php" , line : 63usize , placeholders : & [] , regex : "^'WebIdentityTokenFile' must be an absolute path\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/aws/aws-sdk-php/src/Credentials/AssumeRoleWithWebIdentityCredentialProvider.php" , line : 108usize , placeholders : & ["$this->tokenFile"] , regex : "^Unreadable tokenfile at location (.*)$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/aws/aws-sdk-php/src/Credentials/AssumeRoleWithWebIdentityCredentialProvider.php" , line : 120usize , placeholders : & ["$this->tokenFile"] , regex : "^InvalidIdentityToken from file: (.*)$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/aws/aws-sdk-php/src/Credentials/AssumeRoleWithWebIdentityCredentialProvider.php" , line : 144usize , placeholders : & [] , regex : "^InvalidIdentityToken, retries exhausted$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/aws/aws-sdk-php/src/Credentials/AssumeRoleWithWebIdentityCredentialProvider.php" , line : 149usize , placeholders : & [] , regex : "^Error assuming role from web identity credentials$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/aws/aws-sdk-php/src/Credentials/EcsCredentialProvider.php" , line : 80usize , placeholders : & ["$msg"] , regex : "^Error retrieving credential from ECS \\((.*)\\)$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/aws/aws-sdk-php/src/Credentials/EcsCredentialProvider.php" , line : 130usize , placeholders : & [] , regex : "^Unexpected ECS credential value$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/aws/aws-sdk-php/src/Credentials/AssumeRoleCredentialProvider.php" , line : 58usize , placeholders : & [] , regex : "^Error in retrieving assume role credentials\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/aws/aws-sdk-php/src/Credentials/CredentialProvider.php" , line : 169usize , placeholders : & [] , regex : "^No providers in chain$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/aws/aws-sdk-php/src/Script/Composer/Composer.php" , line : 34usize , placeholders : & [] , regex : "^There are no services listed\\. Did you intend to use this script\\?$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/aws/aws-sdk-php/src/Script/Composer/Composer.php" , line : 56usize , placeholders : & ["$serviceToKeep"] , regex : "^'(.*)' is not a valid AWS service namespace\\. Please check spelling and casing\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/aws/aws-sdk-php/src/EndpointParameterMiddleware.php" , line : 63usize , placeholders : & ["$parameter"] , regex : "^The parameter '(.*)' must be set and not empty\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/aws/aws-sdk-php/src/EndpointParameterMiddleware.php" , line : 81usize , placeholders : & ["$host"] , regex : "^The supplied parameters result in an invalid hostname: '(.*)'\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/aws/aws-sdk-php/src/WrappedHttpHandler.php" , line : 165usize , placeholders : & [] , regex : "^The HTTP handler was rejected without an \"exception\" key value pair\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/aws/aws-sdk-php/src/Signature/SignatureV4.php" , line : 247usize , placeholders : & [] , regex : "^sha256$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/aws/aws-sdk-php/src/Signature/SignatureV4.php" , line : 253usize , placeholders : & [] , regex : "^sha256$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/aws/aws-sdk-php/src/DynamoDb/WriteRequestBatch.php" , line : 70usize , placeholders : & [] , regex : "^\"batch_size\" must be between 2 and 25\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/aws/aws-sdk-php/src/DynamoDb/WriteRequestBatch.php" , line : 75usize , placeholders : & [] , regex : "^\"before\" must be callable\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/aws/aws-sdk-php/src/DynamoDb/WriteRequestBatch.php" , line : 78usize , placeholders : & [] , regex : "^\"error\" must be callable\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/aws/aws-sdk-php/src/DynamoDb/WriteRequestBatch.php" , line : 261usize , placeholders : & [] , regex : "^There was no table specified\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/aws/aws-sdk-php/src/DynamoDb/Marshaler.php" , line : 100usize , placeholders : & [] , regex : "^The JSON document must be valid and be an object at its root\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/aws/aws-sdk-php/src/DynamoDb/Marshaler.php" , line : 298usize , placeholders : & ["$type"] , regex : "^Unexpected type: (.*)\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/aws/aws-sdk-php/src/DynamoDb/Marshaler.php" , line : 318usize , placeholders : & ["$message"] , regex : "^Marshaling error: (.*)\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/aws/aws-sdk-php/src/History.php" , line : 48usize , placeholders : & [] , regex : "^No commands received$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/aws/aws-sdk-php/src/History.php" , line : 63usize , placeholders : & [] , regex : "^No requests received$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/aws/aws-sdk-php/src/History.php" , line : 78usize , placeholders : & [] , regex : "^No entries$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/aws/aws-sdk-php/src/History.php" , line : 91usize , placeholders : & [] , regex : "^No return value for last entry\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/aws/aws-sdk-php/src/History.php" , line : 124usize , placeholders : & [] , regex : "^Invalid history ticket$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/aws/aws-sdk-php/src/History.php" , line : 130usize , placeholders : & [] , regex : "^History entry is already finished$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/aws/aws-sdk-php/src/Endpoint/UseDualstackEndpoint/Configuration.php" , line : 21usize , placeholders : & [] , regex : "^Dual\\-stack is not supported in ISO regions$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/aws/aws-sdk-php/src/Endpoint/Partition.php" , line : 64usize , placeholders : & ["$key"] , regex : "^Partition missing required (.*) field$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/aws/aws-sdk-php/src/functions.php" , line : 337usize , placeholders : & [] , regex : "^Unable to determine what Guzzle version is installed\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/aws/aws-sdk-php/src/functions.php" , line : 366usize , placeholders : & [] , regex : "^Calling handler did not serialize request$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/aws/aws-sdk-php/src/functions.php" , line : 416usize , placeholders : & ["$service"] , regex : "^The service \"(.*)\" is not provided by the AWS SDK for PHP\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/aws/aws-sdk-php/src/CommandPool.php" , line : 125usize , placeholders : & [] , regex : "^before must be callable$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/aws/aws-sdk-php/src/Multipart/AbstractUploader.php" , line : 137usize , placeholders : & [] , regex : "^Source stream must be readable\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/aws/aws-sdk-php/src/MockHandler.php" , line : 57usize , placeholders : & [] , regex : "^Expected an Aws\\\\ResultInterface or Exception\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/aws/aws-sdk-php/src/MockHandler.php" , line : 71usize , placeholders : & [] , regex : "^Expected an \\\\Exception or \\\\Throwable\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/aws/aws-sdk-php/src/Api/Service.php" , line : 126usize , placeholders : & ["$protocol"] , regex : "^Unknown protocol: (.*)$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/aws/aws-sdk-php/src/Api/Service.php" , line : 276usize , placeholders : & ["$name"] , regex : "^Unknown operation: (.*)$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/aws/aws-sdk-php/src/Api/MapShape.php" , line : 29usize , placeholders : & [] , regex : "^No value specified$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/aws/aws-sdk-php/src/Api/ListShape.php" , line : 25usize , placeholders : & [] , regex : "^No member attribute specified$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/aws/aws-sdk-php/src/Api/DateTimeResult.php" , line : 28usize , placeholders : & [] , regex : "^Invalid timestamp value passed to DateTimeResult::fromEpoch$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/aws/aws-sdk-php/src/Api/DateTimeResult.php" , line : 45usize , placeholders : & [] , regex : "^Invalid timestamp value passed to DateTimeResult::fromEpoch$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/aws/aws-sdk-php/src/Api/DateTimeResult.php" , line : 60usize , placeholders : & [] , regex : "^Invalid timestamp value passed to DateTimeResult::fromISO8601$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/aws/aws-sdk-php/src/Api/DateTimeResult.php" , line : 79usize , placeholders : & [] , regex : "^Invalid timestamp value passed to DateTimeResult::fromTimestamp$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/aws/aws-sdk-php/src/Api/DateTimeResult.php" , line : 100usize , placeholders : & [] , regex : "^Invalid timestamp value passed to DateTimeResult::fromTimestamp$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/aws/aws-sdk-php/src/Api/DocModel.php" , line : 22usize , placeholders : & [] , regex : "^The \"tidy\" PHP extension is required\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/aws/aws-sdk-php/src/Api/Parser/PayloadParserTrait.php" , line : 50usize , placeholders : & ["$e->getMessage()"] , regex : "^Error parsing XML: (.*)$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/aws/aws-sdk-php/src/Api/Parser/DecodingEventStreamIterator.php" , line : 97usize , placeholders : & [] , regex : "^Duplicate key in event headers\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/aws/aws-sdk-php/src/Api/Parser/DecodingEventStreamIterator.php" , line : 124usize , placeholders : & [] , regex : "^Prelude checksum mismatch\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/aws/aws-sdk-php/src/Api/Parser/DecodingEventStreamIterator.php" , line : 140usize , placeholders : & [] , regex : "^Message length too long\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/aws/aws-sdk-php/src/Api/Parser/DecodingEventStreamIterator.php" , line : 145usize , placeholders : & [] , regex : "^Headers length too long\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/aws/aws-sdk-php/src/Api/Parser/DecodingEventStreamIterator.php" , line : 163usize , placeholders : & [] , regex : "^Message checksum mismatch\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/aws/aws-sdk-php/src/Api/Parser/DecodingEventStreamIterator.php" , line : 302usize , placeholders : & [] , regex : "^Undefined variable length format\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/aws/aws-sdk-php/src/Api/Parser/DecodingEventStreamIterator.php" , line : 312usize , placeholders : & [] , regex : "^Undefined variable length format\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/aws/aws-sdk-php/src/Api/Parser/Crc32ValidatingParser.php" , line : 32usize , placeholders : & ["$expected" , "$hash"] , regex : "^crc32 mismatch\\. Expected (.*), found (.*)\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/aws/aws-sdk-php/src/Api/Parser/XmlParser.php" , line : 160usize , placeholders : & [] , regex : "^Invalid timestamp value passed to XmlParser::parse_timestamp$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/aws/aws-sdk-php/src/Api/Parser/EventParsingIterator.php" , line : 73usize , placeholders : & [] , regex : "^Failed to parse unknown message type\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/aws/aws-sdk-php/src/Api/Parser/EventParsingIterator.php" , line : 78usize , placeholders : & [] , regex : "^Failed to parse without event type\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/aws/aws-sdk-php/src/Api/Serializer/JsonBody.php" , line : 37usize , placeholders : & [] , regex : "^invalid json$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/aws/aws-sdk-php/src/Api/ApiProvider.php" , line : 214usize , placeholders : & ["$modelsDir"] , regex : "^The specified models directory, (.*), was not found\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/aws/aws-sdk-php/src/Rds/AuthTokenGenerator.php" , line : 50usize , placeholders : & ["$lifetime"] , regex : "^Lifetime must be a positive number less than or equal to 15, was (.*)$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/aws/aws-sdk-php/src/Sts/RegionalEndpoints/Configuration.php" , line : 13usize , placeholders : & [] , regex : "^Configuration parameter must either be 'legacy' or 'regional'\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/aws/aws-sdk-php/src/Sts/StsClient.php" , line : 74usize , placeholders : & [] , regex : "^Result contains no credentials$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/aws/aws-sdk-php/src/InputValidationMiddleware.php" , line : 34usize , placeholders : & [] , regex : "^The mandatory attribute list must be an array of strings$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/aws/aws-sdk-php/src/InputValidationMiddleware.php" , line : 65usize , placeholders : & ["$commandName" , "$member"] , regex : "^The (.*) operation requires non\\-empty parameter: (.*)$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/aws/aws-sdk-php/src/EventBridge/EventBridgeEndpointMiddleware.php" , line : 100usize , placeholders : & [] , regex : "^EventId must be a non\\-empty string$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/aws/aws-sdk-php/src/EventBridge/EventBridgeEndpointMiddleware.php" , line : 103usize , placeholders : & [] , regex : "^EventId must be a valid host$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/aws/aws-sdk-php/src/EventBridge/EventBridgeEndpointMiddleware.php" , line : 107usize , placeholders : & [] , regex : "^EventId is currently not compatible with FIPS pseudo regions$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/aws/aws-sdk-php/src/EventBridge/EventBridgeEndpointMiddleware.php" , line : 112usize , placeholders : & [] , regex : "^EventId is currently not compatible with dualstack$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/aws/aws-sdk-php/src/AbstractConfigurationProvider.php" , line : 67usize , placeholders : & [] , regex : "^No providers in chain$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/aws/aws-sdk-php/src/Crypto/Cipher/Cbc.php" , line : 50usize , placeholders : & [] , regex : "^Invalid initialization vector$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/aws/aws-sdk-php/src/Crypto/KmsMaterialsProvider.php" , line : 40usize , placeholders : & [] , regex : "^Not able to detect the materials description\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/aws/aws-sdk-php/src/Crypto/Polyfill/ByteArray.php" , line : 52usize , placeholders : & [] , regex : "^Argument must be an integer, string, or array of integers\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/aws/aws-sdk-php/src/Crypto/AesEncryptingStream.php" , line : 117usize , placeholders : & [] , regex : "^Unrecognized whence\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/aws/aws-sdk-php/src/ClientSideMonitoring/Configuration.php" , line : 25usize , placeholders : & [] , regex : "^CSM 'port' value must be an integer!$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/aws/aws-sdk-php/src/ClientSideMonitoring/ApiCallAttemptMonitoringMiddleware.php" , line : 122usize , placeholders : & [] , regex : "^Parameter must be an instance of ResultInterface, AwsException or Exception\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/aws/aws-sdk-php/src/ClientSideMonitoring/ApiCallMonitoringMiddleware.php" , line : 85usize , placeholders : & [] , regex : "^Parameter must be an instance of ResultInterface or Exception\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/aws/aws-sdk-php/src/S3/Transfer.php" , line : 117usize , placeholders : & [] , regex : "^mup_threshold must be >= 5MB$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/aws/aws-sdk-php/src/S3/Transfer.php" , line : 124usize , placeholders : & [] , regex : "^before must be a callable\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/aws/aws-sdk-php/src/S3/Transfer.php" , line : 173usize , placeholders : & [] , regex : "^Scheme must be \"s3\" or \"file\"\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/aws/aws-sdk-php/src/S3/Transfer.php" , line : 272usize , placeholders : & ["$dir"] , regex : "^Could not create dir: (.*)$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/aws/aws-sdk-php/src/S3/Transfer.php" , line : 417usize , placeholders : & ["$operation"] , regex : "^Transfer encountered an unexpected operation: (.*)\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/aws/aws-sdk-php/src/S3/ValidateResponseChecksumParser.php" , line : 84usize , placeholders : & [] , regex : "^Calculated response checksum did not match the expected value$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/aws/aws-sdk-php/src/S3/StreamWrapper.php" , line : 321usize , placeholders : & ["$path"] , regex : "^File or directory not found: (.*)$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/aws/aws-sdk-php/src/S3/StreamWrapper.php" , line : 673usize , placeholders : & [] , regex : "^No client in stream context$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/aws/aws-sdk-php/src/S3/S3EndpointMiddleware.php" , line : 305usize , placeholders : & [] , regex : "^Outposts \\+ dualstack is not supported$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/aws/aws-sdk-php/src/S3/S3EndpointMiddleware.php" , line : 312usize , placeholders : & [] , regex : "^Custom Endpoint \\+ Dualstack not supported$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/aws/aws-sdk-php/src/S3/RegionalEndpoint/Configuration.php" , line : 13usize , placeholders : & [] , regex : "^Configuration parameter must either be 'legacy' or 'regional'\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/aws/aws-sdk-php/src/S3/BatchDelete.php" , line : 157usize , placeholders : & [] , regex : "^before must be callable$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/aws/aws-sdk-php/src/S3/BatchDelete.php" , line : 164usize , placeholders : & [] , regex : "^batch_size is not > 0$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/aws/aws-crt-php/src/AWS/CRT/Auth/AwsCredentials.php" , line : 50usize , placeholders : & [] , regex : "^access_key_id must be provided$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/aws/aws-crt-php/src/AWS/CRT/Auth/AwsCredentials.php" , line : 53usize , placeholders : & [] , regex : "^secret_access_key must be provided$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/aws/aws-crt-php/src/AWS/CRT/CRT.php" , line : 28usize , placeholders : & ["$rex"] , regex : "^Unable to initialize AWS CRT via awscrt extension: \n(.*)$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/aws/aws-crt-php/src/AWS/CRT/Internal/Extension.php" , line : 17usize , placeholders : & [] , regex : "^awscrt extension is not loaded$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/aws/aws-crt-php/gen_stub.php" , line : 44usize , placeholders : & ["$stubFile"] , regex : "^File (.*) does not exist$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/aws/aws-crt-php/gen_stub.php" , line : 132usize , placeholders : & [] , regex : "^Unexpected node type$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/aws/aws-crt-php/gen_stub.php" , line : 202usize , placeholders : & ["$this->name"] , regex : "^Not implemented: (.*)$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/aws/aws-crt-php/gen_stub.php" , line : 232usize , placeholders : & ["$this->name"] , regex : "^Not implemented: (.*)$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/aws/aws-crt-php/gen_stub.php" , line : 420usize , placeholders : & [] , regex : "^Invalid sendBy value$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/aws/aws-crt-php/gen_stub.php" , line : 432usize , placeholders : & [] , regex : "^A parameter must have a type$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/aws/aws-crt-php/gen_stub.php" , line : 492usize , placeholders : & [] , regex : "^Namespaced name not supported here$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/aws/aws-crt-php/gen_stub.php" , line : 759usize , placeholders : & [] , regex : "^Cannot happen$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/aws/aws-crt-php/gen_stub.php" , line : 809usize , placeholders : & [] , regex : "^Cannot happen$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/aws/aws-crt-php/gen_stub.php" , line : 1022usize , placeholders : & ["$this->name"] , regex : "^@(.*) does not have a value$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/aws/aws-crt-php/gen_stub.php" , line : 1040usize , placeholders : & ["$this->name" , "$value"] , regex : "^@(.*) doesn't contain a type or has an invalid format \"(.*)\"$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/aws/aws-crt-php/gen_stub.php" , line : 1049usize , placeholders : & ["$this->name"] , regex : "^@(.*) doesn't have any value$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/aws/aws-crt-php/gen_stub.php" , line : 1061usize , placeholders : & ["$this->name" , "$value"] , regex : "^@(.*) doesn't contain a variable name or has an invalid format \"(.*)\"$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/aws/aws-crt-php/gen_stub.php" , line : 1138usize , placeholders : & ["$varName" , "$name"] , regex : "^Duplicate parameter name (.*) for function (.*)$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/aws/aws-crt-php/gen_stub.php" , line : 1151usize , placeholders : & ["$name"] , regex : "^Error in function (.*): only the last parameter can be variadic$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/aws/aws-crt-php/gen_stub.php" , line : 1156usize , placeholders : & ["$name"] , regex : "^Missing parameter type for function (.*)\\(\\)$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/aws/aws-crt-php/gen_stub.php" , line : 1166usize , placeholders : & ["$varName" , "$name"] , regex : "^Parameter (.*) of function (.*) has null default, but is not nullable$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/aws/aws-crt-php/gen_stub.php" , line : 1186usize , placeholders : & ["$var" , "$name"] , regex : "^Found metadata for invalid param (.*) of function (.*)$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/aws/aws-crt-php/gen_stub.php" , line : 1191usize , placeholders : & ["$name"] , regex : "^Missing return type for function (.*)\\(\\)$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/aws/aws-crt-php/gen_stub.php" , line : 1226usize , placeholders : & [] , regex : "^Encountered else without corresponding \\#if$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/aws/aws-crt-php/gen_stub.php" , line : 1232usize , placeholders : & [] , regex : "^Encountered \\#endif without corresponding \\#if$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/aws/aws-crt-php/gen_stub.php" , line : 1236usize , placeholders : & ["$text"] , regex : "^Unrecognized preprocessor directive \"(.*)\"$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/aws/aws-crt-php/gen_stub.php" , line : 1295usize , placeholders : & ["$classStmt->getType()"] , regex : "^Not implemented (.*)$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/aws/aws-crt-php/gen_stub.php" , line : 1309usize , placeholders : & [] , regex : "^Method visibility modifier is required$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/aws/aws-crt-php/gen_stub.php" , line : 1326usize , placeholders : & ["$stmt->getType()"] , regex : "^Unexpected node (.*)$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/aws/aws-crt-php/gen_stub.php" , line : 1766usize , placeholders : & [] , regex : "^Failed to acquire installation lock$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/aws/aws-crt-php/gen_stub.php" , line : 1784usize , placeholders : & [] , regex : "^Failed to download PHP\\-Parser tarball$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/aws/aws-crt-php/gen_stub.php" , line : 1787usize , placeholders : & ["$phpParserDir"] , regex : "^Failed to create directory (.*)$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/aws/aws-crt-php/gen_stub.php" , line : 1791usize , placeholders : & [] , regex : "^Failed to extract PHP\\-Parser tarball$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/aws/aws-crt-php/gen_stub.php" , line : 1808usize , placeholders : & [] , regex : "^The \"tokenizer\" extension is not available$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/brick/math/src/BigRational.php" , line : 393usize , placeholders : & [] , regex : "^This rational number cannot be represented as an integer value without rounding\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/brick/math/src/BigRational.php" , line : 481usize , placeholders : & [] , regex : "^unserialize\\(\\) is an internal function, it must not be called directly\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/brick/math/src/Internal/Calculator.php" , line : 558usize , placeholders : & [] , regex : "^Invalid rounding mode\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/brick/math/src/Internal/Calculator.php" , line : 666usize , placeholders : & [] , regex : "^Invalid bitwise operator\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/brick/math/src/Internal/Calculator/NativeCalculator.php" , line : 47usize , placeholders : & [] , regex : "^The platform is not 32\\-bit or 64\\-bit as expected\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/brick/math/src/BigInteger.php" , line : 84usize , placeholders : & [] , regex : "^The number cannot be empty\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/brick/math/src/BigInteger.php" , line : 102usize , placeholders : & [] , regex : "^The number cannot be empty\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/brick/math/src/BigInteger.php" , line : 151usize , placeholders : & [] , regex : "^The number cannot be empty\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/brick/math/src/BigInteger.php" , line : 157usize , placeholders : & [] , regex : "^The alphabet must contain at least 2 chars\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/brick/math/src/BigInteger.php" , line : 193usize , placeholders : & [] , regex : "^The byte string must not be empty\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/brick/math/src/BigInteger.php" , line : 234usize , placeholders : & [] , regex : "^The number of bits cannot be negative\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/brick/math/src/BigInteger.php" , line : 280usize , placeholders : & [] , regex : "^\\$min cannot be greater than \\$max\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/brick/math/src/BigInteger.php" , line : 621usize , placeholders : & [] , regex : "^Modulus must not be negative\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/brick/math/src/BigInteger.php" , line : 631usize , placeholders : & [] , regex : "^Unable to compute the modInverse for the given modulus\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/brick/math/src/BigInteger.php" , line : 656usize , placeholders : & [] , regex : "^The operands cannot be negative\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/brick/math/src/BigInteger.php" , line : 706usize , placeholders : & [] , regex : "^Cannot calculate the square root of a negative number\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/brick/math/src/BigInteger.php" , line : 916usize , placeholders : & [] , regex : "^The bit to test cannot be negative\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/brick/math/src/BigInteger.php" , line : 1040usize , placeholders : & [] , regex : "^The alphabet must contain at least 2 chars\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/brick/math/src/BigInteger.php" , line : 1073usize , placeholders : & [] , regex : "^Cannot convert a negative number to a byte string when \\$signed is false\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/brick/math/src/BigInteger.php" , line : 1146usize , placeholders : & [] , regex : "^unserialize\\(\\) is an internal function, it must not be called directly\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/brick/math/src/BigDecimal.php" , line : 84usize , placeholders : & [] , regex : "^The scale cannot be negative\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/brick/math/src/BigDecimal.php" , line : 266usize , placeholders : & [] , regex : "^Scale cannot be negative\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/brick/math/src/BigDecimal.php" , line : 457usize , placeholders : & [] , regex : "^Scale cannot be negative\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/brick/math/src/BigDecimal.php" , line : 465usize , placeholders : & [] , regex : "^Cannot calculate the square root of a negative number\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/brick/math/src/BigDecimal.php" , line : 782usize , placeholders : & [] , regex : "^unserialize\\(\\) is an internal function, it must not be called directly\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/brick/math/src/BigNumber.php" , line : 131usize , placeholders : & [] , regex : "^Exponent too large\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/justinrainbow/json-schema/src/JsonSchema/Entity/JsonPointer.php" , line : 40usize , placeholders : & [] , regex : "^Ref value must be a string$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/justinrainbow/json-schema/src/JsonSchema/Constraints/SchemaConstraint.php" , line : 40usize , placeholders : & [] , regex : "^no schema found to verify against$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/justinrainbow/json-schema/src/JsonSchema/Constraints/SchemaConstraint.php" , line : 52usize , placeholders : & [] , regex : "^Cannot validate the schema of a non\\-object$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/justinrainbow/json-schema/src/JsonSchema/Constraints/SchemaConstraint.php" , line : 79usize , placeholders : & [] , regex : "^Schema did not pass validation$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/justinrainbow/json-schema/src/JsonSchema/Uri/Retrievers/Curl.php" , line : 28usize , placeholders : & [] , regex : "^cURL not installed$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/justinrainbow/json-schema/src/JsonSchema/Uri/Retrievers/Curl.php" , line : 48usize , placeholders : & [] , regex : "^JSON schema not found$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/cweagans/composer-patches/src/Patches.php" , line : 274usize , placeholders : & [] , regex : "^There was an error in the supplied patch file$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/cweagans/composer-patches/src/Patches.php" , line : 331usize , placeholders : & ["$description" , "$url"] , regex : "^Cannot apply patch (.*) \\((.*)\\)!$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/cweagans/composer-patches/src/Patches.php" , line : 428usize , placeholders : & ["$patch_url"] , regex : "^Cannot apply patch (.*)$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Info , path : "/3rdparty/web-auth/metadata-service/src/MetadataService.php" , line : 151usize , placeholders : & [] , regex : "^Trying to get the metadata statement for a given entry$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/web-auth/metadata-service/src/MetadataService.php" , line : 156usize , placeholders : & [] , regex : "^The Metadata Statement has not been published$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Info , path : "/3rdparty/web-auth/metadata-service/src/MetadataService.php" , line : 160usize , placeholders : & [] , regex : "^The metadata statement exists$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Debug , path : "/3rdparty/web-auth/metadata-service/src/MetadataService.php" , line : 161usize , placeholders : & [] , regex : "^Metadata Statement$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Error , path : "/3rdparty/web-auth/metadata-service/src/MetadataService.php" , line : 165usize , placeholders : & [] , regex : "^An error occurred$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Info , path : "/3rdparty/web-auth/metadata-service/src/MetadataService.php" , line : 182usize , placeholders : & [] , regex : "^Trying to get the metadata service TOC payload$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Info , path : "/3rdparty/web-auth/metadata-service/src/MetadataService.php" , line : 186usize , placeholders : & [] , regex : "^The TOC payload has been received$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Debug , path : "/3rdparty/web-auth/metadata-service/src/MetadataService.php" , line : 187usize , placeholders : & [] , regex : "^TOC payload$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Error , path : "/3rdparty/web-auth/metadata-service/src/MetadataService.php" , line : 191usize , placeholders : & [] , regex : "^An error occurred$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/web-auth/metadata-service/src/MetadataTOCPayloadEntry.php" , line : 72usize , placeholders : & [] , regex : "^Authenticators cannot support both AAID and AAGUID$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/web-auth/metadata-service/src/MetadataTOCPayloadEntry.php" , line : 75usize , placeholders : & [] , regex : "^If neither AAID nor AAGUID are set, the attestation certificate identifier list shall not be empty$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/web-auth/metadata-service/src/Version.php" , line : 37usize , placeholders : & [] , regex : "^Invalid data\\. Must contain at least one item$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/web-auth/cose-lib/src/Algorithm/Signature/ECDSA/ECSignature.php" , line : 43usize , placeholders : & [] , regex : "^Invalid signature length\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/web-auth/cose-lib/src/Algorithm/Signature/ECDSA/ECSignature.php" , line : 62usize , placeholders : & [] , regex : "^Unable to convert into ASN\\.1$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/web-auth/cose-lib/src/Algorithm/Signature/ECDSA/ECSignature.php" , line : 74usize , placeholders : & [] , regex : "^Invalid data\\. Should start with a sequence\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/web-auth/cose-lib/src/Algorithm/Signature/ECDSA/ECSignature.php" , line : 87usize , placeholders : & [] , regex : "^Unable to convert from ASN\\.1$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/web-auth/cose-lib/src/Algorithm/Signature/ECDSA/ECSignature.php" , line : 125usize , placeholders : & [] , regex : "^Invalid data\\. Should contain an integer\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/web-auth/cose-lib/src/Algorithm/Signature/RSA/PSSRSA.php" , line : 56usize , placeholders : & [] , regex : "^Invalid modulus length$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/web-auth/cose-lib/src/Algorithm/Signature/RSA/PSSRSA.php" , line : 103usize , placeholders : & [] , regex : "^Unable to convert the integer$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/web-auth/cose-lib/src/Algorithm/Signature/RSA/RSA.php" , line : 30usize , placeholders : & [] , regex : "^Unable to sign the data$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/web-auth/cose-lib/src/Algorithm/Signature/EdDSA/EdDSA.php" , line : 40usize , placeholders : & [] , regex : "^Unsupported curve$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/web-auth/cose-lib/src/Algorithm/Signature/EdDSA/EdDSA.php" , line : 52usize , placeholders : & [] , regex : "^Unsupported curve$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/web-auth/cose-lib/src/Key/RsaKey.php" , line : 193usize , placeholders : & [] , regex : "^Unable to convert to an integer$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/web-auth/webauthn-lib/src/AttestationStatement/FidoU2FAttestationStatementSupport.php" , line : 109usize , placeholders : & [] , regex : "^Invalid certificate or certificate chain$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/web-auth/webauthn-lib/src/AttestationStatement/TPMAttestationStatementSupport.php" , line : 98usize , placeholders : & [] , regex : "^Unsupported attestation statement$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/web-auth/webauthn-lib/src/AttestationStatement/TPMAttestationStatementSupport.php" , line : 121usize , placeholders : & [] , regex : "^Invalid or unsupported key type\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/web-auth/webauthn-lib/src/AttestationStatement/TPMAttestationStatementSupport.php" , line : 225usize , placeholders : & [] , regex : "^Unsupported type$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/web-auth/webauthn-lib/src/AttestationStatement/TPMAttestationStatementSupport.php" , line : 246usize , placeholders : & [] , regex : "^Unsupported hash algorithm$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/web-auth/webauthn-lib/src/AttestationStatement/TPMAttestationStatementSupport.php" , line : 307usize , placeholders : & [] , regex : "^ECDAA not supported$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/web-auth/webauthn-lib/src/AttestationStatement/PackedAttestationStatementSupport.php" , line : 90usize , placeholders : & [] , regex : "^Unsupported attestation statement$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/web-auth/webauthn-lib/src/AttestationStatement/PackedAttestationStatementSupport.php" , line : 167usize , placeholders : & [] , regex : "^ECDAA not supported$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/web-auth/webauthn-lib/src/AttestationStatement/PackedAttestationStatementSupport.php" , line : 188usize , placeholders : & [] , regex : "^Invalid algorithm$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/web-auth/webauthn-lib/src/AttestationStatement/AndroidSafetyNetAttestationStatementSupport.php" , line : 76usize , placeholders : & [] , regex : "^The algorithm RS256 is missing\\. Did you forget to install the package web\\-token/jwt\\-signature\\-algorithm\\-rsa\\?$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/web-auth/webauthn-lib/src/AttestationStatement/AndroidSafetyNetAttestationStatementSupport.php" , line : 79usize , placeholders : & [] , regex : "^The class Jose\\\\Component\\\\KeyManagement\\\\JWKFactory is missing\\. Did you forget to install the package web\\-token/jwt\\-key\\-mgmt\\?$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/web-auth/webauthn-lib/src/AttestationStatement/AndroidSafetyNetAttestationStatementSupport.php" , line : 256usize , placeholders : & [] , regex : "^Unrecognized response$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Info , path : "/3rdparty/web-auth/webauthn-lib/src/AttestationStatement/AttestationObjectLoader.php" , line : 76usize , placeholders : & [] , regex : "^Trying to load the data$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Info , path : "/3rdparty/web-auth/webauthn-lib/src/AttestationStatement/AttestationObjectLoader.php" , line : 81usize , placeholders : & [] , regex : "^Loading the Attestation Statement$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Info , path : "/3rdparty/web-auth/webauthn-lib/src/AttestationStatement/AttestationObjectLoader.php" , line : 93usize , placeholders : & [] , regex : "^Attestation Statement loaded$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Debug , path : "/3rdparty/web-auth/webauthn-lib/src/AttestationStatement/AttestationObjectLoader.php" , line : 94usize , placeholders : & [] , regex : "^Attestation Statement loaded$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Info , path : "/3rdparty/web-auth/webauthn-lib/src/AttestationStatement/AttestationObjectLoader.php" , line : 105usize , placeholders : & [] , regex : "^Attested Credential Data is present$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Info , path : "/3rdparty/web-auth/webauthn-lib/src/AttestationStatement/AttestationObjectLoader.php" , line : 113usize , placeholders : & [] , regex : "^Attested Credential Data loaded$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Debug , path : "/3rdparty/web-auth/webauthn-lib/src/AttestationStatement/AttestationObjectLoader.php" , line : 114usize , placeholders : & [] , regex : "^Attested Credential Data loaded$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Info , path : "/3rdparty/web-auth/webauthn-lib/src/AttestationStatement/AttestationObjectLoader.php" , line : 119usize , placeholders : & [] , regex : "^Extension Data loaded$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Info , path : "/3rdparty/web-auth/webauthn-lib/src/AttestationStatement/AttestationObjectLoader.php" , line : 122usize , placeholders : & [] , regex : "^Extension Data loaded$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Debug , path : "/3rdparty/web-auth/webauthn-lib/src/AttestationStatement/AttestationObjectLoader.php" , line : 123usize , placeholders : & [] , regex : "^Extension Data loaded$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Info , path : "/3rdparty/web-auth/webauthn-lib/src/AttestationStatement/AttestationObjectLoader.php" , line : 130usize , placeholders : & [] , regex : "^Attestation Object loaded$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Debug , path : "/3rdparty/web-auth/webauthn-lib/src/AttestationStatement/AttestationObjectLoader.php" , line : 131usize , placeholders : & [] , regex : "^Attestation Object$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Error , path : "/3rdparty/web-auth/webauthn-lib/src/AttestationStatement/AttestationObjectLoader.php" , line : 135usize , placeholders : & [] , regex : "^An error occurred$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/web-auth/webauthn-lib/src/PublicKeyCredentialSource.php" , line : 218usize , placeholders : & [] , regex : "^Unable to load the data$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Info , path : "/3rdparty/web-auth/webauthn-lib/src/AuthenticatorAttestationResponseValidator.php" , line : 123usize , placeholders : & [] , regex : "^Checking the authenticator attestation response$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Info , path : "/3rdparty/web-auth/webauthn-lib/src/AuthenticatorAttestationResponseValidator.php" , line : 214usize , placeholders : & [] , regex : "^The attestation is valid$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Debug , path : "/3rdparty/web-auth/webauthn-lib/src/AuthenticatorAttestationResponseValidator.php" , line : 215usize , placeholders : & [] , regex : "^Public Key Credential Source$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Error , path : "/3rdparty/web-auth/webauthn-lib/src/AuthenticatorAttestationResponseValidator.php" , line : 219usize , placeholders : & [] , regex : "^An error occurred$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Debug , path : "/3rdparty/web-auth/webauthn-lib/src/AuthenticatorAttestationResponseValidator.php" , line : 262usize , placeholders : & [] , regex : "^No attestation is asked\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Debug , path : "/3rdparty/web-auth/webauthn-lib/src/AuthenticatorAttestationResponseValidator.php" , line : 267usize , placeholders : & [] , regex : "^The Attestation Statement is anonymous\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Debug , path : "/3rdparty/web-auth/webauthn-lib/src/AuthenticatorAttestationResponseValidator.php" , line : 272usize , placeholders : & [] , regex : "^Anonymization required\\. AAGUID and Attestation Statement changed\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Debug , path : "/3rdparty/web-auth/webauthn-lib/src/AuthenticatorAttestationResponseValidator.php" , line : 284usize , placeholders : & [] , regex : "^No attestation returned\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Debug , path : "/3rdparty/web-auth/webauthn-lib/src/AuthenticatorAttestationResponseValidator.php" , line : 287usize , placeholders : & [] , regex : "^Anonymization required\\. AAGUID and Attestation Statement changed\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/web-auth/webauthn-lib/src/AuthenticatorAttestationResponseValidator.php" , line : 335usize , placeholders : & [] , regex : "^The authenticator is compromised and cannot be used$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/web-auth/webauthn-lib/src/AuthenticatorAttestationResponseValidator.php" , line : 367usize , placeholders : & [] , regex : "^Invalid attestation type$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/web-auth/webauthn-lib/src/CertificateChainChecker/OpenSSLCertificateChainChecker.php" , line : 98usize , placeholders : & [] , regex : "^Invalid certificate or certificate chain$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/web-auth/webauthn-lib/src/CertificateChainChecker/OpenSSLCertificateChainChecker.php" , line : 148usize , placeholders : & [] , regex : "^Invalid certificate or certificate chain$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Info , path : "/3rdparty/web-auth/webauthn-lib/src/AuthenticatorAssertionResponseValidator.php" , line : 97usize , placeholders : & [] , regex : "^Checking the authenticator assertion response$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Info , path : "/3rdparty/web-auth/webauthn-lib/src/AuthenticatorAssertionResponseValidator.php" , line : 211usize , placeholders : & [] , regex : "^The assertion is valid$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Debug , path : "/3rdparty/web-auth/webauthn-lib/src/AuthenticatorAssertionResponseValidator.php" , line : 212usize , placeholders : & [] , regex : "^Public Key Credential Source$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Error , path : "/3rdparty/web-auth/webauthn-lib/src/AuthenticatorAssertionResponseValidator.php" , line : 216usize , placeholders : & [] , regex : "^An error occurred$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/web-auth/webauthn-lib/src/CertificateToolbox.php" , line : 64usize , placeholders : & [] , regex : "^Invalid certificate or certificate chain$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/web-auth/webauthn-lib/src/CertificateToolbox.php" , line : 98usize , placeholders : & [] , regex : "^Invalid certificate or certificate chain$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Info , path : "/3rdparty/web-auth/webauthn-lib/src/PublicKeyCredentialLoader.php" , line : 82usize , placeholders : & [] , regex : "^Trying to load data from an array$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Info , path : "/3rdparty/web-auth/webauthn-lib/src/PublicKeyCredentialLoader.php" , line : 102usize , placeholders : & [] , regex : "^The data has been loaded$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Debug , path : "/3rdparty/web-auth/webauthn-lib/src/PublicKeyCredentialLoader.php" , line : 103usize , placeholders : & [] , regex : "^Public Key Credential$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Error , path : "/3rdparty/web-auth/webauthn-lib/src/PublicKeyCredentialLoader.php" , line : 107usize , placeholders : & [] , regex : "^An error occurred$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Info , path : "/3rdparty/web-auth/webauthn-lib/src/PublicKeyCredentialLoader.php" , line : 116usize , placeholders : & [] , regex : "^Trying to load data from a string$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Error , path : "/3rdparty/web-auth/webauthn-lib/src/PublicKeyCredentialLoader.php" , line : 122usize , placeholders : & [] , regex : "^An error occurred$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/web-auth/webauthn-lib/src/PublicKeyCredentialLoader.php" , line : 178usize , placeholders : & [] , regex : "^Unable to create the response object$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Error , path : "/3rdparty/web-auth/webauthn-lib/src/Counter/ThrowExceptionIfInvalid.php" , line : 39usize , placeholders : & [] , regex : "^The counter is invalid$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/opis/closure/src/SerializableClosure.php" , line : 200usize , placeholders : & [] , regex : "^Invalid signed closure$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/opis/closure/src/SerializableClosure.php" , line : 223usize , placeholders : & [] , regex : "^Invalid signed closure$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/opis/closure/src/SerializableClosure.php" , line : 236usize , placeholders : & [] , regex : "^Invalid signed closure$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/deepdiver1975/tarstreamer/src/TarStreamer.php" , line : 54usize , placeholders : & ["$encodedArchiveName" , "$headerFile" , "$headerLine"] , regex : "^Unable to send file (.*)\\. HTML Headers have already been sent from (.*) in line (.*)$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/deepdiver1975/tarstreamer/src/TarStreamer.php" , line : 58usize , placeholders : & ["$encodedArchiveName"] , regex : "^Unable to send file (.*)\\. Output buffer already contains text \\(typically warnings or errors\\)\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/icewind/streams/src/Wrapper.php" , line : 40usize , placeholders : & [] , regex : "^Invalid context, source not set$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/icewind/streams/src/WrapperHandler.php" , line : 87usize , placeholders : & [] , regex : "^Invalid stream source$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/icewind/streams/src/UrlCallback.php" , line : 133usize , placeholders : & [] , regex : "^stat is not supported due to php bug 50526$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/icewind/streams/src/IteratorDirectory.php" , line : 48usize , placeholders : & [] , regex : "^Invalid context, iterator or array not set$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/icewind/streams/src/IteratorDirectory.php" , line : 109usize , placeholders : & [] , regex : "^\\$source should be an Iterator or array$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/icewind/streams/src/CountWrapper.php" , line : 64usize , placeholders : & [] , regex : "^Invalid or missing callback$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/icewind/searchdav/src/DAV/SearchHandler.php" , line : 105usize , placeholders : & [] , regex : "^requested order by property is not a valid property for this scope$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/icewind/searchdav/src/DAV/SearchHandler.php" , line : 109usize , placeholders : & [] , regex : "^requested order by property is not sortable$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/icewind/searchdav/src/DAV/SearchHandler.php" , line : 119usize , placeholders : & [] , regex : "^requested property is not selectable$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/icewind/searchdav/src/DAV/SearchHandler.php" , line : 140usize , placeholders : & [] , regex : "^requested search property is not a valid property for this scope$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/icewind/searchdav/src/DAV/SearchHandler.php" , line : 144usize , placeholders : & [] , regex : "^requested search property is not searchable$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/icewind/searchdav/src/XML/BasicSearch.php" , line : 87usize , placeholders : & [] , regex : "^Missing \\{DAV:\\}from when parsing \\{DAV:\\}basicsearch$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/fusonic/linq/src/Fusonic/Linq/Linq.php" , line : 76usize , placeholders : & [] , regex : "^\\$count must be not be negative\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/fusonic/linq/src/Fusonic/Linq/Linq.php" , line : 171usize , placeholders : & [] , regex : "^The input sequence contains no elements\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/fusonic/linq/src/Fusonic/Linq/Linq.php" , line : 186usize , placeholders : & [] , regex : "^chunksize$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/fusonic/linq/src/Fusonic/Linq/Linq.php" , line : 279usize , placeholders : & [] , regex : "^Cannot calculate an average on a none numeric value$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/fusonic/linq/src/Fusonic/Linq/Linq.php" , line : 328usize , placeholders : & [] , regex : "^sum\\(\\) only works on numeric values\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/fusonic/linq/src/Fusonic/Linq/Linq.php" , line : 350usize , placeholders : & [] , regex : "^min\\(\\) only works on numeric values, strings and DateTime objects\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/fusonic/linq/src/Fusonic/Linq/Linq.php" , line : 361usize , placeholders : & [] , regex : "^Cannot calculate min\\(\\) as the Linq sequence contains no elements\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/fusonic/linq/src/Fusonic/Linq/Linq.php" , line : 381usize , placeholders : & [] , regex : "^max\\(\\) only works on numeric values, strings and DateTime objects\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/fusonic/linq/src/Fusonic/Linq/Linq.php" , line : 392usize , placeholders : & [] , regex : "^Cannot calculate max\\(\\) as the Linq sequence contains no elements\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/fusonic/linq/src/Fusonic/Linq/Linq.php" , line : 537usize , placeholders : & [] , regex : "^Index is less than 0 or greater than or equal to the number of elements in the sequence\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/fusonic/linq/src/Fusonic/Linq/Linq.php" , line : 646usize , placeholders : & [] , regex : "^The input sequence contains more than 1 elements\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/fusonic/linq/src/Fusonic/Linq/Linq.php" , line : 653usize , placeholders : & [] , regex : "^The input sequence contains no matching element\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/fusonic/linq/src/Fusonic/Linq/Linq.php" , line : 673usize , placeholders : & [] , regex : "^The input sequence contains no matching element\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/fusonic/linq/src/Fusonic/Linq/Linq.php" , line : 692usize , placeholders : & [] , regex : "^The input sequence contains no matching element\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/fusonic/linq/src/Fusonic/Linq/Iterator/SelectIterator.php" , line : 26usize , placeholders : & [] , regex : "^Selector must not be null\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/fusonic/linq/src/Fusonic/Linq/Helper/LinqHelper.php" , line : 34usize , placeholders : & [] , regex : "^Return type of filter func must be boolean\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/fusonic/linq/src/Fusonic/Linq/Helper/LinqHelper.php" , line : 58usize , placeholders : & [] , regex : "^Value must be an array, or implement either the \\\\IteratorAggregate or \\\\Iterator interface$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/fusonic/opengraph/src/Consumer.php" , line : 58usize , placeholders : & [] , regex : "^To use loadUrl\\(\\) you must provide \\$client and \\$requestFactory when instantiating the consumer\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/mlocati/ip-lib/src/Range/Pattern.php" , line : 184usize , placeholders : & [] , regex : "^@todo$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/mlocati/ip-lib/src/Range/AbstractRange.php" , line : 38usize , placeholders : & [] , regex : "^@todo$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/doctrine/dbal/src/Result.php" , line : 284usize , placeholders : & [] , regex : "^Only invocations with one argument are still supported by this legacy API\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/doctrine/dbal/src/Result.php" , line : 299usize , placeholders : & [] , regex : "^Only fetch modes declared on Doctrine\\\\DBAL\\\\FetchMode are supported by legacy API\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/doctrine/dbal/src/Result.php" , line : 314usize , placeholders : & [] , regex : "^Only invocations with one argument are still supported by this legacy API\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/doctrine/dbal/src/Result.php" , line : 329usize , placeholders : & [] , regex : "^Only fetch modes declared on Doctrine\\\\DBAL\\\\FetchMode are supported by legacy API\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/doctrine/dbal/src/Tools/Console/Command/RunSqlCommand.php" , line : 74usize , placeholders : & [] , regex : "^Argument 'SQL' is required in order to execute this command correctly\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Warn , path : "/3rdparty/doctrine/dbal/src/Tools/Console/Command/RunSqlCommand.php" , line : 80usize , placeholders : & [] , regex : "^Parameter \"depth\" is deprecated and has no effect anymore\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/doctrine/dbal/src/Connections/PrimaryReadReplicaConnection.php" , line : 112usize , placeholders : & [] , regex : "^primary or replica configuration missing$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/doctrine/dbal/src/Connections/PrimaryReadReplicaConnection.php" , line : 116usize , placeholders : & [] , regex : "^You have to configure at least one replica\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/doctrine/dbal/src/Connections/PrimaryReadReplicaConnection.php" , line : 163usize , placeholders : & [] , regex : "^Invalid option to connect\\(\\), only primary or replica allowed\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/doctrine/dbal/src/Id/TableGenerator.php" , line : 83usize , placeholders : & [] , regex : "^Cannot use TableGenerator with SQLite\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/doctrine/dbal/src/Id/TableGenerator.php" , line : 146usize , placeholders : & [] , regex : "^Race\\-condition detected while updating sequence\\. Aborting generation$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/doctrine/dbal/src/Driver/Middleware/AbstractConnectionMiddleware.php" , line : 96usize , placeholders : & [] , regex : "^The underlying connection is not a ServerInfoAwareConnection$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Debug , path : "/3rdparty/doctrine/dbal/src/Logging/Statement.php" , line : 68usize , placeholders : & [] , regex : "^Executing statement: \\{sql\\} \\(parameters: \\{params\\}, types: \\{types\\}\\)$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Info , path : "/3rdparty/doctrine/dbal/src/Logging/Driver.php" , line : 31usize , placeholders : & [] , regex : "^Connecting with parameters \\{params\\}$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Info , path : "/3rdparty/doctrine/dbal/src/Logging/Connection.php" , line : 30usize , placeholders : & [] , regex : "^Disconnecting$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Debug , path : "/3rdparty/doctrine/dbal/src/Logging/Connection.php" , line : 44usize , placeholders : & [] , regex : "^Executing query: \\{sql\\}$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Debug , path : "/3rdparty/doctrine/dbal/src/Logging/Connection.php" , line : 51usize , placeholders : & [] , regex : "^Executing statement: \\{sql\\}$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Debug , path : "/3rdparty/doctrine/dbal/src/Logging/Connection.php" , line : 61usize , placeholders : & [] , regex : "^Beginning transaction$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Debug , path : "/3rdparty/doctrine/dbal/src/Logging/Connection.php" , line : 71usize , placeholders : & [] , regex : "^Committing transaction$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Debug , path : "/3rdparty/doctrine/dbal/src/Logging/Connection.php" , line : 81usize , placeholders : & [] , regex : "^Rolling back transaction$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/doctrine/dbal/src/DriverManager.php" , line : 306usize , placeholders : & [] , regex : "^Malformed parameter \"url\"\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/doctrine/dbal/src/Platforms/SqlitePlatform.php" , line : 723usize , placeholders : & [] , regex : "^Sqlite platform requires for alter table the table diff with reference to original table schema$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/doctrine/dbal/src/Platforms/SqlitePlatform.php" , line : 748usize , placeholders : & [] , regex : "^Sqlite platform requires for alter table the table diff with reference to original table schema$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/doctrine/dbal/src/Platforms/SqlitePlatform.php" , line : 835usize , placeholders : & [] , regex : "^Sqlite platform does not support alter primary key\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/doctrine/dbal/src/Platforms/SqlitePlatform.php" , line : 843usize , placeholders : & [] , regex : "^Sqlite platform does not support alter foreign key\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/doctrine/dbal/src/Platforms/SqlitePlatform.php" , line : 851usize , placeholders : & [] , regex : "^Sqlite platform does not support alter foreign key\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/doctrine/dbal/src/Platforms/SqlitePlatform.php" , line : 861usize , placeholders : & [] , regex : "^Sqlite platform does not support alter constraint\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/doctrine/dbal/src/Platforms/SqlitePlatform.php" , line : 902usize , placeholders : & [] , regex : "^Sqlite platform requires for alter table the table diff with reference to original table schema$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/doctrine/dbal/src/Platforms/SQLServerPlatform.php" , line : 402usize , placeholders : & [] , regex : "^Incomplete column definition\\. 'default' required\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/doctrine/dbal/src/Platforms/AbstractPlatform.php" , line : 1638usize , placeholders : & [] , regex : "^Default implementation of DROP TABLE was overridden with NULL$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/doctrine/dbal/src/Platforms/AbstractPlatform.php" , line : 1756usize , placeholders : & [] , regex : "^Second argument of AbstractPlatform::getCreateTableSQL\\(\\) has to be integer\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/doctrine/dbal/src/Platforms/AbstractPlatform.php" , line : 2036usize , placeholders : & [] , regex : "^Can only create primary or unique constraints, no common indexes with getCreateConstraintSQL\\(\\)\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/doctrine/dbal/src/Platforms/AbstractPlatform.php" , line : 2072usize , placeholders : & [] , regex : "^Incomplete definition\\. 'columns' required\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/doctrine/dbal/src/Platforms/AbstractPlatform.php" , line : 2652usize , placeholders : & [] , regex : "^Incomplete definition\\. 'columns' required\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/doctrine/dbal/src/Platforms/AbstractPlatform.php" , line : 2679usize , placeholders : & [] , regex : "^Incomplete definition\\. 'columns' required\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/doctrine/dbal/src/Platforms/AbstractPlatform.php" , line : 2849usize , placeholders : & [] , regex : "^Incomplete definition\\. 'local' required\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/doctrine/dbal/src/Platforms/AbstractPlatform.php" , line : 2853usize , placeholders : & [] , regex : "^Incomplete definition\\. 'foreign' required\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/doctrine/dbal/src/Platforms/AbstractPlatform.php" , line : 2857usize , placeholders : & [] , regex : "^Incomplete definition\\. 'foreignTable' required\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/doctrine/dbal/src/Platforms/OraclePlatform.php" , line : 51usize , placeholders : & [] , regex : "^Invalid Oracle identifier$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/doctrine/cache/lib/Doctrine/Common/Cache/Psr6/CacheAdapter.php" , line : 261usize , placeholders : & [] , regex : "^Cache key length must be greater than zero\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/scssphp/scssphp/src/Util.php" , line : 47usize , placeholders : & ["$name" , "$val"] , regex : "^(.*) (.*) is not a number\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/scssphp/scssphp/src/Util.php" , line : 62usize , placeholders : & ["$name" , "$val" , "$range->first" , "$range->last" , "$unit"] , regex : "^(.*) (.*) must be between (.*) and (.*)(.*)$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/scssphp/scssphp/src/Util.php" , line : 124usize , placeholders : & [] , regex : "^Either mbstring \\(recommended\\) or iconv is necessary to use Scssphp\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/scssphp/scssphp/src/Util.php" , line : 161usize , placeholders : & [] , regex : "^Either mbstring \\(recommended\\) or iconv is necessary to use Scssphp\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/scssphp/scssphp/src/Util.php" , line : 182usize , placeholders : & [] , regex : "^Either mbstring \\(recommended\\) or iconv is necessary to use Scssphp\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/scssphp/scssphp/src/Node/Number.php" , line : 208usize , placeholders : & [] , regex : "^Number is immutable$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/scssphp/scssphp/src/Node/Number.php" , line : 217usize , placeholders : & [] , regex : "^Number is immutable$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Error , path : "/3rdparty/scssphp/scssphp/src/Compiler.php" , line : 773usize , placeholders : & ["$origin" , "$target" , "$target"] , regex : "^\"(.*)\" failed to @extend \"(.*)\"\\. The selector \"(.*)\" was not found\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Error , path : "/3rdparty/scssphp/scssphp/src/Compiler.php" , line : 2382usize , placeholders : & [] , regex : "^@return may only be used within a function$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Error , path : "/3rdparty/scssphp/scssphp/src/Compiler.php" , line : 3109usize , placeholders : & [] , regex : "^Parent selectors aren't allowed here\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Error , path : "/3rdparty/scssphp/scssphp/src/Compiler.php" , line : 3116usize , placeholders : & [] , regex : "^complex selectors may not be extended\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Error , path : "/3rdparty/scssphp/scssphp/src/Compiler.php" , line : 3274usize , placeholders : & ["$name"] , regex : "^Undefined mixin (.*)$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Error , path : "/3rdparty/scssphp/scssphp/src/Compiler.php" , line : 3333usize , placeholders : & ["$name"] , regex : "^@mixin (.*)\\(\\) without parentEnv$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Debug , path : "/3rdparty/scssphp/scssphp/src/Compiler.php" , line : 3380usize , placeholders : & ["$fname" , "$line" , "$value"] , regex : "^(.*):(.*) DEBUG: (.*)$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Warn , path : "/3rdparty/scssphp/scssphp/src/Compiler.php" , line : 3390usize , placeholders : & ["$value" , "$line" , "$fname"] , regex : "^(.*)\n         on line (.*) of (.*)$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Error , path : "/3rdparty/scssphp/scssphp/src/Compiler.php" , line : 3400usize , placeholders : & ["$fname" , "$line" , "$value"] , regex : "^File (.*) on line (.*) ERROR: (.*)\n$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Error , path : "/3rdparty/scssphp/scssphp/src/Compiler.php" , line : 3403usize , placeholders : & ["$child[0]"] , regex : "^unknown child type: (.*)$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Error , path : "/3rdparty/scssphp/scssphp/src/Compiler.php" , line : 4210usize , placeholders : & [] , regex : "^color: Can't take modulo by zero$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Error , path : "/3rdparty/scssphp/scssphp/src/Compiler.php" , line : 4218usize , placeholders : & [] , regex : "^color: Can't divide by zero$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Error , path : "/3rdparty/scssphp/scssphp/src/Compiler.php" , line : 4231usize , placeholders : & ["$op"] , regex : "^color: unknown op (.*)$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/scssphp/scssphp/src/Compiler.php" , line : 4821usize , placeholders : & [] , regex : "^The argument is not a sass string\\. Did you forgot to use \"assertString\"\\?$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Error , path : "/3rdparty/scssphp/scssphp/src/Compiler.php" , line : 5729usize , placeholders : & [] , regex : "^The Sass indented syntax is not implemented\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Error , path : "/3rdparty/scssphp/scssphp/src/Compiler.php" , line : 5868usize , placeholders : & ["$url"] , regex : "^`(.*)` file not found for @import$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Error , path : "/3rdparty/scssphp/scssphp/src/Compiler.php" , line : 6145usize , placeholders : & [] , regex : "^Error: Only %d arguments allowed in %s\\(\\), but %d were passed\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Error , path : "/3rdparty/scssphp/scssphp/src/Compiler.php" , line : 6158usize , placeholders : & [] , regex : "^Error: %s\\(\\) argument%s %s missing\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Error , path : "/3rdparty/scssphp/scssphp/src/Compiler.php" , line : 6224usize , placeholders : & [] , regex : "^An @import loop has been found: %s imports %s$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Error , path : "/3rdparty/scssphp/scssphp/src/Compiler.php" , line : 6261usize , placeholders : & ["$name"] , regex : "^@function (.*)\\(\\) without parentEnv$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/scssphp/scssphp/src/Compiler.php" , line : 6481usize , placeholders : & [] , regex : "^The argument declaration is invalid\\. The rest argument must be the last one\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/scssphp/scssphp/src/Compiler.php" , line : 6781usize , placeholders : & [] , regex : "^Positional arguments must come before keyword arguments\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Error , path : "/3rdparty/scssphp/scssphp/src/Compiler.php" , line : 7273usize , placeholders : & [] , regex : "^Expected %s to have no units or \"%%\"\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Error , path : "/3rdparty/scssphp/scssphp/src/Compiler.php" , line : 7408usize , placeholders : & [] , regex : "^expecting list, %s received$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/scssphp/scssphp/src/Compiler.php" , line : 7430usize , placeholders : & [] , regex : "^The argument is not a sass argument list\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/scssphp/scssphp/src/Compiler.php" , line : 7918usize , placeholders : & [] , regex : "^Only one positional argument is allowed\\. All other arguments must be passed by name\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Error , path : "/3rdparty/scssphp/scssphp/src/Compiler.php" , line : 7937usize , placeholders : & ["$name" , "$number"] , regex : "^(.*) Passing a number `(.*)` without unit % is deprecated\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/scssphp/scssphp/src/Compiler.php" , line : 7995usize , placeholders : & [] , regex : "^HSL parameters may not be passed along with HWB parameters\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Error , path : "/3rdparty/scssphp/scssphp/src/Compiler.php" , line : 8101usize , placeholders : & [] , regex : "^Error: argument `\\$color` of `ie\\-hex\\-str\\(\\$color\\)` must be a color$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Error , path : "/3rdparty/scssphp/scssphp/src/Compiler.php" , line : 8115usize , placeholders : & [] , regex : "^Error: argument `\\$color` of `red\\(\\$color\\)` must be a color$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Error , path : "/3rdparty/scssphp/scssphp/src/Compiler.php" , line : 8127usize , placeholders : & [] , regex : "^Error: argument `\\$color` of `green\\(\\$color\\)` must be a color$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Error , path : "/3rdparty/scssphp/scssphp/src/Compiler.php" , line : 8139usize , placeholders : & [] , regex : "^Error: argument `\\$color` of `blue\\(\\$color\\)` must be a color$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/scssphp/scssphp/src/Compiler.php" , line : 8238usize , placeholders : & [] , regex : "^Missing argument \\$lightness\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/scssphp/scssphp/src/Compiler.php" , line : 8567usize , placeholders : & [] , regex : "^Only one argument may be passed to the plain\\-CSS invert\\(\\) function\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Error , path : "/3rdparty/scssphp/scssphp/src/Compiler.php" , line : 8714usize , placeholders : & [] , regex : "^At least one argument must be passed\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Error , path : "/3rdparty/scssphp/scssphp/src/Compiler.php" , line : 8737usize , placeholders : & [] , regex : "^At least one argument must be passed\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Error , path : "/3rdparty/scssphp/scssphp/src/Compiler.php" , line : 8800usize , placeholders : & [] , regex : "^Invalid argument for \"n\"$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/scssphp/scssphp/src/Compiler.php" , line : 8978usize , placeholders : & [] , regex : "^Expected \\$args to contain a key\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/scssphp/scssphp/src/Compiler.php" , line : 8981usize , placeholders : & [] , regex : "^Expected \\$args to contain a value\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Error , path : "/3rdparty/scssphp/scssphp/src/Compiler.php" , line : 9370usize , placeholders : & [] , regex : "^Invalid argument\\(s\\) for \"comparable\"$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/scssphp/scssphp/src/Compiler.php" , line : 9617usize , placeholders : & ["$n"] , regex : "^\\$limit: Must be greater than 0, was (.*)\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Error , path : "/3rdparty/scssphp/scssphp/src/Compiler.php" , line : 9807usize , placeholders : & [] , regex : "^Invalid super selector for isSuperSelector\\(\\)$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Error , path : "/3rdparty/scssphp/scssphp/src/Compiler.php" , line : 9811usize , placeholders : & [] , regex : "^Invalid sub selector for isSuperSelector\\(\\)$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Error , path : "/3rdparty/scssphp/scssphp/src/Compiler.php" , line : 9911usize , placeholders : & [] , regex : "^selector\\-append\\(\\) needs at least 1 argument$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Error , path : "/3rdparty/scssphp/scssphp/src/Compiler.php" , line : 9936usize , placeholders : & [] , regex : "^Invalid selector list in selector\\-append\\(\\)$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Error , path : "/3rdparty/scssphp/scssphp/src/Compiler.php" , line : 9943usize , placeholders : & [] , regex : "^Invalid selector list in selector\\-append\\(\\)$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Error , path : "/3rdparty/scssphp/scssphp/src/Compiler.php" , line : 9983usize , placeholders : & [] , regex : "^selector\\-extend\\(\\) invalid arguments$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Error , path : "/3rdparty/scssphp/scssphp/src/Compiler.php" , line : 10004usize , placeholders : & [] , regex : "^selector\\-replace\\(\\) invalid arguments$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Error , path : "/3rdparty/scssphp/scssphp/src/Compiler.php" , line : 10033usize , placeholders : & [] , regex : "^Can't extend complex selector\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Error , path : "/3rdparty/scssphp/scssphp/src/Compiler.php" , line : 10071usize , placeholders : & [] , regex : "^selector\\-nest\\(\\) needs at least 1 argument$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Error , path : "/3rdparty/scssphp/scssphp/src/Compiler.php" , line : 10118usize , placeholders : & [] , regex : "^selector\\-unify\\(\\) invalid arguments$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Warn , path : "/3rdparty/scssphp/scssphp/src/Compiler.php" , line : 10413usize , placeholders : & [] , regex : "^The \"scssphp\\-glob\" function is deprecated an will be removed in ScssPhp 2\\.0\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Warn , path : "/3rdparty/scssphp/scssphp/src/Parser.php" , line : 539usize , placeholders : & ["$file" , "$line" , "$column"] , regex : "^The \"@scssphp\\-import\\-once\" directive is deprecated and will be removed in ScssPhp 2\\.0, in \"(.*)\", line (.*), column (.*)\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Warn , path : "/3rdparty/scssphp/scssphp/src/Parser.php" , line : 1626usize , placeholders : & ["$file" , "$line" , "$column"] , regex : "^Unterminated interpolations in multiline comments are deprecated and will be removed in ScssPhp 2\\.0, in \"(.*)\", line (.*), column (.*)\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/scssphp/scssphp/src/Cache.php" , line : 90usize , placeholders : & [] , regex : "^cacheDir not set$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/scssphp/scssphp/src/Cache.php" , line : 98usize , placeholders : & [] , regex : "^prefix not set$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/scssphp/scssphp/src/Warn.php" , line : 79usize , placeholders : & [] , regex : "^The warning Reporter may only be called within a custom function or importer callback\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/scssphp/scssphp/scss.inc.php" , line : 4usize , placeholders : & [] , regex : "^scssphp requires PHP 5\\.6 or above$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/giggsey/libphonenumber-for-php/src/prefixmapper/PrefixFileReader.php" , line : 39usize , placeholders : & ["$mapPath"] , regex : "^Invalid data directory: (.*)$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/giggsey/libphonenumber-for-php/src/prefixmapper/PrefixFileReader.php" , line : 73usize , placeholders : & [] , regex : "^Data does not exist$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/giggsey/libphonenumber-for-php/src/Leniency/AbstractLeniency.php" , line : 44usize , placeholders : & [] , regex : "^\\$level should be defined$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/giggsey/libphonenumber-for-php/src/PhoneNumberMatch.php" , line : 35usize , placeholders : & [] , regex : "^Start index must be >= 0\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/giggsey/libphonenumber-for-php/src/PhoneNumberMatch.php" , line : 39usize , placeholders : & [] , regex : "^\\$rawString must be a string$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/giggsey/libphonenumber-for-php/src/PhoneNumberToTimeZonesMapper.php" , line : 42usize , placeholders : & [] , regex : "^Mapping file can not be found$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/giggsey/locale/src/Locale.php" , line : 134usize , placeholders : & [] , regex : "^Locale is not supported$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/composer/ClassLoader.php" , line : 250usize , placeholders : & [] , regex : "^A non\\-empty PSR\\-4 prefix must end with a namespace separator\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/composer/ClassLoader.php" , line : 305usize , placeholders : & [] , regex : "^A non\\-empty PSR\\-4 prefix must end with a namespace separator\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/symfony/process/Process.php" , line : 143usize , placeholders : & [] , regex : "^The Process class relies on proc_open, which is not available on your PHP installation\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/symfony/process/Process.php" , line : 297usize , placeholders : & [] , regex : "^Process is already running\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/symfony/process/Process.php" , line : 355usize , placeholders : & [] , regex : "^Unable to launch a new process\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/symfony/process/Process.php" , line : 391usize , placeholders : & [] , regex : "^Process is already running\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/symfony/process/Process.php" , line : 424usize , placeholders : & [] , regex : "^Pass the callback to the \"Process::start\" method or call enableOutput to use a callback with \"Process::wait\"\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/symfony/process/Process.php" , line : 465usize , placeholders : & [] , regex : "^Pass the callback to the \"Process::start\" method or call enableOutput to use a callback with \"Process::waitUntil\"\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/symfony/process/Process.php" , line : 532usize , placeholders : & [] , regex : "^Disabling output while the process is running is not possible\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/symfony/process/Process.php" , line : 535usize , placeholders : & [] , regex : "^Output can not be disabled while an idle timeout is set\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/symfony/process/Process.php" , line : 553usize , placeholders : & [] , regex : "^Enabling output while the process is running is not possible\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/symfony/process/Process.php" , line : 819usize , placeholders : & [] , regex : "^This PHP has been compiled with \\-\\-enable\\-sigchild\\. Term signal can not be retrieved\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/symfony/process/Process.php" , line : 1065usize , placeholders : & [] , regex : "^Idle timeout can not be set while the output is disabled\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/symfony/process/Process.php" , line : 1085usize , placeholders : & [] , regex : "^TTY mode is not supported on Windows platform\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/symfony/process/Process.php" , line : 1089usize , placeholders : & [] , regex : "^TTY mode requires /dev/tty to be read/writable\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/symfony/process/Process.php" , line : 1222usize , placeholders : & [] , regex : "^Input can not be set while the process is running\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/symfony/process/Process.php" , line : 1244usize , placeholders : & [] , regex : "^Not inheriting environment variables is not supported\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/symfony/process/Process.php" , line : 1416usize , placeholders : & [] , regex : "^Output has been disabled\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/symfony/process/Process.php" , line : 1436usize , placeholders : & [] , regex : "^The timeout value must be a valid positive integer or float number\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/symfony/process/Process.php" , line : 1529usize , placeholders : & [] , regex : "^Can not send signal on a non running process\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/symfony/process/PhpProcess.php" , line : 79usize , placeholders : & [] , regex : "^Unable to find the PHP executable\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/symfony/process/Exception/ProcessFailedException.php" , line : 28usize , placeholders : & [] , regex : "^Expected a failed process, but the given process was successful\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/symfony/polyfill-intl-grapheme/Grapheme.php" , line : 74usize , placeholders : & [] , regex : "^grapheme_extract\\(\\): Argument \\#3 \\(\\$type\\) must be one of GRAPHEME_EXTR_COUNT, GRAPHEME_EXTR_MAXBYTES, or GRAPHEME_EXTR_MAXCHARS$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/symfony/console/Question/Question.php" , line : 105usize , placeholders : & [] , regex : "^A hidden question cannot use the autocompleter\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/symfony/console/Question/Question.php" , line : 192usize , placeholders : & [] , regex : "^A hidden question cannot use the autocompleter\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/symfony/console/Question/Question.php" , line : 234usize , placeholders : & [] , regex : "^Maximum number of attempts must be a positive value\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/symfony/console/Question/ChoiceQuestion.php" , line : 36usize , placeholders : & [] , regex : "^Choice question must have at least 1 choice available\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/symfony/console/Application.php" , line : 119usize , placeholders : & [] , regex : "^Signals are not supported\\. Make sure that the `pcntl` extension is installed and that \"pcntl_\\*\" functions are not disabled by your php\\.ini's \"disable_functions\" directive\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/symfony/console/Application.php" , line : 1003usize , placeholders : & [] , regex : "^Unable to subscribe to signal events\\. Make sure that the `pcntl` extension is installed and that \"pcntl_\\*\" functions are not disabled by your php\\.ini's \"disable_functions\" directive\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/symfony/console/Input/InputOption.php" , line : 69usize , placeholders : & [] , regex : "^An option name cannot be empty\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/symfony/console/Input/InputOption.php" , line : 85usize , placeholders : & [] , regex : "^An option shortcut cannot be empty\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/symfony/console/Input/InputOption.php" , line : 101usize , placeholders : & [] , regex : "^Impossible to have an option mode VALUE_IS_ARRAY if the option does not accept a value\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/symfony/console/Input/InputOption.php" , line : 104usize , placeholders : & [] , regex : "^Impossible to have an option mode VALUE_NEGATABLE if the option also accepts a value\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/symfony/console/Input/InputOption.php" , line : 181usize , placeholders : & [] , regex : "^Cannot set a default value when using InputOption::VALUE_NONE mode\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/symfony/console/Input/InputOption.php" , line : 188usize , placeholders : & [] , regex : "^A default value for an array option must be an array\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/symfony/console/Input/InputArgument.php" , line : 96usize , placeholders : & [] , regex : "^Cannot set a default value except for InputArgument::OPTIONAL mode\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/symfony/console/Input/InputArgument.php" , line : 103usize , placeholders : & [] , regex : "^A default value for an array argument must be an array\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/symfony/console/Style/SymfonyStyle.php" , line : 241usize , placeholders : & [] , regex : "^Value should be an array, string, or an instance of TableSeparator\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/symfony/console/Style/SymfonyStyle.php" , line : 432usize , placeholders : & [] , regex : "^The ProgressBar is not started\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/symfony/console/Command/LockableTrait.php" , line : 36usize , placeholders : & [] , regex : "^To enable the locking feature you must install the symfony/lock component\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/symfony/console/Command/LockableTrait.php" , line : 40usize , placeholders : & [] , regex : "^A lock is already in place\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/symfony/console/Command/Command.php" , line : 208usize , placeholders : & [] , regex : "^You must override the execute\\(\\) method in the concrete command class\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/symfony/console/Command/CompleteCommand.php" , line : 80usize , placeholders : & [] , regex : "^The \"\\-\\-shell\" option must be set\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Unknown , path : "/3rdparty/symfony/console/Command/CompleteCommand.php" , line : 102usize , placeholders : & [] , regex : "^  No command found, completing using the Application class\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Unknown , path : "/3rdparty/symfony/console/Command/CompleteCommand.php" , line : 110usize , placeholders : & [] , regex : "^  No command found, completing using the Application class\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Unknown , path : "/3rdparty/symfony/console/Command/CompleteCommand.php" , line : 138usize , placeholders : & [] , regex : "^<info>Suggestions:</>$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Unknown , path : "/3rdparty/symfony/console/Command/CompleteCommand.php" , line : 144usize , placeholders : & [] , regex : "^  <comment>No suggestions were provided</>$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/symfony/console/Command/CompleteCommand.php" , line : 168usize , placeholders : & [] , regex : "^The \"\\-\\-current\" option must be set and it must be an integer\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Unknown , path : "/3rdparty/symfony/console/CI/GithubActionReporter.php" , line : 62usize , placeholders : & [] , regex : "^error$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Unknown , path : "/3rdparty/symfony/console/CI/GithubActionReporter.php" , line : 72usize , placeholders : & [] , regex : "^warning$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Unknown , path : "/3rdparty/symfony/console/CI/GithubActionReporter.php" , line : 82usize , placeholders : & [] , regex : "^debug$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/symfony/console/Output/StreamOutput.php" , line : 45usize , placeholders : & [] , regex : "^The StreamOutput class needs a stream as its first argument\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/symfony/console/Completion/CompletionInput.php" , line : 224usize , placeholders : & [] , regex : "^Current index is invalid, it must be the number of input tokens or one more\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/symfony/console/Helper/TableCell.php" , line : 38usize , placeholders : & [] , regex : "^The style option must be an instance of \"TableCellStyle\"\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/symfony/console/Helper/Table.php" , line : 278usize , placeholders : & [] , regex : "^A row must be an array or a TableSeparator instance\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/symfony/console/Helper/ProgressBar.php" , line : 529usize , placeholders : & [] , regex : "^Unable to display the remaining time if the maximum number of steps is not set\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/symfony/console/Helper/ProgressBar.php" , line : 536usize , placeholders : & [] , regex : "^Unable to display the estimated time if the maximum number of steps is not set\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/symfony/console/Helper/ProcessHelper.php" , line : 38usize , placeholders : & [] , regex : "^The ProcessHelper cannot be run as the Process component is not installed\\. Try running \"compose require symfony/process\"\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/symfony/console/Helper/ProgressIndicator.php" , line : 68usize , placeholders : & [] , regex : "^Must have at least 2 indicator value characters\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/symfony/console/Helper/ProgressIndicator.php" , line : 93usize , placeholders : & [] , regex : "^Progress indicator already started\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/symfony/console/Helper/ProgressIndicator.php" , line : 111usize , placeholders : & [] , regex : "^Progress indicator has not yet been started\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/symfony/console/Helper/ProgressIndicator.php" , line : 138usize , placeholders : & [] , regex : "^Progress indicator has not yet been started\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/symfony/console/Helper/QuestionHelper.php" , line : 133usize , placeholders : & [] , regex : "^Aborted\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/symfony/console/Helper/QuestionHelper.php" , line : 276usize , placeholders : & [] , regex : "^Aborted\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/symfony/console/Helper/QuestionHelper.php" , line : 435usize , placeholders : & [] , regex : "^Unable to hide the response\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/symfony/console/Helper/QuestionHelper.php" , line : 445usize , placeholders : & [] , regex : "^Aborted\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/symfony/console/Helper/TableStyle.php" , line : 59usize , placeholders : & [] , regex : "^The padding char must not be empty\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/symfony/console/Helper/TableStyle.php" , line : 329usize , placeholders : & [] , regex : "^Invalid padding type\\. Expected one of \\(STR_PAD_LEFT, STR_PAD_RIGHT, STR_PAD_BOTH\\)\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/symfony/console/Formatter/OutputFormatterStyleStack.php" , line : 76usize , placeholders : & [] , regex : "^Incorrectly nested style tag found\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Critical , path : "/3rdparty/symfony/console/EventListener/ErrorListener.php" , line : 43usize , placeholders : & [] , regex : "^An error occurred while using the console\\. Message: \"\\{message\\}\"$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Critical , path : "/3rdparty/symfony/console/EventListener/ErrorListener.php" , line : 48usize , placeholders : & [] , regex : "^Error thrown while running command \"\\{command\\}\"\\. Message: \"\\{message\\}\"$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Debug , path : "/3rdparty/symfony/console/EventListener/ErrorListener.php" , line : 64usize , placeholders : & [] , regex : "^The console exited with code \"\\{code\\}\"$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Debug , path : "/3rdparty/symfony/console/EventListener/ErrorListener.php" , line : 69usize , placeholders : & [] , regex : "^Command \"\\{command\\}\" exited with code \"\\{code\\}\"$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/symfony/console/Tester/TesterTrait.php" , line : 45usize , placeholders : & [] , regex : "^Output not initialized, did you execute the command before requesting the display\\?$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/symfony/console/Tester/TesterTrait.php" , line : 69usize , placeholders : & [] , regex : "^The error output is not available when the tester is run without \"capture_stderr_separately\" option set\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/symfony/console/Tester/TesterTrait.php" , line : 113usize , placeholders : & [] , regex : "^Status code not initialized, did you execute the command before requesting the status code\\?$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/symfony/routing/Loader/AnnotationFileLoader.php" , line : 32usize , placeholders : & [] , regex : "^The Tokenizer extension is required for the routing annotation loaders\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/symfony/routing/Loader/XmlFileLoader.php" , line : 177usize , placeholders : & [] , regex : "^You cannot use both the attribute \"exclude\" and <exclude> tags at the same time\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/symfony/routing/Matcher/UrlMatcher.php" , line : 266usize , placeholders : & [] , regex : "^Unable to use expressions as the Symfony ExpressionLanguage component is not installed\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/symfony/routing/Matcher/Dumper/CompiledUrlMatcherDumper.php" , line : 450usize , placeholders : & [] , regex : "^Unable to use expressions as the Symfony ExpressionLanguage component is not installed\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/symfony/routing/Matcher/Dumper/CompiledUrlMatcherDumper.php" , line : 473usize , placeholders : & [] , regex : "^Symfony\\\\Component\\\\Routing\\\\Route cannot contain objects\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/symfony/routing/RouteCollectionBuilder.php" , line : 355usize , placeholders : & [] , regex : "^Cannot import other routing resources: you must pass a LoaderInterface when constructing RouteCollectionBuilder\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/symfony/mime/Message.php" , line : 78usize , placeholders : & [] , regex : "^An email must have a \"From\" or a \"Sender\" header\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/symfony/mime/Message.php" , line : 128usize , placeholders : & [] , regex : "^An email must have a \"To\", \"Cc\", or \"Bcc\" header\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/symfony/mime/Message.php" , line : 132usize , placeholders : & [] , regex : "^An email must have a \"From\" or a \"Sender\" header\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/symfony/mime/Message.php" , line : 145usize , placeholders : & [] , regex : "^An email must have a \"From\" or a \"Sender\" header\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/symfony/mime/Email.php" , line : 432usize , placeholders : & [] , regex : "^A message must have a text or an HTML part or attachments\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/symfony/mime/Encoder/Base64ContentEncoder.php" , line : 32usize , placeholders : & [] , regex : "^Unable to set the base64 content encoder to the filter\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/symfony/mime/Crypto/SMimeEncrypter.php" , line : 32usize , placeholders : & [] , regex : "^PHP extension \"openssl\" is required to use SMime\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/symfony/mime/Crypto/DkimSigner.php" , line : 45usize , placeholders : & [] , regex : "^PHP extension \"openssl\" is required to use DKIM\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/symfony/mime/Crypto/SMimeSigner.php" , line : 37usize , placeholders : & [] , regex : "^PHP extension \"openssl\" is required to use SMime\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/symfony/mime/MimeTypes.php" , line : 140usize , placeholders : & [] , regex : "^Unable to guess the MIME type as no guessers are available \\(have you enabled the php_fileinfo extension\\?\\)\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/symfony/http-foundation/ExpressionRequestMatcher.php" , line : 35usize , placeholders : & [] , regex : "^Unable to match the request as the expression language is not available\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/symfony/http-foundation/Cookie.php" , line : 100usize , placeholders : & [] , regex : "^The cookie name cannot be empty\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/symfony/http-foundation/Cookie.php" , line : 169usize , placeholders : & [] , regex : "^The cookie expiration time is not valid\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/symfony/http-foundation/Cookie.php" , line : 246usize , placeholders : & [] , regex : "^The \"sameSite\" parameter value is not valid\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/symfony/http-foundation/HeaderUtils.php" , line : 173usize , placeholders : & [] , regex : "^The filename fallback must only contain ASCII characters\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/symfony/http-foundation/HeaderUtils.php" , line : 178usize , placeholders : & [] , regex : "^The filename fallback cannot contain the \"%\" character\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/symfony/http-foundation/HeaderUtils.php" , line : 183usize , placeholders : & [] , regex : "^The filename and the fallback cannot contain the \"/\" and \"\\\\\" characters\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/symfony/http-foundation/StreamedResponse.php" , line : 106usize , placeholders : & [] , regex : "^The Response callback must not be null\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/symfony/http-foundation/StreamedResponse.php" , line : 124usize , placeholders : & [] , regex : "^The content cannot be set on a StreamedResponse instance\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/symfony/http-foundation/JsonResponse.php" , line : 124usize , placeholders : & [] , regex : "^The callback name is not valid\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/symfony/http-foundation/Session/Storage/MockArraySessionStorage.php" , line : 123usize , placeholders : & [] , regex : "^Cannot set session ID after the session has started\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/symfony/http-foundation/Session/Storage/MockArraySessionStorage.php" , line : 151usize , placeholders : & [] , regex : "^Trying to save a session that was not started yet or was already closed\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/symfony/http-foundation/Session/Storage/Proxy/AbstractProxy.php" , line : 89usize , placeholders : & [] , regex : "^Cannot change the ID of an active session\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/symfony/http-foundation/Session/Storage/Proxy/AbstractProxy.php" , line : 113usize , placeholders : & [] , regex : "^Cannot change the name of an active session\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/symfony/http-foundation/Session/Storage/PhpBridgeSessionStorage.php" , line : 29usize , placeholders : & [] , regex : "^PHP extension \"session\" is required\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/symfony/http-foundation/Session/Storage/Handler/PdoSessionHandler.php" , line : 493usize , placeholders : & [] , regex : "^URLs without scheme are not supported to configure the PdoSessionHandler\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/symfony/http-foundation/Session/Storage/Handler/PdoSessionHandler.php" , line : 687usize , placeholders : & [] , regex : "^Failed to read session: INSERT reported a duplicate id but next SELECT did not return any data\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/symfony/http-foundation/Session/Storage/Handler/PdoSessionHandler.php" , line : 774usize , placeholders : & [] , regex : "^SQLite does not support advisory locks\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/symfony/http-foundation/Session/Storage/Handler/MongoDbSessionHandler.php" , line : 74usize , placeholders : & [] , regex : "^You must provide the \"database\" and \"collection\" option for MongoDBSessionHandler\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/symfony/http-foundation/Session/Storage/NativeSessionStorage.php" , line : 103usize , placeholders : & [] , regex : "^PHP extension \"session\" is required\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/symfony/http-foundation/Session/Storage/NativeSessionStorage.php" , line : 141usize , placeholders : & [] , regex : "^Failed to start the session: already started by PHP\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/symfony/http-foundation/Session/Storage/NativeSessionStorage.php" , line : 156usize , placeholders : & [] , regex : "^Failed to start the session\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/symfony/http-foundation/Session/Storage/NativeSessionStorage.php" , line : 304usize , placeholders : & [] , regex : "^Cannot register a bag when the session is already started\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/symfony/http-foundation/Session/Storage/NativeSessionStorage.php" , line : 431usize , placeholders : & [] , regex : "^Must be instance of AbstractProxy; implement \\\\SessionHandlerInterface; or be null\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/symfony/http-foundation/Session/Storage/MockFileSessionStorage.php" , line : 90usize , placeholders : & [] , regex : "^Trying to save a session that was not started yet or was already closed\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/symfony/http-foundation/Request.php" , line : 739usize , placeholders : & [] , regex : "^Session has not been set\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/symfony/http-foundation/Request.php" , line : 1583usize , placeholders : & [] , regex : "^Request body is empty\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/symfony/http-foundation/Request.php" , line : 1589usize , placeholders : & [] , regex : "^Could not decode request body\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/symfony/http-foundation/Request.php" , line : 2033usize , placeholders : & [] , regex : "^The Request factory must return an instance of Symfony\\\\Component\\\\HttpFoundation\\\\Request\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/symfony/http-foundation/FileBag.php" , line : 49usize , placeholders : & [] , regex : "^An uploaded file must be an array or an instance of UploadedFile\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/symfony/http-foundation/IpUtils.php" , line : 135usize , placeholders : & [] , regex : "^Unable to check Ipv6\\. Check that PHP was not compiled with option \"disable\\-ipv6\"\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/symfony/http-foundation/RedirectResponse.php" , line : 86usize , placeholders : & [] , regex : "^Cannot redirect to an empty URL\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/symfony/http-foundation/BinaryFileResponse.php" , line : 98usize , placeholders : & [] , regex : "^File must be readable\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/symfony/http-foundation/BinaryFileResponse.php" , line : 329usize , placeholders : & [] , regex : "^The content cannot be set on a BinaryFileResponse instance\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/symfony/http-foundation/File/UploadedFile.php" , line : 137usize , placeholders : & [] , regex : "^You cannot guess the extension as the Mime component is not installed\\. Try running \"composer require symfony/mime\"\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/symfony/http-foundation/File/File.php" , line : 58usize , placeholders : & [] , regex : "^You cannot guess the extension as the Mime component is not installed\\. Try running \"composer require symfony/mime\"\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/symfony/http-foundation/File/File.php" , line : 78usize , placeholders : & [] , regex : "^You cannot guess the mime type as the Mime component is not installed\\. Try running \"composer require symfony/mime\"\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/symfony/dom-crawler/Form.php" , line : 409usize , placeholders : & [] , regex : "^The selected node does not have a form ancestor\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/symfony/dom-crawler/Crawler.php" , line : 309usize , placeholders : & [] , regex : "^Attaching DOM nodes from multiple documents in the same crawler is forbidden\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/symfony/dom-crawler/Crawler.php" , line : 425usize , placeholders : & [] , regex : "^The current node list is empty\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/symfony/dom-crawler/Crawler.php" , line : 453usize , placeholders : & [] , regex : "^The current node list is empty\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/symfony/dom-crawler/Crawler.php" , line : 480usize , placeholders : & [] , regex : "^The current node list is empty\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/symfony/dom-crawler/Crawler.php" , line : 496usize , placeholders : & [] , regex : "^The current node list is empty\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/symfony/dom-crawler/Crawler.php" , line : 526usize , placeholders : & [] , regex : "^The current node list is empty\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/symfony/dom-crawler/Crawler.php" , line : 552usize , placeholders : & [] , regex : "^The current node list is empty\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/symfony/dom-crawler/Crawler.php" , line : 577usize , placeholders : & [] , regex : "^The current node list is empty\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/symfony/dom-crawler/Crawler.php" , line : 595usize , placeholders : & [] , regex : "^The current node list is empty\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/symfony/dom-crawler/Crawler.php" , line : 620usize , placeholders : & [] , regex : "^The current node list is empty\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/symfony/dom-crawler/Crawler.php" , line : 656usize , placeholders : & [] , regex : "^The current node list is empty\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/symfony/dom-crawler/Crawler.php" , line : 677usize , placeholders : & [] , regex : "^The current node list is empty\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/symfony/dom-crawler/Crawler.php" , line : 701usize , placeholders : & [] , regex : "^Cannot evaluate the expression on an uninitialized crawler\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/symfony/dom-crawler/Crawler.php" , line : 837usize , placeholders : & [] , regex : "^The current node list is empty\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/symfony/dom-crawler/Crawler.php" , line : 880usize , placeholders : & [] , regex : "^The current node list is empty\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/symfony/dom-crawler/Crawler.php" , line : 921usize , placeholders : & [] , regex : "^The current node list is empty\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/symfony/dom-crawler/Crawler.php" , line : 1273usize , placeholders : & [] , regex : "^To filter with a CSS selector, install the CssSelector component \\(\"composer require symfony/css\\-selector\"\\)\\. Or use filterXpath instead\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/symfony/dom-crawler/Field/InputFormField.php" , line : 37usize , placeholders : & [] , regex : "^Checkboxes should be instances of ChoiceFormField\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/symfony/dom-crawler/Field/InputFormField.php" , line : 41usize , placeholders : & [] , regex : "^File inputs should be instances of FileFormField\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/symfony/dom-crawler/UriResolver.php" , line : 41usize , placeholders : & [] , regex : "^The URI is relative, so you must define its base URI passing an absolute URL\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/symfony/translation/Command/XliffLintCommand.php" , line : 95usize , placeholders : & [] , regex : "^Please provide a filename or pipe file content to STDIN\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/symfony/translation/Catalogue/AbstractOperation.php" , line : 67usize , placeholders : & [] , regex : "^Operated catalogues must belong to the same locale\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Debug , path : "/3rdparty/symfony/translation/LoggingTranslator.php" , line : 147usize , placeholders : & [] , regex : "^Translation use fallback catalogue\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Warn , path : "/3rdparty/symfony/translation/LoggingTranslator.php" , line : 149usize , placeholders : & [] , regex : "^Translation not found\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/symfony/translation/Dumper/YamlFileDumper.php" , line : 39usize , placeholders : & [] , regex : "^Dumping translations in the YAML format requires the Symfony Yaml component\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/symfony/translation/Dumper/FileDumper.php" , line : 57usize , placeholders : & [] , regex : "^The backup feature is no longer supported\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/symfony/translation/Dumper/FileDumper.php" , line : 67usize , placeholders : & [] , regex : "^The file dumper needs a path option\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/symfony/translation/Loader/MoFileLoader.php" , line : 51usize , placeholders : & [] , regex : "^MO stream content has an invalid format\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/symfony/translation/Loader/MoFileLoader.php" , line : 61usize , placeholders : & [] , regex : "^MO stream content has an invalid format\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/symfony/translation/Loader/YamlFileLoader.php" , line : 36usize , placeholders : & [] , regex : "^Loading translations from the YAML format requires the Symfony Yaml component\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/symfony/translation/Formatter/IntlFormatter.php" , line : 38usize , placeholders : & [] , regex : "^Cannot parse message translation: please install the \"intl\" PHP extension or the \"symfony/polyfill\\-intl\\-messageformatter\" package\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/symfony/polyfill-mbstring/Mbstring.php" , line : 557usize , placeholders : & [] , regex : "^Argument \\#2 \\(\\$length\\) must be greater than 0$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/symfony/polyfill-mbstring/Mbstring.php" , line : 610usize , placeholders : & [] , regex : "^Argument \\#1 \\(\\$substitute_character\\) must be \"none\", \"long\", \"entity\" or a valid codepoint$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/symfony/mailer/Envelope.php" , line : 39usize , placeholders : & [] , regex : "^Cannot send a RawMessage instance without an explicit Envelope\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/symfony/mailer/Envelope.php" , line : 69usize , placeholders : & [] , regex : "^An envelope must have at least one recipient\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/symfony/mailer/Transport/AbstractTransportFactory.php" , line : 46usize , placeholders : & [] , regex : "^User is not set\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/symfony/mailer/Transport/AbstractTransportFactory.php" , line : 56usize , placeholders : & [] , regex : "^Password is not set\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/symfony/mailer/Transport/NativeTransportFactory.php" , line : 37usize , placeholders : & [] , regex : "^sendmail_path is not configured in php\\.ini\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/symfony/mailer/Transport/NativeTransportFactory.php" , line : 46usize , placeholders : & [] , regex : "^smtp or smtp_port is not configured in php\\.ini\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/symfony/mailer/Transport/Smtp/Stream/AbstractStream.php" , line : 46usize , placeholders : & [] , regex : "^Unable to write bytes on the wire\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/symfony/mailer/Transport/Smtp/EsmtpTransport.php" , line : 116usize , placeholders : & [] , regex : "^Unable to connect with STARTTLS\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/symfony/mailer/Transport/Smtp/SmtpTransport.php" , line : 297usize , placeholders : & [] , regex : "^You must set the expected response code\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/symfony/mailer/DelayedEnvelope.php" , line : 96usize , placeholders : & [] , regex : "^Unable to determine the sender of the message\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Info , path : "/3rdparty/symfony/event-dispatcher/Debug/TraceableEventDispatcher.php" , line : 219usize , placeholders : & [] , regex : "^An exception was thrown while getting the uncalled listeners\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Debug , path : "/3rdparty/symfony/event-dispatcher/Debug/TraceableEventDispatcher.php" , line : 363usize , placeholders : & [] , regex : "^Notified event \"\\{event\\}\" to listener \"\\{listener\\}\"\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Debug , path : "/3rdparty/symfony/event-dispatcher/Debug/TraceableEventDispatcher.php" , line : 370usize , placeholders : & [] , regex : "^Listener \"\\{listener\\}\" was not called for event \"\\{event\\}\"\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Debug , path : "/3rdparty/symfony/event-dispatcher/Debug/TraceableEventDispatcher.php" , line : 375usize , placeholders : & [] , regex : "^Listener \"\\{listener\\}\" stopped propagation of the event \"\\{event\\}\"\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/symfony/event-dispatcher/ImmutableEventDispatcher.php" , line : 52usize , placeholders : & [] , regex : "^Unmodifiable event dispatchers must not be modified\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/symfony/event-dispatcher/ImmutableEventDispatcher.php" , line : 60usize , placeholders : & [] , regex : "^Unmodifiable event dispatchers must not be modified\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/symfony/event-dispatcher/ImmutableEventDispatcher.php" , line : 68usize , placeholders : & [] , regex : "^Unmodifiable event dispatchers must not be modified\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/symfony/event-dispatcher/ImmutableEventDispatcher.php" , line : 76usize , placeholders : & [] , regex : "^Unmodifiable event dispatchers must not be modified\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/symfony/polyfill-intl-normalizer/Normalizer.php" , line : 77usize , placeholders : & [] , regex : "^normalizer_normalize\\(\\): Argument \\#2 \\(\\$form\\) must be a a valid normalization form$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/symfony/polyfill-intl-idn/Idn.php" , line : 586usize , placeholders : & [] , regex : "^Invalid input$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/symfony/polyfill-intl-idn/Idn.php" , line : 602usize , placeholders : & [] , regex : "^Invalid input$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/symfony/polyfill-intl-idn/Idn.php" , line : 608usize , placeholders : & [] , regex : "^Invalid input$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/symfony/polyfill-intl-idn/Idn.php" , line : 612usize , placeholders : & [] , regex : "^Integer overflow$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/symfony/polyfill-intl-idn/Idn.php" , line : 632usize , placeholders : & [] , regex : "^Integer overflow$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/symfony/polyfill-intl-idn/Idn.php" , line : 642usize , placeholders : & [] , regex : "^Integer overflow$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/symfony/polyfill-intl-idn/Idn.php" , line : 697usize , placeholders : & [] , regex : "^Integer overflow$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/symfony/polyfill-intl-idn/Idn.php" , line : 705usize , placeholders : & [] , regex : "^Integer overflow$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/symfony/string/CodePointString.php" , line : 30usize , placeholders : & [] , regex : "^Invalid UTF\\-8 string\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/symfony/string/CodePointString.php" , line : 42usize , placeholders : & [] , regex : "^Invalid UTF\\-8 string\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/symfony/string/CodePointString.php" , line : 51usize , placeholders : & [] , regex : "^The chunk length must be greater than zero\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/symfony/string/CodePointString.php" , line : 162usize , placeholders : & [] , regex : "^Invalid UTF\\-8 string\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/symfony/string/CodePointString.php" , line : 177usize , placeholders : & [] , regex : "^Invalid UTF\\-8 string\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/symfony/string/CodePointString.php" , line : 200usize , placeholders : & [] , regex : "^Invalid UTF\\-8 string\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/symfony/string/CodePointString.php" , line : 214usize , placeholders : & [] , regex : "^Split limit must be a positive integer\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/symfony/string/CodePointString.php" , line : 218usize , placeholders : & [] , regex : "^Split delimiter is empty\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/symfony/string/CodePointString.php" , line : 226usize , placeholders : & [] , regex : "^Split delimiter is not a valid UTF\\-8 string\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/symfony/string/AbstractUnicodeString.php" , line : 211usize , placeholders : & [] , regex : "^Invalid UTF\\-8 string\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/symfony/string/AbstractUnicodeString.php" , line : 245usize , placeholders : & [] , regex : "^Matching failed with unknown error code\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/symfony/string/AbstractUnicodeString.php" , line : 257usize , placeholders : & [] , regex : "^Unsupported normalization form\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/symfony/string/AbstractUnicodeString.php" , line : 269usize , placeholders : & [] , regex : "^Invalid UTF\\-8 string\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/symfony/string/AbstractUnicodeString.php" , line : 281usize , placeholders : & [] , regex : "^Invalid UTF\\-8 string\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/symfony/string/AbstractUnicodeString.php" , line : 293usize , placeholders : & [] , regex : "^Invalid UTF\\-8 string\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/symfony/string/AbstractUnicodeString.php" , line : 314usize , placeholders : & [] , regex : "^Replace callback must return a valid UTF\\-8 string\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/symfony/string/AbstractUnicodeString.php" , line : 320usize , placeholders : & [] , regex : "^Invalid UTF\\-8 string\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/symfony/string/AbstractUnicodeString.php" , line : 337usize , placeholders : & [] , regex : "^Matching failed with unknown error code\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/symfony/string/AbstractUnicodeString.php" , line : 381usize , placeholders : & [] , regex : "^Invalid UTF\\-8 chars\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/symfony/string/AbstractUnicodeString.php" , line : 394usize , placeholders : & [] , regex : "^Invalid UTF\\-8 chars\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/symfony/string/AbstractUnicodeString.php" , line : 427usize , placeholders : & [] , regex : "^Invalid UTF\\-8 chars\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/symfony/string/AbstractUnicodeString.php" , line : 529usize , placeholders : & [] , regex : "^Invalid padding type\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/symfony/string/AbstractString.php" , line : 444usize , placeholders : & [] , regex : "^Split behavior when \\$flags is null must be implemented by child classes\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/symfony/string/AbstractString.php" , line : 463usize , placeholders : & [] , regex : "^Splitting failed with unknown error code\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/symfony/string/UnicodeString.php" , line : 40usize , placeholders : & [] , regex : "^Invalid UTF\\-8 string\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/symfony/string/UnicodeString.php" , line : 51usize , placeholders : & [] , regex : "^Invalid UTF\\-8 string\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/symfony/string/UnicodeString.php" , line : 60usize , placeholders : & [] , regex : "^The chunk length must be greater than zero\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/symfony/string/UnicodeString.php" , line : 199usize , placeholders : & [] , regex : "^Unsupported normalization form\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/symfony/string/UnicodeString.php" , line : 215usize , placeholders : & [] , regex : "^Invalid UTF\\-8 string\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/symfony/string/UnicodeString.php" , line : 241usize , placeholders : & [] , regex : "^Invalid UTF\\-8 string\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/symfony/string/UnicodeString.php" , line : 275usize , placeholders : & [] , regex : "^Invalid UTF\\-8 string\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/symfony/string/UnicodeString.php" , line : 284usize , placeholders : & [] , regex : "^Split limit must be a positive integer\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/symfony/string/UnicodeString.php" , line : 288usize , placeholders : & [] , regex : "^Split delimiter is empty\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/symfony/string/UnicodeString.php" , line : 298usize , placeholders : & [] , regex : "^Split delimiter is not a valid UTF\\-8 string\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/symfony/string/Slugger/AsciiSlugger.php" , line : 19usize , placeholders : & [] , regex : "^You cannot use the \"Symfony\\\\Component\\\\String\\\\Slugger\\\\AsciiSlugger\" as the \"symfony/translation\\-contracts\" package is not installed\\. Try running \"composer require symfony/translation\\-contracts\"\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/symfony/string/ByteString.php" , line : 55usize , placeholders : & [] , regex : "^The length of the alphabet must in the \\[2\\^1, 2\\^56\\] range\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/symfony/string/ByteString.php" , line : 117usize , placeholders : & [] , regex : "^The chunk length must be greater than zero\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/symfony/string/ByteString.php" , line : 251usize , placeholders : & [] , regex : "^Matching failed with unknown error code\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/symfony/string/ByteString.php" , line : 323usize , placeholders : & [] , regex : "^Matching failed with unknown error code\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/symfony/string/ByteString.php" , line : 370usize , placeholders : & [] , regex : "^Split limit must be a positive integer\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/symfony/string/ByteString.php" , line : 374usize , placeholders : & [] , regex : "^Split delimiter is empty\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/symfony/css-selector/Parser/TokenStream.php" , line : 91usize , placeholders : & [] , regex : "^Unexpected token stream end\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/symfony/css-selector/XPath/Translator.php" , line : 100usize , placeholders : & [] , regex : "^Pseudo\\-elements are not supported\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/symfony/css-selector/XPath/Extension/PseudoClassExtension.php" , line : 73usize , placeholders : & [] , regex : "^\"\\*:first\\-of\\-type\" is not implemented\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/symfony/css-selector/XPath/Extension/PseudoClassExtension.php" , line : 87usize , placeholders : & [] , regex : "^\"\\*:last\\-of\\-type\" is not implemented\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/symfony/css-selector/XPath/Extension/FunctionExtension.php" , line : 122usize , placeholders : & [] , regex : "^\"\\*:nth\\-of\\-type\\(\\)\" is not implemented\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/pimple/pimple/src/Pimple/Container.php" , line : 174usize , placeholders : & [] , regex : "^Service definition is not a Closure or invokable object\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/pimple/pimple/src/Pimple/Container.php" , line : 196usize , placeholders : & [] , regex : "^Callable is not a Closure or invokable object\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/pimple/pimple/src/Pimple/Container.php" , line : 261usize , placeholders : & [] , regex : "^Extension service definition is not a Closure or invokable object\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/christophwurst/id3parser/src/getID3/getid3.php" , line : 94usize , placeholders : & [] , regex : "^Remote files are not supported \\- please copy the file locally first$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/christophwurst/id3parser/src/getID3/getid3_lib.php" , line : 278usize , placeholders : & [] , regex : "^ERROR: self::BigEndian2String\\(\\) does not support negative numbers$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/mtdowling/jmespath.php/src/FnDispatcher.php" , line : 88usize , placeholders : & [] , regex : "^not_null\\(\\) expects 1 or more arguments, 0 were provided$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/mtdowling/jmespath.php/src/FnDispatcher.php" , line : 167usize , placeholders : & [] , regex : "^Cannot reverse provided argument$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/mtdowling/jmespath.php/src/FnDispatcher.php" , line : 259usize , placeholders : & [] , regex : "^merge\\(\\) expects 1 or more arguments, 0 were provided$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/mtdowling/jmespath.php/src/FnDispatcher.php" , line : 405usize , placeholders : & ["$name"] , regex : "^Call to undefined function (.*)$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/mtdowling/jmespath.php/src/TreeInterpreter.php" , line : 214usize , placeholders : & ["$node['type']"] , regex : "^Unknown node type: (.*)$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/mtdowling/jmespath.php/src/TreeInterpreter.php" , line : 232usize , placeholders : & ["$cmp"] , regex : "^Invalid comparison: (.*)$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/mtdowling/jmespath.php/src/Parser.php" , line : 517usize , placeholders : & ["$method"] , regex : "^Call to undefined method (.*)$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/mtdowling/jmespath.php/src/Utils.php" , line : 196usize , placeholders : & [] , regex : "^Expects string or array$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/mtdowling/jmespath.php/src/Utils.php" , line : 221usize , placeholders : & [] , regex : "^step cannot be 0$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/mtdowling/jmespath.php/src/CompilerRuntime.php" , line : 33usize , placeholders : & ["$dir"] , regex : "^Unable to create cache directory: (.*)$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/stecman/symfony-console-completion/src/CompletionHandler.php" , line : 106usize , placeholders : & [] , regex : "^A CompletionContext must be set before requesting completion\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/stecman/symfony-console-completion/src/EnvironmentCompletionContext.php" , line : 41usize , placeholders : & [] , regex : "^Failed to read word breaks from environment; Environment var CMDLINE_WORDBREAKS not set$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/stecman/symfony-console-completion/src/CompletionCommand.php" , line : 208usize , placeholders : & [] , regex : "^Could not read SHELL environment variable\\. Please specify your shell type using the \\-\\-shell\\-type option\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/egulias/email-validator/src/Validation/DNSGetRecordWrapper.php" , line : 16usize , placeholders : & ["$errorMessage"] , regex : "^Unable to get DNS record for the host: (.*)$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/punic/punic/code/Plural.php" , line : 88usize , placeholders : & [] , regex : "^Bad formula!$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/ramsey/collection/src/AbstractCollection.php" , line : 129usize , placeholders : & [] , regex : "^Can't determine first item\\. Collection is empty$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/ramsey/collection/src/AbstractCollection.php" , line : 146usize , placeholders : & [] , regex : "^Can't determine last item\\. Collection is empty$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/ramsey/collection/src/DoubleEndedQueue.php" , line : 117usize , placeholders : & [] , regex : "^Can't return element from Queue\\. Queue is empty\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/ramsey/collection/src/DoubleEndedQueue.php" , line : 162usize , placeholders : & [] , regex : "^Can't return element from Queue\\. Queue is empty\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/ramsey/collection/src/Tool/ValueExtractorTrait.php" , line : 43usize , placeholders : & [] , regex : "^Unable to extract a value from a non\\-object$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/ramsey/collection/src/Queue.php" , line : 103usize , placeholders : & [] , regex : "^Can't return element from Queue\\. Queue is empty\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/ramsey/collection/src/Queue.php" , line : 159usize , placeholders : & [] , regex : "^Can't return element from Queue\\. Queue is empty\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/ramsey/uuid/src/Codec/OrderedTimeCodec.php" , line : 64usize , placeholders : & [] , regex : "^Expected RFC 4122 version 1 \\(time\\-based\\) UUID$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/ramsey/uuid/src/Codec/OrderedTimeCodec.php" , line : 88usize , placeholders : & [] , regex : "^\\$bytes string should contain 16 characters\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/ramsey/uuid/src/Codec/StringCodec.php" , line : 95usize , placeholders : & [] , regex : "^\\$bytes string should contain 16 characters\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/ramsey/uuid/src/Generator/DefaultTimeGenerator.php" , line : 142usize , placeholders : & [] , regex : "^Invalid node value$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/ramsey/uuid/src/Generator/DceSecurityGenerator.php" , line : 88usize , placeholders : & [] , regex : "^Local domain must be a valid DCE Security domain$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/ramsey/uuid/src/Generator/DceSecurityGenerator.php" , line : 94usize , placeholders : & [] , regex : "^Local identifier out of bounds; it must be a value between 0 and 4294967295$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/ramsey/uuid/src/Generator/DceSecurityGenerator.php" , line : 100usize , placeholders : & [] , regex : "^Clock sequence out of bounds; it must be a value between 0 and 63$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/ramsey/uuid/src/Generator/DceSecurityGenerator.php" , line : 108usize , placeholders : & [] , regex : "^A local identifier must be provided for the org domain$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/ramsey/uuid/src/Generator/DceSecurityGenerator.php" , line : 136usize , placeholders : & [] , regex : "^Local identifier out of bounds; it must be a value between 0 and 4294967295$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/ramsey/uuid/src/DeprecatedUuidMethodsTrait.php" , line : 141usize , placeholders : & [] , regex : "^Not a time\\-based UUID$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/ramsey/uuid/src/DeprecatedUuidMethodsTrait.php" , line : 318usize , placeholders : & [] , regex : "^Not a time\\-based UUID$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/ramsey/uuid/src/DeprecatedUuidMethodsTrait.php" , line : 332usize , placeholders : & [] , regex : "^Not a time\\-based UUID$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/ramsey/uuid/src/Builder/FallbackBuilder.php" , line : 70usize , placeholders : & [] , regex : "^Could not find a suitable builder for the provided codec and fields$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/ramsey/uuid/src/Rfc4122/VariantTrait.php" , line : 58usize , placeholders : & [] , regex : "^Invalid number of bytes$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/ramsey/uuid/src/Rfc4122/Fields.php" , line : 73usize , placeholders : & [] , regex : "^The byte string received does not conform to the RFC 4122 variant$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/ramsey/uuid/src/Rfc4122/Fields.php" , line : 79usize , placeholders : & [] , regex : "^The byte string received does not contain a valid RFC 4122 version$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/ramsey/uuid/src/Provider/Node/SystemNodeProvider.php" , line : 63usize , placeholders : & [] , regex : "^Unable to fetch a node for this system$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/ramsey/uuid/src/Provider/Node/StaticNodeProvider.php" , line : 47usize , placeholders : & [] , regex : "^Static node value cannot be greater than 12 hexadecimal characters$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/ramsey/uuid/src/Provider/Node/FallbackNodeProvider.php" , line : 56usize , placeholders : & [] , regex : "^Unable to find a suitable node provider$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/ramsey/uuid/src/Guid/Fields.php" , line : 84usize , placeholders : & [] , regex : "^The byte string received does not contain a valid version$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/ramsey/uuid/src/Type/Hexadecimal.php" , line : 53usize , placeholders : & [] , regex : "^Value must be a hexadecimal number$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/ramsey/uuid/src/Type/Time.php" , line : 105usize , placeholders : & [] , regex : "^Attempted to unserialize an invalid value$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/ramsey/uuid/src/Lazy/LazyUuidFromString.php" , line : 503usize , placeholders : & [] , regex : "^Not a time\\-based UUID$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/sabre/dav/lib/CalDAV/SharingPlugin.php" , line : 76usize , placeholders : & [] , regex : "^The generic \"sharing\" plugin must be loaded before the caldav sharing plugin\\. Call \\$server\\->addPlugin\\(new \\\\Sabre\\\\DAV\\\\Sharing\\\\Plugin\\(\\)\\); before this one\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/sabre/dav/lib/CalDAV/CalendarHome.php" , line : 104usize , placeholders : & [] , regex : "^Creating new files in this collection is not supported$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/sabre/dav/lib/CalDAV/CalendarHome.php" , line : 116usize , placeholders : & [] , regex : "^Creating new collections in this collection is not supported$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/sabre/dav/lib/CalDAV/CalendarHome.php" , line : 247usize , placeholders : & [] , regex : "^This backend does not support subscriptions$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/sabre/dav/lib/CalDAV/CalendarHome.php" , line : 253usize , placeholders : & [] , regex : "^You can only create calendars and subscriptions in this collection$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/sabre/dav/lib/CalDAV/CalendarHome.php" , line : 327usize , placeholders : & [] , regex : "^Sharing support is not implemented by this backend\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/sabre/dav/lib/CalDAV/Calendar.php" , line : 108usize , placeholders : & [] , regex : "^Calendar object not found$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/sabre/dav/lib/CalDAV/Calendar.php" , line : 180usize , placeholders : & [] , regex : "^Creating collections in calendar objects is not allowed$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/sabre/dav/lib/CalDAV/Calendar.php" , line : 218usize , placeholders : & [] , regex : "^Renaming calendars is not yet supported$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/sabre/dav/lib/CalDAV/Principal/ProxyWrite.php" , line : 73usize , placeholders : & [] , regex : "^Permission denied to delete node$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/sabre/dav/lib/CalDAV/Principal/ProxyWrite.php" , line : 85usize , placeholders : & [] , regex : "^Permission denied to rename file$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/sabre/dav/lib/CalDAV/Principal/User.php" , line : 45usize , placeholders : & [] , regex : "^Permission denied to create directory$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/sabre/dav/lib/CalDAV/Principal/ProxyRead.php" , line : 73usize , placeholders : & [] , regex : "^Permission denied to delete node$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/sabre/dav/lib/CalDAV/Principal/ProxyRead.php" , line : 85usize , placeholders : & [] , regex : "^Permission denied to rename file$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/sabre/dav/lib/CalDAV/Schedule/SchedulingObject.php" , line : 38usize , placeholders : & [] , regex : "^The objectData argument must contain an 'uri' property$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/sabre/dav/lib/CalDAV/Schedule/SchedulingObject.php" , line : 67usize , placeholders : & [] , regex : "^Updating scheduling objects is not supported$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/sabre/dav/lib/CalDAV/Schedule/Plugin.php" , line : 709usize , placeholders : & [] , regex : "^We expected at least one VTODO, VJOURNAL, VFREEBUSY or VEVENT component$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/sabre/dav/lib/CalDAV/Schedule/Plugin.php" , line : 715usize , placeholders : & [] , regex : "^A METHOD property must be specified in iTIP messages$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/sabre/dav/lib/CalDAV/Schedule/Plugin.php" , line : 732usize , placeholders : & [] , regex : "^We only support VFREEBUSY \\(REQUEST\\) on this endpoint$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/sabre/dav/lib/CalDAV/Schedule/Plugin.php" , line : 756usize , placeholders : & [] , regex : "^The organizer in the request did not match any of the addresses for the owner of this inbox$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/sabre/dav/lib/CalDAV/Schedule/Plugin.php" , line : 760usize , placeholders : & [] , regex : "^You must at least specify 1 attendee$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/sabre/dav/lib/CalDAV/Schedule/Plugin.php" , line : 769usize , placeholders : & [] , regex : "^DTSTART and DTEND must both be specified$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/sabre/dav/lib/CalDAV/Backend/PDO.php" , line : 296usize , placeholders : & [] , regex : "^The value passed to \\$calendarId is expected to be an array with a calendarId and an instanceId$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/sabre/dav/lib/CalDAV/Backend/PDO.php" , line : 340usize , placeholders : & [] , regex : "^The value passed to \\$calendarId is expected to be an array with a calendarId and an instanceId$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/sabre/dav/lib/CalDAV/Backend/PDO.php" , line : 409usize , placeholders : & [] , regex : "^The value passed to \\$calendarId is expected to be an array with a calendarId and an instanceId$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/sabre/dav/lib/CalDAV/Backend/PDO.php" , line : 451usize , placeholders : & [] , regex : "^The value passed to \\$calendarId is expected to be an array with a calendarId and an instanceId$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/sabre/dav/lib/CalDAV/Backend/PDO.php" , line : 489usize , placeholders : & [] , regex : "^The value passed to \\$calendarId is expected to be an array with a calendarId and an instanceId$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/sabre/dav/lib/CalDAV/Backend/PDO.php" , line : 541usize , placeholders : & [] , regex : "^The value passed to \\$calendarId is expected to be an array with a calendarId and an instanceId$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/sabre/dav/lib/CalDAV/Backend/PDO.php" , line : 587usize , placeholders : & [] , regex : "^The value passed to \\$calendarId is expected to be an array with a calendarId and an instanceId$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/sabre/dav/lib/CalDAV/Backend/PDO.php" , line : 633usize , placeholders : & [] , regex : "^Calendar objects must have a VJOURNAL, VEVENT or VTODO component$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/sabre/dav/lib/CalDAV/Backend/PDO.php" , line : 700usize , placeholders : & [] , regex : "^The value passed to \\$calendarId is expected to be an array with a calendarId and an instanceId$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/sabre/dav/lib/CalDAV/Backend/PDO.php" , line : 765usize , placeholders : & [] , regex : "^The value passed to \\$calendarId is expected to be an array with a calendarId and an instanceId$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/sabre/dav/lib/CalDAV/Backend/PDO.php" , line : 947usize , placeholders : & [] , regex : "^The value passed to \\$calendarId is expected to be an array with a calendarId and an instanceId$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/sabre/dav/lib/CalDAV/Backend/PDO.php" , line : 1144usize , placeholders : & [] , regex : "^The \\{http://calendarserver\\.org/ns/\\}source property is required when creating subscriptions$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/sabre/dav/lib/CalDAV/Backend/PDO.php" , line : 1332usize , placeholders : & [] , regex : "^The value passed to \\$calendarId is expected to be an array with a calendarId and an instanceId$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/sabre/dav/lib/CalDAV/Backend/PDO.php" , line : 1441usize , placeholders : & [] , regex : "^The value passed to getInvites\\(\\) is expected to be an array with a calendarId and an instanceId$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/sabre/dav/lib/CalDAV/Backend/PDO.php" , line : 1485usize , placeholders : & [] , regex : "^Not implemented$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/sabre/dav/lib/CalDAV/CalendarObject.php" , line : 58usize , placeholders : & [] , regex : "^The objectData argument must contain an 'uri' property$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/sabre/dav/lib/CalDAV/Xml/Property/SupportedCalendarComponentSet.php" , line : 113usize , placeholders : & [] , regex : "^supported\\-calendar\\-component\\-set must have at least one CALDAV:comp element$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/sabre/dav/lib/CalDAV/Xml/Property/ScheduleCalendarTransp.php" , line : 49usize , placeholders : & [] , regex : "^The value must either be specified as \"transparent\" or \"opaque\"$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/sabre/dav/lib/CalDAV/Xml/Request/CalendarQueryReport.php" , line : 117usize , placeholders : & [] , regex : "^Only one top\\-level comp\\-filter may be defined$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/sabre/dav/lib/CalDAV/Xml/Request/InviteReply.php" , line : 140usize , placeholders : & [] , regex : "^The \\{http://calendarserver\\.org/ns/\\}hosturl/\\{DAV:\\}href element must exist$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/sabre/dav/lib/CalDAV/Xml/Request/FreeBusyQueryReport.php" , line : 76usize , placeholders : & [] , regex : "^The freebusy report must have a time\\-range element$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/sabre/dav/lib/CalDAV/Xml/Filter/CalendarData.php" , line : 69usize , placeholders : & [] , regex : "^The \"start\" and \"end\" attributes are required when expanding calendar\\-data$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/sabre/dav/lib/CalDAV/Xml/Filter/CalendarData.php" , line : 72usize , placeholders : & [] , regex : "^The end\\-date must be larger than the start\\-date when expanding calendar\\-data$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/sabre/dav/lib/CalDAV/Xml/Filter/PropFilter.php" , line : 79usize , placeholders : & [] , regex : "^The end\\-date must be larger than the start\\-date$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/sabre/dav/lib/CalDAV/Xml/Filter/CompFilter.php" , line : 78usize , placeholders : & [] , regex : "^You cannot add time\\-range filters on the VCALENDAR component$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/sabre/dav/lib/CalDAV/Xml/Filter/CompFilter.php" , line : 85usize , placeholders : & [] , regex : "^The end\\-date must be larger than the start\\-date$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/sabre/dav/lib/CalDAV/ICSExportPlugin.php" , line : 114usize , placeholders : & [] , regex : "^The start= parameter must contain a unix timestamp$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/sabre/dav/lib/CalDAV/ICSExportPlugin.php" , line : 120usize , placeholders : & [] , regex : "^The end= parameter must contain a unix timestamp$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/sabre/dav/lib/CalDAV/ICSExportPlugin.php" , line : 126usize , placeholders : & [] , regex : "^If you'd like to expand recurrences, you must specify both a start= and end= parameter\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/sabre/dav/lib/CalDAV/Plugin.php" , line : 591usize , placeholders : & [] , regex : "^A calendar\\-query REPORT on a calendar with a Depth: 0 is undefined\\. Set Depth to 1$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/sabre/dav/lib/CalDAV/Plugin.php" , line : 648usize , placeholders : & [] , regex : "^The free\\-busy\\-query REPORT is only implemented on calendars$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/sabre/dav/lib/CalDAV/Plugin.php" , line : 810usize , placeholders : & [] , regex : "^This collection can only support iCalendar objects\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/sabre/dav/lib/DAV/FS/Node.php" , line : 75usize , placeholders : & [] , regex : "^This node cannot be renamed$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/sabre/dav/lib/DAV/Auth/Backend/AbstractDigest.php" , line : 116usize , placeholders : & [] , regex : "^The returned value from getDigestHash must be a string or null$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/sabre/dav/lib/DAV/Auth/Backend/File.php" , line : 51usize , placeholders : & [] , regex : "^Malformed htdigest file\\. Every line should contain 2 colons$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/sabre/dav/lib/DAV/Auth/Backend/File.php" , line : 56usize , placeholders : & [] , regex : "^Malformed htdigest file\\. Invalid md5 hash$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/sabre/dav/lib/DAV/Auth/Plugin.php" , line : 175usize , placeholders : & [] , regex : "^No authentication backends were configured on this server\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/sabre/dav/lib/DAV/Auth/Plugin.php" , line : 185usize , placeholders : & [] , regex : "^The authentication backend did not return a correct value from the check\\(\\) method\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/sabre/dav/lib/DAV/Browser/Plugin.php" , line : 553usize , placeholders : & [] , regex : "^Path does not exist, or escaping from the base path was detected$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/sabre/dav/lib/DAV/Browser/Plugin.php" , line : 559usize , placeholders : & [] , regex : "^Path does not exist, or escaping from the base path was detected$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/sabre/dav/lib/DAV/TemporaryFileFilterPlugin.php" , line : 220usize , placeholders : & [] , regex : "^The resource already exists, and an If\\-None\\-Match header was supplied$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/sabre/dav/lib/DAV/CorePlugin.php" , line : 442usize , placeholders : & [] , regex : "^Content\\-Range on PUT requests are forbidden\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/sabre/dav/lib/DAV/CorePlugin.php" , line : 471usize , placeholders : & [] , regex : "^This server is not compatible with OS/X finder\\. Consider using a different WebDAV client or webserver\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/sabre/dav/lib/DAV/CorePlugin.php" , line : 490usize , placeholders : & [] , regex : "^PUT is not allowed on non\\-files\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/sabre/dav/lib/DAV/CorePlugin.php" , line : 537usize , placeholders : & [] , regex : "^The request body for the MKCOL request must have an xml Content\\-Type$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/sabre/dav/lib/DAV/CorePlugin.php" , line : 549usize , placeholders : & [] , regex : "^The mkcol request must include a \\{DAV:\\}resourcetype property$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/sabre/dav/lib/DAV/FSExt/Directory.php" , line : 48usize , placeholders : & [] , regex : "^Permission denied to \\. and \\.\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/sabre/dav/lib/DAV/FSExt/Directory.php" , line : 70usize , placeholders : & [] , regex : "^Permission denied to \\. and \\.\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/sabre/dav/lib/DAV/FSExt/Directory.php" , line : 94usize , placeholders : & [] , regex : "^File could not be located$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/sabre/dav/lib/DAV/FSExt/Directory.php" , line : 97usize , placeholders : & [] , regex : "^Permission denied to \\. and \\.\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/sabre/dav/lib/DAV/FSExt/Directory.php" , line : 116usize , placeholders : & [] , regex : "^Permission denied to \\. and \\.\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/sabre/dav/lib/DAV/Collection.php" , line : 104usize , placeholders : & [] , regex : "^Permission denied to create directory$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/sabre/dav/lib/DAV/Server.php" , line : 227usize , placeholders : & [] , regex : "^Invalid argument passed to constructor\\. Argument must either be an instance of Sabre\\\\DAV\\\\Tree, Sabre\\\\DAV\\\\INode, an array or null$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/sabre/dav/lib/DAV/Server.php" , line : 487usize , placeholders : & [] , regex : "^No subsystem set a valid HTTP status code\\. Something must have interrupted the request without providing further detail\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/sabre/dav/lib/DAV/Server.php" , line : 725usize , placeholders : & [] , regex : "^The destination header was not supplied$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/sabre/dav/lib/DAV/Server.php" , line : 739usize , placeholders : & [] , regex : "^The HTTP Overwrite header should be either T or F$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/sabre/dav/lib/DAV/Server.php" , line : 746usize , placeholders : & [] , regex : "^The destination node is not a collection$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/sabre/dav/lib/DAV/Server.php" , line : 750usize , placeholders : & [] , regex : "^The destination node is not found$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/sabre/dav/lib/DAV/Server.php" , line : 759usize , placeholders : & [] , regex : "^The destination node already exists, and the overwrite header is set to false$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/sabre/dav/lib/DAV/Server.php" , line : 768usize , placeholders : & [] , regex : "^Source and destination uri are identical\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/sabre/dav/lib/DAV/Server.php" , line : 771usize , placeholders : & [] , regex : "^The destination may not be part of the same subtree as the source path\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/sabre/dav/lib/DAV/Server.php" , line : 1081usize , placeholders : & [] , regex : "^Files cannot be created in non\\-existent collections$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/sabre/dav/lib/DAV/Server.php" , line : 1085usize , placeholders : & [] , regex : "^Files can only be created as children of collections$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/sabre/dav/lib/DAV/Server.php" , line : 1171usize , placeholders : & [] , regex : "^Parent node does not exist$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/sabre/dav/lib/DAV/Server.php" , line : 1176usize , placeholders : & [] , regex : "^Parent node is not a collection$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/sabre/dav/lib/DAV/Server.php" , line : 1184usize , placeholders : & [] , regex : "^The resource you tried to create already exists$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/sabre/dav/lib/DAV/Server.php" , line : 1207usize , placeholders : & [] , regex : "^The \\{DAV:\\}resourcetype you specified is not supported here\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/sabre/dav/lib/DAV/Server.php" , line : 1304usize , placeholders : & [] , regex : "^An If\\-Match header was specified and the resource did not exist$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/sabre/dav/lib/DAV/Server.php" , line : 1331usize , placeholders : & [] , regex : "^An If\\-Match header was specified, but none of the specified ETags matched\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/sabre/dav/lib/DAV/Server.php" , line : 1377usize , placeholders : & [] , regex : "^An If\\-None\\-Match header was specified, but the ETag matched \\(or \\* was specified\\)\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/sabre/dav/lib/DAV/Server.php" , line : 1423usize , placeholders : & [] , regex : "^An If\\-Unmodified\\-Since header was specified, but the entity has been changed since the specified date\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/sabre/dav/lib/DAV/PropPatch.php" , line : 270usize , placeholders : & [] , regex : "^A callback sent to handle\\(\\) did not return an int or a bool$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/sabre/dav/lib/DAV/PropPatch.php" , line : 314usize , placeholders : & [] , regex : "^A callback sent to handle\\(\\) did not return an array or a bool$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/sabre/dav/lib/DAV/PartialUpdate/Plugin.php" , line : 115usize , placeholders : & [] , regex : "^The target resource does not support the PATCH method\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/sabre/dav/lib/DAV/PartialUpdate/Plugin.php" , line : 121usize , placeholders : & [] , regex : "^No valid \"X\\-Update\\-Range\" found in the headers$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/sabre/dav/lib/DAV/PartialUpdate/Plugin.php" , line : 134usize , placeholders : & [] , regex : "^A Content\\-Length header is required$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/sabre/dav/lib/DAV/SimpleCollection.php" , line : 53usize , placeholders : & [] , regex : "^Children must be specified as strings, arrays or instances of Sabre\\\\DAV\\\\INode$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/sabre/dav/lib/DAV/File.php" , line : 42usize , placeholders : & [] , regex : "^Permission denied to change data$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/sabre/dav/lib/DAV/File.php" , line : 54usize , placeholders : & [] , regex : "^Permission denied to read this file$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/sabre/dav/lib/DAV/Locks/Plugin.php" , line : 202usize , placeholders : & [] , regex : "^An xml body is required for lock requests$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/sabre/dav/lib/DAV/Locks/Plugin.php" , line : 260usize , placeholders : & [] , regex : "^No lock token was supplied$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/sabre/dav/lib/DAV/Locks/Plugin.php" , line : 360usize , placeholders : & [] , regex : "^Invalid HTTP timeout header$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/sabre/dav/lib/DAV/Client.php" , line : 119usize , placeholders : & [] , regex : "^A baseUri must be provided$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/sabre/dav/lib/DAV/Sync/Plugin.php" , line : 105usize , placeholders : & [] , regex : "^The \\{DAV:\\}sync\\-collection REPORT is not supported on this url\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/sabre/dav/lib/DAV/Sync/Plugin.php" , line : 109usize , placeholders : & [] , regex : "^No sync information is available at this node$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/sabre/dav/lib/DAV/Sync/Plugin.php" , line : 116usize , placeholders : & [] , regex : "^Invalid or unknown sync token$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/sabre/dav/lib/DAV/Sync/Plugin.php" , line : 124usize , placeholders : & [] , regex : "^Invalid or unknown sync token$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/sabre/dav/lib/DAV/Sharing/Plugin.php" , line : 115usize , placeholders : & [] , regex : "^Sharing is not allowed on this node$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/sabre/dav/lib/DAV/Sharing/Plugin.php" , line : 285usize , placeholders : & [] , regex : "^The \"href\" POST parameter is required$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/sabre/dav/lib/DAV/Sharing/Plugin.php" , line : 288usize , placeholders : & [] , regex : "^The \"access\" POST parameter is required$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/sabre/dav/lib/DAV/Sharing/Plugin.php" , line : 298usize , placeholders : & [] , regex : "^The \"access\" POST must be readwrite, read or no\\-access$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/sabre/dav/lib/DAV/Node.php" , line : 37usize , placeholders : & [] , regex : "^Permission denied to delete node$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/sabre/dav/lib/DAV/Node.php" , line : 49usize , placeholders : & [] , regex : "^Permission denied to rename file$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/sabre/dav/lib/DAV/Xml/Element/Sharee.php" , line : 172usize , placeholders : & [] , regex : "^Every \\{DAV:\\}sharee must have a \\{DAV:\\}href child\\-element$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/sabre/dav/lib/DAV/Xml/Element/Sharee.php" , line : 183usize , placeholders : & [] , regex : "^Every \\{DAV:\\}sharee must have a \\{DAV:\\}share\\-access child element$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/sabre/dav/lib/DAV/Xml/Property/SupportedReportSet.php" , line : 66usize , placeholders : & [] , regex : "^Reportname must be in clark\\-notation$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/sabre/dav/lib/DAV/Xml/Property/ShareAccess.php" , line : 133usize , placeholders : & [] , regex : "^Invalid value for \\{DAV:\\}share\\-access element$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/sabre/dav/lib/CardDAV/Plugin.php" , line : 335usize , placeholders : & [] , regex : "^This collection can only support vcard objects\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/sabre/dav/lib/CardDAV/Plugin.php" , line : 407usize , placeholders : & [] , regex : "^The addressbook\\-query report is not supported on this url with Depth: 0$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/sabre/dav/lib/CardDAV/AddressBook.php" , line : 67usize , placeholders : & [] , regex : "^Card not found$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/sabre/dav/lib/CardDAV/AddressBook.php" , line : 121usize , placeholders : & [] , regex : "^Creating collections in addressbooks is not allowed$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/sabre/dav/lib/CardDAV/AddressBook.php" , line : 162usize , placeholders : & [] , regex : "^Renaming addressbooks is not yet supported$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/sabre/dav/lib/CardDAV/AddressBookHome.php" , line : 100usize , placeholders : & [] , regex : "^Creating new files in this collection is not supported$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/sabre/dav/lib/CardDAV/AddressBookHome.php" , line : 112usize , placeholders : & [] , regex : "^Creating new collections in this collection is not supported$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/sabre/dav/lib/CardDAV/AddressBookHome.php" , line : 160usize , placeholders : & [] , regex : "^Unknown resourceType for this collection$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/sabre/dav/lib/CardDAV/Xml/Request/AddressBookQueryReport.php" , line : 155usize , placeholders : & [] , regex : "^The \"test\" attribute must be one of \"allof\" or \"anyof\"$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/sabre/dav/lib/DAVACL/Principal.php" , line : 49usize , placeholders : & [] , regex : "^The principal properties must at least contain the 'uri' key$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/sabre/dav/lib/DAVACL/Xml/Request/PrincipalSearchPropertySetReport.php" , line : 48usize , placeholders : & [] , regex : "^The \\{DAV:\\}principal\\-search\\-property\\-set element must be empty$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/sabre/dav/lib/DAVACL/Xml/Request/PrincipalPropertySearchReport.php" , line : 105usize , placeholders : & [] , regex : "^The \\{DAV:\\}property\\-search element must contain one \\{DAV:\\}match and one \\{DAV:\\}prop element$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/sabre/dav/lib/DAVACL/Xml/Request/PrincipalPropertySearchReport.php" , line : 117usize , placeholders : & [] , regex : "^The \\{DAV:\\}principal\\-property\\-search report must contain at least 1 \\{DAV:\\}property\\-search element$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/sabre/dav/lib/DAVACL/Xml/Property/Acl.php" , line : 178usize , placeholders : & [] , regex : "^Each \\{DAV:\\}ace element must have one \\{DAV:\\}principal element$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/sabre/dav/lib/DAVACL/Xml/Property/Acl.php" , line : 200usize , placeholders : & [] , regex : "^Every \\{DAV:\\}ace element must have a \\{DAV:\\}grant element\\. \\{DAV:\\}deny is not yet supported$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/sabre/dav/lib/DAVACL/Xml/Property/Principal.php" , line : 68usize , placeholders : & [] , regex : "^The href argument must be specified for the HREF principal type\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/sabre/dav/lib/DAVACL/ACLTrait.php" , line : 75usize , placeholders : & [] , regex : "^Setting ACL is not supported on this node$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/sabre/dav/lib/DAVACL/Plugin.php" , line : 770usize , placeholders : & [] , regex : "^The Auth plugin must be loaded before the ACL plugin if you want to allow unauthenticated access\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/sabre/dav/lib/DAVACL/Plugin.php" , line : 1048usize , placeholders : & [] , regex : "^The group\\-member\\-set property MUST be an instance of Sabre\\\\DAV\\\\Property\\\\HrefList or null$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/sabre/dav/lib/DAVACL/Plugin.php" , line : 1114usize , placeholders : & [] , regex : "^XML body expected in ACL request$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/sabre/dav/lib/DAVACL/Plugin.php" , line : 1127usize , placeholders : & [] , regex : "^This node does not support the ACL method$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/sabre/dav/lib/DAVACL/Plugin.php" , line : 1153usize , placeholders : & [] , regex : "^This resource contained a protected \\{DAV:\\}ace, but this privilege did not occur in the ACL request$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/sabre/dav/lib/DAVACL/Plugin.php" , line : 1202usize , placeholders : & [] , regex : "^The principal\\-match report is only defined on Depth: 0$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/sabre/dav/lib/DAVACL/Plugin.php" , line : 1375usize , placeholders : & [] , regex : "^This report is only defined when Depth: 0$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/sabre/dav/lib/DAVACL/Plugin.php" , line : 1426usize , placeholders : & [] , regex : "^Depth must be 0$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/sabre/dav/lib/DAVACL/Plugin.php" , line : 1461usize , placeholders : & [] , regex : "^The \\{DAV:\\}acl\\-principal\\-prop\\-set REPORT only supports Depth 0$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/sabre/dav/lib/DAVACL/Plugin.php" , line : 1470usize , placeholders : & [] , regex : "^Could not fetch ACL rules for this path$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/sabre/dav/lib/DAVACL/AbstractPrincipalCollection.php" , line : 93usize , placeholders : & [] , regex : "^Listing members of this collection is disabled$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/sabre/dav/lib/DAVACL/FS/Collection.php" , line : 69usize , placeholders : & [] , regex : "^File could not be located$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/sabre/dav/lib/DAVACL/FS/Collection.php" , line : 72usize , placeholders : & [] , regex : "^Permission denied to \\. and \\.\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/sabre/dav/lib/DAVACL/PrincipalBackend/PDO.php" , line : 355usize , placeholders : & [] , regex : "^Principal not found$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/sabre/dav/lib/DAVACL/PrincipalBackend/PDO.php" , line : 379usize , placeholders : & [] , regex : "^Principal not found$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/sabre/dav/lib/DAVACL/PrincipalBackend/PDO.php" , line : 416usize , placeholders : & [] , regex : "^Principal not found$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/sabre/dav/lib/DAVACL/PrincipalCollection.php" , line : 65usize , placeholders : & [] , regex : "^Only resources of type \\{DAV:\\}principal may be created here$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/sabre/event/lib/Promise.php" , line : 137usize , placeholders : & [] , regex : "^This promise is already resolved, and you're not allowed to resolve a promise more than once$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/sabre/event/lib/Promise.php" , line : 152usize , placeholders : & [] , regex : "^This promise is already resolved, and you're not allowed to resolve a promise more than once$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/sabre/event/lib/Promise.php" , line : 180usize , placeholders : & [] , regex : "^There were no more events in the loop\\. This promise will never be fulfilled\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/sabre/event/lib/coroutine.php" , line : 59usize , placeholders : & [] , regex : "^You must pass a generator function$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/sabre/vobject/lib/Recur/EventIterator.php" , line : 112usize , placeholders : & [] , regex : "^The UID argument is required when a VCALENDAR is passed to this constructor$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/sabre/vobject/lib/Recur/EventIterator.php" , line : 115usize , placeholders : & [] , regex : "^No events found in this calendar$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/sabre/vobject/lib/Recur/EventIterator.php" , line : 192usize , placeholders : & [] , regex : "^This recurrence rule does not generate any valid instances$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/sabre/vobject/lib/Recur/RRuleIterator.php" , line : 775usize , placeholders : & [] , regex : "^BYYEARDAY in RRULE must have value\\(s\\) from 1 to 366, or \\-366 to \\-1!$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/sabre/vobject/lib/Recur/RRuleIterator.php" , line : 784usize , placeholders : & [] , regex : "^BYWEEKNO in RRULE must have value\\(s\\) from 1 to 53, or \\-53 to \\-1!$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/sabre/vobject/lib/Recur/RRuleIterator.php" , line : 793usize , placeholders : & [] , regex : "^BYMONTH in RRULE must have value\\(s\\) between 1 and 12!$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/sabre/vobject/lib/Parser/XML.php" , line : 85usize , placeholders : & [] , regex : "^End of input stream, or no input supplied$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/sabre/vobject/lib/Parser/XML.php" , line : 107usize , placeholders : & [] , regex : "^Unsupported XML standard$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/sabre/vobject/lib/Parser/Json.php" , line : 57usize , placeholders : & [] , regex : "^End of input stream, or no input supplied$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/sabre/vobject/lib/Parser/Json.php" , line : 72usize , placeholders : & [] , regex : "^The root component must either be a vcalendar, or a vcard$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/sabre/vobject/lib/Parser/MimeDir.php" , line : 135usize , placeholders : & [] , regex : "^This parser can only read from strings or streams\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/sabre/vobject/lib/Parser/MimeDir.php" , line : 163usize , placeholders : & [] , regex : "^This parser only supports VCARD and VCALENDAR files$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/sabre/vobject/lib/Parser/MimeDir.php" , line : 284usize , placeholders : & [] , regex : "^End of document reached prematurely$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/sabre/vobject/lib/Parser/MimeDir.php" , line : 287usize , placeholders : & [] , regex : "^Error reading from input stream$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/sabre/vobject/lib/Parser/MimeDir.php" , line : 406usize , placeholders : & [] , regex : "^This code should not be reachable$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/sabre/vobject/lib/Splitter/VCard.php" , line : 66usize , placeholders : & [] , regex : "^The supplied input contained non\\-VCARD data\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/sabre/vobject/lib/Splitter/ICalendar.php" , line : 51usize , placeholders : & [] , regex : "^Supplied input could not be parsed as VCALENDAR\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/sabre/vobject/lib/FreeBusyGenerator.php" , line : 148usize , placeholders : & [] , regex : "^You can only pass strings or \\\\Sabre\\\\VObject\\\\Component arguments to setObjects$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/sabre/vobject/lib/ElementList.php" , line : 31usize , placeholders : & [] , regex : "^You can not add new objects to an ElementList$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/sabre/vobject/lib/ElementList.php" , line : 44usize , placeholders : & [] , regex : "^You can not remove objects from an ElementList$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/sabre/vobject/lib/Component.php" , line : 109usize , placeholders : & [] , regex : "^The second argument must not be specified, when passing a VObject Node$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/sabre/vobject/lib/Component.php" , line : 116usize , placeholders : & [] , regex : "^The first argument must either be a \\\\Sabre\\\\VObject\\\\Node or a string$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/sabre/vobject/lib/Component.php" , line : 164usize , placeholders : & [] , regex : "^The item you passed to remove\\(\\) was not a child of this component$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/sabre/vobject/lib/Component.php" , line : 440usize , placeholders : & [] , regex : "^Starting sabre/vobject 4\\.0 the children property is now protected\\. You should use the children\\(\\) method instead$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/sabre/vobject/lib/VCardConverter.php" , line : 39usize , placeholders : & [] , regex : "^Only vCard 2\\.1, 3\\.0 and 4\\.0 are supported for the input data$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/sabre/vobject/lib/VCardConverter.php" , line : 42usize , placeholders : & [] , regex : "^You can only use vCard 3\\.0 or 4\\.0 for the target version$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/sabre/vobject/lib/Cli.php" , line : 201usize , placeholders : & [] , regex : "^Inputfile is a required argument$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/sabre/vobject/lib/Cli.php" , line : 205usize , placeholders : & [] , regex : "^Too many arguments$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Unknown , path : "/3rdparty/sabre/vobject/lib/Cli.php" , line : 267usize , placeholders : & [] , regex : "^Usage:$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Unknown , path : "/3rdparty/sabre/vobject/lib/Cli.php" , line : 268usize , placeholders : & [] , regex : "^  vobject \\[options\\] command \\[arguments\\]$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Unknown , path : "/3rdparty/sabre/vobject/lib/Cli.php" , line : 269usize , placeholders : & [] , regex : "^$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Unknown , path : "/3rdparty/sabre/vobject/lib/Cli.php" , line : 270usize , placeholders : & [] , regex : "^Options:$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Unknown , path : "/3rdparty/sabre/vobject/lib/Cli.php" , line : 275usize , placeholders : & [] , regex : "^                vcard30, vcard40, icalendar20, jcal, jcard, json, mimedir\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Unknown , path : "/3rdparty/sabre/vobject/lib/Cli.php" , line : 277usize , placeholders : & [] , regex : "^                must be specified here\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Unknown , path : "/3rdparty/sabre/vobject/lib/Cli.php" , line : 282usize , placeholders : & [] , regex : "^$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Unknown , path : "/3rdparty/sabre/vobject/lib/Cli.php" , line : 283usize , placeholders : & [] , regex : "^Commands:$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Unknown , path : "/3rdparty/sabre/vobject/lib/Cli.php" , line : 298usize , placeholders : & [] , regex : "^Examples:$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Unknown , path : "/3rdparty/sabre/vobject/lib/Cli.php" , line : 299usize , placeholders : & [] , regex : "^   vobject convert contact\\.vcf contact\\.json$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Unknown , path : "/3rdparty/sabre/vobject/lib/Cli.php" , line : 300usize , placeholders : & [] , regex : "^   vobject convert \\-\\-format=vcard40 old\\.vcf new\\.vcf$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Unknown , path : "/3rdparty/sabre/vobject/lib/Cli.php" , line : 301usize , placeholders : & [] , regex : "^   vobject convert \\-\\-inputformat=json \\-\\-format=mimedir \\- \\-$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Unknown , path : "/3rdparty/sabre/vobject/lib/Cli.php" , line : 302usize , placeholders : & [] , regex : "^   vobject color calendar\\.ics$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Unknown , path : "/3rdparty/sabre/vobject/lib/Cli.php" , line : 303usize , placeholders : & [] , regex : "^$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Unknown , path : "/3rdparty/sabre/vobject/lib/Cli.php" , line : 304usize , placeholders : & [] , regex : "^https://github\\.com/fruux/sabre\\-vobject$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Unknown , path : "/3rdparty/sabre/vobject/lib/Cli.php" , line : 327usize , placeholders : & [] , regex : "^  No warnings!$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Unknown , path : "/3rdparty/sabre/vobject/lib/Cli.php" , line : 367usize , placeholders : & [] , regex : "^  No warnings!$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/sabre/vobject/lib/Component/VCalendar.php" , line : 322usize , placeholders : & [] , regex : "^Every VEVENT object must have a UID property$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/sabre/vobject/lib/Component/VAlarm.php" , line : 51usize , placeholders : & [] , regex : "^time\\-range filters on VALARM components are only supported when they are a child of VTODO or VEVENT$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/sabre/vobject/lib/ITip/Broker.php" , line : 833usize , placeholders : & [] , regex : "^If a calendar contained more than one event, they must have the same UID\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/sabre/vobject/lib/ITip/Broker.php" , line : 838usize , placeholders : & [] , regex : "^An event MUST have a DTSTART property\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/sabre/vobject/lib/ITip/Broker.php" , line : 847usize , placeholders : & [] , regex : "^Every instance of the event must have the same organizer\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/sabre/vobject/lib/BirthdayCalendarGenerator.php" , line : 70usize , placeholders : & [] , regex : "^String could not be parsed as \\\\Sabre\\\\VObject\\\\Component\\\\VCard by setObjects$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/sabre/vobject/lib/BirthdayCalendarGenerator.php" , line : 77usize , placeholders : & [] , regex : "^You can only pass strings or \\\\Sabre\\\\VObject\\\\Component\\\\VCard arguments to setObjects$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/sabre/vobject/lib/Property/Binary.php" , line : 44usize , placeholders : & [] , regex : "^The argument must either be a string or an array with only one child$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/sabre/vobject/lib/Property/ICalendar/Recur.php" , line : 64usize , placeholders : & [] , regex : "^You must either pass a string, or a key=>value array$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/sabre/vobject/lib/Property/VCard/DateAndOrTime.php" , line : 52usize , placeholders : & [] , regex : "^Only one value allowed$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/sabre/uri/lib/functions.php" , line : 354usize , placeholders : & [] , regex : "^Invalid, or could not parse URI$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/sabre/http/lib/Response.php" , line : 163usize , placeholders : & [] , regex : "^The HTTP status code must be exactly 3 digits$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/sabre/http/lib/Sapi.php" , line : 229usize , placeholders : & [] , regex : "^The _SERVER array must have a REQUEST_URI key$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/sabre/http/lib/Sapi.php" , line : 233usize , placeholders : & [] , regex : "^The _SERVER array must have a REQUEST_METHOD key$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/sabre/xml/lib/Reader.php" , line : 153usize , placeholders : & [] , regex : "^This should never happen \\(famous last words\\)$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/sabre/xml/lib/Reader.php" , line : 183usize , placeholders : & [] , regex : "^We hit the end of the document prematurely\\. This likely means that some parser \"eats\" too many elements\\. Do not attempt to continue parsing\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/sabre/xml/lib/Service.php" , line : 122usize , placeholders : & [] , regex : "^The input element to parse is empty\\. Do not attempt to parse$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/sabre/xml/lib/Service.php" , line : 166usize , placeholders : & [] , regex : "^The input element to parse is empty\\. Do not attempt to parse$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/swiftmailer/swiftmailer/lib/classes/Swift/ByteStream/FileByteStream.php" , line : 45usize , placeholders : & [] , regex : "^The path cannot be empty$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/swiftmailer/swiftmailer/lib/classes/Swift/ByteStream/FileByteStream.php" , line : 198usize , placeholders : & [] , regex : "^Unable to copy the file to make it seekable, sys_temp_dir is not writable, php://memory not available$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/swiftmailer/swiftmailer/lib/classes/Swift/ByteStream/TemporaryFileByteStream.php" , line : 21usize , placeholders : & [] , regex : "^Failed to retrieve temporary file name\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/swiftmailer/swiftmailer/lib/classes/Swift/ByteStream/TemporaryFileByteStream.php" , line : 30usize , placeholders : & [] , regex : "^Failed to get temporary file content\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/swiftmailer/swiftmailer/lib/classes/Swift/Transport/EsmtpTransport.php" , line : 349usize , placeholders : & [] , regex : "^Unable to connect with TLS encryption$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/swiftmailer/swiftmailer/lib/classes/Swift/Transport/LoadBalancedTransport.php" , line : 149usize , placeholders : & [] , regex : "^All Transports in LoadBalancedTransport failed, or no Transports available$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/swiftmailer/swiftmailer/lib/classes/Swift/Transport/Esmtp/Auth/NTLMAuthenticator.php" , line : 41usize , placeholders : & [] , regex : "^The OpenSSL extension must be enabled to use the NTLM authenticator\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/swiftmailer/swiftmailer/lib/classes/Swift/Transport/Esmtp/Auth/NTLMAuthenticator.php" , line : 45usize , placeholders : & [] , regex : "^The BCMath functions must be enabled to use the NTLM authenticator\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/swiftmailer/swiftmailer/lib/classes/Swift/Transport/FailoverTransport.php" , line : 83usize , placeholders : & [] , regex : "^All Transports in FailoverTransport failed, or no Transports available$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/swiftmailer/swiftmailer/lib/classes/Swift/AddressEncoder/IdnAddressEncoder.php" , line : 40usize , placeholders : & [] , regex : "^Non\\-ASCII characters not supported in local\\-part$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/swiftmailer/swiftmailer/lib/classes/Swift/Signers/DKIMSigner.php" , line : 299usize , placeholders : & [] , regex : "^Unable to set sha256 as it is not supported by OpenSSL\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/swiftmailer/swiftmailer/lib/classes/Swift/Signers/DKIMSigner.php" , line : 303usize , placeholders : & [] , regex : "^Unable to set the hash algorithm, must be one of rsa\\-sha1 or rsa\\-sha256 \\(%s given\\)\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/swiftmailer/swiftmailer/lib/classes/Swift/Signers/DomainKeySigner.php" , line : 440usize , placeholders : & [] , regex : "^Invalid new line sequence in mail found \\\\n without preceding \\\\r$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/swiftmailer/swiftmailer/lib/classes/Swift/Signers/OpenDKIMSigner.php" , line : 35usize , placeholders : & [] , regex : "^php\\-opendkim extension not found$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/swiftmailer/swiftmailer/lib/classes/Swift/DependencyContainer.php" , line : 349usize , placeholders : & [] , regex : "^Component must first be registered by calling register\\(\\)$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/swiftmailer/swiftmailer/lib/classes/Swift/Mime/SimpleMimeEntity.php" , line : 698usize , placeholders : & [] , regex : "^Mime boundary set is not RFC 2046 compliant\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/swiftmailer/swiftmailer/lib/classes/Swift/Mime/Headers/PathHeader.php" , line : 150usize , placeholders : & [] , regex : "^Address set in PathHeader does not comply with addr\\-spec of RFC 2822\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/league/uri/src/Uri.php" , line : 876usize , placeholders : & [] , regex : "^The user info could not be detected$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/league/uri/src/Uri.php" , line : 916usize , placeholders : & [] , regex : "^The host could not be detected$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/league/uri/src/Uri.php" , line : 1118usize , placeholders : & [] , regex : "^If an authority is present the path must be empty or start with a `/`\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/league/uri/src/Uri.php" , line : 1131usize , placeholders : & [] , regex : "^In absence of a scheme and an authority the first path segment cannot contain a colon \\(\":\"\\) character\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/league/uri/src/Uri.php" , line : 1453usize , placeholders : & [] , regex : "^A path must be a string NULL given\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/core/Service/LoginFlowV2Service.php" , line : 79usize , placeholders : & [] , regex : "^Invalid token$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/core/Service/LoginFlowV2Service.php" , line : 87usize , placeholders : & [] , regex : "^Token not yet ready$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/core/Service/LoginFlowV2Service.php" , line : 98usize , placeholders : & [] , regex : "^Apptoken could not be decrypted$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/core/Service/LoginFlowV2Service.php" , line : 113usize , placeholders : & [] , regex : "^Login token invalid$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/core/Service/LoginFlowV2Service.php" , line : 234usize , placeholders : & [] , regex : "^Could not initialize keys$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/core/Service/LoginFlowV2Service.php" , line : 239usize , placeholders : & [] , regex : "^OpenSSL reported a problem$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Info , path : "/core/BackgroundJobs/BackgroundCleanupUpdaterBackupsJob.php" , line : 58usize , placeholders : & ["$backupFolderPath"] , regex : "^(.*) exists \\- start to clean it up$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Info , path : "/core/BackgroundJobs/BackgroundCleanupUpdaterBackupsJob.php" , line : 84usize , placeholders : & ["$dir"] , regex : "^Removing (.*) \\.\\.\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Info , path : "/core/BackgroundJobs/BackgroundCleanupUpdaterBackupsJob.php" , line : 87usize , placeholders : & [] , regex : "^Cleanup finished$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Info , path : "/core/BackgroundJobs/BackgroundCleanupUpdaterBackupsJob.php" , line : 89usize , placeholders : & ["$backupFolderPath"] , regex : "^Could not find updater directory (.*) \\- cleanup step not needed$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/core/Command/User/Setting.php" , line : 127usize , placeholders : & [] , regex : "^The \"default\\-value\" option can only be used when specifying a key\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/core/Command/User/Setting.php" , line : 131usize , placeholders : & [] , regex : "^The value argument can only be used when specifying a key\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/core/Command/User/Setting.php" , line : 134usize , placeholders : & [] , regex : "^The value argument can not be used together with \"default\\-value\"\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/core/Command/User/Setting.php" , line : 137usize , placeholders : & [] , regex : "^The \"update\\-only\" option can only be used together with \"value\"\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/core/Command/User/Setting.php" , line : 141usize , placeholders : & [] , regex : "^The \"delete\" option can only be used when specifying a key\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/core/Command/User/Setting.php" , line : 144usize , placeholders : & [] , regex : "^The \"delete\" option can not be used together with \"default\\-value\"\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/core/Command/User/Setting.php" , line : 147usize , placeholders : & [] , regex : "^The \"delete\" option can not be used together with \"value\"\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/core/Command/User/Setting.php" , line : 150usize , placeholders : & [] , regex : "^The \"error\\-if\\-not\\-exists\" option can only be used together with \"delete\"\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/core/Command/Encryption/EncryptAll.php" , line : 101usize , placeholders : & [] , regex : "^Server side encryption is not enabled$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/core/Command/Encryption/ChangeKeyStorageRoot.php" , line : 127usize , placeholders : & [] , regex : "^New root folder doesn't exist\\. Please create the folder or check the permissions and try again\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/core/Command/Encryption/ChangeKeyStorageRoot.php" , line : 136usize , placeholders : & [] , regex : "^Can't access the new root folder\\. Please check the permissions and make sure that the folder is in your data folder$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/core/Command/Encryption/ChangeKeyStorageRoot.php" , line : 247usize , placeholders : & ["$path"] , regex : "^new folder '(.*)' already exists$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/core/Command/Config/Import.php" , line : 124usize , placeholders : & [] , regex : "^The file must contain a valid json array$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/core/Command/Config/Import.php" , line : 154usize , placeholders : & [] , regex : "^The system config array is not an array$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/core/Command/Config/Import.php" , line : 162usize , placeholders : & [] , regex : "^The apps config array is not an array$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/core/Command/Config/System/DeleteConfig.php" , line : 102usize , placeholders : & [] , regex : "^Config parameter does not exist$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/core/Command/Config/System/DeleteConfig.php" , line : 105usize , placeholders : & [] , regex : "^Config parameter does not exist$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/core/Command/Config/System/SetConfig.php" , line : 88usize , placeholders : & [] , regex : "^Config parameter does not exist$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/core/Command/Config/System/SetConfig.php" , line : 109usize , placeholders : & [] , regex : "^Non\\-numeric value specified$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/core/Command/Config/System/SetConfig.php" , line : 119usize , placeholders : & [] , regex : "^Non\\-numeric value specified$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/core/Command/Config/System/SetConfig.php" , line : 143usize , placeholders : & [] , regex : "^Unable to parse value as boolean$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/core/Command/Config/System/SetConfig.php" , line : 161usize , placeholders : & [] , regex : "^Invalid type$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/core/Command/Config/System/SetConfig.php" , line : 187usize , placeholders : & [] , regex : "^Config parameter does not exist$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/core/Command/L10n/CreateJs.php" , line : 135usize , placeholders : & ["$phpFile"] , regex : "^PHP translation file <(.*)> does not exist\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/core/Command/Db/Migrations/GenerateCommand.php" , line : 238usize , placeholders : & [] , regex : "^Failed to generate new migration step\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/core/Command/Db/Migrations/GenerateCommand.php" , line : 250usize , placeholders : & ["$directory"] , regex : "^Could not create folder \"(.*)\"$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/core/Command/Db/Migrations/GenerateCommand.php" , line : 256usize , placeholders : & ["$directory"] , regex : "^Could not create folder \"(.*)\"$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/core/Command/Db/ConvertType.php" , line : 131usize , placeholders : & [] , regex : "^Converting to SQLite \\(sqlite3\\) is currently not supported\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/core/Command/Db/ConvertType.php" , line : 146usize , placeholders : & [] , regex : "^The \\-\\-clear\\-schema option is not supported when converting to Oracle \\(oci\\)\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/core/Command/Log/File.php" , line : 122usize , placeholders : & [] , regex : "^Error parsing log rotation file size$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/core/Command/Log/File.php" , line : 125usize , placeholders : & [] , regex : "^Log rotation file size must be non\\-negative$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/core/Command/Log/Manage.php" , line : 124usize , placeholders : & [] , regex : "^Invalid backend$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/core/Command/Log/Manage.php" , line : 157usize , placeholders : & [] , regex : "^Invalid log level string$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/core/Command/Log/Manage.php" , line : 178usize , placeholders : & [] , regex : "^Invalid log level number$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/core/Command/Base.php" , line : 155usize , placeholders : & [] , regex : "^Command interrupted by user$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/core/Command/Maintenance/Install.php" , line : 164usize , placeholders : & [] , regex : "^Database user not provided\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/core/Command/Maintenance/Install.php" , line : 167usize , placeholders : & [] , regex : "^Database name not provided\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Error , path : "/core/Command/Preview/Repair.php" , line : 262usize , placeholders : & ["$name" , "$previewName"] , regex : "^Failed to delete preview at preview/(.*)/(.*)$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Error , path : "/core/Command/Preview/Repair.php" , line : 272usize , placeholders : & ["$name" , "$previewName" , "$newFoldername"] , regex : "^Failed to move preview from preview/(.*)/(.*) to preview/(.*)$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Error , path : "/core/Command/Preview/Repair.php" , line : 288usize , placeholders : & ["$name"] , regex : "^Failed to delete empty folder preview/(.*)$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/core/Db/LoginFlowV2Mapper.php" , line : 99usize , placeholders : & [] , regex : "^Token expired$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/core/Controller/OCMController.php" , line : 77usize , placeholders : & [] , regex : "^loaded class does not implements OCP\\\\Capabilities\\\\ICapability$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Error , path : "/core/Controller/OCMController.php" , line : 89usize , placeholders : & [] , regex : "^issue during OCM discovery request$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Warn , path : "/core/Controller/TwoFactorChallengeController.php" , line : 200usize , placeholders : & ["$uid" , "$ip"] , regex : "^Two\\-factor challenge failed: (.*) \\(Remote IP: (.*)\\)$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/core/Controller/ProfileApiController.php" , line : 71usize , placeholders : & [] , regex : "^User does not exist$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/core/Controller/ProfileApiController.php" , line : 75usize , placeholders : & [] , regex : "^Users can only edit their own visibility settings$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/core/Controller/ClientFlowLoginV2Controller.php" , line : 212usize , placeholders : & [] , regex : "^login name does not match$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/core/Controller/ClientFlowLoginV2Controller.php" , line : 344usize , placeholders : & [] , regex : "^Login token not set in session$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Debug , path : "/core/Controller/WebAuthnController.php" , line : 70usize , placeholders : & [] , regex : "^Starting WebAuthn login$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Debug , path : "/core/Controller/WebAuthnController.php" , line : 72usize , placeholders : & [] , regex : "^Converting login name to UID$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Debug , path : "/core/Controller/WebAuthnController.php" , line : 94usize , placeholders : & [] , regex : "^Validating WebAuthn login$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Debug , path : "/core/Controller/WebAuthnController.php" , line : 97usize , placeholders : & [] , regex : "^Trying to finish WebAuthn login without session data$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Error , path : "/core/Controller/LostController.php" , line : 243usize , placeholders : & [] , regex : "^$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/core/Controller/LostController.php" , line : 256usize , placeholders : & [] , regex : "^Password too long$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/core/Controller/LostController.php" , line : 298usize , placeholders : & [] , regex : "^Could not send reset e\\-mail, 5 of them were already sent in the last 30 minutes$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/core/Controller/AppPasswordController.php" , line : 76usize , placeholders : & [] , regex : "^You cannot request an new apppassword with an apppassword$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/core/Controller/AppPasswordController.php" , line : 119usize , placeholders : & [] , regex : "^no app password in use$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/core/Controller/AppPasswordController.php" , line : 127usize , placeholders : & [] , regex : "^could not remove apptoken$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/core/Controller/AppPasswordController.php" , line : 139usize , placeholders : & [] , regex : "^no app password in use$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/core/Controller/AppPasswordController.php" , line : 147usize , placeholders : & [] , regex : "^could not rotate apptoken$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Error , path : "/core/Controller/GuestAvatarController.php" , line : 89usize , placeholders : & [] , regex : "^error while creating guest avatar$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/core/Controller/ClientFlowLoginController.php" , line : 356usize , placeholders : & [] , regex : "^login name does not match$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/core/Controller/WhatsNewController.php" , line : 77usize , placeholders : & [] , regex : "^Acting user cannot be resolved$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/core/Controller/WhatsNewController.php" , line : 117usize , placeholders : & [] , regex : "^Acting user cannot be resolved$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/core/Controller/CollaborationResourcesController.php" , line : 71usize , placeholders : & [] , regex : "^Not found$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/core/Controller/CollaborationResourcesController.php" , line : 261usize , placeholders : & [] , regex : "^Can not access collection$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/core/Controller/CollaborationResourcesController.php" , line : 288usize , placeholders : & [] , regex : "^Can not access resource$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/core/Notification/CoreNotifier.php" , line : 100usize , placeholders : & [] , regex : "^Invalid subject$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/updater/index.php" , line : 92usize , placeholders : & [] , regex : "^Could not find config\\.php\\. Is this file in the \"updater\" subfolder of Nextcloud\\?$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/updater/index.php" , line : 107usize , placeholders : & [] , regex : "^Could not read data directory from config\\.php\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/updater/index.php" , line : 377usize , placeholders : & [] , regex : "^Could not find config\\.php\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/updater/index.php" , line : 388usize , placeholders : & [] , regex : "^Could not write to config\\.php$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/updater/index.php" , line : 413usize , placeholders : & [] , regex : "^Could not create backup folder location$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/updater/index.php" , line : 537usize , placeholders : & [] , regex : "^Could not parse updater server XML response$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/updater/index.php" , line : 541usize , placeholders : & [] , regex : "^Could not JSON encode updater server response$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/updater/index.php" , line : 545usize , placeholders : & [] , regex : "^Could not JSON decode updater server response\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/updater/index.php" , line : 568usize , placeholders : & [] , regex : "^Could not mkdir storage location$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/updater/index.php" , line : 638usize , placeholders : & [] , regex : "^There are more files than the downloaded archive in the downloads/ folder\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/updater/index.php" , line : 658usize , placeholders : & [] , regex : "^No signature specified for defined update$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/updater/index.php" , line : 699usize , placeholders : & [] , regex : "^Signature of update is not valid$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/updater/index.php" , line : 721usize , placeholders : & ["$versionFile"] , regex : "^OC_Version not found in (.*)$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/updater/index.php" , line : 753usize , placeholders : & [] , regex : "^Downloaded version is lower than installed version$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/updater/index.php" , line : 847usize , placeholders : & [] , regex : "^core/shipped\\.json is not available$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/updater/index.php" , line : 852usize , placeholders : & [] , regex : "^core/shipped\\.json is not available in the new release$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/updater/index.php" , line : 871usize , placeholders : & [] , regex : "^Could not unlink sample config$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/updater/index.php" , line : 882usize , placeholders : & [] , regex : "^Could not delete themes README$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/updater/index.php" , line : 1029usize , placeholders : & [] , regex : "^Could not rmdir \\$storagelocation$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/updater/index.php" , line : 1034usize , placeholders : & [] , regex : "^Could not rmdir \\.step$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/updater/index.php" , line : 1056usize , placeholders : & [] , regex : "^Could not create \\$updaterDir$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/updater/index.php" , line : 1061usize , placeholders : & [] , regex : "^Could not create \\.step$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/updater/index.php" , line : 1067usize , placeholders : & [] , regex : "^Could not write to \\.step$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/updater/index.php" , line : 1101usize , placeholders : & [] , regex : "^Could not read from \\.step$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/updater/index.php" , line : 1106usize , placeholders : & [] , regex : "^Can't decode \\.step JSON data$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/updater/index.php" , line : 1126usize , placeholders : & [] , regex : "^Could not delete \\.step$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/updater/index.php" , line : 1169usize , placeholders : & [] , regex : "^Could not open updater\\.log$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/updater/index.php" , line : 1186usize , placeholders : & [] , regex : "^Could not write to updater\\.log$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Unknown , path : "/updater/index.php" , line : 1325usize , placeholders : & [] , regex : "^\\[info\\] request to updater$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/updater/index.php" , line : 1364usize , placeholders : & [] , regex : "^Not authenticated$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/updater/index.php" , line : 1369usize , placeholders : & [] , regex : "^Invalid step$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Unknown , path : "/updater/index.php" , line : 1417usize , placeholders : & [] , regex : "^\\[error\\] POST request failed with UpdateException$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Unknown , path : "/updater/index.php" , line : 1432usize , placeholders : & [] , regex : "^\\[error\\] POST request failed with other exception$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Unknown , path : "/updater/index.php" , line : 1448usize , placeholders : & [] , regex : "^\\[info\\] show HTML page$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/composer/composer/ClassLoader.php" , line : 250usize , placeholders : & [] , regex : "^A non\\-empty PSR\\-4 prefix must end with a namespace separator\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/composer/composer/ClassLoader.php" , line : 305usize , placeholders : & [] , regex : "^A non\\-empty PSR\\-4 prefix must end with a namespace separator\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Error , path : "/lib/private/CapabilitiesManager.php" , line : 61usize , placeholders : & [] , regex : "^CapabilitiesManager$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/Federation/CloudIdManager.php" , line : 105usize , placeholders : & [] , regex : "^Invalid cloud id$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/Federation/CloudIdManager.php" , line : 135usize , placeholders : & [] , regex : "^Invalid cloud id$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/Preview/Generator.php" , line : 132usize , placeholders : & [] , regex : "^Cannot read file$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/Preview/Generator.php" , line : 153usize , placeholders : & [] , regex : "^Max preview size 0, invalid!$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/Preview/Generator.php" , line : 159usize , placeholders : & [] , regex : "^The maximum preview sizes are zero or less pixels$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/Preview/Generator.php" , line : 204usize , placeholders : & [] , regex : "^$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/Preview/Generator.php" , line : 209usize , placeholders : & [] , regex : "^Cached preview size 0, invalid!$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/Preview/Generator.php" , line : 392usize , placeholders : & [] , regex : "^No provider successfully handled the preview generation$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/Preview/Generator.php" , line : 532usize , placeholders : & [] , regex : "^Failed to generate preview, failed to load image$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Info , path : "/lib/private/Preview/Movie.php" , line : 166usize , placeholders : & [] , regex : "^Movie preview generation failed Output: \\{output\\}$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Error , path : "/lib/private/Preview/Imaginary.php" , line : 74usize , placeholders : & [] , regex : "^Imaginary preview provider is enabled, but no url is configured\\. Please provide the url of your imaginary server to the 'preview_imaginary_url' config variable\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/Share20/Share.php" , line : 121usize , placeholders : & [] , regex : "^String expected\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/Share20/Share.php" , line : 125usize , placeholders : & [] , regex : "^Not allowed to assign a new internal id to a share$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/Share20/Share.php" , line : 154usize , placeholders : & [] , regex : "^String expected\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/Share20/Share.php" , line : 158usize , placeholders : & [] , regex : "^Not allowed to assign a new provider id to a share$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/Share20/PublicShareTemplateFactory.php" , line : 44usize , placeholders : & [] , regex : "^Can't retrieve public share template providers as context is not defined$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/Share20/Manager.php" , line : 192usize , placeholders : & [] , regex : "^Passwords are enforced for link and mail shares$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/Share20/Manager.php" , line : 219usize , placeholders : & [] , regex : "^SharedWith is not a valid user$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/Share20/Manager.php" , line : 224usize , placeholders : & [] , regex : "^SharedWith is not a valid group$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/Share20/Manager.php" , line : 229usize , placeholders : & [] , regex : "^SharedWith should be empty$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/Share20/Manager.php" , line : 233usize , placeholders : & [] , regex : "^SharedWith should not be empty$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/Share20/Manager.php" , line : 237usize , placeholders : & [] , regex : "^SharedWith should not be empty$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/Share20/Manager.php" , line : 241usize , placeholders : & [] , regex : "^SharedWith should not be empty$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/Share20/Manager.php" , line : 246usize , placeholders : & [] , regex : "^SharedWith is not a valid circle$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/Share20/Manager.php" , line : 253usize , placeholders : & [] , regex : "^unknown share type$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/Share20/Manager.php" , line : 258usize , placeholders : & [] , regex : "^SharedBy should be set$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/Share20/Manager.php" , line : 264usize , placeholders : & [] , regex : "^Cannot share with yourself$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/Share20/Manager.php" , line : 269usize , placeholders : & [] , regex : "^Path should be set$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/Share20/Manager.php" , line : 275usize , placeholders : & [] , regex : "^Path should be either a file or a folder$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/Share20/Manager.php" , line : 285usize , placeholders : & [] , regex : "^You cannot share your root folder$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/Share20/Manager.php" , line : 296usize , placeholders : & [] , regex : "^A share requires permissions$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/Share20/Manager.php" , line : 366usize , placeholders : & [] , regex : "^Shares need at least read permissions$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/Share20/Manager.php" , line : 443usize , placeholders : & [] , regex : "^Expiration date is enforced$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/Share20/Manager.php" , line : 524usize , placeholders : & [] , regex : "^Expiration date is enforced$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/Share20/Manager.php" , line : 627usize , placeholders : & [] , regex : "^Group sharing is now allowed$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/Share20/Manager.php" , line : 635usize , placeholders : & [] , regex : "^Sharing is only allowed within your own groups$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/Share20/Manager.php" , line : 656usize , placeholders : & [] , regex : "^Path is already shared with this group$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/Share20/Manager.php" , line : 670usize , placeholders : & [] , regex : "^Link sharing is not allowed$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/Share20/Manager.php" , line : 676usize , placeholders : & [] , regex : "^Public upload is not allowed$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/Share20/Manager.php" , line : 711usize , placeholders : & [] , regex : "^Path contains files shared with you$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/Share20/Manager.php" , line : 725usize , placeholders : & [] , regex : "^Sharing is disabled$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/Share20/Manager.php" , line : 729usize , placeholders : & [] , regex : "^Sharing is disabled for you$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/Share20/Manager.php" , line : 813usize , placeholders : & [] , regex : "^Cannot share with the share owner$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Debug , path : "/lib/private/Share20/Manager.php" , line : 889usize , placeholders : & [] , regex : "^Share notification not sent because mailsend is false\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Error , path : "/lib/private/Share20/Manager.php" , line : 977usize , placeholders : & [] , regex : "^Share notification mail could not be sent$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/Share20/Manager.php" , line : 996usize , placeholders : & [] , regex : "^Share does not have a full id$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/Share20/Manager.php" , line : 1001usize , placeholders : & [] , regex : "^Cannot change share type$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/Share20/Manager.php" , line : 1007usize , placeholders : & [] , regex : "^Can only update recipient on user shares$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/Share20/Manager.php" , line : 1013usize , placeholders : & [] , regex : "^Cannot share with the share owner$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/Share20/Manager.php" , line : 1048usize , placeholders : & [] , regex : "^Cannot enable sending the password by Talk with an empty password$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/Share20/Manager.php" , line : 1058usize , placeholders : & [] , regex : "^Cannot enable sending the password by Talk without setting a new password$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/Share20/Manager.php" , line : 1061usize , placeholders : & [] , regex : "^Cannot disable sending the password by Talk without setting a new password$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/Share20/Manager.php" , line : 1143usize , placeholders : & [] , regex : "^Share provider does not support accepting$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/Share20/Manager.php" , line : 1252usize , placeholders : & [] , regex : "^Share does not have a full id$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/Share20/Manager.php" , line : 1307usize , placeholders : & [] , regex : "^Cannot change target of link share$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/Share20/Manager.php" , line : 1311usize , placeholders : & [] , regex : "^Invalid recipient$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/Share20/Manager.php" , line : 1321usize , placeholders : & [] , regex : "^Invalid recipient$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/Share20/Manager.php" , line : 1354usize , placeholders : & [] , regex : "^invalid path$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/Share20/DefaultShareProvider.php" , line : 181usize , placeholders : & [] , regex : "^invalid share type!$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/Share20/DefaultShareProvider.php" , line : 238usize , placeholders : & [] , regex : "^Newly created share could not be found$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/Share20/DefaultShareProvider.php" , line : 365usize , placeholders : & [] , regex : "^Recipient not in receiving group$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/Share20/DefaultShareProvider.php" , line : 395usize , placeholders : & [] , regex : "^Recipient does not match$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/Share20/DefaultShareProvider.php" , line : 400usize , placeholders : & [] , regex : "^Invalid shareType$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/Share20/DefaultShareProvider.php" , line : 532usize , placeholders : & [] , regex : "^Recipient does not match$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/Share20/DefaultShareProvider.php" , line : 538usize , placeholders : & [] , regex : "^Invalid shareType$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/Share20/DefaultShareProvider.php" , line : 1022usize , placeholders : & [] , regex : "^Invalid backend$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/Log.php" , line : 403usize , placeholders : & [] , regex : "^Log implementation has no path$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/Installer.php" , line : 107usize , placeholders : & [] , regex : "^App not found in any app directory$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/Installer.php" , line : 249usize , placeholders : & [] , regex : "^Could not validate CRL signature$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Debug , path : "/lib/private/Installer.php" , line : 541usize , placeholders : & [] , regex : "^Installing shipped apps$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/DirectEditing/Manager.php" , line : 103usize , placeholders : & [] , regex : "^No matching editor found$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/DirectEditing/Manager.php" , line : 135usize , placeholders : & [] , regex : "^File already exists$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/DirectEditing/Manager.php" , line : 138usize , placeholders : & [] , regex : "^Invalid path$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/DirectEditing/Manager.php" , line : 153usize , placeholders : & [] , regex : "^No creator found$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/DirectEditing/Manager.php" , line : 180usize , placeholders : & ["$editorId"] , regex : "^Editor (.*) is unknown$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/DirectEditing/Manager.php" , line : 192usize , placeholders : & [] , regex : "^No default editor found for files mimetype$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/DirectEditing/Manager.php" , line : 200usize , placeholders : & [] , regex : "^Token has already been used and can only be used for followup requests$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/DirectEditing/Manager.php" , line : 223usize , placeholders : & [] , regex : "^No editor found$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/DirectEditing/Manager.php" , line : 236usize , placeholders : & [] , regex : "^Failed to validate the token$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/Remote/Api/OCS.php" , line : 59usize , placeholders : & [] , regex : "^Invalid ocs response$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/AppFramework/Bootstrap/RegistrationContext.php" , line : 487usize , placeholders : & [] , regex : "^Only the Talk app is allowed to register a Talk backend$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/AppFramework/Bootstrap/RegistrationContext.php" , line : 490usize , placeholders : & [] , regex : "^There can only be one Talk backend$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Error , path : "/lib/private/AppFramework/Bootstrap/RegistrationContext.php" , line : 535usize , placeholders : & ["$appId"] , regex : "^App (.*) not loaded for the capability registration$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Error , path : "/lib/private/AppFramework/Bootstrap/RegistrationContext.php" , line : 610usize , placeholders : & ["$appId"] , regex : "^App (.*) not loaded for the container service registration$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Error , path : "/lib/private/AppFramework/Bootstrap/RegistrationContext.php" , line : 638usize , placeholders : & ["$appId"] , regex : "^App (.*) not loaded for the container alias registration$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Error , path : "/lib/private/AppFramework/Bootstrap/RegistrationContext.php" , line : 662usize , placeholders : & ["$appId"] , regex : "^App (.*) not loaded for the container parameter registration$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/AppFramework/Http/Request.php" , line : 207usize , placeholders : & [] , regex : "^You cannot change the contents of the request object$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/AppFramework/Http/Request.php" , line : 215usize , placeholders : & [] , regex : "^You cannot change the contents of the request object$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/AppFramework/Http/Request.php" , line : 224usize , placeholders : & [] , regex : "^You cannot change the contents of the request object$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/AppFramework/Http/Request.php" , line : 289usize , placeholders : & [] , regex : "^You cannot change the contents of the request object$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/AppFramework/Http/Request.php" , line : 741usize , placeholders : & ["$requestUri" , "$scriptName"] , regex : "^The requested uri\\((.*)\\) cannot be processed by the script '(.*)'\\)$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Debug , path : "/lib/private/AppFramework/Http/Dispatcher.php" , line : 145usize , placeholders : & [] , regex : "^Controller \\{class\\}::\\{method\\} created \\{count\\} QueryBuilder objects, please check if they are created inside a loop by accident\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Warn , path : "/lib/private/AppFramework/Http/Dispatcher.php" , line : 153usize , placeholders : & [] , regex : "^Controller \\{class\\}::\\{method\\} executed \\{count\\} queries\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/AppFramework/Routing/RouteParser.php" , line : 96usize , placeholders : & [] , regex : "^Invalid route name: use the format foo\\#bar to reference FooController::bar$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/AppFramework/Routing/RouteConfig.php" , line : 132usize , placeholders : & [] , regex : "^Invalid route name: use the format foo\\#bar to reference FooController::bar$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/AppFramework/Middleware/Security/CORSMiddleware.php" , line : 108usize , placeholders : & [] , regex : "^CORS requires basic auth$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/AppFramework/Middleware/Security/CORSMiddleware.php" , line : 111usize , placeholders : & [] , regex : "^Password login forbidden, use token instead$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/AppFramework/Middleware/PublicShare/PublicShareMiddleware.php" , line : 68usize , placeholders : & [] , regex : "^Link sharing is disabled$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/Template/TemplateFileLocator.php" , line : 46usize , placeholders : & [] , regex : "^Empty template name$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Error , path : "/lib/private/Template/CSSResourceLocator.php" , line : 56usize , placeholders : & [] , regex : "^Could not find resource \\{resource\\} to load$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Error , path : "/lib/private/Template/CSSResourceLocator.php" , line : 90usize , placeholders : & [] , regex : "^ResourceLocator can not find a web root \\(root: \\{root\\}, file: \\{file\\}, webRoot: \\{webRoot\\}, throw: \\{throw\\}\\)$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Error , path : "/lib/private/Template/JSResourceLocator.php" , line : 120usize , placeholders : & [] , regex : "^Could not find resource \\{resource\\} to load$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Error , path : "/lib/private/Template/ResourceLocator.php" , line : 173usize , placeholders : & [] , regex : "^ResourceLocator can not find a web root \\(root: \\{root\\}, file: \\{file\\}, webRoot: \\{webRoot\\}, throw: \\{throw\\}\\)$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/TextProcessing/Manager.php" , line : 115usize , placeholders : & [] , regex : "^No text processing provider is installed that can handle this task$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/TextProcessing/Manager.php" , line : 161usize , placeholders : & [] , regex : "^Could not run task$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/TextProcessing/Manager.php" , line : 170usize , placeholders : & [] , regex : "^No LanguageModel provider is installed that can handle this task$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/TextProcessing/Manager.php" , line : 205usize , placeholders : & [] , regex : "^Could not find task with the provided id$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/TextProcessing/Manager.php" , line : 207usize , placeholders : & [] , regex : "^Could not uniquely identify task with given id$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/TextProcessing/Manager.php" , line : 228usize , placeholders : & [] , regex : "^Could not find task with the provided id and user id$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/TextProcessing/Manager.php" , line : 230usize , placeholders : & [] , regex : "^Could not uniquely identify task with given id and user id$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Warn , path : "/lib/private/TextProcessing/RemoveOldTasksBackgroundJob.php" , line : 56usize , placeholders : & [] , regex : "^Failed to delete stale language model tasks$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/Authentication/Token/Manager.php" , line : 87usize , placeholders : & [] , regex : "^Token conflict handled, but UIDs do not match\\. This should not happen$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Error , path : "/lib/private/Authentication/Token/PublicKeyTokenProvider.php" , line : 110usize , placeholders : & [] , regex : "^Invalid token provided when generating new token$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/Authentication/Token/PublicKeyTokenProvider.php" , line : 160usize , placeholders : & [] , regex : "^Token is too short for a generated token, should be the password during basic auth$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/Authentication/Token/PublicKeyTokenProvider.php" , line : 248usize , placeholders : & [] , regex : "^Invalid token type$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/Authentication/Token/PublicKeyTokenProvider.php" , line : 302usize , placeholders : & [] , regex : "^Invalid token type$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/Authentication/Token/PublicKeyTokenProvider.php" , line : 310usize , placeholders : & [] , regex : "^Invalid token type$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/Authentication/Token/PublicKeyTokenProvider.php" , line : 331usize , placeholders : & [] , regex : "^Invalid token type$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/Authentication/Token/PublicKeyTokenProvider.php" , line : 347usize , placeholders : & [] , regex : "^Invalid token type$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/Authentication/Token/PublicKeyTokenProvider.php" , line : 372usize , placeholders : & [] , regex : "^Invalid token type$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/Authentication/Token/PublicKeyTokenProvider.php" , line : 459usize , placeholders : & [] , regex : "^OpenSSL reported a problem$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/Authentication/Token/PublicKeyTokenProvider.php" , line : 464usize , placeholders : & [] , regex : "^OpenSSL reported a problem$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/Authentication/Token/PublicKeyTokenProvider.php" , line : 476usize , placeholders : & [] , regex : "^Trying to save a password with more than 469 characters is not supported\\. If you want to use big passwords, disable the auth\\.storeCryptedPassword option in config\\.php$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/Authentication/Token/PublicKeyTokenProvider.php" , line : 495usize , placeholders : & [] , regex : "^Invalid token type$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/Authentication/Token/PublicKeyTokenMapper.php" , line : 87usize , placeholders : & [] , regex : "^token does not exist$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/Authentication/Token/PublicKeyTokenMapper.php" , line : 109usize , placeholders : & [] , regex : "^token does not exist$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Debug , path : "/lib/private/Authentication/LoginCredentials/Store.php" , line : 100usize , placeholders : & [] , regex : "^could not get login credentials because session is unavailable$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Debug , path : "/lib/private/Authentication/LoginCredentials/Store.php" , line : 105usize , placeholders : & [] , regex : "^could not get login credentials because the token has no password$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/Authentication/Listeners/UserDeletedFilesCleanupListener.php" , line : 58usize , placeholders : & [] , regex : "^User has no home storage$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/Authentication/Listeners/UserDeletedFilesCleanupListener.php" , line : 71usize , placeholders : & [] , regex : "^UserDeletedEvent fired without matching BeforeUserDeletedEvent$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/Authentication/Listeners/UserDeletedFilesCleanupListener.php" , line : 79usize , placeholders : & [] , regex : "^Home storage has invalid cache$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Warn , path : "/lib/private/Authentication/Listeners/RemoteWipeEmailListener.php" , line : 77usize , placeholders : & ["$uid"] , regex : "^not sending a wipe started email because user <(.*)> does not exist \\(anymore\\)$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Info , path : "/lib/private/Authentication/Listeners/RemoteWipeEmailListener.php" , line : 81usize , placeholders : & ["$uid"] , regex : "^not sending a wipe started email because user <(.*)> has no email set$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Error , path : "/lib/private/Authentication/Listeners/RemoteWipeEmailListener.php" , line : 90usize , placeholders : & ["$uid"] , regex : "^Could not send remote wipe started email to <(.*)>$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Warn , path : "/lib/private/Authentication/Listeners/RemoteWipeEmailListener.php" , line : 98usize , placeholders : & ["$uid"] , regex : "^not sending a wipe finished email because user <(.*)> does not exist \\(anymore\\)$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Info , path : "/lib/private/Authentication/Listeners/RemoteWipeEmailListener.php" , line : 102usize , placeholders : & ["$uid"] , regex : "^not sending a wipe finished email because user <(.*)> has no email set$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Error , path : "/lib/private/Authentication/Listeners/RemoteWipeEmailListener.php" , line : 111usize , placeholders : & ["$uid"] , regex : "^Could not send remote wipe finished email to <(.*)>$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Warn , path : "/lib/private/Authentication/Listeners/RemoteWipeActivityListener.php" , line : 74usize , placeholders : & [] , regex : "^could not publish activity$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/Authentication/TwoFactorAuth/ProviderLoader.php" , line : 73usize , placeholders : & ["$class"] , regex : "^Could not load two\\-factor auth provider (.*)$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Alert , path : "/lib/private/Authentication/TwoFactorAuth/Manager.php" , line : 218usize , placeholders : & ["$providerId"] , regex : "^two\\-factor auth provider '(.*)' failed to load$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Warn , path : "/lib/private/Authentication/TwoFactorAuth/Manager.php" , line : 323usize , placeholders : & [] , regex : "^could not publish activity$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/Authentication/WebAuthn/Manager.php" , line : 156usize , placeholders : & [] , regex : "^Not an authenticator attestation response$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/Authentication/WebAuthn/Manager.php" , line : 229usize , placeholders : & [] , regex : "^Not an authenticator attestation response$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Warn , path : "/lib/private/Authentication/WebAuthn/Manager.php" , line : 255usize , placeholders : & ["$id"] , regex : "^WebAuthn device (.*) does not exist, can't delete it$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Warn , path : "/lib/private/Authentication/Login/UserDisabledCheckCommand.php" , line : 52usize , placeholders : & ["$username" , "$ip"] , regex : "^Login failed: (.*) disabled \\(Remote IP: (.*)\\)$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Warn , path : "/lib/private/Authentication/Login/LoggedInCheckCommand.php" , line : 53usize , placeholders : & ["$loginName" , "$ip"] , regex : "^Login failed: (.*) \\(Remote IP: (.*)\\)$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/L10N/Factory.php" , line : 421usize , placeholders : & [] , regex : "^Failed to get an IUser instance$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Warn , path : "/lib/private/L10N/L10N.php" , line : 237usize , placeholders : & ["$translationFile" , "$jsonError"] , regex : "^Failed to load (.*) \\- json error code: (.*)$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/Avatar/AvatarManager.php" , line : 116usize , placeholders : & [] , regex : "^user does not exist$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Debug , path : "/lib/private/Avatar/AvatarManager.php" , line : 180usize , placeholders : & ["$userId"] , regex : "^No cache for the user (.*)\\. Ignoring avatar deletion$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Error , path : "/lib/private/Avatar/AvatarManager.php" , line : 182usize , placeholders : & ["$userId"] , regex : "^Unable to delete user avatars for (.*)\\. gnoring avatar deletion$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Debug , path : "/lib/private/Avatar/AvatarManager.php" , line : 184usize , placeholders : & ["$userId"] , regex : "^User (.*) not found\\. gnoring avatar deletion$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/Http/Client/DnsPinMiddleware.php" , line : 143usize , placeholders : & [] , regex : "^Host violates local access rules$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/Http/Client/Client.php" , line : 192usize , placeholders : & [] , regex : "^Could not detect any host$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/Http/Client/Client.php" , line : 195usize , placeholders : & [] , regex : "^Host violates local access rules$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Error , path : "/lib/private/Http/Client/GuzzlePromiseAdapter.php" , line : 104usize , placeholders : & [] , regex : "^Unexpected promise state \"\\{state\\}\" returned by Guzzle$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/Http/WellKnown/RequestManager.php" , line : 94usize , placeholders : & [] , regex : "^Well known handlers requested before the apps had been fully registered$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Error , path : "/lib/private/Http/WellKnown/RequestManager.php" , line : 109usize , placeholders : & ["$class"] , regex : "^Well known handler (.*) is invalid$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Error , path : "/lib/private/Http/WellKnown/RequestManager.php" , line : 116usize , placeholders : & ["$class"] , regex : "^Could not load well known handler (.*)$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Error , path : "/lib/private/Cache/File.php" , line : 63usize , placeholders : & [] , regex : "^Can't get cache storage, user not logged in$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/Cache/File.php" , line : 64usize , placeholders : & [] , regex : "^Can\\\\t get cache storage, user not logged in$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/Accounts/AccountPropertyCollection.php" , line : 60usize , placeholders : & [] , regex : "^Provided property does not match collection name$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/Accounts/AccountManager.php" , line : 235usize , placeholders : & [] , regex : "^scope$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/Accounts/AccountManager.php" , line : 244usize , placeholders : & [] , regex : "^scope$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Info , path : "/lib/private/Accounts/AccountManager.php" , line : 499usize , placeholders : & [] , regex : "^Failed to send verification mail$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Critical , path : "/lib/private/Accounts/AccountManager.php" , line : 606usize , placeholders : & [] , regex : "^User data of \\{uid\\} contained invalid JSON \\(error \\{json_error\\}\\), hence falling back to a default user record$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/Accounts/AccountProperty.php" , line : 95usize , placeholders : & [] , regex : "^Invalid scope$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/Accounts/AccountProperty.php" , line : 191usize , placeholders : & [] , regex : "^Provided verification value is invalid$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/Accounts/Account.php" , line : 53usize , placeholders : & [] , regex : "^setProperty cannot set an IAccountsPropertyCollection$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/Accounts/Account.php" , line : 61usize , placeholders : & [] , regex : "^getProperty cannot retrieve an IAccountsPropertyCollection$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/Accounts/Account.php" , line : 158usize , placeholders : & [] , regex : "^Requested collection is not an IAccountPropertyCollection$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/TemplateLayout.php" , line : 392usize , placeholders : & [] , regex : "^\\$filePath is not under the \\\\OC::\\$SERVERROOT$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/Setup.php" , line : 178usize , placeholders : & [] , regex : "^Supported databases are not properly configured\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/Setup.php" , line : 487usize , placeholders : & [] , regex : "^overwrite\\.cli\\.url is empty$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/Setup.php" , line : 490usize , placeholders : & [] , regex : "^invalid value for overwrite\\.cli\\.url$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/App/CompareVersion.php" , line : 52usize , placeholders : & ["$actual"] , regex : "^version specification (.*) is invalid$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/App/CompareVersion.php" , line : 63usize , placeholders : & ["$required"] , regex : "^required version (.*) is invalid$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Error , path : "/lib/private/App/AppManager.php" , line : 407usize , placeholders : & [] , regex : "^/appinfo/app\\.php is not loaded when \\\\OCP\\\\AppFramework\\\\Bootstrap\\\\IBootstrap on the application class is used\\. Migrate everything from app\\.php to the Application class\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Debug , path : "/lib/private/App/AppManager.php" , line : 412usize , placeholders : & [] , regex : "^/appinfo/app\\.php is deprecated, use \\\\OCP\\\\AppFramework\\\\Bootstrap\\\\IBootstrap on the application class instead\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/App/AppManager.php" , line : 582usize , placeholders : & ["$appId"] , regex : "^(.*) can't be enabled for groups\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/App/AppManager.php" , line : 615usize , placeholders : & ["$appId"] , regex : "^(.*) can't be disabled\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/App/AppManager.php" , line : 798usize , placeholders : & ["$shippedJson"] , regex : "^File not found: (.*)$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/App/AppStore/Bundles/BundleFetcher.php" , line : 63usize , placeholders : & [] , regex : "^Bundle with specified identifier does not exist$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Debug , path : "/lib/private/Collaboration/Reference/LinkReferenceProvider.php" , line : 111usize , placeholders : & [] , regex : "^Failed to perform HEAD request to get target metadata$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Debug , path : "/lib/private/Collaboration/Reference/LinkReferenceProvider.php" , line : 116usize , placeholders : & [] , regex : "^Skip resolving links pointing to content length > 5 MB$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Debug , path : "/lib/private/Collaboration/Reference/LinkReferenceProvider.php" , line : 125usize , placeholders : & [] , regex : "^Skip resolving links pointing to content type that is not \"text/html\"$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Debug , path : "/lib/private/Collaboration/Reference/LinkReferenceProvider.php" , line : 131usize , placeholders : & [] , regex : "^Failed to fetch link for obtaining open graph data$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/Collaboration/Collaborators/RemotePlugin.php" , line : 193usize , placeholders : & [] , regex : "^Invalid Federated Cloud ID$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/Collaboration/Collaborators/Search.php" , line : 111usize , placeholders : & [] , regex : "^Provided ShareType is invalid$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/Collaboration/Collaborators/RemoteGroupPlugin.php" , line : 91usize , placeholders : & [] , regex : "^Invalid Federated Cloud ID$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Warn , path : "/lib/private/Collaboration/AutoComplete/Manager.php" , line : 49usize , placeholders : & [] , regex : "^No sorter for ID \"\\{id\\}\", skipping$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Notice , path : "/lib/private/Collaboration/AutoComplete/Manager.php" , line : 66usize , placeholders : & [] , regex : "^Skipping sorter which is not an instance of ISorter\\. Class name: \\{class\\}$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Notice , path : "/lib/private/Collaboration/AutoComplete/Manager.php" , line : 72usize , placeholders : & [] , regex : "^Skipping sorter with empty ID\\. Class name: \\{class\\}$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/Collaboration/Resources/Collection.php" , line : 130usize , placeholders : & [] , regex : "^Already part of the collection$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/Collaboration/Resources/Collection.php" , line : 147usize , placeholders : & [] , regex : "^Already part of the collection$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/Collaboration/Resources/Manager.php" , line : 81usize , placeholders : & [] , regex : "^Collection not found$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/Collaboration/Resources/Manager.php" , line : 113usize , placeholders : & [] , regex : "^Collection not found$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/Collaboration/Resources/Manager.php" , line : 230usize , placeholders : & [] , regex : "^Resource not found$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Debug , path : "/lib/private/Collaboration/Resources/Manager.php" , line : 512usize , placeholders : & [] , regex : "^\\\\OC\\\\Collaboration\\\\Resources\\\\Manager::registerResourceProvider is deprecated$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Warn , path : "/lib/private/Support/Subscription/Registry.php" , line : 214usize , placeholders : & ["$userCount" , "$disabledUsersCount"] , regex : "^Total user count was negative \\(users: (.*), disabled: (.*)\\)$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Warn , path : "/lib/private/Support/Subscription/Registry.php" , line : 242usize , placeholders : & [] , regex : "^The user limit was reached and the new user was not created$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Warn , path : "/lib/private/TempManager.php" , line : 106usize , placeholders : & [] , regex : "^Can not create a temporary file in directory \\{dir\\}\\. Check it exists and has correct permissions$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Warn , path : "/lib/private/TempManager.php" , line : 138usize , placeholders : & [] , regex : "^Can not create a temporary folder in directory \\{dir\\}\\. Check it exists and has correct permissions$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Warn , path : "/lib/private/TempManager.php" , line : 164usize , placeholders : & [] , regex : "^Error deleting temporary file/folder: \\{file\\} \\- Reason: \\{error\\}$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/TempManager.php" , line : 248usize , placeholders : & [] , regex : "^Unable to detect system temporary directory$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Warn , path : "/lib/private/TempManager.php" , line : 266usize , placeholders : & [] , regex : "^Temporary directory \\{dir\\} is not present or writable$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/Translation/TranslationManager.php" , line : 66usize , placeholders : & [] , regex : "^No translation providers available$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/Translation/TranslationManager.php" , line : 98usize , placeholders : & [] , regex : "^Could not detect language$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Warn , path : "/lib/private/Translation/TranslationManager.php" , line : 110usize , placeholders : & ["$fromLanguage" , "$toLanguage" , "$provider->getName()"] , regex : "^Failed to translate from (.*) to (.*) using provider (.*)$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/StreamImage.php" , line : 78usize , placeholders : & [] , regex : "^Not implemented$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/StreamImage.php" , line : 82usize , placeholders : & [] , regex : "^Not implemented$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/StreamImage.php" , line : 86usize , placeholders : & [] , regex : "^Not implemented$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/StreamImage.php" , line : 90usize , placeholders : & [] , regex : "^Not implemented$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/StreamImage.php" , line : 106usize , placeholders : & [] , regex : "^Not implemented$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/StreamImage.php" , line : 110usize , placeholders : & [] , regex : "^Not implemented$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/StreamImage.php" , line : 114usize , placeholders : & [] , regex : "^Not implemented$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/StreamImage.php" , line : 118usize , placeholders : & [] , regex : "^Not implemented$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/StreamImage.php" , line : 122usize , placeholders : & [] , regex : "^Not implemented$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/StreamImage.php" , line : 126usize , placeholders : & [] , regex : "^Not implemented$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/StreamImage.php" , line : 130usize , placeholders : & [] , regex : "^Not implemented$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/StreamImage.php" , line : 134usize , placeholders : & [] , regex : "^Not implemented$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/StreamImage.php" , line : 138usize , placeholders : & [] , regex : "^Not implemented$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/StreamImage.php" , line : 142usize , placeholders : & [] , regex : "^Not implemented$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/StreamImage.php" , line : 146usize , placeholders : & [] , regex : "^Not implemented$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/StreamImage.php" , line : 150usize , placeholders : & [] , regex : "^Not implemented$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/Activity/Event.php" , line : 101usize , placeholders : & [] , regex : "^The given app is invalid$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/Activity/Event.php" , line : 124usize , placeholders : & [] , regex : "^The given type is invalid$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/Activity/Event.php" , line : 147usize , placeholders : & [] , regex : "^The given affected user is invalid$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/Activity/Event.php" , line : 170usize , placeholders : & [] , regex : "^The given author user is invalid$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/Activity/Event.php" , line : 214usize , placeholders : & [] , regex : "^The given subject is invalid$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/Activity/Event.php" , line : 243usize , placeholders : & [] , regex : "^The given parsed subject is invalid$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/Activity/Event.php" , line : 266usize , placeholders : & [] , regex : "^The given parsed subject is invalid$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/Activity/Event.php" , line : 288usize , placeholders : & ["$requiredField"] , regex : "^Invalid rich object, (.*) field is missing$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/Activity/Event.php" , line : 329usize , placeholders : & [] , regex : "^The given message is invalid$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/Activity/Event.php" , line : 415usize , placeholders : & [] , regex : "^The given object type is invalid$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/Activity/Event.php" , line : 418usize , placeholders : & [] , regex : "^The given object name is invalid$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/Activity/Event.php" , line : 457usize , placeholders : & [] , regex : "^The given link is invalid$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/Activity/Event.php" , line : 478usize , placeholders : & [] , regex : "^The given icon is invalid$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/Activity/Manager.php" , line : 106usize , placeholders : & [] , regex : "^The given consumer does not implement the \\\\OCP\\\\Activity\\\\IConsumer interface$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/Activity/Manager.php" , line : 153usize , placeholders : & [] , regex : "^The given event is invalid$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/Activity/Manager.php" , line : 198usize , placeholders : & [] , regex : "^Invalid activity filter registered$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/Activity/Manager.php" , line : 221usize , placeholders : & [] , regex : "^Requested filter does not exist$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/Activity/Manager.php" , line : 248usize , placeholders : & [] , regex : "^Invalid activity provider registered$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/Activity/Manager.php" , line : 286usize , placeholders : & [] , regex : "^Invalid activity filter registered$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/Activity/Manager.php" , line : 309usize , placeholders : & [] , regex : "^Requested setting does not exist$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/Activity/Manager.php" , line : 353usize , placeholders : & [] , regex : "^The given current user is invalid$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/Activity/Manager.php" , line : 387usize , placeholders : & [] , regex : "^The token is invalid$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/Activity/Manager.php" , line : 394usize , placeholders : & [] , regex : "^The token is invalid$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Debug , path : "/lib/private/EventDispatcher/GenericEventWrapper.php" , line : 57usize , placeholders : & [] , regex : "^Deprecated event type for \\{name\\}: \\{class\\} is used$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Debug , path : "/lib/private/EventDispatcher/SymfonyAdapter.php" , line : 108usize , placeholders : & [] , regex : "^Deprecated event type for \\{name\\}: \\{class\\}$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/Comments/Manager.php" , line : 162usize , placeholders : & [] , regex : "^Actor, Object and Verb information must be provided for saving$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/Comments/Manager.php" , line : 167usize , placeholders : & [] , regex : "^Reactions can only be a single emoji$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/Comments/Manager.php" , line : 285usize , placeholders : & [] , regex : "^IDs must be translatable to a number in this implementation\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/Comments/Manager.php" , line : 954usize , placeholders : & [] , regex : "^Parameter must be string$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/Comments/Manager.php" , line : 1027usize , placeholders : & [] , regex : "^Comment related with reaction not found$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/Comments/Manager.php" , line : 1111usize , placeholders : & [] , regex : "^The database does not support reactions$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/Comments/Manager.php" , line : 1381usize , placeholders : & [] , regex : "^Comment to update does ceased to exist$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/Comments/Manager.php" , line : 1596usize , placeholders : & [] , regex : "^String expected\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/Comments/Manager.php" , line : 1599usize , placeholders : & [] , regex : "^Displayname resolver for this type already registered$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/Comments/Manager.php" , line : 1619usize , placeholders : & [] , regex : "^String expected\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/Comments/Manager.php" , line : 1622usize , placeholders : & [] , regex : "^No Displayname resolver for this type registered$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/Comments/Manager.php" , line : 1641usize , placeholders : & [] , regex : "^The given entity does not implement the ICommentsEntity interface$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/Comments/Comment.php" , line : 92usize , placeholders : & [] , regex : "^String expected\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/Comments/Comment.php" , line : 101usize , placeholders : & [] , regex : "^Not allowed to assign a new ID to an already saved comment\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/Comments/Comment.php" , line : 123usize , placeholders : & [] , regex : "^String expected\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/Comments/Comment.php" , line : 149usize , placeholders : & [] , regex : "^String expected\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/Comments/Comment.php" , line : 174usize , placeholders : & [] , regex : "^Integer expected\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/Comments/Comment.php" , line : 201usize , placeholders : & [] , regex : "^String expected\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/Comments/Comment.php" , line : 273usize , placeholders : & [] , regex : "^Non\\-empty String expected\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/Comments/Comment.php" , line : 312usize , placeholders : & [] , regex : "^String expected\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/Comments/Comment.php" , line : 394usize , placeholders : & [] , regex : "^String expected\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/Comments/Comment.php" , line : 424usize , placeholders : & [] , regex : "^Non empty string expected\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/Console/Application.php" , line : 174usize , placeholders : & [] , regex : "^Environment not properly prepared\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/Console/Application.php" , line : 233usize , placeholders : & ["$command"] , regex : "^Console command '(.*)' is unknown and could not be loaded$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/Lockdown/Filesystem/NullStorage.php" , line : 41usize , placeholders : & [] , regex : "^This request is not allowed to access the filesystem$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/Lockdown/Filesystem/NullStorage.php" , line : 45usize , placeholders : & [] , regex : "^This request is not allowed to access the filesystem$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/Lockdown/Filesystem/NullStorage.php" , line : 61usize , placeholders : & [] , regex : "^This request is not allowed to access the filesystem$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/Lockdown/Filesystem/NullStorage.php" , line : 69usize , placeholders : & [] , regex : "^This request is not allowed to access the filesystem$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/Lockdown/Filesystem/NullStorage.php" , line : 105usize , placeholders : & [] , regex : "^This request is not allowed to access the filesystem$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/Lockdown/Filesystem/NullStorage.php" , line : 109usize , placeholders : & [] , regex : "^This request is not allowed to access the filesystem$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/Lockdown/Filesystem/NullStorage.php" , line : 113usize , placeholders : & [] , regex : "^This request is not allowed to access the filesystem$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/Lockdown/Filesystem/NullStorage.php" , line : 117usize , placeholders : & [] , regex : "^This request is not allowed to access the filesystem$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/Lockdown/Filesystem/NullStorage.php" , line : 121usize , placeholders : & [] , regex : "^This request is not allowed to access the filesystem$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/Lockdown/Filesystem/NullStorage.php" , line : 125usize , placeholders : & [] , regex : "^This request is not allowed to access the filesystem$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/Lockdown/Filesystem/NullStorage.php" , line : 129usize , placeholders : & [] , regex : "^This request is not allowed to access the filesystem$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/Lockdown/Filesystem/NullStorage.php" , line : 133usize , placeholders : & [] , regex : "^This request is not allowed to access the filesystem$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/Lockdown/Filesystem/NullStorage.php" , line : 141usize , placeholders : & [] , regex : "^This request is not allowed to access the filesystem$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/Lockdown/Filesystem/NullStorage.php" , line : 165usize , placeholders : & [] , regex : "^This request is not allowed to access the filesystem$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/Lockdown/Filesystem/NullStorage.php" , line : 169usize , placeholders : & [] , regex : "^This request is not allowed to access the filesystem$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/Lockdown/Filesystem/NullCache.php" , line : 66usize , placeholders : & [] , regex : "^This request is not allowed to access the filesystem$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/Lockdown/Filesystem/NullCache.php" , line : 70usize , placeholders : & [] , regex : "^This request is not allowed to access the filesystem$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/Lockdown/Filesystem/NullCache.php" , line : 74usize , placeholders : & [] , regex : "^This request is not allowed to access the filesystem$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/Lockdown/Filesystem/NullCache.php" , line : 90usize , placeholders : & [] , regex : "^This request is not allowed to access the filesystem$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/Lockdown/Filesystem/NullCache.php" , line : 94usize , placeholders : & [] , regex : "^This request is not allowed to access the filesystem$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/Lockdown/Filesystem/NullCache.php" , line : 98usize , placeholders : & [] , regex : "^This request is not allowed to access the filesystem$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/Lockdown/Filesystem/NullCache.php" , line : 130usize , placeholders : & [] , regex : "^This request is not allowed to access the filesystem$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/Talk/Broker.php" , line : 66usize , placeholders : & [] , regex : "^Not all apps have been registered yet$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/Talk/Broker.php" , line : 100usize , placeholders : & [] , regex : "^The Talk broker has no registered backend$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/Talk/Broker.php" , line : 112usize , placeholders : & [] , regex : "^The Talk broker has no registered backend$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Warn , path : "/lib/private/Search.php" , line : 68usize , placeholders : & [] , regex : "^Ignoring Unknown search provider$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/BackgroundJob/JobList.php" , line : 71usize , placeholders : & [] , regex : "^Background job arguments can't exceed 4000 characters \\(json encoded\\)$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Debug , path : "/lib/private/BackgroundJob/JobList.php" , line : 422usize , placeholders : & [] , regex : "^Querying reserved jobs failed$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/Dashboard/Manager.php" , line : 55usize , placeholders : & [] , regex : "^Dashboard widget with this id has already been registered$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Info , path : "/lib/private/Dashboard/Manager.php" , line : 118usize , placeholders : & [] , regex : "^Dashboard widget \\{widget\\} took \\{duration\\} seconds to load\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/Memcache/Memcached.php" , line : 84usize , placeholders : & ["$options"] , regex : "^Expected 'memcached_options' config to be an array, got (.*)$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/DB/MigrationService.php" , line : 242usize , placeholders : & ["$version"] , regex : "^Cannot load a migrations with the name '(.*)' because it is a reserved number$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/DB/MigrationService.php" , line : 388usize , placeholders : & ["$version"] , regex : "^Version (.*) is unknown\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Debug , path : "/lib/private/DB/MigrationService.php" , line : 409usize , placeholders : & [] , regex : "^Migrating schema only$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Debug , path : "/lib/private/DB/MigrationService.php" , line : 452usize , placeholders : & [] , regex : "^\\- Checking target database schema$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Debug , path : "/lib/private/DB/MigrationService.php" , line : 459usize , placeholders : & [] , regex : "^\\- Migrate database schema$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Debug , path : "/lib/private/DB/MigrationService.php" , line : 464usize , placeholders : & [] , regex : "^\\- Mark migrations as executed$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/DB/MigrationService.php" , line : 499usize , placeholders : & [] , regex : "^Not a valid migration$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/DB/MigrationService.php" , line : 505usize , placeholders : & ["$class"] , regex : "^Migration step '(.*)' is unknown$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/DB/AdapterOCI8.php" , line : 30usize , placeholders : & [] , regex : "^Oracle requires a table name to be passed into lastInsertId\\(\\)$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/DB/PreparedStatement.php" , line : 98usize , placeholders : & [] , regex : "^You have to execute the prepared statement before accessing the results$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/DB/Connection.php" , line : 98usize , placeholders : & [] , regex : "^adapter not set$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/DB/Connection.php" , line : 101usize , placeholders : & [] , regex : "^tablePrefix not set$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Debug , path : "/lib/private/DB/Connection.php" , line : 178usize , placeholders : & [] , regex : "^Doctrine QueryBuilder retrieved in \\{backtrace\\}$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Debug , path : "/lib/private/DB/Connection.php" , line : 191usize , placeholders : & [] , regex : "^Doctrine ExpressionBuilder retrieved in \\{backtrace\\}$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/DB/Connection.php" , line : 450usize , placeholders : & [] , regex : "^Can not lock a new table until the previous lock is released\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/DB/ResultAdapter.php" , line : 55usize , placeholders : & [] , regex : "^Fetch mode needs to be assoc, num or column\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/DB/ConnectionFactory.php" , line : 107usize , placeholders : & ["$type"] , regex : "^Unsupported type: (.*)$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Debug , path : "/lib/private/DB/QueryBuilder/QueryBuilder.php" , line : 216usize , placeholders : & [] , regex : "^DB QueryBuilder: '\\{query\\}'$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Debug , path : "/lib/private/DB/QueryBuilder/QueryBuilder.php" , line : 221usize , placeholders : & [] , regex : "^DB QueryBuilder: '\\{query\\}' with parameters: \\{params\\}$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Error , path : "/lib/private/DB/QueryBuilder/QueryBuilder.php" , line : 229usize , placeholders : & [] , regex : "^DB QueryBuilder: error trying to log SQL query$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/DB/QueryBuilder/QueryBuilder.php" , line : 289usize , placeholders : & [] , regex : "^Invalid query type, expected SELECT query$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/DB/QueryBuilder/QueryBuilder.php" , line : 302usize , placeholders : & [] , regex : "^Invalid return type for query$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/DB/QueryBuilder/QueryBuilder.php" , line : 322usize , placeholders : & [] , regex : "^Invalid query type, expected INSERT, DELETE or UPDATE statement$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/DB/QueryBuilder/QueryBuilder.php" , line : 332usize , placeholders : & [] , regex : "^Invalid return type for statement$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/DB/QueryBuilder/QueryBuilder.php" , line : 1295usize , placeholders : & [] , regex : "^Invalid call to getLastInsertId without using insert\\(\\) before\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/DB/QueryBuilder/QuoteHelper.php" , line : 61usize , placeholders : & [] , regex : "^Only strings, Literals and Parameters are allowed$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Warn , path : "/lib/private/OCM/OCMDiscoveryService.php" , line : 105usize , placeholders : & [] , regex : "^error while discovering ocm provider$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/OCM/OCMDiscoveryService.php" , line : 109usize , placeholders : & [] , regex : "^error while requesting remote ocm provider$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/OCM/OCMDiscoveryService.php" , line : 113usize , placeholders : & [] , regex : "^API version not supported$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/OCM/Model/OCMProvider.php" , line : 139usize , placeholders : & [] , regex : "^protocol not found$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/OCM/Model/OCMProvider.php" , line : 146usize , placeholders : & [] , regex : "^resource not found$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/OCM/Model/OCMProvider.php" , line : 171usize , placeholders : & [] , regex : "^remote provider does not look valid$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/RedisFactory.php" , line : 59usize , placeholders : & [] , regex : "^Redis Cluster support is not available$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/RedisFactory.php" , line : 90usize , placeholders : & [] , regex : "^Redis cluster config is missing the \"seeds\" attribute$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/RedisFactory.php" , line : 175usize , placeholders : & [] , regex : "^Redis support is not available$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/Command/CronBus.php" , line : 61usize , placeholders : & [] , regex : "^Invalid command$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/Command/CronBus.php" , line : 75usize , placeholders : & [] , regex : "^Invalid command$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/Command/ClosureJob.php" , line : 36usize , placeholders : & [] , regex : "^Invalid serialized callable$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/Command/CommandJob.php" , line : 37usize , placeholders : & [] , regex : "^Invalid serialized command$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/Command/QueueBus.php" , line : 60usize , placeholders : & [] , regex : "^Trying to push a command which serialized form can not be stored in the database \\(>4000 character\\)$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/Command/CallableJob.php" , line : 32usize , placeholders : & [] , regex : "^Invalid serialized callable$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/Updater/ChangesCheck.php" , line : 60usize , placeholders : & [] , regex : "^Unable to decode changes info$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Debug , path : "/lib/private/Updater/ChangesCheck.php" , line : 105usize , placeholders : & [] , regex : "^Unexpected return code \\{code\\} from changelog server$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/Updater/ChangesMapper.php" , line : 58usize , placeholders : & [] , regex : "^Changes info is not present$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/FullTextSearch/FullTextSearchManager.php" , line : 101usize , placeholders : & [] , regex : "^No IProviderService registered$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/FullTextSearch/FullTextSearchManager.php" , line : 114usize , placeholders : & [] , regex : "^No IIndexService registered$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/FullTextSearch/FullTextSearchManager.php" , line : 127usize , placeholders : & [] , regex : "^No ISearchService registered$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/Session/Memory.php" , line : 105usize , placeholders : & [] , regex : "^Memory session does not have an ID$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/Session/Memory.php" , line : 124usize , placeholders : & [] , regex : "^Session has been closed \\- no further changes to the session are allowed$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Critical , path : "/lib/private/Session/CryptoSessionData.php" , line : 96usize , placeholders : & [] , regex : "^Could not decrypt or decode encrypted session data$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/Session/Internal.php" , line : 62usize , placeholders : & [] , regex : "^Failed to start session$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/Session/Internal.php" , line : 206usize , placeholders : & [] , regex : "^Session has been closed \\- no further changes to the session are allowed$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/IntegrityCheck/Helpers/AppLocator.php" , line : 47usize , placeholders : & [] , regex : "^App not found$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/IntegrityCheck/Checker.php" , line : 337usize , placeholders : & [] , regex : "^Signature data not found\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/IntegrityCheck/Checker.php" , line : 355usize , placeholders : & [] , regex : "^Certificate is not valid\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/IntegrityCheck/Checker.php" , line : 372usize , placeholders : & [] , regex : "^Signature could not get verified\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/IntegrityCheck/Checker.php" , line : 417usize , placeholders : & [] , regex : "^Invalid behaviour in file hash comparison experienced\\. Please report this error to the developers\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/Config.php" , line : 280usize , placeholders : & [] , regex : "^Can't write into config directory!$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/Config.php" , line : 316usize , placeholders : & [] , regex : "^Config is set to be read\\-only via option \"config_is_read_only\"\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/Notification/Action.php" , line : 64usize , placeholders : & [] , regex : "^The given label is invalid$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/Notification/Action.php" , line : 86usize , placeholders : & [] , regex : "^The given parsed label is invalid$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/Notification/Action.php" , line : 127usize , placeholders : & [] , regex : "^The given link is invalid$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/Notification/Action.php" , line : 136usize , placeholders : & [] , regex : "^The given request type is invalid$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/Notification/Notification.php" , line : 133usize , placeholders : & [] , regex : "^The given app name is invalid$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/Notification/Notification.php" , line : 155usize , placeholders : & [] , regex : "^The given user id is invalid$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/Notification/Notification.php" , line : 177usize , placeholders : & [] , regex : "^The given date time is invalid$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/Notification/Notification.php" , line : 200usize , placeholders : & [] , regex : "^The given object type is invalid$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/Notification/Notification.php" , line : 205usize , placeholders : & [] , regex : "^The given object id is invalid$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/Notification/Notification.php" , line : 236usize , placeholders : & [] , regex : "^The given subject is invalid$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/Notification/Notification.php" , line : 269usize , placeholders : & [] , regex : "^The given parsed subject is invalid$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/Notification/Notification.php" , line : 292usize , placeholders : & [] , regex : "^The given parsed subject is invalid$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/Notification/Notification.php" , line : 315usize , placeholders : & ["$requiredField"] , regex : "^Invalid rich object, (.*) field is missing$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/Notification/Notification.php" , line : 354usize , placeholders : & [] , regex : "^The given message is invalid$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/Notification/Notification.php" , line : 387usize , placeholders : & [] , regex : "^The given parsed message is invalid$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/Notification/Notification.php" , line : 410usize , placeholders : & [] , regex : "^The given parsed message is invalid$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/Notification/Notification.php" , line : 447usize , placeholders : & [] , regex : "^The given link is invalid$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/Notification/Notification.php" , line : 469usize , placeholders : & [] , regex : "^The given icon is invalid$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/Notification/Notification.php" , line : 499usize , placeholders : & [] , regex : "^The given action is invalid$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/Notification/Notification.php" , line : 504usize , placeholders : & [] , regex : "^The notification already has a primary action$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/Notification/Notification.php" , line : 530usize , placeholders : & [] , regex : "^The given parsed action is invalid$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/Notification/Notification.php" , line : 535usize , placeholders : & [] , regex : "^The notification already has a primary action$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/Notification/Manager.php" , line : 322usize , placeholders : & [] , regex : "^The given notification is invalid$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/Notification/Manager.php" , line : 373usize , placeholders : & [] , regex : "^The given notification has been processed$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/Notification/Manager.php" , line : 377usize , placeholders : & [] , regex : "^The given notification has not been handled$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/Notification/Manager.php" , line : 382usize , placeholders : & [] , regex : "^The given notification has not been handled$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/Security/CSRF/TokenStorage/SessionStorage.php" , line : 64usize , placeholders : & [] , regex : "^Session does not contain a requesttoken$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/Security/Crypto.php" , line : 106usize , placeholders : & [] , regex : "^Encrypting failed\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/Security/Crypto.php" , line : 146usize , placeholders : & [] , regex : "^Authenticated ciphertext could not be decoded\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/Security/Crypto.php" , line : 169usize , placeholders : & [] , regex : "^HMAC does not match\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/Security/Crypto.php" , line : 174usize , placeholders : & [] , regex : "^Decryption failed$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/Security/IdentityProof/Manager.php" , line : 77usize , placeholders : & [] , regex : "^OpenSSL reported a problem$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/Security/IdentityProof/Manager.php" , line : 82usize , placeholders : & [] , regex : "^OpenSSL reported a problem$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/Security/IdentityProof/Manager.php" , line : 158usize , placeholders : & [] , regex : "^no instance id!$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/Security/Certificate.php" , line : 61usize , placeholders : & [] , regex : "^Certificate could not get parsed\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/Security/Certificate.php" , line : 66usize , placeholders : & [] , regex : "^Certificate could not get parsed\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Error , path : "/lib/private/Security/CertificateManager.php" , line : 90usize , placeholders : & ["$path"] , regex : "^Failed to read certificate from (.*)$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Error , path : "/lib/private/Security/CertificateManager.php" , line : 93usize , placeholders : & ["$path"] , regex : "^Failed to read certificate from (.*)$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/Security/CertificateManager.php" , line : 186usize , placeholders : & [] , regex : "^Filename is not valid$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Error , path : "/lib/private/Security/CertificateManager.php" , line : 259usize , placeholders : & [] , regex : "^Failed to get absolute bundle path\\. Fallback to default ca\\-bundle\\.crt$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Error , path : "/lib/private/Security/Bruteforce/Throttler.php" , line : 194usize , placeholders : & [] , regex : "^Bruteforce has to use less than 48 hours$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Info , path : "/lib/private/Security/Bruteforce/Throttler.php" , line : 295usize , placeholders : & [] , regex : "^IP address blocked because it reached the maximum failed attempts in the last 30 minutes \\[action: \\{action\\}, ip: \\{ip\\}\\]$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/Security/Bruteforce/Throttler.php" , line : 300usize , placeholders : & [] , regex : "^Reached maximum delay$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Info , path : "/lib/private/Security/Bruteforce/Throttler.php" , line : 303usize , placeholders : & [] , regex : "^IP address throttled because it reached the attempts limit in the last 30 minutes \\[action: \\{action\\}, delay: \\{delay\\}, ip: \\{ip\\}\\]$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Warn , path : "/lib/private/Security/RemoteHostValidator.php" , line : 74usize , placeholders : & ["$host"] , regex : "^Host (.*) was not connected to because it violates local access rules$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/Route/Router.php" , line : 319usize , placeholders : & [] , regex : "^not a callable action$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/Route/Router.php" , line : 329usize , placeholders : & [] , regex : "^no action available$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/Search/Result/File.php" , line : 136usize , placeholders : & [] , regex : "^Search result not in user folder$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/Search/SearchComposer.php" , line : 157usize , placeholders : & ["$providerId"] , regex : "^Provider (.*) is unknown$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Error , path : "/lib/private/Metadata/FileEventListener.php" , line : 82usize , placeholders : & [] , regex : "^Detecting deletion of a file with possible metadata but file system setup is not setup$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/Metadata/FileMetadataMapper.php" , line : 121usize , placeholders : & [] , regex : "^Entity should be a FileMetadata entity$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/Metadata/FileMetadataMapper.php" , line : 127usize , placeholders : & [] , regex : "^Entity which should be updated has no id$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/Metadata/FileMetadataMapper.php" , line : 133usize , placeholders : & [] , regex : "^Entity which should be updated has no group_name$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/Contacts/ContactsMenu/ActionProviderStore.php" , line : 74usize , placeholders : & [] , regex : "^Could not load contacts menu action provider$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/Files/FileInfo.php" , line : 106usize , placeholders : & [] , regex : "^Null offset not supported$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Warn , path : "/lib/private/Files/Cache/Updater.php" , line : 269usize , placeholders : & [] , regex : "^Error while updating parent storage_mtime, should be safe to ignore$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/Files/Cache/SearchBuilder.php" , line : 109usize , placeholders : & [] , regex : "^Binary operators inside \"not\" is not supported$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/Files/Cache/FailedCache.php" , line : 142usize , placeholders : & [] , regex : "^Invalid cache$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Debug , path : "/lib/private/Files/Cache/Scanner.php" , line : 127usize , placeholders : & ["$path"] , regex : "^!!! Path '(.*)' is not accessible or present !!!$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Warn , path : "/lib/private/Files/Cache/Propagator.php" , line : 146usize , placeholders : & [] , regex : "^Retrying propagation query after retryable exception\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/Files/Cache/Propagator.php" , line : 194usize , placeholders : & [] , regex : "^Not in batch$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/Files/Cache/QuerySearchHelper.php" , line : 213usize , placeholders : & [] , regex : "^This search operation requires the user to be set in the query$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/Files/Cache/Cache.php" , line : 348usize , placeholders : & [] , regex : "^File entry could not be inserted but could also not be selected with getId\\(\\) in order to perform an update\\. Please try again\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/Files/Cache/Cache.php" , line : 1175usize , placeholders : & [] , regex : "^Invalid source cache entry on copyFromCache$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Error , path : "/lib/private/Files/Mount/RootMountProvider.php" , line : 59usize , placeholders : & [] , regex : "^No class given for objectstore$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Error , path : "/lib/private/Files/Mount/ObjectStorePreviewCacheMountProvider.php" , line : 106usize , placeholders : & [] , regex : "^No class given for objectstore$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Error , path : "/lib/private/Files/Mount/ObjectStorePreviewCacheMountProvider.php" , line : 135usize , placeholders : & [] , regex : "^No class given for objectstore$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/Files/Mount/Manager.php" , line : 109usize , placeholders : & [] , regex : "^No mounts even after explicitly setting up the root mounts$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Error , path : "/lib/private/Files/Mount/ObjectHomeMountProvider.php" , line : 83usize , placeholders : & [] , regex : "^No class given for objectstore$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Error , path : "/lib/private/Files/Mount/ObjectHomeMountProvider.php" , line : 108usize , placeholders : & [] , regex : "^No class given for objectstore$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/Files/Mount/MountPoint.php" , line : 132usize , placeholders : & ["$mountProvider"] , regex : "^Mount provider (.*) name exceeds the limit of 128 characters$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/Files/Mount/MountPoint.php" , line : 175usize , placeholders : & [] , regex : "^The root storage could not be initialized\\. Please contact your local administrator\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/Files/View.php" , line : 464usize , placeholders : & [] , regex : "^fseek error$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/Files/View.php" , line : 472usize , placeholders : & ["$connectionStatus"] , regex : "^Connection lost\\. Status: (.*)$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Info , path : "/lib/private/Files/View.php" , line : 558usize , placeholders : & [] , regex : "^Error while setting modified time$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/Files/View.php" , line : 632usize , placeholders : & ["$absolutePath"] , regex : "^Path (.*) is not in the expected root$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/Files/View.php" , line : 737usize , placeholders : & [] , regex : "^Moving a folder into a child folder is forbidden$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Info , path : "/lib/private/Files/View.php" , line : 990usize , placeholders : & [] , regex : "^Trying to open a file with a mode other than \"r\" or \"w\" can cause severe performance issues with some backends$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/Files/View.php" , line : 1788usize , placeholders : & ["$pathLen" , "$maxLen" , "$path"] , regex : "^Path length\\((.*)\\) exceeds max path length\\((.*)\\): (.*)$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Debug , path : "/lib/private/Files/View.php" , line : 1815usize , placeholders : & [] , regex : "^It is not allowed to move one mount point into a shared folder$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Error , path : "/lib/private/Files/View.php" , line : 2135usize , placeholders : & [] , regex : "^\\$absolutePath must be relative to \"files\", value is \"\\{absolutePath\\}\"$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/Files/View.php" , line : 2141usize , placeholders : & [] , regex : "^\\$absolutePath must be relative to \"files\"$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/Files/Type/Loader.php" , line : 142usize , placeholders : & [] , regex : "^Database threw an unique constraint on inserting a new mimetype, but couldn't return the ID for this very mimetype$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Warn , path : "/lib/private/Files/Filesystem.php" , line : 203usize , placeholders : & [] , regex : "^Storage wrapper '\\{wrapper\\}' was not registered via the 'OC_Filesystem \\- preSetup' hook which could cause potential problems\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/Files/Node/File.php" , line : 74usize , placeholders : & [] , regex : "^file_put_contents failed$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/Files/Node/Root.php" , line : 385usize , placeholders : & ["$userId"] , regex : "^User folder for (.*) exists as a file$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/Files/Node/Root.php" , line : 456usize , placeholders : & [] , regex : "^getByIdInPath with non folder$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/Files/Node/Node.php" , line : 76usize , placeholders : & [] , regex : "^The view passed to the node should not have any fake root set$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/Files/Node/Node.php" , line : 94usize , placeholders : & [] , regex : "^Must be implemented by subclasses$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/Files/Node/Node.php" , line : 177usize , placeholders : & [] , regex : "^No storage for node$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/Files/Node/Folder.php" , line : 181usize , placeholders : & [] , regex : "^Could not create as provided path is empty$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/Files/Node/Folder.php" , line : 229usize , placeholders : & [] , regex : "^searching by owner is only allowed in the users home folder$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/Files/Node/LazyUserFolder.php" , line : 91usize , placeholders : & [] , regex : "^No mountpoint for user folder$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/Files/Config/CachedMountInfo.php" , line : 66usize , placeholders : & ["$mountProvider"] , regex : "^Mount provider (.*) name exceeds the limit of 128 characters$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/Files/Config/MountProviderCollection.php" , line : 243usize , placeholders : & [] , regex : "^No root mounts provided by any provider$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/Files/Lock/LockManager.php" , line : 17usize , placeholders : & [] , regex : "^There is already a registered lock provider$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/Files/Lock/LockManager.php" , line : 51usize , placeholders : & [] , regex : "^No lock provider available$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/Files/Lock/LockManager.php" , line : 59usize , placeholders : & [] , regex : "^No lock provider available$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/Files/Lock/LockManager.php" , line : 67usize , placeholders : & [] , regex : "^No lock provider available$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/Files/AppData/AppData.php" , line : 67usize , placeholders : & [] , regex : "^no instance id!$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/Files/AppData/AppData.php" , line : 84usize , placeholders : & [] , regex : "^Could not get appdata folder$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/Files/SimpleFS/SimpleFile.php" , line : 122usize , placeholders : & [] , regex : "^File does not exist$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/Files/SimpleFS/NewSimpleFile.php" , line : 140usize , placeholders : & [] , regex : "^File not set$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/Files/SimpleFS/NewSimpleFile.php" , line : 156usize , placeholders : & [] , regex : "^File does not exist$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/Files/Storage/Local.php" , line : 80usize , placeholders : & [] , regex : "^No data directory set for local storage$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Error , path : "/lib/private/Files/Storage/Local.php" , line : 543usize , placeholders : & ["$fullPath" , "$realPath" , "$this->realDataDir"] , regex : "^Following symlinks is not allowed \\('(.*)' \\-> '(.*)' not inside '(.*)'\\)$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/Files/Storage/Local.php" , line : 544usize , placeholders : & [] , regex : "^Following symlinks is not allowed$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/Files/Storage/Local.php" , line : 659usize , placeholders : & ["$path"] , regex : "^Failed write stream to (.*)$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/Files/Storage/Wrapper/Quota.php" , line : 68usize , placeholders : & [] , regex : "^No quota or quota callback provider$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Error , path : "/lib/private/Files/Storage/Wrapper/Encryption.php" , line : 810usize , placeholders : & [] , regex : "^Could not find mount point, can't keep encryption keys$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Warn , path : "/lib/private/Files/Storage/Common.php" , line : 247usize , placeholders : & ["$source" , "$target"] , regex : "^Failed to write data while copying (.*) to (.*)$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Info , path : "/lib/private/Files/Storage/Common.php" , line : 469usize , placeholders : & [] , regex : "^External storage not available: stat\\(\\) failed$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Warn , path : "/lib/private/Files/Storage/Common.php" , line : 639usize , placeholders : & [] , regex : "^Failed to copy stream to storage$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/Files/Storage/Common.php" , line : 882usize , placeholders : & ["$path"] , regex : "^Failed to open (.*) for writing$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/Files/Storage/Common.php" , line : 887usize , placeholders : & [] , regex : "^Failed to copy stream$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/Files/Storage/Common.php" , line : 900usize , placeholders : & [] , regex : "^Directory listing failed$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/Files/Storage/StorageFactory.php" , line : 102usize , placeholders : & [] , regex : "^Invalid result from storage wrapper$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/Files/Storage/FailedStorage.php" , line : 46usize , placeholders : & [] , regex : "^Missing \"exception\" argument in FailedStorage constructor$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/Files/Storage/DAV.php" , line : 151usize , placeholders : & [] , regex : "^Invalid webdav storage configuration$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/Files/Storage/DAV.php" , line : 817usize , placeholders : & [] , regex : "^root is gone$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/Files/Utils/Scanner.php" , line : 202usize , placeholders : & [] , regex : "^Invalid path to scan$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/Files/Utils/Scanner.php" , line : 226usize , placeholders : & ["$fullPath"] , regex : "^User folder (.*) exists in cache but not on disk$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/Files/Utils/Scanner.php" , line : 232usize , placeholders : & ["$fullPath" , "$owner" , "$permissions"] , regex : "^User folder (.*) is not writable, folders is owned by (.*) and has mode (.*)$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/Files/ObjectStore/HomeObjectStoreStorage.php" , line : 38usize , placeholders : & [] , regex : "^missing user object in parameters$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/Files/ObjectStore/AppdataPreviewObjectStoreStorage.php" , line : 34usize , placeholders : & [] , regex : "^missing id in parameters$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/Files/ObjectStore/S3ConnectionTrait.php" , line : 83usize , placeholders : & [] , regex : "^Bucket has to be configured\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Debug , path : "/lib/private/Files/ObjectStore/S3ConnectionTrait.php" , line : 186usize , placeholders : & [] , regex : "^Invalid remote storage\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/Files/ObjectStore/Swift.php" , line : 120usize , placeholders : & ["$urn"] , regex : "^object (.*) not found in object store$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/Files/ObjectStore/SwiftFactory.php" , line : 82usize , placeholders : & [] , regex : "^Unauthenticated ObjectStore connection$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/Files/ObjectStore/SwiftFactory.php" , line : 162usize , placeholders : & [] , regex : "^Scope has to be defined for V3 requests$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Debug , path : "/lib/private/Files/ObjectStore/SwiftFactory.php" , line : 189usize , placeholders : & [] , regex : "^Cached token for swift expired$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Debug , path : "/lib/private/Files/ObjectStore/SwiftFactory.php" , line : 203usize , placeholders : & [] , regex : "^Cached token for swift expired$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/Files/ObjectStore/SwiftFactory.php" , line : 217usize , placeholders : & [] , regex : "^Failed to connect to keystone, verify the keystone url$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/Files/ObjectStore/SwiftFactory.php" , line : 221usize , placeholders : & [] , regex : "^Keystone not found, verify the keystone url$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/Files/ObjectStore/SwiftFactory.php" , line : 223usize , placeholders : & [] , regex : "^Precondition failed, verify the keystone url$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/Files/ObjectStore/SwiftFactory.php" , line : 225usize , placeholders : & [] , regex : "^Authentication failed, verify the username, password and possibly tenant$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/Files/ObjectStore/SwiftFactory.php" , line : 227usize , placeholders : & [] , regex : "^Unknown error$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/Files/ObjectStore/SwiftFactory.php" , line : 230usize , placeholders : & [] , regex : "^Connection reset while connecting to keystone, verify the keystone url$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/Files/ObjectStore/SwiftFactory.php" , line : 276usize , placeholders : & [] , regex : "^Invalid response while trying to get container info$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Error , path : "/lib/private/Files/ObjectStore/SwiftFactory.php" , line : 282usize , placeholders : & ["$host"] , regex : "^Can't connect to object storage server at (.*)$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/Files/ObjectStore/SwiftFactory.php" , line : 283usize , placeholders : & ["$host"] , regex : "^Can't connect to object storage server at (.*)$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/Files/ObjectStore/ObjectStoreStorage.php" , line : 78usize , placeholders : & [] , regex : "^missing IObjectStore instance$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Warn , path : "/lib/private/Files/ObjectStore/ObjectStoreStorage.php" , line : 98usize , placeholders : & ["$path"] , regex : "^Tried to create an object store folder that already exists: (.*)$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Warn , path : "/lib/private/Files/ObjectStore/ObjectStoreStorage.php" , line : 122usize , placeholders : & ["$parent"] , regex : "^Parent folder \\((.*)\\) doesn't exist and couldn't be created$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Warn , path : "/lib/private/Files/ObjectStore/ObjectStoreStorage.php" , line : 127usize , placeholders : & ["$parent"] , regex : "^Parent \\((.*)\\) is a file$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/Files/ObjectStore/ObjectStoreStorage.php" , line : 570usize , placeholders : & ["$urn" , "$path"] , regex : "^Object not found after writing \\(urn: (.*), path: (.*)\\)$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/Files/ObjectStore/ObjectStoreStorage.php" , line : 614usize , placeholders : & [] , regex : "^Source object not found$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/Files/ObjectStore/ObjectStoreStorage.php" , line : 645usize , placeholders : & [] , regex : "^Invalid source cache for object store copy$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/Files/ObjectStore/ObjectStoreStorage.php" , line : 663usize , placeholders : & [] , regex : "^Object store does not support multipart upload$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/Files/ObjectStore/ObjectStoreStorage.php" , line : 682usize , placeholders : & [] , regex : "^Object store does not support multipart upload$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/Files/ObjectStore/ObjectStoreStorage.php" , line : 698usize , placeholders : & [] , regex : "^Object store does not support multipart upload$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/Files/ObjectStore/ObjectStoreStorage.php" , line : 721usize , placeholders : & [] , regex : "^Could not write chunked file$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/Files/ObjectStore/ObjectStoreStorage.php" , line : 728usize , placeholders : & [] , regex : "^Object store does not support multipart upload$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/Files/ObjectStore/S3.php" , line : 55usize , placeholders : & [] , regex : "^No upload id returned$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/Files/ObjectStore/S3ObjectTrait.php" , line : 93usize , placeholders : & ["$urn"] , regex : "^Failed to read object (.*)$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/Files/ObjectStore/S3ObjectTrait.php" , line : 147usize , placeholders : & [] , regex : "^Error while uploading to S3 bucket$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Debug , path : "/lib/private/Files/SetupManager.php" , line : 444usize , placeholders : & [] , regex : "^mount has no provider set, performing full setup$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Warn , path : "/lib/private/InitialStateService.php" , line : 96usize , placeholders : & [] , regex : "^Lazy initial state provider for \\{key\\} took \\{duration\\} seconds\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/User/Database.php" , line : 228usize , placeholders : & [] , regex : "^Invalid displayname$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/User/Database.php" , line : 504usize , placeholders : & [] , regex : "^key uid is expected to be set in \\$param$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/User/User.php" , line : 226usize , placeholders : & [] , regex : "^Only verified emails can be set as primary$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Debug , path : "/lib/private/User/Session.php" , line : 796usize , placeholders : & [] , regex : "^Session token is invalid because it does not exist$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Warn , path : "/lib/private/User/Session.php" , line : 809usize , placeholders : & [] , regex : "^Session token credentials are invalid$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Error , path : "/lib/private/User/Session.php" , line : 832usize , placeholders : & [] , regex : "^App token login name does not match$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Info , path : "/lib/private/User/Session.php" , line : 912usize , placeholders : & [] , regex : "^Tried to log in but could not verify token$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Debug , path : "/lib/private/User/Session.php" , line : 922usize , placeholders : & [] , regex : "^Remember\\-me token replaced$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Debug , path : "/lib/private/User/Session.php" , line : 930usize , placeholders : & [] , regex : "^Session token replaced$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Critical , path : "/lib/private/User/Session.php" , line : 935usize , placeholders : & [] , regex : "^Could not renew session token for \\{uid\\} because the session is unavailable$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Debug , path : "/lib/private/User/Session.php" , line : 987usize , placeholders : & [] , regex : "^Session token invalidated before logout$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Debug , path : "/lib/private/User/Session.php" , line : 993usize , placeholders : & [] , regex : "^Logging out$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/Log/Systemdlog.php" , line : 63usize , placeholders : & [] , regex : "^PHP extension php\\-systemd is not available\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/Log/PsrLoggerAdapter.php" , line : 254usize , placeholders : & [] , regex : "^Nextcloud allows only integer log levels$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Warn , path : "/lib/private/Calendar/Manager.php" , line : 265usize , placeholders : & [] , regex : "^Wrong method provided for processing$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Warn , path : "/lib/private/Calendar/Manager.php" , line : 273usize , placeholders : & [] , regex : "^Recipient and ORGANIZER must be identical$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Warn , path : "/lib/private/Calendar/Manager.php" , line : 281usize , placeholders : & [] , regex : "^Only events in the future are processed$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Error , path : "/lib/private/Calendar/Manager.php" , line : 316usize , placeholders : & [] , regex : "^Could not update calendar for iMIP processing$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Warn , path : "/lib/private/Calendar/Manager.php" , line : 349usize , placeholders : & [] , regex : "^Wrong method provided for processing$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Warn , path : "/lib/private/Calendar/Manager.php" , line : 355usize , placeholders : & [] , regex : "^Recipient must be an ATTENDEE of this event$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Warn , path : "/lib/private/Calendar/Manager.php" , line : 366usize , placeholders : & [] , regex : "^Sender must be the ORGANIZER of this event$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Warn , path : "/lib/private/Calendar/Manager.php" , line : 374usize , placeholders : & [] , regex : "^Only events in the future are processed$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Error , path : "/lib/private/Calendar/Manager.php" , line : 413usize , placeholders : & [] , regex : "^Could not update calendar for iMIP processing$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/AllConfig.php" , line : 254usize , placeholders : & [] , regex : "^Only integers, floats and strings are allowed as value$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/Server.php" , line : 894usize , placeholders : & [] , regex : "^Invalid database type$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/Server.php" , line : 1004usize , placeholders : & ["$app"] , regex : "^The app providing the command bus \\((.*)\\) is not enabled$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Debug , path : "/lib/private/legacy/OC_Files.php" , line : 203usize , placeholders : & [] , regex : "^File given, but no Node available\\. Name \\{file\\}$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/legacy/OC_Util.php" , line : 207usize , placeholders : & [] , regex : "^no instance id!$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Warn , path : "/lib/private/legacy/OC_Util.php" , line : 757usize , placeholders : & [] , regex : "^Error occurred while checking PostgreSQL version, assuming >= 9$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/legacy/OC_Util.php" , line : 953usize , placeholders : & [] , regex : "^Can't create test file to check for working \\.htaccess file\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/legacy/OC_Image.php" , line : 99usize , placeholders : & [] , regex : "^The first parameter in the constructor is not supported anymore\\. Please use any of the load\\* methods of the image object to load an image\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/legacy/OC_Image.php" , line : 307usize , placeholders : & [] , regex : "^\\\\OC_Image::_output\\(\\): imagexbm\\(\\) is not supported\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/legacy/OC_Image.php" , line : 345usize , placeholders : & [] , regex : "^Supplied resource is not of type \"gd\"\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Info , path : "/lib/private/legacy/OC_Image.php" , line : 396usize , placeholders : & [] , regex : "^OC_Image\\->data\\. Could not guess mime\\-type, defaulting to png$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Error , path : "/lib/private/legacy/OC_Image.php" , line : 400usize , placeholders : & [] , regex : "^OC_Image\\->data\\. Error getting image data\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Debug , path : "/lib/private/legacy/OC_Image.php" , line : 436usize , placeholders : & [] , regex : "^OC_Image\\->fixOrientation\\(\\) Image is not a JPEG\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Debug , path : "/lib/private/legacy/OC_Image.php" , line : 440usize , placeholders : & [] , regex : "^OC_Image\\->fixOrientation\\(\\) Exif module not enabled\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Debug , path : "/lib/private/legacy/OC_Image.php" , line : 444usize , placeholders : & [] , regex : "^OC_Image\\->fixOrientation\\(\\) No image loaded\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Debug , path : "/lib/private/legacy/OC_Image.php" , line : 448usize , placeholders : & [] , regex : "^OC_Image\\->fixOrientation\\(\\) No readable file path set\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Debug , path : "/lib/private/legacy/OC_Image.php" , line : 464usize , placeholders : & [] , regex : "^OC_Image\\->fixOrientation\\(\\) Exif module not enabled\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Debug , path : "/lib/private/legacy/OC_Image.php" , line : 468usize , placeholders : & [] , regex : "^OC_Image\\->fixOrientation\\(\\) No image loaded\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Debug , path : "/lib/private/legacy/OC_Image.php" , line : 541usize , placeholders : & [] , regex : "^OC_Image\\->fixOrientation\\(\\) Error during alpha\\-saving$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Debug , path : "/lib/private/legacy/OC_Image.php" , line : 545usize , placeholders : & [] , regex : "^OC_Image\\->fixOrientation\\(\\) Error during alpha\\-blending$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Debug , path : "/lib/private/legacy/OC_Image.php" , line : 549usize , placeholders : & [] , regex : "^OC_Image\\->fixOrientation\\(\\) Error during orientation fixing$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Debug , path : "/lib/private/legacy/OC_Image.php" , line : 763usize , placeholders : & [] , regex : "^OC_Image\\->loadFromFile, Default$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Debug , path : "/lib/private/legacy/OC_Image.php" , line : 794usize , placeholders : & [] , regex : "^OC_Image\\->loadFromFile, could not load$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Debug , path : "/lib/private/legacy/OC_Image.php" , line : 817usize , placeholders : & [] , regex : "^OC_Image\\->loadFromBase64, could not load$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Debug , path : "/lib/private/legacy/OC_Image.php" , line : 929usize , placeholders : & [] , regex : "^OC_Image\\->centerCrop, No image loaded$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Debug , path : "/lib/private/legacy/OC_Image.php" , line : 956usize , placeholders : & [] , regex : "^OC_Image\\->centerCrop, Error creating true color image$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Error , path : "/lib/private/legacy/OC_Template.php" , line : 301usize , placeholders : & ["$error_msg" , "$hint"] , regex : "^(.*) (.*)$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Error , path : "/lib/private/legacy/OC_App.php" , line : 483usize , placeholders : & [] , regex : "^Failed to detect current app from script path$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Debug , path : "/lib/private/legacy/OC_App.php" , line : 525usize , placeholders : & [] , regex : "^OC_App::registerLogIn\\(\\) is deprecated, please register your alternative login option using the registerAlternativeLogin\\(\\) on the RegistrationContext in your Application class implementing the OCP\\\\Authentication\\\\IAlternativeLogin interface$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Error , path : "/lib/private/legacy/OC_App.php" , line : 538usize , placeholders : & [] , regex : "^Alternative login option \\{option\\} does not implement \\{interface\\} and is therefore ignored\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Warn , path : "/lib/private/legacy/OC_Helper.php" , line : 537usize , placeholders : & [] , regex : "^Error while getting quota info, using root quota$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Info , path : "/lib/private/Repair/RepairDavShares.php" , line : 114usize , placeholders : & [] , regex : "^Repaired principal for dav share \\{id\\} from \\{old\\} to \\{new\\}$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Info , path : "/lib/private/Repair/RepairDavShares.php" , line : 118usize , placeholders : & [] , regex : "^Could not repair principal for dav share \\{id\\} from \\{old\\} to \\{new\\}: \\{message\\}$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Info , path : "/lib/private/Repair/RepairDavShares.php" , line : 132usize , placeholders : & [] , regex : "^Repaired DAV group shares$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Info , path : "/lib/private/Repair/RepairDavShares.php" , line : 134usize , placeholders : & [] , regex : "^Invalid shares might be left in the database, running \"occ dav:remove\\-invalid\\-shares\" can remove them\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Info , path : "/lib/private/Repair/RemoveLinkShares.php" , line : 227usize , placeholders : & [] , regex : "^Sending notifications to admins and affected users$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Info , path : "/lib/private/Repair/RemoveLinkShares.php" , line : 233usize , placeholders : & [] , regex : "^No need to remove link shares\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Info , path : "/lib/private/Repair/RemoveLinkShares.php" , line : 237usize , placeholders : & [] , regex : "^Removing potentially over exposing link shares$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Info , path : "/lib/private/Repair/RemoveLinkShares.php" , line : 239usize , placeholders : & [] , regex : "^Removed potentially over exposing link shares$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Warn , path : "/lib/private/Repair/NC21/ValidatePhoneNumber.php" , line : 58usize , placeholders : & [] , regex : "^Can not validate phone numbers without `default_phone_region` being set in the config file$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Warn , path : "/lib/private/Repair/NC25/AddMissingSecretJob.php" , line : 50usize , placeholders : & [] , regex : "^passwordsalt is missing from your config\\.php and your config\\.php is read only\\. Please fix it manually\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Warn , path : "/lib/private/Repair/NC25/AddMissingSecretJob.php" , line : 59usize , placeholders : & [] , regex : "^secret is missing from your config\\.php and your config\\.php is read only\\. Please fix it manually\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Info , path : "/lib/private/Repair/MoveUpdaterStepFile.php" , line : 55usize , placeholders : & [] , regex : "^\\.step file exists$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Info , path : "/lib/private/Repair/MoveUpdaterStepFile.php" , line : 62usize , placeholders : & [] , regex : "^\\.step\\-previous\\-update removed$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Info , path : "/lib/private/Repair/MoveUpdaterStepFile.php" , line : 64usize , placeholders : & [] , regex : "^\\.step\\-previous\\-update can't be removed \\- abort move of \\.step file$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Info , path : "/lib/private/Repair/MoveUpdaterStepFile.php" , line : 71usize , placeholders : & [] , regex : "^\\.step file moved to \\.step\\-previous\\-update$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Warn , path : "/lib/private/Repair/MoveUpdaterStepFile.php" , line : 73usize , placeholders : & [] , regex : "^\\.step file can't be moved$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Info , path : "/lib/private/Repair/Collation.php" , line : 73usize , placeholders : & [] , regex : "^Not a mysql database \\-> nothing to do$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Info , path : "/lib/private/Repair/Collation.php" , line : 81usize , placeholders : & ["$table"] , regex : "^Change row format for (.*) \\.\\.\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Info , path : "/lib/private/Repair/Collation.php" , line : 93usize , placeholders : & ["$table"] , regex : "^Change collation for (.*) \\.\\.\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Info , path : "/lib/private/Repair/Collation.php" , line : 106usize , placeholders : & [] , regex : "^All tables already have the correct collation \\-> nothing to do$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Info , path : "/lib/private/Repair/NC20/ShippedDashboardEnable.php" , line : 48usize , placeholders : & [] , regex : "^Removed old dashboard app config$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Info , path : "/lib/private/Repair/ClearFrontendCaches.php" , line : 53usize , placeholders : & [] , regex : "^Image cache cleared$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Info , path : "/lib/private/Repair/ClearFrontendCaches.php" , line : 56usize , placeholders : & [] , regex : "^JS cache cleared$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Warn , path : "/lib/private/Repair/ClearFrontendCaches.php" , line : 58usize , placeholders : & [] , regex : "^Unable to clear the frontend cache$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Info , path : "/lib/private/Repair/AddMetadataMigrationJob.php" , line : 62usize , placeholders : & [] , regex : "^Removing metadata column from the file_metadata table\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Info , path : "/lib/private/Repair/NC11/FixMountStorages.php" , line : 76usize , placeholders : & [] , regex : "^No mounts updated$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Info , path : "/lib/private/Repair/RepairMimeTypes.php" , line : 243usize , placeholders : & [] , regex : "^Fixed image mime types$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Info , path : "/lib/private/Repair/RepairMimeTypes.php" , line : 247usize , placeholders : & [] , regex : "^Fixed windows program mime types$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Info , path : "/lib/private/Repair/RepairMimeTypes.php" , line : 251usize , placeholders : & [] , regex : "^Fixed geospatial mime types$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Info , path : "/lib/private/Repair/RepairMimeTypes.php" , line : 255usize , placeholders : & [] , regex : "^Fixed internet\\-shortcut mime types$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Info , path : "/lib/private/Repair/RepairMimeTypes.php" , line : 259usize , placeholders : & [] , regex : "^Fixed streaming mime types$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Info , path : "/lib/private/Repair/RepairMimeTypes.php" , line : 263usize , placeholders : & [] , regex : "^Fixed visio mime types$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Info , path : "/lib/private/Repair/RepairMimeTypes.php" , line : 267usize , placeholders : & [] , regex : "^Fixed comicbook mime types$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Info , path : "/lib/private/Repair/RepairMimeTypes.php" , line : 271usize , placeholders : & [] , regex : "^Fixed OpenDocument template mime types$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Info , path : "/lib/private/Repair/RepairMimeTypes.php" , line : 275usize , placeholders : & [] , regex : "^Fixed orgmode mime types$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Info , path : "/lib/private/Repair/RepairMimeTypes.php" , line : 279usize , placeholders : & [] , regex : "^Fixed Flat OpenDocument mime types$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Info , path : "/lib/private/Repair/RepairMimeTypes.php" , line : 283usize , placeholders : & [] , regex : "^Fixed ONLYOFFICE Forms OpenXML mime types$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Info , path : "/lib/private/Repair/RepairMimeTypes.php" , line : 287usize , placeholders : & [] , regex : "^Fixed AsciiDoc mime types$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Error , path : "/lib/private/Repair/NC16/CleanupCardDAVPhotoCache.php" , line : 72usize , placeholders : & [] , regex : "^Failed to fetch directory listing in CleanupCardDAVPhotoCache$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Info , path : "/lib/private/Repair/ClearGeneratedAvatarCache.php" , line : 63usize , placeholders : & [] , regex : "^Avatar cache clearing job added$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Warn , path : "/lib/private/Repair/ClearGeneratedAvatarCache.php" , line : 65usize , placeholders : & [] , regex : "^Unable to clear the avatar cache$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Info , path : "/lib/private/Repair/Owncloud/MigrateOauthTables.php" , line : 50usize , placeholders : & [] , regex : "^oauth2_clients table does not exist\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Info , path : "/lib/private/Repair/Owncloud/MigrateOauthTables.php" , line : 54usize , placeholders : & [] , regex : "^Update the oauth2_access_tokens table schema\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Info , path : "/lib/private/Repair/Owncloud/MigrateOauthTables.php" , line : 76usize , placeholders : & [] , regex : "^Update the oauth2_clients table schema\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Info , path : "/lib/private/Repair/Owncloud/MigrateOauthTables.php" , line : 123usize , placeholders : & [] , regex : "^Move identifier column's data to the new client_identifier column\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Info , path : "/lib/private/Repair/Owncloud/MigrateOauthTables.php" , line : 140usize , placeholders : & [] , regex : "^Drop the identifier column\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Info , path : "/lib/private/Repair/Owncloud/MigrateOauthTables.php" , line : 147usize , placeholders : & [] , regex : "^Delete clients \\(and their related access tokens\\) with the redirect_uri starting with oc:// or ending with \\*$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Info , path : "/lib/private/Repair/Owncloud/MoveAvatars.php" , line : 59usize , placeholders : & [] , regex : "^Repair step already executed$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Info , path : "/lib/private/Repair/Owncloud/MoveAvatars.php" , line : 63usize , placeholders : & [] , regex : "^Avatars are disabled$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Info , path : "/lib/private/Repair/Owncloud/MoveAvatars.php" , line : 65usize , placeholders : & [] , regex : "^Add background job$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Info , path : "/lib/private/Repair/Owncloud/MoveAvatarsBackgroundJob.php" , line : 60usize , placeholders : & [] , regex : "^Started migrating avatars to AppData folder$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Info , path : "/lib/private/Repair/Owncloud/MoveAvatarsBackgroundJob.php" , line : 62usize , placeholders : & [] , regex : "^All avatars migrated to AppData folder$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Info , path : "/lib/private/Repair/Owncloud/MoveAvatarsBackgroundJob.php" , line : 67usize , placeholders : & [] , regex : "^No legacy avatars available, skipping migration$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/Repair/Owncloud/MoveAvatarsBackgroundJob.php" , line : 84usize , placeholders : & [] , regex : "^Failed to open old avatar file for reading$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Info , path : "/lib/private/Repair/Owncloud/MoveAvatarsBackgroundJob.php" , line : 94usize , placeholders : & [] , regex : "^\\{amount\\} avatars migrated$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/Repair/Owncloud/MoveAvatarsBackgroundJob.php" , line : 109usize , placeholders : & ["$path"] , regex : "^(.*)/avatar\\.jpg\\|png$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/LargeFileHelper.php" , line : 60usize , placeholders : & [] , regex : "^This class assumes floats to be double precision or \"better\"\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/LargeFileHelper.php" , line : 87usize , placeholders : & [] , regex : "^Expected int, float or base\\-10 string$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/LDAP/NullLDAPProviderFactory.php" , line : 36usize , placeholders : & [] , regex : "^No LDAP provider is available$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Error , path : "/lib/private/Setup/MySQL.php" , line : 93usize , placeholders : & [] , regex : "^Database creation failed\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Debug , path : "/lib/private/Setup/MySQL.php" , line : 105usize , placeholders : & [] , regex : "^Could not automatically grant privileges, this can be ignored if database user already had privileges\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Error , path : "/lib/private/Setup/MySQL.php" , line : 135usize , placeholders : & [] , regex : "^Database user creation failed\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Info , path : "/lib/private/Setup/MySQL.php" , line : 197usize , placeholders : & [] , regex : "^Can not create a new MySQL user, will continue with the provided user\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Warn , path : "/lib/private/Setup/PostgreSQL.php" , line : 98usize , placeholders : & [] , regex : "^Error trying to connect as \"postgres\", assuming database is setup and tables need to be created$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Error , path : "/lib/private/Setup/PostgreSQL.php" , line : 129usize , placeholders : & [] , regex : "^Error while trying to create database$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Error , path : "/lib/private/Setup/PostgreSQL.php" , line : 138usize , placeholders : & [] , regex : "^Error while trying to restrict database permissions$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Error , path : "/lib/private/Setup/PostgreSQL.php" , line : 182usize , placeholders : & [] , regex : "^Error while trying to create database user$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/Encryption/Util.php" , line : 120usize , placeholders : & [] , regex : "^Default encryption module missing$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/Encryption/Util.php" , line : 229usize , placeholders : & [] , regex : "^path needs to be relative to the system wide data folder and point to a user specific file$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/Encryption/Manager.php" , line : 93usize , placeholders : & [] , regex : "^Key Storage is not ready$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/Encryption/Keys/Storage.php" , line : 231usize , placeholders : & [] , regex : "^Key is invalid$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/Encryption/Keys/Storage.php" , line : 246usize , placeholders : & [] , regex : "^Key has been modified$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/Encryption/Keys/Storage.php" , line : 285usize , placeholders : & [] , regex : "^Could not decrypt key$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/Encryption/Keys/Storage.php" , line : 290usize , placeholders : & [] , regex : "^Invalid encryption key$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/Encryption/Keys/Storage.php" , line : 309usize , placeholders : & [] , regex : "^Invalid encryption key$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/Encryption/HookManager.php" , line : 60usize , placeholders : & [] , regex : "^Inconsistent data, File unshared, but owner not found\\. Should not happen$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/RichObjectStrings/Validator.php" , line : 70usize , placeholders : & [] , regex : "^Parameter is undefined$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/RichObjectStrings/Validator.php" , line : 77usize , placeholders : & [] , regex : "^Parameter is malformed$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/RichObjectStrings/Validator.php" , line : 90usize , placeholders : & [] , regex : "^Object type is undefined$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/RichObjectStrings/Validator.php" , line : 98usize , placeholders : & [] , regex : "^Object is invalid$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Error , path : "/lib/private/Updater.php" , line : 128usize , placeholders : & [] , regex : "^Could not cleanup CAN_INSTALL from your config folder\\. Please remove this file manually\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/Updater.php" , line : 240usize , placeholders : & [] , regex : "^Updates between multiple major versions and downgrades are unsupported\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Info , path : "/lib/private/Updater.php" , line : 485usize , placeholders : & [] , regex : "^\\\\OC\\\\Updater::maintenanceEnabled: Turned on maintenance mode$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Info , path : "/lib/private/Updater.php" , line : 488usize , placeholders : & [] , regex : "^\\\\OC\\\\Updater::maintenanceDisabled: Turned off maintenance mode$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Info , path : "/lib/private/Updater.php" , line : 491usize , placeholders : & [] , regex : "^\\\\OC\\\\Updater::maintenanceActive: Maintenance mode is kept active$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Info , path : "/lib/private/Updater.php" , line : 495usize , placeholders : & [] , regex : "^\\\\OC\\\\Updater::updateEnd: Update successful$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Error , path : "/lib/private/Updater.php" , line : 497usize , placeholders : & [] , regex : "^\\\\OC\\\\Updater::updateEnd: Update failed$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Info , path : "/lib/private/Updater.php" , line : 501usize , placeholders : & [] , regex : "^\\\\OC\\\\Updater::dbUpgradeBefore: Updating database schema$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Info , path : "/lib/private/Updater.php" , line : 504usize , placeholders : & [] , regex : "^\\\\OC\\\\Updater::dbUpgrade: Updated database$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Info , path : "/lib/private/Updater.php" , line : 531usize , placeholders : & [] , regex : "^\\\\OC\\\\Updater::setDebugLogLevel: Set log level to debug$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Info , path : "/lib/private/Updater.php" , line : 537usize , placeholders : & [] , regex : "^\\\\OC\\\\Updater::startCheckCodeIntegrity: Starting code integrity check\\.\\.\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Info , path : "/lib/private/Updater.php" , line : 540usize , placeholders : & [] , regex : "^\\\\OC\\\\Updater::finishedCheckCodeIntegrity: Finished code integrity check$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/Repair.php" , line : 157usize , placeholders : & ["$repairStep"] , regex : "^Repair step '(.*)' is unknown$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/Repair.php" , line : 164usize , placeholders : & ["$repairStep"] , regex : "^Repair step '(.*)' is not of type \\\\OCP\\\\Migration\\\\IRepairStep$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/SpeechToText/SpeechToTextManager.php" , line : 97usize , placeholders : & [] , regex : "^No SpeechToText providers have been registered$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/SpeechToText/SpeechToTextManager.php" , line : 113usize , placeholders : & [] , regex : "^No SpeechToText providers have been registered$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/SpeechToText/SpeechToTextManager.php" , line : 135usize , placeholders : & [] , regex : "^Could not transcribe file$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/Share/Share.php" , line : 484usize , placeholders : & [] , regex : "^Wrong SQL query$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/SystemTag/SystemTagManager.php" , line : 104usize , placeholders : & [] , regex : "^Tag id must be integer$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/SystemTag/SystemTagManager.php" , line : 131usize , placeholders : & [] , regex : "^Tag id\\(s\\) not found$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/SystemTag/SystemTagManager.php" , line : 247usize , placeholders : & [] , regex : "^Tag does not exist$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/SystemTag/SystemTagManager.php" , line : 275usize , placeholders : & [] , regex : "^Tag does not exist$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/SystemTag/SystemTagManager.php" , line : 338usize , placeholders : & [] , regex : "^Tag id\\(s\\) not found$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/SystemTag/SystemTagsInFilesDetector.php" , line : 68usize , placeholders : & [] , regex : "^Could not climb up to root folder$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/SystemTag/SystemTagObjectMapper.php" , line : 121usize , placeholders : & [] , regex : "^Limit is only allowed with a single tag$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/SystemTag/SystemTagObjectMapper.php" , line : 271usize , placeholders : & [] , regex : "^Tags not found$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/public/AppFramework/App.php" , line : 144usize , placeholders : & [] , regex : "^Can only setup routes with real router$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Error , path : "/lib/public/AppFramework/Http/Response.php" , line : 216usize , placeholders : & [] , regex : "^Setting custom header on a 204 or 304 is not supported \\(Header: \\{header\\}\\)$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/public/AppFramework/Http/ZipResponse.php" , line : 61usize , placeholders : & [] , regex : "^No resource provided$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/public/AppFramework/Http/Template/PublicTemplateResponse.php" , line : 94usize , placeholders : & [] , regex : "^Actions must be of type IMenuAction$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/public/AppFramework/Http/Template/PublicTemplateResponse.php" , line : 112usize , placeholders : & [] , regex : "^No header actions have been set$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/public/AppFramework/Db/QBMapper.php" , line : 193usize , placeholders : & [] , regex : "^Entity which should be updated has no id$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Warn , path : "/lib/public/AppFramework/Db/TTransactional.php" , line : 102usize , placeholders : & [] , regex : "^Retrying operation after retryable exception\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/public/SystemTag/ManagerEvent.php" , line : 98usize , placeholders : & [] , regex : "^getTagBefore is only available on the update Event$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/public/Collaboration/Collaborators/SearchResultType.php" , line : 64usize , placeholders : & [] , regex : "^Type must not be empty$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/public/Collaboration/Collaborators/SearchResultType.php" , line : 68usize , placeholders : & [] , regex : "^Provided type is a reserved word$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/public/RichObjectStrings/Definitions.php" , line : 677usize , placeholders : & [] , regex : "^Object type is undefined$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/public/WorkflowEngine/GenericEntityEvent.php" , line : 49usize , placeholders : & [] , regex : "^DisplayName must not be empty$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/public/WorkflowEngine/GenericEntityEvent.php" , line : 52usize , placeholders : & [] , regex : "^EventName must not be empty$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/public/BackgroundJob/TimedJob.php" , line : 77usize , placeholders : & [] , regex : "^Invalid sensitivity$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/public/Files/SimpleFS/InMemoryFile.php" , line : 141usize , placeholders : & [] , regex : "^Stream reading is unsupported for in memory files$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/public/Files/SimpleFS/InMemoryFile.php" , line : 153usize , placeholders : & [] , regex : "^Stream writing is unsupported for in memory files$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/base.php" , line : 283usize , placeholders : & [] , regex : "^Not installed$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Warn , path : "/lib/base.php" , line : 572usize , placeholders : & [] , regex : "^Request does not pass strict cookie check$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/base.php" , line : 620usize , placeholders : & [] , regex : "^Composer autoloader not found, unable to continue\\. Check the folder \"3rdparty\"\\. Running \"git submodule update \\-\\-init\" will initialize the git submodule that handles the subfolder \"3rdparty\"\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Unknown , path : "/lib/base.php" , line : 638usize , placeholders : & [] , regex : "^autoloader$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/base.php" , line : 651usize , placeholders : & [] , regex : "^Could not set timezone to UTC$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Info , path : "/lib/base.php" , line : 834usize , placeholders : & [] , regex : "^Trusted domain error\\. \"\\{remoteAddress\\}\" tried to access using \"\\{host\\}\" as host\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Unknown , path : "/lib/base.php" , line : 851usize , placeholders : & [] , regex : "^init$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Warn , path : "/lib/base.php" , line : 887usize , placeholders : & [] , regex : "^Exception when running cache gc\\.$" , }
];

