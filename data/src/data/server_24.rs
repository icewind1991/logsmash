pub const STATEMENTS: &[crate::LoggingStatement] = &[

	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/remote.php" , line : 74usize , placeholders : & ["$class" , "$msg"] , regex : "^(.*): (.*)$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/remote.php" , line : 128usize , placeholders : & [] , regex : "^Service unavailable$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/remote.php" , line : 134usize , placeholders : & [] , regex : "^Path not found$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/remote.php" , line : 144usize , placeholders : & [] , regex : "^Path not found$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/updater/index.php" , line : 173usize , placeholders : & [] , regex : "^Could not find config\\.php\\. Is this file in the \"updater\" subfolder of Nextcloud\\?$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/updater/index.php" , line : 188usize , placeholders : & [] , regex : "^Could not read data directory from config\\.php\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/updater/index.php" , line : 459usize , placeholders : & [] , regex : "^Could not find config\\.php\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/updater/index.php" , line : 470usize , placeholders : & [] , regex : "^Could not write to config\\.php$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/updater/index.php" , line : 495usize , placeholders : & [] , regex : "^Could not create backup folder location$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/updater/index.php" , line : 618usize , placeholders : & [] , regex : "^Could not parse updater server XML response$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/updater/index.php" , line : 622usize , placeholders : & [] , regex : "^Could not JSON encode updater server response$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/updater/index.php" , line : 626usize , placeholders : & [] , regex : "^Could not JSON decode updater server response\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/updater/index.php" , line : 648usize , placeholders : & [] , regex : "^Could not mkdir storage location$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/updater/index.php" , line : 718usize , placeholders : & [] , regex : "^There are more files than the downloaded archive in the downloads/ folder\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/updater/index.php" , line : 738usize , placeholders : & [] , regex : "^No signature specified for defined update$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/updater/index.php" , line : 779usize , placeholders : & [] , regex : "^Signature of update is not valid$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/updater/index.php" , line : 800usize , placeholders : & ["$versionFile"] , regex : "^OC_Version not found in (.*)$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/updater/index.php" , line : 832usize , placeholders : & [] , regex : "^Downloaded version is lower than installed version$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/updater/index.php" , line : 926usize , placeholders : & [] , regex : "^core/shipped\\.json is not available$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/updater/index.php" , line : 931usize , placeholders : & [] , regex : "^core/shipped\\.json is not available in the new release$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/updater/index.php" , line : 950usize , placeholders : & [] , regex : "^Could not unlink sample config$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/updater/index.php" , line : 961usize , placeholders : & [] , regex : "^Could not delete themes README$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/updater/index.php" , line : 1108usize , placeholders : & [] , regex : "^Could not rmdir \\$storagelocation$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/updater/index.php" , line : 1112usize , placeholders : & [] , regex : "^Could not rmdir \\.step$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/updater/index.php" , line : 1134usize , placeholders : & [] , regex : "^Could not create \\$updaterDir$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/updater/index.php" , line : 1139usize , placeholders : & [] , regex : "^Could not create \\.step$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/updater/index.php" , line : 1145usize , placeholders : & [] , regex : "^Could not write to \\.step$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/updater/index.php" , line : 1179usize , placeholders : & [] , regex : "^Could not read from \\.step$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/updater/index.php" , line : 1184usize , placeholders : & [] , regex : "^Can't decode \\.step JSON data$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/updater/index.php" , line : 1204usize , placeholders : & [] , regex : "^Could not delete \\.step$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/updater/index.php" , line : 1247usize , placeholders : & [] , regex : "^Could not open updater\\.log$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/updater/index.php" , line : 1264usize , placeholders : & [] , regex : "^Could not write to updater\\.log$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Unknown , path : "/updater/index.php" , line : 1312usize , placeholders : & [] , regex : "^\\[info\\] request to updater$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/updater/index.php" , line : 1351usize , placeholders : & [] , regex : "^Not authenticated$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/updater/index.php" , line : 1356usize , placeholders : & [] , regex : "^Invalid step$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Unknown , path : "/updater/index.php" , line : 1404usize , placeholders : & [] , regex : "^\\[error\\] POST request failed with UpdateException$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Unknown , path : "/updater/index.php" , line : 1419usize , placeholders : & [] , regex : "^\\[error\\] POST request failed with other exception$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Unknown , path : "/updater/index.php" , line : 1435usize , placeholders : & [] , regex : "^\\[info\\] show HTML page$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Debug , path : "/cron.php" , line : 46usize , placeholders : & [] , regex : "^Update required, skipping cron$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Debug , path : "/cron.php" , line : 50usize , placeholders : & [] , regex : "^We are in maintenance mode, skipping cron$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/base.php" , line : 282usize , placeholders : & [] , regex : "^Not installed$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/base.php" , line : 587usize , placeholders : & [] , regex : "^Composer autoloader not found, unable to continue\\. Check the folder \"3rdparty\"\\. Running \"git submodule update \\-\\-init\" will initialize the git submodule that handles the subfolder \"3rdparty\"\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Unknown , path : "/lib/base.php" , line : 605usize , placeholders : & [] , regex : "^autoloader$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/base.php" , line : 623usize , placeholders : & [] , regex : "^Could not set timezone to UTC$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Info , path : "/lib/base.php" , line : 798usize , placeholders : & [] , regex : "^Trusted domain error\\. \"\\{remoteAddress\\}\" tried to access using \"\\{host\\}\" as host\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Unknown , path : "/lib/base.php" , line : 815usize , placeholders : & [] , regex : "^init$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/public/RichObjectStrings/Definitions.php" , line : 639usize , placeholders : & [] , regex : "^Object type is undefined$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/public/Collaboration/Collaborators/SearchResultType.php" , line : 64usize , placeholders : & [] , regex : "^Type must not be empty$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/public/Collaboration/Collaborators/SearchResultType.php" , line : 68usize , placeholders : & [] , regex : "^Provided type is a reserved word$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/public/Files/SimpleFS/InMemoryFile.php" , line : 145usize , placeholders : & [] , regex : "^Stream reading is unsupported for in memory files$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/public/Files/SimpleFS/InMemoryFile.php" , line : 157usize , placeholders : & [] , regex : "^Stream writing is unsupported for in memory files$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/public/AppFramework/Db/Mapper.php" , line : 157usize , placeholders : & [] , regex : "^Entity which should be updated has no id$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/public/AppFramework/Db/QBMapper.php" , line : 195usize , placeholders : & [] , regex : "^Entity which should be updated has no id$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/public/AppFramework/Http/Template/PublicTemplateResponse.php" , line : 94usize , placeholders : & [] , regex : "^Actions must be of type IMenuAction$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/public/AppFramework/Http/Template/PublicTemplateResponse.php" , line : 112usize , placeholders : & [] , regex : "^No header actions have been set$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/public/AppFramework/Http/ZipResponse.php" , line : 61usize , placeholders : & [] , regex : "^No resource provided$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Error , path : "/lib/public/AppFramework/Http/Response.php" , line : 215usize , placeholders : & [] , regex : "^Setting custom header on a 204 or 304 is not supported \\(Header: \\{header\\}\\)$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/public/AppFramework/App.php" , line : 145usize , placeholders : & [] , regex : "^Can only setup routes with real router$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/public/SystemTag/ManagerEvent.php" , line : 99usize , placeholders : & [] , regex : "^getTagBefore is only available on the update Event$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/public/WorkflowEngine/GenericEntityEvent.php" , line : 50usize , placeholders : & [] , regex : "^DisplayName must not be empty$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/public/WorkflowEngine/GenericEntityEvent.php" , line : 53usize , placeholders : & [] , regex : "^EventName must not be empty$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/public/BackgroundJob/TimedJob.php" , line : 79usize , placeholders : & [] , regex : "^Invalid sensitivity$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/Config.php" , line : 276usize , placeholders : & [] , regex : "^Can't write into config directory!$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/Config.php" , line : 312usize , placeholders : & [] , regex : "^Config is set to be read\\-only via option \"config_is_read_only\"\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/App/AppStore/Bundles/BundleFetcher.php" , line : 83usize , placeholders : & [] , regex : "^Bundle with specified identifier does not exist$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/App/CompareVersion.php" , line : 52usize , placeholders : & ["$actual"] , regex : "^version specification (.*) is invalid$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/App/CompareVersion.php" , line : 63usize , placeholders : & ["$required"] , regex : "^required version (.*) is invalid$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/App/AppManager.php" , line : 359usize , placeholders : & ["$appId"] , regex : "^(.*) can't be enabled for groups\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/App/AppManager.php" , line : 390usize , placeholders : & ["$appId"] , regex : "^(.*) can't be disabled\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/App/AppManager.php" , line : 572usize , placeholders : & ["$shippedJson"] , regex : "^File not found: (.*)$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Info , path : "/lib/private/Repair/Owncloud/MigrateOauthTables.php" , line : 51usize , placeholders : & [] , regex : "^oauth2_clients table does not exist\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Info , path : "/lib/private/Repair/Owncloud/MigrateOauthTables.php" , line : 55usize , placeholders : & [] , regex : "^Update the oauth2_access_tokens table schema\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Info , path : "/lib/private/Repair/Owncloud/MigrateOauthTables.php" , line : 77usize , placeholders : & [] , regex : "^Update the oauth2_clients table schema\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Info , path : "/lib/private/Repair/Owncloud/MigrateOauthTables.php" , line : 100usize , placeholders : & [] , regex : "^Move identifier column's data to the new client_identifier column\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Info , path : "/lib/private/Repair/Owncloud/MigrateOauthTables.php" , line : 117usize , placeholders : & [] , regex : "^Drop the identifier column\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Info , path : "/lib/private/Repair/Owncloud/MoveAvatars.php" , line : 60usize , placeholders : & [] , regex : "^Repair step already executed$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Info , path : "/lib/private/Repair/Owncloud/MoveAvatars.php" , line : 64usize , placeholders : & [] , regex : "^Avatars are disabled$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Info , path : "/lib/private/Repair/Owncloud/MoveAvatars.php" , line : 66usize , placeholders : & [] , regex : "^Add background job$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Info , path : "/lib/private/Repair/Owncloud/MoveAvatarsBackgroundJob.php" , line : 61usize , placeholders : & [] , regex : "^Started migrating avatars to AppData folder$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Info , path : "/lib/private/Repair/Owncloud/MoveAvatarsBackgroundJob.php" , line : 63usize , placeholders : & [] , regex : "^All avatars migrated to AppData folder$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Info , path : "/lib/private/Repair/Owncloud/MoveAvatarsBackgroundJob.php" , line : 68usize , placeholders : & [] , regex : "^No legacy avatars available, skipping migration$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/Repair/Owncloud/MoveAvatarsBackgroundJob.php" , line : 85usize , placeholders : & [] , regex : "^Failed to open old avatar file for reading$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Info , path : "/lib/private/Repair/Owncloud/MoveAvatarsBackgroundJob.php" , line : 95usize , placeholders : & [] , regex : "^\\{amount\\} avatars migrated$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/Repair/Owncloud/MoveAvatarsBackgroundJob.php" , line : 110usize , placeholders : & ["$path"] , regex : "^(.*)/avatar\\.jpg\\|png$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Info , path : "/lib/private/Repair/Owncloud/InstallCoreBundle.php" , line : 73usize , placeholders : & [] , regex : "^Successfully installed core app bundle\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Info , path : "/lib/private/Repair/NC11/FixMountStorages.php" , line : 77usize , placeholders : & [] , regex : "^No mounts updated$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Info , path : "/lib/private/Repair/RepairDavShares.php" , line : 114usize , placeholders : & [] , regex : "^Repaired principal for dav share \\{id\\} from \\{old\\} to \\{new\\}$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Info , path : "/lib/private/Repair/RepairDavShares.php" , line : 118usize , placeholders : & [] , regex : "^Could not repair principal for dav share \\{id\\} from \\{old\\} to \\{new\\}: \\{message\\}$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Info , path : "/lib/private/Repair/RepairDavShares.php" , line : 132usize , placeholders : & [] , regex : "^Repaired DAV group shares$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Info , path : "/lib/private/Repair/RepairDavShares.php" , line : 134usize , placeholders : & [] , regex : "^Invalid shares might be left in the database, running \"occ dav:remove\\-invalid\\-shares\" can remove them\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Info , path : "/lib/private/Repair/Collation.php" , line : 73usize , placeholders : & [] , regex : "^Not a mysql database \\-> nothing to do$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Info , path : "/lib/private/Repair/Collation.php" , line : 81usize , placeholders : & ["$table"] , regex : "^Change row format for (.*) \\.\\.\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Info , path : "/lib/private/Repair/Collation.php" , line : 93usize , placeholders : & ["$table"] , regex : "^Change collation for (.*) \\.\\.\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Info , path : "/lib/private/Repair/Collation.php" , line : 106usize , placeholders : & [] , regex : "^All tables already have the correct collation \\-> nothing to do$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Info , path : "/lib/private/Repair/RemoveLinkShares.php" , line : 227usize , placeholders : & [] , regex : "^Sending notifications to admins and affected users$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Info , path : "/lib/private/Repair/RemoveLinkShares.php" , line : 233usize , placeholders : & [] , regex : "^No need to remove link shares\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Info , path : "/lib/private/Repair/RemoveLinkShares.php" , line : 237usize , placeholders : & [] , regex : "^Removing potentially over exposing link shares$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Info , path : "/lib/private/Repair/RemoveLinkShares.php" , line : 239usize , placeholders : & [] , regex : "^Removed potentially over exposing link shares$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Error , path : "/lib/private/Repair/NC16/CleanupCardDAVPhotoCache.php" , line : 73usize , placeholders : & [] , regex : "^Failed to fetch directory listing in CleanupCardDAVPhotoCache$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Info , path : "/lib/private/Repair/NC20/ShippedDashboardEnable.php" , line : 49usize , placeholders : & [] , regex : "^Removed old dashboard app config$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Info , path : "/lib/private/Repair/MoveUpdaterStepFile.php" , line : 56usize , placeholders : & [] , regex : "^\\.step file exists$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Info , path : "/lib/private/Repair/MoveUpdaterStepFile.php" , line : 63usize , placeholders : & [] , regex : "^\\.step\\-previous\\-update removed$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Info , path : "/lib/private/Repair/MoveUpdaterStepFile.php" , line : 65usize , placeholders : & [] , regex : "^\\.step\\-previous\\-update can't be removed \\- abort move of \\.step file$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Info , path : "/lib/private/Repair/MoveUpdaterStepFile.php" , line : 72usize , placeholders : & [] , regex : "^\\.step file moved to \\.step\\-previous\\-update$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Warn , path : "/lib/private/Repair/MoveUpdaterStepFile.php" , line : 74usize , placeholders : & [] , regex : "^\\.step file can't be moved$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Info , path : "/lib/private/Repair/ClearGeneratedAvatarCache.php" , line : 65usize , placeholders : & [] , regex : "^Avatar cache cleared$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Warn , path : "/lib/private/Repair/ClearGeneratedAvatarCache.php" , line : 67usize , placeholders : & [] , regex : "^Unable to clear the avatar cache$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Info , path : "/lib/private/Repair/ClearFrontendCaches.php" , line : 60usize , placeholders : & [] , regex : "^Image cache cleared$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Info , path : "/lib/private/Repair/ClearFrontendCaches.php" , line : 63usize , placeholders : & [] , regex : "^SCSS cache cleared$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Info , path : "/lib/private/Repair/ClearFrontendCaches.php" , line : 66usize , placeholders : & [] , regex : "^JS cache cleared$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Warn , path : "/lib/private/Repair/ClearFrontendCaches.php" , line : 68usize , placeholders : & [] , regex : "^Unable to clear the frontend cache$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Info , path : "/lib/private/Repair/RepairMimeTypes.php" , line : 225usize , placeholders : & [] , regex : "^Fixed image mime types$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Info , path : "/lib/private/Repair/RepairMimeTypes.php" , line : 229usize , placeholders : & [] , regex : "^Fixed windows program mime types$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Info , path : "/lib/private/Repair/RepairMimeTypes.php" , line : 233usize , placeholders : & [] , regex : "^Fixed geospatial mime types$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Info , path : "/lib/private/Repair/RepairMimeTypes.php" , line : 237usize , placeholders : & [] , regex : "^Fixed internet\\-shortcut mime types$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Info , path : "/lib/private/Repair/RepairMimeTypes.php" , line : 241usize , placeholders : & [] , regex : "^Fixed streaming mime types$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Info , path : "/lib/private/Repair/RepairMimeTypes.php" , line : 245usize , placeholders : & [] , regex : "^Fixed visio mime types$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Info , path : "/lib/private/Repair/RepairMimeTypes.php" , line : 249usize , placeholders : & [] , regex : "^Fixed comicbook mime types$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Info , path : "/lib/private/Repair/RepairMimeTypes.php" , line : 253usize , placeholders : & [] , regex : "^Fixed OpenDocument template mime types$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Info , path : "/lib/private/Repair/RepairMimeTypes.php" , line : 257usize , placeholders : & [] , regex : "^Fixed orgmode mime types$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Info , path : "/lib/private/Repair/RepairMimeTypes.php" , line : 261usize , placeholders : & [] , regex : "^Fixed Flat OpenDocument mime types$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Warn , path : "/lib/private/Repair/NC21/ValidatePhoneNumber.php" , line : 59usize , placeholders : & [] , regex : "^Can not validate phone numbers without `default_phone_region` being set in the config file$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/AppFramework/Bootstrap/RegistrationContext.php" , line : 403usize , placeholders : & [] , regex : "^Only the Talk app is allowed to register a Talk backend$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/AppFramework/Bootstrap/RegistrationContext.php" , line : 406usize , placeholders : & [] , regex : "^There can only be one Talk backend$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Error , path : "/lib/private/AppFramework/Bootstrap/RegistrationContext.php" , line : 442usize , placeholders : & ["$appId"] , regex : "^App (.*) not loaded for the capability registration$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Error , path : "/lib/private/AppFramework/Bootstrap/RegistrationContext.php" , line : 517usize , placeholders : & ["$appId"] , regex : "^App (.*) not loaded for the container service registration$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Error , path : "/lib/private/AppFramework/Bootstrap/RegistrationContext.php" , line : 545usize , placeholders : & ["$appId"] , regex : "^App (.*) not loaded for the container alias registration$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Error , path : "/lib/private/AppFramework/Bootstrap/RegistrationContext.php" , line : 569usize , placeholders : & ["$appId"] , regex : "^App (.*) not loaded for the container parameter registration$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Error , path : "/lib/private/AppFramework/Bootstrap/RegistrationContext.php" , line : 598usize , placeholders : & ["$appId"] , regex : "^App (.*) not loaded for the container middleware registration$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/AppFramework/Routing/RouteConfig.php" , line : 133usize , placeholders : & [] , regex : "^Invalid route name: use the format foo\\#bar to reference FooController::bar$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/AppFramework/Routing/RouteParser.php" , line : 96usize , placeholders : & [] , regex : "^Invalid route name: use the format foo\\#bar to reference FooController::bar$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/AppFramework/Middleware/PublicShare/PublicShareMiddleware.php" , line : 68usize , placeholders : & [] , regex : "^Link sharing is disabled$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/AppFramework/Middleware/Security/CORSMiddleware.php" , line : 93usize , placeholders : & [] , regex : "^CORS requires basic auth$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/AppFramework/Middleware/Security/CORSMiddleware.php" , line : 96usize , placeholders : & [] , regex : "^Password login forbidden, use token instead$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Debug , path : "/lib/private/AppFramework/Http/Dispatcher.php" , line : 141usize , placeholders : & [] , regex : "^Controller \\{class\\}::\\{method\\} created \\{count\\} QueryBuilder objects, please check if they are created inside a loop by accident\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Warn , path : "/lib/private/AppFramework/Http/Dispatcher.php" , line : 149usize , placeholders : & [] , regex : "^Controller \\{class\\}::\\{method\\} executed \\{count\\} queries\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/AppFramework/Http/Request.php" , line : 213usize , placeholders : & [] , regex : "^You cannot change the contents of the request object$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/AppFramework/Http/Request.php" , line : 221usize , placeholders : & [] , regex : "^You cannot change the contents of the request object$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/AppFramework/Http/Request.php" , line : 230usize , placeholders : & [] , regex : "^You cannot change the contents of the request object$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/AppFramework/Http/Request.php" , line : 292usize , placeholders : & [] , regex : "^You cannot change the contents of the request object$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/AppFramework/Http/Request.php" , line : 759usize , placeholders : & ["$requestUri" , "$scriptName"] , regex : "^The requested uri\\((.*)\\) cannot be processed by the script '(.*)'\\)$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Debug , path : "/lib/private/legacy/OC_Files.php" , line : 201usize , placeholders : & [] , regex : "^File given, but no Node available\\. Name \\{file\\}$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Warn , path : "/lib/private/legacy/OC_Helper.php" , line : 556usize , placeholders : & [] , regex : "^Error while getting quota info, using root quota$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/legacy/OC_Image.php" , line : 96usize , placeholders : & [] , regex : "^The first parameter in the constructor is not supported anymore\\. Please use any of the load\\* methods of the image object to load an image\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/legacy/OC_Image.php" , line : 302usize , placeholders : & [] , regex : "^\\\\OC_Image::_output\\(\\): imagexbm\\(\\) is not supported\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/legacy/OC_Image.php" , line : 340usize , placeholders : & [] , regex : "^Supplied resource is not of type \"gd\"\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Info , path : "/lib/private/legacy/OC_Image.php" , line : 394usize , placeholders : & [] , regex : "^OC_Image\\->data\\. Could not guess mime\\-type, defaulting to png$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Error , path : "/lib/private/legacy/OC_Image.php" , line : 398usize , placeholders : & [] , regex : "^OC_Image\\->data\\. Error getting image data\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Debug , path : "/lib/private/legacy/OC_Image.php" , line : 433usize , placeholders : & [] , regex : "^OC_Image\\->fixOrientation\\(\\) Image is not a JPEG\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Debug , path : "/lib/private/legacy/OC_Image.php" , line : 437usize , placeholders : & [] , regex : "^OC_Image\\->fixOrientation\\(\\) Exif module not enabled\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Debug , path : "/lib/private/legacy/OC_Image.php" , line : 441usize , placeholders : & [] , regex : "^OC_Image\\->fixOrientation\\(\\) No image loaded\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Debug , path : "/lib/private/legacy/OC_Image.php" , line : 445usize , placeholders : & [] , regex : "^OC_Image\\->fixOrientation\\(\\) No readable file path set\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Debug , path : "/lib/private/legacy/OC_Image.php" , line : 461usize , placeholders : & [] , regex : "^OC_Image\\->fixOrientation\\(\\) Exif module not enabled\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Debug , path : "/lib/private/legacy/OC_Image.php" , line : 465usize , placeholders : & [] , regex : "^OC_Image\\->fixOrientation\\(\\) No image loaded\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Debug , path : "/lib/private/legacy/OC_Image.php" , line : 538usize , placeholders : & [] , regex : "^OC_Image\\->fixOrientation\\(\\) Error during alpha\\-saving$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Debug , path : "/lib/private/legacy/OC_Image.php" , line : 542usize , placeholders : & [] , regex : "^OC_Image\\->fixOrientation\\(\\) Error during alpha\\-blending$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Debug , path : "/lib/private/legacy/OC_Image.php" , line : 546usize , placeholders : & [] , regex : "^OC_Image\\->fixOrientation\\(\\) Error during orientation fixing$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Debug , path : "/lib/private/legacy/OC_Image.php" , line : 760usize , placeholders : & [] , regex : "^OC_Image\\->loadFromFile, Default$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Debug , path : "/lib/private/legacy/OC_Image.php" , line : 794usize , placeholders : & [] , regex : "^OC_Image\\->loadFromFile, could not load$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Debug , path : "/lib/private/legacy/OC_Image.php" , line : 820usize , placeholders : & [] , regex : "^OC_Image\\->loadFromBase64, could not load$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Debug , path : "/lib/private/legacy/OC_Image.php" , line : 1091usize , placeholders : & [] , regex : "^OC_Image\\->centerCrop, No image loaded$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Debug , path : "/lib/private/legacy/OC_Image.php" , line : 1118usize , placeholders : & [] , regex : "^OC_Image\\->centerCrop, Error creating true color image$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Error , path : "/lib/private/legacy/OC_Template.php" , line : 304usize , placeholders : & ["$error_msg" , "$hint"] , regex : "^(.*) (.*)$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Error , path : "/lib/private/legacy/OC_App.php" , line : 178usize , placeholders : & [] , regex : "^/appinfo/app\\.php is not loaded when \\\\OCP\\\\AppFramework\\\\Bootstrap\\\\IBootstrap on the application class is used\\. Migrate everything from app\\.php to the Application class\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Debug , path : "/lib/private/legacy/OC_App.php" , line : 182usize , placeholders : & [] , regex : "^/appinfo/app\\.php is deprecated, use \\\\OCP\\\\AppFramework\\\\Bootstrap\\\\IBootstrap on the application class instead\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Error , path : "/lib/private/legacy/OC_App.php" , line : 644usize , placeholders : & [] , regex : "^Failed to detect current app from script path$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Debug , path : "/lib/private/legacy/OC_App.php" , line : 705usize , placeholders : & [] , regex : "^OC_App::registerLogIn\\(\\) is deprecated, please register your alternative login option using the registerAlternativeLogin\\(\\) on the RegistrationContext in your Application class implementing the OCP\\\\Authentication\\\\IAlternativeLogin interface$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Error , path : "/lib/private/legacy/OC_App.php" , line : 718usize , placeholders : & [] , regex : "^Alternative login option \\{option\\} does not implement \\{interface\\} and is therefore ignored\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/legacy/OC_Util.php" , line : 207usize , placeholders : & [] , regex : "^no instance id!$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Warn , path : "/lib/private/legacy/OC_Util.php" , line : 758usize , placeholders : & [] , regex : "^Error occurred while checking PostgreSQL version, assuming >= 9$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/legacy/OC_Util.php" , line : 954usize , placeholders : & [] , regex : "^Can't create test file to check for working \\.htaccess file\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/FullTextSearch/FullTextSearchManager.php" , line : 103usize , placeholders : & [] , regex : "^No IProviderService registered$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/FullTextSearch/FullTextSearchManager.php" , line : 116usize , placeholders : & [] , regex : "^No IIndexService registered$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/FullTextSearch/FullTextSearchManager.php" , line : 129usize , placeholders : & [] , regex : "^No ISearchService registered$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/Session/Internal.php" , line : 62usize , placeholders : & [] , regex : "^Failed to start session$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/Session/Internal.php" , line : 175usize , placeholders : & [] , regex : "^The session cannot be reopened \\- reopen\\(\\) is ony to be used in unit testing\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/Session/Internal.php" , line : 194usize , placeholders : & [] , regex : "^Session has been closed \\- no further changes to the session are allowed$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/Session/Memory.php" , line : 107usize , placeholders : & [] , regex : "^Memory session does not have an ID$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/Session/Memory.php" , line : 124usize , placeholders : & [] , regex : "^Session has been closed \\- no further changes to the session are allowed$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/Updater/ChangesMapper.php" , line : 55usize , placeholders : & [] , regex : "^Changes info is not present$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/Updater/ChangesCheck.php" , line : 60usize , placeholders : & [] , regex : "^Unable to decode changes info$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Debug , path : "/lib/private/Updater/ChangesCheck.php" , line : 105usize , placeholders : & [] , regex : "^Unexpected return code \\{code\\} from changelog server$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/Command/CommandJob.php" , line : 37usize , placeholders : & [] , regex : "^Invalid serialized command$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/Command/ClosureJob.php" , line : 33usize , placeholders : & [] , regex : "^Invalid serialized callable$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/Command/CronBus.php" , line : 60usize , placeholders : & [] , regex : "^Invalid command$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/Command/CronBus.php" , line : 74usize , placeholders : & [] , regex : "^Invalid command$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/Command/QueueBus.php" , line : 60usize , placeholders : & [] , regex : "^Trying to push a command which serialized form can not be stored in the database \\(>4000 character\\)$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/Command/CallableJob.php" , line : 32usize , placeholders : & [] , regex : "^Invalid serialized callable$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Warn , path : "/lib/private/Search.php" , line : 68usize , placeholders : & [] , regex : "^Ignoring Unknown search provider$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/IntegrityCheck/Helpers/AppLocator.php" , line : 47usize , placeholders : & [] , regex : "^App not found$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/IntegrityCheck/Checker.php" , line : 336usize , placeholders : & [] , regex : "^Signature data not found\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/IntegrityCheck/Checker.php" , line : 354usize , placeholders : & [] , regex : "^Certificate is not valid\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/IntegrityCheck/Checker.php" , line : 371usize , placeholders : & [] , regex : "^Signature could not get verified\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/IntegrityCheck/Checker.php" , line : 416usize , placeholders : & [] , regex : "^Invalid behaviour in file hash comparison experienced\\. Please report this error to the developers\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Error , path : "/lib/private/CapabilitiesManager.php" , line : 62usize , placeholders : & [] , regex : "^CapabilitiesManager$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Error , path : "/lib/private/User/Session.php" , line : 801usize , placeholders : & [] , regex : "^App token login name does not match$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Info , path : "/lib/private/User/Session.php" , line : 885usize , placeholders : & [] , regex : "^Tried to log in \\{uid\\} but could not verify token$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Warn , path : "/lib/private/User/Session.php" , line : 900usize , placeholders : & [] , regex : "^Could not renew session token for \\{uid\\} because the session is unavailable$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Warn , path : "/lib/private/User/Session.php" , line : 906usize , placeholders : & [] , regex : "^Renewing session token failed$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/User/User.php" , line : 213usize , placeholders : & [] , regex : "^Only verified emails can be set as primary$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/User/Database.php" , line : 495usize , placeholders : & [] , regex : "^key uid is expected to be set in \\$param$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Error , path : "/lib/private/Template/CSSResourceLocator.php" , line : 72usize , placeholders : & [] , regex : "^Could not find resource \\{resource\\} to load$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Error , path : "/lib/private/Template/CSSResourceLocator.php" , line : 132usize , placeholders : & [] , regex : "^ResourceLocator can not find a web root \\(root: \\{root\\}, file: \\{file\\}, webRoot: \\{webRoot\\}, throw: \\{throw\\}\\)$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Error , path : "/lib/private/Template/JSResourceLocator.php" , line : 103usize , placeholders : & [] , regex : "^Could not find resource \\{resource\\} to load$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Debug , path : "/lib/private/Template/SCSSCacher.php" , line : 141usize , placeholders : & [] , regex : "^SCSSCacher::process ordinary check follows$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Debug , path : "/lib/private/Template/SCSSCacher.php" , line : 162usize , placeholders : & [] , regex : "^SCSSCacher::process check in while loop follows$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Debug , path : "/lib/private/Template/SCSSCacher.php" , line : 165usize , placeholders : & ["$app" , "$fileNameCSS" , "$retry"] , regex : "^SCSSCacher::process cached file for app '(.*)' and file '(.*)' is now available after (.*) s\\. Moving on\\.\\.\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Debug , path : "/lib/private/Template/SCSSCacher.php" , line : 223usize , placeholders : & ["$fileNameCSS"] , regex : "^SCSSCacher::isCached (.*) isCachedCache is expired or unset$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Debug , path : "/lib/private/Template/SCSSCacher.php" , line : 229usize , placeholders : & ["$app"] , regex : "^SCSSCacher::isCached app data folder for (.*) could not be fetched$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Debug , path : "/lib/private/Template/SCSSCacher.php" , line : 250usize , placeholders : & ["$fileNameCSS" , "$file"] , regex : "^SCSSCacher::isCached (.*) is not considered as cached due to deps file (.*)$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Debug , path : "/lib/private/Template/SCSSCacher.php" , line : 255usize , placeholders : & ["$fileNameCSS"] , regex : "^SCSSCacher::isCached (.*) dependencies successfully cached for 5 minutes$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Debug , path : "/lib/private/Template/SCSSCacher.php" , line : 260usize , placeholders : & ["$fileNameCSS" , "$cacheValue"] , regex : "^SCSSCacher::isCached (.*) is not considered as cached cacheValue: (.*)$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Debug , path : "/lib/private/Template/SCSSCacher.php" , line : 377usize , placeholders : & [] , regex : "^SCSSCacher::resetCache$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Debug , path : "/lib/private/Template/SCSSCacher.php" , line : 379usize , placeholders : & [] , regex : "^SCSSCacher::resetCache Locked$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Debug , path : "/lib/private/Template/SCSSCacher.php" , line : 382usize , placeholders : & [] , regex : "^SCSSCacher::resetCache Lock acquired$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Debug , path : "/lib/private/Template/SCSSCacher.php" , line : 399usize , placeholders : & [] , regex : "^SCSSCacher::resetCache css cache cleared!$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Debug , path : "/lib/private/Template/SCSSCacher.php" , line : 401usize , placeholders : & [] , regex : "^SCSSCacher::resetCache Locking removed$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/Template/TemplateFileLocator.php" , line : 46usize , placeholders : & [] , regex : "^Empty template name$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Error , path : "/lib/private/Template/ResourceLocator.php" , line : 178usize , placeholders : & [] , regex : "^ResourceLocator can not find a web root \\(root: \\{root\\}, file: \\{file\\}, webRoot: \\{webRoot\\}, throw: \\{throw\\}\\)$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/RichObjectStrings/Validator.php" , line : 71usize , placeholders : & [] , regex : "^Parameter is undefined$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/RichObjectStrings/Validator.php" , line : 78usize , placeholders : & [] , regex : "^Parameter is malformed$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/RichObjectStrings/Validator.php" , line : 91usize , placeholders : & [] , regex : "^Object type is undefined$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/RichObjectStrings/Validator.php" , line : 99usize , placeholders : & [] , regex : "^Object is invalid$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/AllConfig.php" , line : 257usize , placeholders : & [] , regex : "^Only integers, floats and strings are allowed as value$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/LDAP/NullLDAPProviderFactory.php" , line : 36usize , placeholders : & [] , regex : "^No LDAP provider is available$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/Accounts/Account.php" , line : 53usize , placeholders : & [] , regex : "^setProperty cannot set an IAccountsPropertyCollection$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/Accounts/Account.php" , line : 61usize , placeholders : & [] , regex : "^getProperty cannot retrieve an IAccountsPropertyCollection$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/Accounts/Account.php" , line : 158usize , placeholders : & [] , regex : "^Requested collection is not an IAccountPropertyCollection$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/Accounts/AccountManager.php" , line : 215usize , placeholders : & [] , regex : "^scope$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/Accounts/AccountManager.php" , line : 224usize , placeholders : & [] , regex : "^scope$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Info , path : "/lib/private/Accounts/AccountManager.php" , line : 476usize , placeholders : & [] , regex : "^Failed to send verification mail$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Critical , path : "/lib/private/Accounts/AccountManager.php" , line : 582usize , placeholders : & [] , regex : "^User data of \\{uid\\} contained invalid JSON \\(error \\{json_error\\}\\), hence falling back to a default user record$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/Accounts/AccountPropertyCollection.php" , line : 61usize , placeholders : & [] , regex : "^Provided property does not match collection name$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/Accounts/AccountProperty.php" , line : 96usize , placeholders : & [] , regex : "^Invalid scope$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/Accounts/AccountProperty.php" , line : 192usize , placeholders : & [] , regex : "^Provided verification value is invalid$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/Cache/File.php" , line : 65usize , placeholders : & [] , regex : "^Can\\\\t get cache storage, user not logged in$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/Server.php" , line : 874usize , placeholders : & [] , regex : "^Invalid database type$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/Server.php" , line : 997usize , placeholders : & ["$app"] , regex : "^The app providing the command bus \\((.*)\\) is not enabled$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/Comments/Manager.php" , line : 159usize , placeholders : & [] , regex : "^Actor, Object and Verb information must be provided for saving$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/Comments/Manager.php" , line : 164usize , placeholders : & [] , regex : "^Reactions can only be a single emoji$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/Comments/Manager.php" , line : 283usize , placeholders : & [] , regex : "^IDs must be translatable to a number in this implementation\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/Comments/Manager.php" , line : 936usize , placeholders : & [] , regex : "^Parameter must be string$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/Comments/Manager.php" , line : 1009usize , placeholders : & [] , regex : "^Comment related with reaction not found$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/Comments/Manager.php" , line : 1092usize , placeholders : & [] , regex : "^The database does not support reactions$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/Comments/Manager.php" , line : 1354usize , placeholders : & [] , regex : "^Comment to update does ceased to exist$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/Comments/Manager.php" , line : 1569usize , placeholders : & [] , regex : "^String expected\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/Comments/Manager.php" , line : 1572usize , placeholders : & [] , regex : "^Displayname resolver for this type already registered$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/Comments/Manager.php" , line : 1592usize , placeholders : & [] , regex : "^String expected\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/Comments/Manager.php" , line : 1595usize , placeholders : & [] , regex : "^No Displayname resolver for this type registered$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/Comments/Manager.php" , line : 1614usize , placeholders : & [] , regex : "^The given entity does not implement the ICommentsEntity interface$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/Comments/Comment.php" , line : 91usize , placeholders : & [] , regex : "^String expected\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/Comments/Comment.php" , line : 100usize , placeholders : & [] , regex : "^Not allowed to assign a new ID to an already saved comment\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/Comments/Comment.php" , line : 122usize , placeholders : & [] , regex : "^String expected\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/Comments/Comment.php" , line : 148usize , placeholders : & [] , regex : "^String expected\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/Comments/Comment.php" , line : 173usize , placeholders : & [] , regex : "^Integer expected\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/Comments/Comment.php" , line : 200usize , placeholders : & [] , regex : "^String expected\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/Comments/Comment.php" , line : 272usize , placeholders : & [] , regex : "^Non\\-empty String expected\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/Comments/Comment.php" , line : 311usize , placeholders : & [] , regex : "^String expected\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/Comments/Comment.php" , line : 397usize , placeholders : & [] , regex : "^String expected\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/Comments/Comment.php" , line : 427usize , placeholders : & [] , regex : "^Non empty string expected\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/Repair.php" , line : 153usize , placeholders : & ["$repairStep"] , regex : "^Repair step '(.*)' is unknown$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/Repair.php" , line : 160usize , placeholders : & ["$repairStep"] , regex : "^Repair step '(.*)' is not of type \\\\OCP\\\\Migration\\\\IRepairStep$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/Activity/Event.php" , line : 102usize , placeholders : & [] , regex : "^The given app is invalid$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/Activity/Event.php" , line : 125usize , placeholders : & [] , regex : "^The given type is invalid$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/Activity/Event.php" , line : 148usize , placeholders : & [] , regex : "^The given affected user is invalid$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/Activity/Event.php" , line : 171usize , placeholders : & [] , regex : "^The given author user is invalid$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/Activity/Event.php" , line : 215usize , placeholders : & [] , regex : "^The given subject is invalid$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/Activity/Event.php" , line : 244usize , placeholders : & [] , regex : "^The given parsed subject is invalid$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/Activity/Event.php" , line : 267usize , placeholders : & [] , regex : "^The given parsed subject is invalid$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/Activity/Event.php" , line : 302usize , placeholders : & [] , regex : "^The given message is invalid$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/Activity/Event.php" , line : 384usize , placeholders : & [] , regex : "^The given object type is invalid$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/Activity/Event.php" , line : 387usize , placeholders : & [] , regex : "^The given object name is invalid$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/Activity/Event.php" , line : 426usize , placeholders : & [] , regex : "^The given link is invalid$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/Activity/Event.php" , line : 447usize , placeholders : & [] , regex : "^The given icon is invalid$" , }
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
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/Search/Result/File.php" , line : 137usize , placeholders : & [] , regex : "^Search result not in user folder$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/Search/SearchComposer.php" , line : 158usize , placeholders : & ["$providerId"] , regex : "^Provider (.*) is unknown$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/BackgroundJob/JobList.php" , line : 77usize , placeholders : & [] , regex : "^Background job arguments can't exceed 4000 characters \\(json encoded\\)$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/Setup.php" , line : 177usize , placeholders : & [] , regex : "^Supported databases are not properly configured\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/Setup.php" , line : 478usize , placeholders : & [] , regex : "^overwrite\\.cli\\.url is empty$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/Setup.php" , line : 481usize , placeholders : & [] , regex : "^invalid value for overwrite\\.cli\\.url$" , }
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
	crate :: LoggingStatement { level : crate :: LogLevel :: Error , path : "/lib/private/Updater.php" , line : 122usize , placeholders : & [] , regex : "^Could not cleanup CAN_INSTALL from your config folder\\. Please remove this file manually\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/Updater.php" , line : 234usize , placeholders : & [] , regex : "^Updates between multiple major versions and downgrades are unsupported\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Info , path : "/lib/private/Updater.php" , line : 496usize , placeholders : & [] , regex : "^\\\\OC\\\\Repair::finishProgress$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Info , path : "/lib/private/Updater.php" , line : 523usize , placeholders : & [] , regex : "^\\\\OC\\\\Updater::maintenanceEnabled: Turned on maintenance mode$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Info , path : "/lib/private/Updater.php" , line : 526usize , placeholders : & [] , regex : "^\\\\OC\\\\Updater::maintenanceDisabled: Turned off maintenance mode$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Info , path : "/lib/private/Updater.php" , line : 529usize , placeholders : & [] , regex : "^\\\\OC\\\\Updater::maintenanceActive: Maintenance mode is kept active$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Info , path : "/lib/private/Updater.php" , line : 533usize , placeholders : & [] , regex : "^\\\\OC\\\\Updater::updateEnd: Update successful$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Error , path : "/lib/private/Updater.php" , line : 535usize , placeholders : & [] , regex : "^\\\\OC\\\\Updater::updateEnd: Update failed$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Info , path : "/lib/private/Updater.php" , line : 539usize , placeholders : & [] , regex : "^\\\\OC\\\\Updater::dbUpgradeBefore: Updating database schema$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Info , path : "/lib/private/Updater.php" , line : 542usize , placeholders : & [] , regex : "^\\\\OC\\\\Updater::dbUpgrade: Updated database$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Info , path : "/lib/private/Updater.php" , line : 569usize , placeholders : & [] , regex : "^\\\\OC\\\\Updater::setDebugLogLevel: Set log level to debug$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Info , path : "/lib/private/Updater.php" , line : 575usize , placeholders : & [] , regex : "^\\\\OC\\\\Updater::startCheckCodeIntegrity: Starting code integrity check\\.\\.\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Info , path : "/lib/private/Updater.php" , line : 578usize , placeholders : & [] , regex : "^\\\\OC\\\\Updater::finishedCheckCodeIntegrity: Finished code integrity check$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/Encryption/Manager.php" , line : 94usize , placeholders : & [] , regex : "^Key Storage is not ready$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/Encryption/Util.php" , line : 126usize , placeholders : & [] , regex : "^Default encryption module missing$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/Encryption/Util.php" , line : 235usize , placeholders : & [] , regex : "^path needs to be relative to the system wide data folder and point to a user specific file$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/Encryption/Keys/Storage.php" , line : 232usize , placeholders : & [] , regex : "^Key is invalid$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/Encryption/Keys/Storage.php" , line : 247usize , placeholders : & [] , regex : "^Key has been modified$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/Encryption/Keys/Storage.php" , line : 286usize , placeholders : & [] , regex : "^Could not decrypt key$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/Encryption/Keys/Storage.php" , line : 291usize , placeholders : & [] , regex : "^Invalid encryption key$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/Encryption/Keys/Storage.php" , line : 310usize , placeholders : & [] , regex : "^Invalid encryption key$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/Encryption/HookManager.php" , line : 60usize , placeholders : & [] , regex : "^Inconsistent data, File unshared, but owner not found\\. Should not happen$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Error , path : "/lib/private/Preview/Imaginary.php" , line : 74usize , placeholders : & [] , regex : "^Imaginary preview provider is enabled, but no url is configured\\. Please provide the url of your imaginary server to the 'preview_imaginary_url' config variable\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/Preview/Generator.php" , line : 130usize , placeholders : & [] , regex : "^Cannot read file$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/Preview/Generator.php" , line : 155usize , placeholders : & [] , regex : "^Cached preview size 0, invalid!$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/Preview/Generator.php" , line : 166usize , placeholders : & [] , regex : "^Max preview size 0, invalid!$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/Preview/Generator.php" , line : 211usize , placeholders : & [] , regex : "^$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/Preview/Generator.php" , line : 216usize , placeholders : & [] , regex : "^Cached preview size 0, invalid!$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/Preview/Generator.php" , line : 505usize , placeholders : & [] , regex : "^Failed to generate preview, failed to load image$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Error , path : "/lib/private/Preview/Movie.php" , line : 160usize , placeholders : & [] , regex : "^Movie preview generation failed Output: \\{output\\}$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/Memcache/Memcached.php" , line : 84usize , placeholders : & ["$options"] , regex : "^Expected 'memcached_options' config to be an array, got (.*)$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/Log.php" , line : 382usize , placeholders : & [] , regex : "^Log implementation has no path$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/Share20/DefaultShareProvider.php" , line : 182usize , placeholders : & [] , regex : "^invalid share type!$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/Share20/DefaultShareProvider.php" , line : 239usize , placeholders : & [] , regex : "^Newly created share could not be found$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/Share20/DefaultShareProvider.php" , line : 366usize , placeholders : & [] , regex : "^Recipient not in receiving group$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/Share20/DefaultShareProvider.php" , line : 396usize , placeholders : & [] , regex : "^Recipient does not match$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/Share20/DefaultShareProvider.php" , line : 401usize , placeholders : & [] , regex : "^Invalid shareType$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/Share20/DefaultShareProvider.php" , line : 533usize , placeholders : & [] , regex : "^Recipient does not match$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/Share20/DefaultShareProvider.php" , line : 539usize , placeholders : & [] , regex : "^Invalid shareType$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/Share20/DefaultShareProvider.php" , line : 1001usize , placeholders : & [] , regex : "^Invalid backend$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/Share20/Manager.php" , line : 190usize , placeholders : & [] , regex : "^Passwords are enforced for link and mail shares$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/Share20/Manager.php" , line : 217usize , placeholders : & [] , regex : "^SharedWith is not a valid user$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/Share20/Manager.php" , line : 222usize , placeholders : & [] , regex : "^SharedWith is not a valid group$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/Share20/Manager.php" , line : 227usize , placeholders : & [] , regex : "^SharedWith should be empty$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/Share20/Manager.php" , line : 231usize , placeholders : & [] , regex : "^SharedWith should not be empty$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/Share20/Manager.php" , line : 235usize , placeholders : & [] , regex : "^SharedWith should not be empty$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/Share20/Manager.php" , line : 239usize , placeholders : & [] , regex : "^SharedWith should not be empty$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/Share20/Manager.php" , line : 244usize , placeholders : & [] , regex : "^SharedWith is not a valid circle$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/Share20/Manager.php" , line : 250usize , placeholders : & [] , regex : "^unknown share type$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/Share20/Manager.php" , line : 255usize , placeholders : & [] , regex : "^SharedBy should be set$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/Share20/Manager.php" , line : 261usize , placeholders : & [] , regex : "^Cannot share with yourself$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/Share20/Manager.php" , line : 266usize , placeholders : & [] , regex : "^Path should be set$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/Share20/Manager.php" , line : 272usize , placeholders : & [] , regex : "^Path should be either a file or a folder$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/Share20/Manager.php" , line : 282usize , placeholders : & [] , regex : "^You cannot share your root folder$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/Share20/Manager.php" , line : 293usize , placeholders : & [] , regex : "^A share requires permissions$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/Share20/Manager.php" , line : 357usize , placeholders : & [] , regex : "^Shares need at least read permissions$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/Share20/Manager.php" , line : 431usize , placeholders : & [] , regex : "^Expiration date is enforced$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/Share20/Manager.php" , line : 507usize , placeholders : & [] , regex : "^Expiration date is enforced$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/Share20/Manager.php" , line : 606usize , placeholders : & [] , regex : "^Group sharing is now allowed$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/Share20/Manager.php" , line : 614usize , placeholders : & [] , regex : "^Sharing is only allowed within your own groups$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/Share20/Manager.php" , line : 635usize , placeholders : & [] , regex : "^Path is already shared with this group$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/Share20/Manager.php" , line : 649usize , placeholders : & [] , regex : "^Link sharing is not allowed$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/Share20/Manager.php" , line : 655usize , placeholders : & [] , regex : "^Public upload is not allowed$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/Share20/Manager.php" , line : 691usize , placeholders : & [] , regex : "^Path contains files shared with you$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/Share20/Manager.php" , line : 705usize , placeholders : & [] , regex : "^Sharing is disabled$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/Share20/Manager.php" , line : 709usize , placeholders : & [] , regex : "^Sharing is disabled for you$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/Share20/Manager.php" , line : 793usize , placeholders : & [] , regex : "^Cannot share with the share owner$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Debug , path : "/lib/private/Share20/Manager.php" , line : 865usize , placeholders : & [] , regex : "^Share notification not sent because mailsend is false\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Error , path : "/lib/private/Share20/Manager.php" , line : 953usize , placeholders : & [] , regex : "^Share notification mail could not be sent$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/Share20/Manager.php" , line : 972usize , placeholders : & [] , regex : "^Share does not have a full id$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/Share20/Manager.php" , line : 977usize , placeholders : & [] , regex : "^Cannot change share type$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/Share20/Manager.php" , line : 983usize , placeholders : & [] , regex : "^Can only update recipient on user shares$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/Share20/Manager.php" , line : 989usize , placeholders : & [] , regex : "^Cannot share with the share owner$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/Share20/Manager.php" , line : 1024usize , placeholders : & [] , regex : "^Cannot enable sending the password by Talk with an empty password$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/Share20/Manager.php" , line : 1034usize , placeholders : & [] , regex : "^Cannot enable sending the password by Talk without setting a new password$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/Share20/Manager.php" , line : 1037usize , placeholders : & [] , regex : "^Cannot disable sending the password by Talk without setting a new password$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/Share20/Manager.php" , line : 1119usize , placeholders : & [] , regex : "^Share provider does not support accepting$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/Share20/Manager.php" , line : 1228usize , placeholders : & [] , regex : "^Share does not have a full id$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/Share20/Manager.php" , line : 1283usize , placeholders : & [] , regex : "^Cannot change target of link share$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/Share20/Manager.php" , line : 1287usize , placeholders : & [] , regex : "^Invalid recipient$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/Share20/Manager.php" , line : 1297usize , placeholders : & [] , regex : "^Invalid recipient$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/Share20/Manager.php" , line : 1330usize , placeholders : & [] , regex : "^invalid path$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/Share20/Share.php" , line : 121usize , placeholders : & [] , regex : "^String expected\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/Share20/Share.php" , line : 125usize , placeholders : & [] , regex : "^Not allowed to assign a new internal id to a share$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/Share20/Share.php" , line : 154usize , placeholders : & [] , regex : "^String expected\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/Share20/Share.php" , line : 158usize , placeholders : & [] , regex : "^Not allowed to assign a new provider id to a share$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Warn , path : "/lib/private/InitialStateService.php" , line : 93usize , placeholders : & [] , regex : "^Lazy initial state provider for \\{key\\} took \\{duration\\} seconds\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/Federation/CloudIdManager.php" , line : 62usize , placeholders : & [] , regex : "^Invalid cloud id$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/Federation/CloudIdManager.php" , line : 89usize , placeholders : & [] , regex : "^Invalid cloud id$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/Security/IdentityProof/Manager.php" , line : 77usize , placeholders : & [] , regex : "^OpenSSL reported a problem$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/Security/IdentityProof/Manager.php" , line : 82usize , placeholders : & [] , regex : "^OpenSSL reported a problem$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/Security/IdentityProof/Manager.php" , line : 158usize , placeholders : & [] , regex : "^no instance id!$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/Security/CSRF/TokenStorage/SessionStorage.php" , line : 64usize , placeholders : & [] , regex : "^Session does not contain a requesttoken$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/Security/Crypto.php" , line : 106usize , placeholders : & [] , regex : "^Encrypting failed\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/Security/Crypto.php" , line : 133usize , placeholders : & [] , regex : "^Authenticated ciphertext could not be decoded\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/Security/Crypto.php" , line : 156usize , placeholders : & [] , regex : "^HMAC does not match\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/Security/Crypto.php" , line : 161usize , placeholders : & [] , regex : "^Decryption failed$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/Security/Certificate.php" , line : 61usize , placeholders : & [] , regex : "^Certificate could not get parsed\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/Security/Certificate.php" , line : 66usize , placeholders : & [] , regex : "^Certificate could not get parsed\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/Security/CertificateManager.php" , line : 185usize , placeholders : & [] , regex : "^Filename is not valid$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Error , path : "/lib/private/Security/Bruteforce/Throttler.php" , line : 224usize , placeholders : & [] , regex : "^Bruteforce has to use less than 48 hours$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Info , path : "/lib/private/Security/Bruteforce/Throttler.php" , line : 350usize , placeholders : & [] , regex : "^IP address blocked because it reached the maximum failed attempts in the last 30 minutes \\[action: \\{action\\}, ip: \\{ip\\}\\]$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/Security/Bruteforce/Throttler.php" , line : 355usize , placeholders : & [] , regex : "^Reached maximum delay$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Info , path : "/lib/private/Security/Bruteforce/Throttler.php" , line : 358usize , placeholders : & [] , regex : "^IP address throttled because it reached the attempts limit in the last 30 minutes \\[action: \\{action\\}, delay: \\{delay\\}, ip: \\{ip\\}\\]$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/Files/SimpleFS/NewSimpleFile.php" , line : 164usize , placeholders : & [] , regex : "^File does not exist$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/Files/SimpleFS/SimpleFile.php" , line : 138usize , placeholders : & [] , regex : "^File does not exist$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/Files/Utils/Scanner.php" , line : 196usize , placeholders : & [] , regex : "^Invalid path to scan$" , }
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
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/Files/ObjectStore/AppdataPreviewObjectStoreStorage.php" , line : 35usize , placeholders : & [] , regex : "^missing id in parameters$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/Files/ObjectStore/HomeObjectStoreStorage.php" , line : 38usize , placeholders : & [] , regex : "^missing user object in parameters$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/Files/ObjectStore/ObjectStoreStorage.php" , line : 68usize , placeholders : & [] , regex : "^missing IObjectStore instance$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/Files/ObjectStore/ObjectStoreStorage.php" , line : 529usize , placeholders : & ["$urn" , "$path"] , regex : "^Object not found after writing \\(urn: (.*), path: (.*)\\)$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/Files/ObjectStore/ObjectStoreStorage.php" , line : 568usize , placeholders : & [] , regex : "^Source object not found$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/Files/ObjectStore/ObjectStoreStorage.php" , line : 599usize , placeholders : & [] , regex : "^Invalid source cache for object store copy$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/Files/ObjectStore/S3ObjectTrait.php" , line : 137usize , placeholders : & [] , regex : "^Error while uploading to S3 bucket$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/Files/ObjectStore/S3ConnectionTrait.php" , line : 74usize , placeholders : & [] , regex : "^Bucket has to be configured\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/Files/ObjectStore/Swift.php" , line : 120usize , placeholders : & ["$urn"] , regex : "^object (.*) not found in object store$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/Files/AppData/AppData.php" , line : 76usize , placeholders : & [] , regex : "^no instance id!$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/Files/AppData/AppData.php" , line : 93usize , placeholders : & [] , regex : "^Could not get appdata folder$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/Files/Storage/Local.php" , line : 72usize , placeholders : & [] , regex : "^No data directory set for local storage$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/Files/Storage/Local.php" , line : 496usize , placeholders : & [] , regex : "^Following symlinks is not allowed$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/Files/Storage/Local.php" , line : 608usize , placeholders : & ["$path"] , regex : "^Failed write stream to (.*)$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Warn , path : "/lib/private/Files/Storage/Common.php" , line : 240usize , placeholders : & ["$path1" , "$path2"] , regex : "^Failed to write data while copying (.*) to (.*)$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Info , path : "/lib/private/Files/Storage/Common.php" , line : 462usize , placeholders : & [] , regex : "^External storage not available: stat\\(\\) failed$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/Files/Storage/Common.php" , line : 868usize , placeholders : & ["$path"] , regex : "^Failed to open (.*) for writing$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/Files/Storage/Common.php" , line : 873usize , placeholders : & [] , regex : "^Failed to copy stream$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/Files/Storage/DAV.php" , line : 130usize , placeholders : & [] , regex : "^Invalid webdav storage configuration$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/Files/Storage/DAV.php" , line : 789usize , placeholders : & [] , regex : "^root is gone$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Error , path : "/lib/private/Files/Storage/Wrapper/Encryption.php" , line : 798usize , placeholders : & [] , regex : "^Could not find mount point, can't keep encryption keys$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/Files/Storage/FailedStorage.php" , line : 47usize , placeholders : & [] , regex : "^Missing \"exception\" argument in FailedStorage constructor$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/Files/Storage/StorageFactory.php" , line : 102usize , placeholders : & [] , regex : "^Invalid result from storage wrapper$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Debug , path : "/lib/private/Files/SetupManager.php" , line : 437usize , placeholders : & [] , regex : "^mount has no provider set, performing full setup$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Warn , path : "/lib/private/Files/Filesystem.php" , line : 216usize , placeholders : & [] , regex : "^Storage wrapper '\\{wrapper\\}' was not registered via the 'OC_Filesystem \\- preSetup' hook which could cause potential problems\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/Files/Type/Loader.php" , line : 131usize , placeholders : & ["$mimetype"] , regex : "^Failed to get mimetype id for (.*) after trying to store it$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/Files/Mount/MountPoint.php" , line : 131usize , placeholders : & ["$mountProvider"] , regex : "^Mount provider (.*) name exceeds the limit of 128 characters$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/Files/Mount/MountPoint.php" , line : 174usize , placeholders : & [] , regex : "^The root storage could not be initialized\\. Please contact your local administrator\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Error , path : "/lib/private/Files/Mount/ObjectStorePreviewCacheMountProvider.php" , line : 106usize , placeholders : & [] , regex : "^No class given for objectstore$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Error , path : "/lib/private/Files/Mount/ObjectStorePreviewCacheMountProvider.php" , line : 135usize , placeholders : & [] , regex : "^No class given for objectstore$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Error , path : "/lib/private/Files/Mount/RootMountProvider.php" , line : 59usize , placeholders : & [] , regex : "^No class given for objectstore$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/Files/View.php" , line : 115usize , placeholders : & [] , regex : "^Root can't be null$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/Files/View.php" , line : 494usize , placeholders : & [] , regex : "^fseek error$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Info , path : "/lib/private/Files/View.php" , line : 1007usize , placeholders : & [] , regex : "^Trying to open a file with a mode other than \"r\" or \"w\" can cause severe performance issues with some backends$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/Files/View.php" , line : 1797usize , placeholders : & ["$pathLen" , "$maxLen" , "$path"] , regex : "^Path length\\((.*)\\) exceeds max path length\\((.*)\\): (.*)$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Error , path : "/lib/private/Files/View.php" , line : 2158usize , placeholders : & [] , regex : "^\\$absolutePath must be relative to \"files\", value is \"%s\"$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/Files/View.php" , line : 2164usize , placeholders : & [] , regex : "^\\$absolutePath must be relative to \"files\"$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/Files/Node/File.php" , line : 72usize , placeholders : & [] , regex : "^file_put_contents failed$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/Files/Node/Node.php" , line : 85usize , placeholders : & [] , regex : "^Must be implemented by subclasses$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/Files/Node/Node.php" , line : 160usize , placeholders : & [] , regex : "^No storage for node$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/Files/Node/Root.php" , line : 389usize , placeholders : & ["$userId"] , regex : "^User folder for (.*) exists as a file$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/Files/Node/Root.php" , line : 460usize , placeholders : & [] , regex : "^getByIdInPath with non folder$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/Files/Node/Folder.php" , line : 71usize , placeholders : & [] , regex : "^Invalid path$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/Files/Node/Folder.php" , line : 165usize , placeholders : & [] , regex : "^Could not create folder$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/Files/Node/Folder.php" , line : 171usize , placeholders : & [] , regex : "^No create permission for folder$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/Files/Node/Folder.php" , line : 183usize , placeholders : & [] , regex : "^Could not create as provided path is empty$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/Files/Node/Folder.php" , line : 195usize , placeholders : & [] , regex : "^Could not create path$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/Files/Node/Folder.php" , line : 201usize , placeholders : & [] , regex : "^No create permission for path$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/Files/Node/Folder.php" , line : 231usize , placeholders : & [] , regex : "^searching by owner is only allows on the users home folder$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/Files/Node/Folder.php" , line : 394usize , placeholders : & [] , regex : "^No delete permission for path$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/Files/Lock/LockManager.php" , line : 17usize , placeholders : & [] , regex : "^There is already a registered lock provider$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/Files/Lock/LockManager.php" , line : 51usize , placeholders : & [] , regex : "^No lock provider available$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/Files/Lock/LockManager.php" , line : 59usize , placeholders : & [] , regex : "^No lock provider available$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/Files/Lock/LockManager.php" , line : 67usize , placeholders : & [] , regex : "^No lock provider available$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/Files/Config/CachedMountInfo.php" , line : 65usize , placeholders : & ["$mountProvider"] , regex : "^Mount provider (.*) name exceeds the limit of 128 characters$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Warn , path : "/lib/private/Files/Cache/Propagator.php" , line : 142usize , placeholders : & [] , regex : "^Retrying propagation query after retryable exception\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/Files/Cache/Propagator.php" , line : 190usize , placeholders : & [] , regex : "^Not in batch$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/Files/Cache/SearchBuilder.php" , line : 112usize , placeholders : & [] , regex : "^Binary operators inside \"not\" is not supported$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/Files/Cache/Cache.php" , line : 344usize , placeholders : & [] , regex : "^File entry could not be inserted but could also not be selected with getId\\(\\) in order to perform an update\\. Please try again\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/Files/Cache/Cache.php" , line : 1076usize , placeholders : & [] , regex : "^Invalid source cache entry on copyFromCache$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/Files/Cache/FailedCache.php" , line : 142usize , placeholders : & [] , regex : "^Invalid cache$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/Files/Cache/QuerySearchHelper.php" , line : 110usize , placeholders : & [] , regex : "^Searching by tag requires the user to be set in the query$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/TemplateLayout.php" , line : 390usize , placeholders : & [] , regex : "^\\$filePath is not under the \\\\OC::\\$SERVERROOT$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/Route/Router.php" , line : 302usize , placeholders : & [] , regex : "^not a callable action$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/Route/Router.php" , line : 310usize , placeholders : & [] , regex : "^no action available$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/Dashboard/DashboardManager.php" , line : 121usize , placeholders : & [] , regex : "^No IWidgetsService registered$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/Dashboard/DashboardManager.php" , line : 134usize , placeholders : & [] , regex : "^No IEventsService registered$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/Dashboard/Manager.php" , line : 55usize , placeholders : & [] , regex : "^Dashboard widget with this id has already been registered$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Error , path : "/lib/private/Dashboard/Manager.php" , line : 112usize , placeholders : & [] , regex : "^Dashboard widget \\{widget\\} took \\{duration\\} seconds to load\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/Collaboration/Collaborators/Search.php" , line : 111usize , placeholders : & [] , regex : "^Provided ShareType is invalid$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/Collaboration/Collaborators/RemotePlugin.php" , line : 193usize , placeholders : & [] , regex : "^Invalid Federated Cloud ID$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/Collaboration/Collaborators/RemoteGroupPlugin.php" , line : 91usize , placeholders : & [] , regex : "^Invalid Federated Cloud ID$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Warn , path : "/lib/private/Collaboration/AutoComplete/Manager.php" , line : 49usize , placeholders : & [] , regex : "^No sorter for ID \"\\{id\\}\", skipping$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Notice , path : "/lib/private/Collaboration/AutoComplete/Manager.php" , line : 66usize , placeholders : & [] , regex : "^Skipping sorter which is not an instance of ISorter\\. Class name: \\{class\\}$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Notice , path : "/lib/private/Collaboration/AutoComplete/Manager.php" , line : 72usize , placeholders : & [] , regex : "^Skipping sorter with empty ID\\. Class name: \\{class\\}$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/Collaboration/Resources/Manager.php" , line : 81usize , placeholders : & [] , regex : "^Collection not found$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/Collaboration/Resources/Manager.php" , line : 113usize , placeholders : & [] , regex : "^Collection not found$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/Collaboration/Resources/Manager.php" , line : 230usize , placeholders : & [] , regex : "^Resource not found$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Debug , path : "/lib/private/Collaboration/Resources/Manager.php" , line : 512usize , placeholders : & [] , regex : "^\\\\OC\\\\Collaboration\\\\Resources\\\\Manager::registerResourceProvider is deprecated$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/Collaboration/Resources/Collection.php" , line : 131usize , placeholders : & [] , regex : "^Already part of the collection$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/Collaboration/Resources/Collection.php" , line : 148usize , placeholders : & [] , regex : "^Already part of the collection$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/L10N/Factory.php" , line : 411usize , placeholders : & [] , regex : "^Failed to get an IUser instance$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Warn , path : "/lib/private/L10N/L10N.php" , line : 233usize , placeholders : & ["$translationFile" , "$jsonError"] , regex : "^Failed to load (.*) \\- json error code: (.*)$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/LargeFileHelper.php" , line : 60usize , placeholders : & [] , regex : "^This class assumes floats to be double precision or \"better\"\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/LargeFileHelper.php" , line : 87usize , placeholders : & [] , regex : "^Expected int, float or base\\-10 string$" , }
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
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/Notification/Manager.php" , line : 330usize , placeholders : & [] , regex : "^The given notification is invalid$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/Notification/Manager.php" , line : 381usize , placeholders : & [] , regex : "^The given notification has been processed$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/Notification/Manager.php" , line : 385usize , placeholders : & [] , regex : "^The given notification has not been handled$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/Notification/Manager.php" , line : 390usize , placeholders : & [] , regex : "^The given notification has not been handled$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/Notification/Notification.php" , line : 134usize , placeholders : & [] , regex : "^The given app name is invalid$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/Notification/Notification.php" , line : 156usize , placeholders : & [] , regex : "^The given user id is invalid$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/Notification/Notification.php" , line : 178usize , placeholders : & [] , regex : "^The given date time is invalid$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/Notification/Notification.php" , line : 201usize , placeholders : & [] , regex : "^The given object type is invalid$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/Notification/Notification.php" , line : 206usize , placeholders : & [] , regex : "^The given object id is invalid$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/Notification/Notification.php" , line : 237usize , placeholders : & [] , regex : "^The given subject is invalid$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/Notification/Notification.php" , line : 270usize , placeholders : & [] , regex : "^The given parsed subject is invalid$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/Notification/Notification.php" , line : 293usize , placeholders : & [] , regex : "^The given parsed subject is invalid$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/Notification/Notification.php" , line : 327usize , placeholders : & [] , regex : "^The given message is invalid$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/Notification/Notification.php" , line : 360usize , placeholders : & [] , regex : "^The given parsed message is invalid$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/Notification/Notification.php" , line : 383usize , placeholders : & [] , regex : "^The given parsed message is invalid$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/Notification/Notification.php" , line : 416usize , placeholders : & [] , regex : "^The given link is invalid$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/Notification/Notification.php" , line : 438usize , placeholders : & [] , regex : "^The given icon is invalid$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/Notification/Notification.php" , line : 468usize , placeholders : & [] , regex : "^The given action is invalid$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/Notification/Notification.php" , line : 473usize , placeholders : & [] , regex : "^The notification already has a primary action$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/Notification/Notification.php" , line : 499usize , placeholders : & [] , regex : "^The given parsed action is invalid$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/Notification/Notification.php" , line : 504usize , placeholders : & [] , regex : "^The notification already has a primary action$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/Notification/Action.php" , line : 65usize , placeholders : & [] , regex : "^The given label is invalid$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/Notification/Action.php" , line : 87usize , placeholders : & [] , regex : "^The given parsed label is invalid$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/Notification/Action.php" , line : 128usize , placeholders : & [] , regex : "^The given link is invalid$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/Notification/Action.php" , line : 137usize , placeholders : & [] , regex : "^The given request type is invalid$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/DB/ResultAdapter.php" , line : 56usize , placeholders : & [] , regex : "^Fetch mode needs to be assoc, num or column\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/DB/PreparedStatement.php" , line : 99usize , placeholders : & [] , regex : "^You have to execute the prepared statement before accessing the results$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/DB/AdapterOCI8.php" , line : 30usize , placeholders : & [] , regex : "^Oracle requires a table name to be passed into lastInsertId\\(\\)$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/DB/MigrationService.php" , line : 250usize , placeholders : & ["$version"] , regex : "^Cannot load a migrations with the name '(.*)' because it is a reserved number$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/DB/MigrationService.php" , line : 396usize , placeholders : & ["$version"] , regex : "^Version (.*) is unknown\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/DB/MigrationService.php" , line : 501usize , placeholders : & [] , regex : "^Not a valid migration$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/DB/MigrationService.php" , line : 507usize , placeholders : & ["$class"] , regex : "^Migration step '(.*)' is unknown$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/DB/Connection.php" , line : 95usize , placeholders : & [] , regex : "^adapter not set$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/DB/Connection.php" , line : 98usize , placeholders : & [] , regex : "^tablePrefix not set$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Debug , path : "/lib/private/DB/Connection.php" , line : 170usize , placeholders : & [] , regex : "^Doctrine QueryBuilder retrieved in \\{backtrace\\}$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Debug , path : "/lib/private/DB/Connection.php" , line : 183usize , placeholders : & [] , regex : "^Doctrine ExpressionBuilder retrieved in \\{backtrace\\}$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/DB/Connection.php" , line : 440usize , placeholders : & [] , regex : "^Can not lock a new table until the previous lock is released\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/DB/QueryBuilder/QuoteHelper.php" , line : 61usize , placeholders : & [] , regex : "^Only strings, Literals and Parameters are allowed$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Debug , path : "/lib/private/DB/QueryBuilder/QueryBuilder.php" , line : 217usize , placeholders : & [] , regex : "^DB QueryBuilder: '\\{query\\}'$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Debug , path : "/lib/private/DB/QueryBuilder/QueryBuilder.php" , line : 222usize , placeholders : & [] , regex : "^DB QueryBuilder: '\\{query\\}' with parameters: \\{params\\}$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Error , path : "/lib/private/DB/QueryBuilder/QueryBuilder.php" , line : 230usize , placeholders : & [] , regex : "^DB QueryBuilder: error trying to log SQL query$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/DB/QueryBuilder/QueryBuilder.php" , line : 290usize , placeholders : & [] , regex : "^Invalid query type, expected SELECT query$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/DB/QueryBuilder/QueryBuilder.php" , line : 303usize , placeholders : & [] , regex : "^Invalid return type for query$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/DB/QueryBuilder/QueryBuilder.php" , line : 323usize , placeholders : & [] , regex : "^Invalid query type, expected INSERT, DELETE or UPDATE statement$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/DB/QueryBuilder/QueryBuilder.php" , line : 333usize , placeholders : & [] , regex : "^Invalid return type for statement$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/DB/QueryBuilder/QueryBuilder.php" , line : 1294usize , placeholders : & [] , regex : "^Invalid call to getLastInsertId without using insert\\(\\) before\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/DB/ConnectionFactory.php" , line : 107usize , placeholders : & ["$type"] , regex : "^Unsupported type: (.*)$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Error , path : "/lib/private/Metadata/FileEventListener.php" , line : 77usize , placeholders : & [] , regex : "^Detecting deletion of a file with possible metadata but file system setup is not setup$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/Contacts/ContactsMenu/ActionProviderStore.php" , line : 79usize , placeholders : & [] , regex : "^Could not load contacts menu action provider$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Debug , path : "/lib/private/Authentication/LoginCredentials/Store.php" , line : 101usize , placeholders : & [] , regex : "^could not get login credentials because session is unavailable$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Debug , path : "/lib/private/Authentication/LoginCredentials/Store.php" , line : 106usize , placeholders : & [] , regex : "^could not get login credentials because the token has no password$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Warn , path : "/lib/private/Authentication/Login/LoggedInCheckCommand.php" , line : 53usize , placeholders : & ["$loginName" , "$ip"] , regex : "^Login failed: (.*) \\(Remote IP: (.*)\\)$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Warn , path : "/lib/private/Authentication/Login/UserDisabledCheckCommand.php" , line : 53usize , placeholders : & ["$username" , "$ip"] , regex : "^Login failed: (.*) disabled \\(Remote IP: (.*)\\)$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/Authentication/WebAuthn/Manager.php" , line : 156usize , placeholders : & [] , regex : "^Not an authenticator attestation response$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/Authentication/WebAuthn/Manager.php" , line : 227usize , placeholders : & [] , regex : "^Not an authenticator attestation response$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Warn , path : "/lib/private/Authentication/WebAuthn/Manager.php" , line : 253usize , placeholders : & ["$id"] , regex : "^WebAuthn device (.*) does not exist, can't delete it$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Warn , path : "/lib/private/Authentication/Listeners/RemoteWipeActivityListener.php" , line : 75usize , placeholders : & [] , regex : "^could not publish activity$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/Authentication/Listeners/UserDeletedFilesCleanupListener.php" , line : 58usize , placeholders : & [] , regex : "^User has no home storage$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/Authentication/Listeners/UserDeletedFilesCleanupListener.php" , line : 71usize , placeholders : & [] , regex : "^UserDeletedEvent fired without matching BeforeUserDeletedEvent$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/Authentication/Listeners/UserDeletedFilesCleanupListener.php" , line : 78usize , placeholders : & [] , regex : "^Home storage has invalid cache$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Warn , path : "/lib/private/Authentication/Listeners/RemoteWipeEmailListener.php" , line : 78usize , placeholders : & ["$uid"] , regex : "^not sending a wipe started email because user <(.*)> does not exist \\(anymore\\)$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Info , path : "/lib/private/Authentication/Listeners/RemoteWipeEmailListener.php" , line : 82usize , placeholders : & ["$uid"] , regex : "^not sending a wipe started email because user <(.*)> has no email set$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Error , path : "/lib/private/Authentication/Listeners/RemoteWipeEmailListener.php" , line : 91usize , placeholders : & ["$uid"] , regex : "^Could not send remote wipe started email to <(.*)>$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Warn , path : "/lib/private/Authentication/Listeners/RemoteWipeEmailListener.php" , line : 99usize , placeholders : & ["$uid"] , regex : "^not sending a wipe finished email because user <(.*)> does not exist \\(anymore\\)$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Info , path : "/lib/private/Authentication/Listeners/RemoteWipeEmailListener.php" , line : 103usize , placeholders : & ["$uid"] , regex : "^not sending a wipe finished email because user <(.*)> has no email set$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Error , path : "/lib/private/Authentication/Listeners/RemoteWipeEmailListener.php" , line : 112usize , placeholders : & ["$uid"] , regex : "^Could not send remote wipe finished email to <(.*)>$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/Authentication/TwoFactorAuth/ProviderLoader.php" , line : 73usize , placeholders : & ["$class"] , regex : "^Could not load two\\-factor auth provider (.*)$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Alert , path : "/lib/private/Authentication/TwoFactorAuth/Manager.php" , line : 217usize , placeholders : & ["$providerId"] , regex : "^two\\-factor auth provider '(.*)' failed to load$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Warn , path : "/lib/private/Authentication/TwoFactorAuth/Manager.php" , line : 322usize , placeholders : & [] , regex : "^could not publish activity$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/Authentication/Token/PublicKeyTokenMapper.php" , line : 89usize , placeholders : & [] , regex : "^token does not exist$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/Authentication/Token/PublicKeyTokenMapper.php" , line : 111usize , placeholders : & [] , regex : "^token does not exist$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/Authentication/Token/Manager.php" , line : 88usize , placeholders : & [] , regex : "^Token conflict handled, but UIDs do not match\\. This should not happen$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/Authentication/Token/PublicKeyTokenProvider.php" , line : 173usize , placeholders : & [] , regex : "^Invalid token type$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/Authentication/Token/PublicKeyTokenProvider.php" , line : 224usize , placeholders : & [] , regex : "^Invalid token type$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/Authentication/Token/PublicKeyTokenProvider.php" , line : 233usize , placeholders : & [] , regex : "^Invalid token type$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/Authentication/Token/PublicKeyTokenProvider.php" , line : 253usize , placeholders : & [] , regex : "^Invalid token type$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/Authentication/Token/PublicKeyTokenProvider.php" , line : 271usize , placeholders : & [] , regex : "^Invalid token type$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/Authentication/Token/PublicKeyTokenProvider.php" , line : 290usize , placeholders : & [] , regex : "^Invalid token type$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/Authentication/Token/PublicKeyTokenProvider.php" , line : 365usize , placeholders : & [] , regex : "^OpenSSL reported a problem$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/Authentication/Token/PublicKeyTokenProvider.php" , line : 370usize , placeholders : & [] , regex : "^OpenSSL reported a problem$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/Authentication/Token/PublicKeyTokenProvider.php" , line : 382usize , placeholders : & [] , regex : "^Trying to save a password with more than 469 characters is not supported\\. If you want to use big passwords, disable the auth\\.storeCryptedPassword option in config\\.php$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/Authentication/Token/PublicKeyTokenProvider.php" , line : 402usize , placeholders : & [] , regex : "^Invalid token type$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Info , path : "/lib/private/EventDispatcher/GenericEventWrapper.php" , line : 57usize , placeholders : & [] , regex : "^Deprecated event type for \\{name\\}: \\{class\\} is used$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Info , path : "/lib/private/EventDispatcher/SymfonyAdapter.php" , line : 109usize , placeholders : & [] , regex : "^Deprecated event type for \\{name\\}: \\{class\\}$" , }
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
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/Remote/Api/OCS.php" , line : 59usize , placeholders : & [] , regex : "^Invalid ocs response$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/Http/WellKnown/RequestManager.php" , line : 95usize , placeholders : & [] , regex : "^Well known handlers requested before the apps had been fully registered$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Error , path : "/lib/private/Http/WellKnown/RequestManager.php" , line : 110usize , placeholders : & ["$class"] , regex : "^Well known handler (.*) is invalid$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Error , path : "/lib/private/Http/WellKnown/RequestManager.php" , line : 117usize , placeholders : & ["$class"] , regex : "^Could not load well known handler (.*)$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Warn , path : "/lib/private/Http/Client/LocalAddressChecker.php" , line : 68usize , placeholders : & ["$ip"] , regex : "^Host (.*) was not connected to because it violates local access rules$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/Http/Client/LocalAddressChecker.php" , line : 69usize , placeholders : & [] , regex : "^Host violates local access rules$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Warn , path : "/lib/private/Http/Client/LocalAddressChecker.php" , line : 76usize , placeholders : & ["$uri"] , regex : "^Could not detect any host in (.*)$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/Http/Client/LocalAddressChecker.php" , line : 77usize , placeholders : & [] , regex : "^Could not detect any host$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Warn , path : "/lib/private/Http/Client/LocalAddressChecker.php" , line : 90usize , placeholders : & ["$host"] , regex : "^Host (.*) was not connected to because it violates local access rules$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/Http/Client/LocalAddressChecker.php" , line : 91usize , placeholders : & [] , regex : "^Host violates local access rules$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Warn , path : "/lib/private/Http/Client/LocalAddressChecker.php" , line : 96usize , placeholders : & ["$host"] , regex : "^Host (.*) was not connected to because it violates local access rules$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/Http/Client/LocalAddressChecker.php" , line : 97usize , placeholders : & [] , regex : "^Host violates local access rules$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/Http/IpUtils.php" , line : 116usize , placeholders : & [] , regex : "^Unable to check Ipv6\\. Check that PHP was not compiled with option \"disable\\-ipv6\"\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Warn , path : "/lib/private/Support/Subscription/Registry.php" , line : 214usize , placeholders : & ["$userCount" , "$disabledUsersCount"] , regex : "^Total user count was negative \\(users: (.*), disabled: (.*)\\)$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Warn , path : "/lib/private/Support/Subscription/Registry.php" , line : 242usize , placeholders : & [] , regex : "^The user limit was reached and the new user was not created$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/SystemTag/SystemTagObjectMapper.php" , line : 119usize , placeholders : & [] , regex : "^Limit is only allowed with a single tag$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/SystemTag/SystemTagObjectMapper.php" , line : 268usize , placeholders : & [] , regex : "^Tags not found$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/SystemTag/SystemTagManager.php" , line : 104usize , placeholders : & [] , regex : "^Tag id must be integer$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/SystemTag/SystemTagManager.php" , line : 126usize , placeholders : & [] , regex : "^Tag id\\(s\\) not found$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/SystemTag/SystemTagManager.php" , line : 240usize , placeholders : & [] , regex : "^Tag does not exist$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/SystemTag/SystemTagManager.php" , line : 266usize , placeholders : & [] , regex : "^Tag does not exist$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/SystemTag/SystemTagManager.php" , line : 329usize , placeholders : & [] , regex : "^Tag id\\(s\\) not found$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/Avatar/AvatarManager.php" , line : 114usize , placeholders : & [] , regex : "^user does not exist$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Debug , path : "/lib/private/Avatar/AvatarManager.php" , line : 168usize , placeholders : & ["$userId"] , regex : "^No cache for the user (.*)\\. Ignoring\\.\\.\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Debug , path : "/lib/private/Avatar/AvatarManager.php" , line : 179usize , placeholders : & ["$userId"] , regex : "^No cache for the user (.*)\\. Ignoring avatar deletion$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Error , path : "/lib/private/Avatar/AvatarManager.php" , line : 181usize , placeholders : & ["$userId"] , regex : "^Unable to delete user avatars for (.*)\\. gnoring avatar deletion$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Debug , path : "/lib/private/Avatar/AvatarManager.php" , line : 183usize , placeholders : & ["$userId"] , regex : "^User (.*) not found\\. gnoring avatar deletion$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/DirectEditing/Manager.php" , line : 99usize , placeholders : & [] , regex : "^No matching editor found$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/DirectEditing/Manager.php" , line : 131usize , placeholders : & [] , regex : "^File already exists$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/DirectEditing/Manager.php" , line : 134usize , placeholders : & [] , regex : "^Invalid path$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/DirectEditing/Manager.php" , line : 149usize , placeholders : & [] , regex : "^No creator found$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/DirectEditing/Manager.php" , line : 160usize , placeholders : & ["$editorId"] , regex : "^Editor (.*) is unknown$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/DirectEditing/Manager.php" , line : 172usize , placeholders : & [] , regex : "^No default editor found for files mimetype$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/DirectEditing/Manager.php" , line : 180usize , placeholders : & [] , regex : "^Token has already been used and can only be used for followup requests$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/DirectEditing/Manager.php" , line : 197usize , placeholders : & [] , regex : "^No editor found$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/DirectEditing/Manager.php" , line : 210usize , placeholders : & [] , regex : "^Failed to validate the token$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/Installer.php" , line : 107usize , placeholders : & [] , regex : "^App not found in any app directory$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/Installer.php" , line : 249usize , placeholders : & [] , regex : "^Could not validate CRL signature$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/Console/Application.php" , line : 164usize , placeholders : & [] , regex : "^Environment not properly prepared\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/Console/Application.php" , line : 226usize , placeholders : & ["$command"] , regex : "^Console command '(.*)' is unknown and could not be loaded$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/Log/Systemdlog.php" , line : 63usize , placeholders : & [] , regex : "^PHP extension php\\-systemd is not available\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/Log/PsrLoggerAdapter.php" , line : 250usize , placeholders : & [] , regex : "^Nextcloud allows only integer log levels$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/Talk/Broker.php" , line : 66usize , placeholders : & [] , regex : "^Not all apps have been registered yet$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/Talk/Broker.php" , line : 100usize , placeholders : & [] , regex : "^The Talk broker has no registered backend$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Error , path : "/lib/private/Setup/MySQL.php" , line : 86usize , placeholders : & [] , regex : "^Database creation failed\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Debug , path : "/lib/private/Setup/MySQL.php" , line : 98usize , placeholders : & [] , regex : "^Could not automatically grant privileges, this can be ignored if database user already had privileges\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Error , path : "/lib/private/Setup/MySQL.php" , line : 128usize , placeholders : & [] , regex : "^Database user creation failed\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Info , path : "/lib/private/Setup/MySQL.php" , line : 192usize , placeholders : & [] , regex : "^Can not create a new MySQL user, will continue with the provided user\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Warn , path : "/lib/private/Setup/PostgreSQL.php" , line : 85usize , placeholders : & [] , regex : "^Error trying to connect as \"postgres\", assuming database is setup and tables need to be created$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Error , path : "/lib/private/Setup/PostgreSQL.php" , line : 116usize , placeholders : & [] , regex : "^Error while trying to create database$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Error , path : "/lib/private/Setup/PostgreSQL.php" , line : 125usize , placeholders : & [] , regex : "^Error while trying to restrict database permissions$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Error , path : "/lib/private/Setup/PostgreSQL.php" , line : 169usize , placeholders : & [] , regex : "^Error while trying to create database user$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/RedisFactory.php" , line : 59usize , placeholders : & [] , regex : "^Redis Cluster support is not available$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/RedisFactory.php" , line : 90usize , placeholders : & [] , regex : "^Redis cluster config is missing the \"seeds\" attribute$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/private/RedisFactory.php" , line : 175usize , placeholders : & [] , regex : "^Redis support is not available$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/composer/composer/ClassLoader.php" , line : 252usize , placeholders : & [] , regex : "^A non\\-empty PSR\\-4 prefix must end with a namespace separator\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/lib/composer/composer/ClassLoader.php" , line : 307usize , placeholders : & [] , regex : "^A non\\-empty PSR\\-4 prefix must end with a namespace separator\\.$" , }
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
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/notifications/lib/App.php" , line : 59usize , placeholders : & [] , regex : "^Error while preparing push notification$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/notifications/lib/Notifier/AdminNotifications.php" , line : 78usize , placeholders : & [] , regex : "^Unknown app$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/notifications/lib/Notifier/AdminNotifications.php" , line : 96usize , placeholders : & [] , regex : "^Unknown subject$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Debug , path : "/apps/notifications/lib/Push.php" , line : 419usize , placeholders : & [] , regex : "^Could not send notification to push server \\[\\{url\\}\\]: \\{error\\}$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Warn , path : "/apps/notifications/lib/Push.php" , line : 454usize , placeholders : & [] , regex : "^Could not send notification to push server \\[\\{url\\}\\]: \\{error\\}$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Info , path : "/apps/notifications/lib/Push.php" , line : 462usize , placeholders : & [] , regex : "^Push notification sent but response was not parsable, using an outdated push proxy\\? \\[\\{url\\}\\]: \\{error\\}$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/notifications/lib/Push.php" , line : 541usize , placeholders : & [] , regex : "^Failed to encrypt message for device$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/notifications/lib/Push.php" , line : 584usize , placeholders : & [] , regex : "^Failed to encrypt message for device$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/notifications/lib/Handler.php" , line : 206usize , placeholders : & [] , regex : "^No entry returned from database$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/notifications/lib/Handler.php" , line : 212usize , placeholders : & [] , regex : "^Could not create notification from database row$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/notifications/lib/FakeUser.php" , line : 40usize , placeholders : & [] , regex : "^Not implemented$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/notifications/lib/FakeUser.php" , line : 44usize , placeholders : & [] , regex : "^Not implemented$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/notifications/lib/FakeUser.php" , line : 48usize , placeholders : & [] , regex : "^Not implemented$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/notifications/lib/FakeUser.php" , line : 52usize , placeholders : & [] , regex : "^Not implemented$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/notifications/lib/FakeUser.php" , line : 56usize , placeholders : & [] , regex : "^Not implemented$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/notifications/lib/FakeUser.php" , line : 60usize , placeholders : & [] , regex : "^Not implemented$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/notifications/lib/FakeUser.php" , line : 64usize , placeholders : & [] , regex : "^Not implemented$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/notifications/lib/FakeUser.php" , line : 68usize , placeholders : & [] , regex : "^Not implemented$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/notifications/lib/FakeUser.php" , line : 72usize , placeholders : & [] , regex : "^Not implemented$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/notifications/lib/FakeUser.php" , line : 76usize , placeholders : & [] , regex : "^Not implemented$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/notifications/lib/FakeUser.php" , line : 80usize , placeholders : & [] , regex : "^Not implemented$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/notifications/lib/FakeUser.php" , line : 84usize , placeholders : & [] , regex : "^Not implemented$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/notifications/lib/FakeUser.php" , line : 88usize , placeholders : & [] , regex : "^Not implemented$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/notifications/lib/FakeUser.php" , line : 92usize , placeholders : & [] , regex : "^Not implemented$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/notifications/lib/FakeUser.php" , line : 96usize , placeholders : & [] , regex : "^Not implemented$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/notifications/lib/FakeUser.php" , line : 100usize , placeholders : & [] , regex : "^Not implemented$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/notifications/lib/FakeUser.php" , line : 104usize , placeholders : & [] , regex : "^Not implemented$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/notifications/lib/FakeUser.php" , line : 108usize , placeholders : & [] , regex : "^Not implemented$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/notifications/lib/FakeUser.php" , line : 112usize , placeholders : & [] , regex : "^Not implemented$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/notifications/lib/FakeUser.php" , line : 116usize , placeholders : & [] , regex : "^Not implemented$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/notifications/lib/FakeUser.php" , line : 120usize , placeholders : & [] , regex : "^Not implemented$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/notifications/lib/FakeUser.php" , line : 124usize , placeholders : & [] , regex : "^Not implemented$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/notifications/lib/FakeUser.php" , line : 128usize , placeholders : & [] , regex : "^Not implemented$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/notifications/lib/FakeUser.php" , line : 132usize , placeholders : & [] , regex : "^Not implemented$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/federatedfilesharing/lib/FederatedShareProvider.php" , line : 370usize , placeholders : & [] , regex : "^share not found in share_external table$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/federatedfilesharing/lib/FederatedShareProvider.php" , line : 637usize , placeholders : & [] , regex : "^not implemented$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/federatedfilesharing/lib/FederatedShareProvider.php" , line : 850usize , placeholders : & [] , regex : "^Share not found$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/federatedfilesharing/lib/FederatedShareProvider.php" , line : 856usize , placeholders : & [] , regex : "^Share not found$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/federatedfilesharing/lib/AddressHandler.php" , line : 79usize , placeholders : & [] , regex : "^Invalid Federated Cloud ID$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/federatedfilesharing/lib/OCM/CloudFederationProviderFiles.php" , line : 191usize , placeholders : & [] , regex : "^Server does not support federated cloud sharing$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/federatedfilesharing/lib/OCM/CloudFederationProviderFiles.php" , line : 196usize , placeholders : & [] , regex : "^Unsupported protocol for data exchange\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/federatedfilesharing/lib/OCM/CloudFederationProviderFiles.php" , line : 225usize , placeholders : & [] , regex : "^The mountpoint name contains invalid characters\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/federatedfilesharing/lib/OCM/CloudFederationProviderFiles.php" , line : 239usize , placeholders : & [] , regex : "^User does not exists$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/federatedfilesharing/lib/OCM/CloudFederationProviderFiles.php" , line : 246usize , placeholders : & [] , regex : "^Group does not exists$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/federatedfilesharing/lib/OCM/CloudFederationProviderFiles.php" , line : 286usize , placeholders : & [] , regex : "^server can not add remote share, missing parameter$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/federatedfilesharing/lib/OCM/CloudFederationProviderFiles.php" , line : 372usize , placeholders : & [] , regex : "^Server does not support federated cloud sharing$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/federatedfilesharing/lib/OCM/CloudFederationProviderFiles.php" , line : 442usize , placeholders : & [] , regex : "^Server does not support federated cloud sharing$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/federatedfilesharing/lib/OCM/CloudFederationProviderFiles.php" , line : 536usize , placeholders : & [] , regex : "^incoming shares disabled!$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/federatedfilesharing/lib/OCM/CloudFederationProviderFiles.php" , line : 684usize , placeholders : & [] , regex : "^Updating reshares not allowed$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/federatedfilesharing/lib/Controller/RequestHandlerController.php" , line : 172usize , placeholders : & [] , regex : "^Server does not support federated cloud sharing$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/federatedfilesharing/lib/Controller/RequestHandlerController.php" , line : 224usize , placeholders : & [] , regex : "^Server does not support federated cloud sharing$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/federatedfilesharing/lib/Controller/RequestHandlerController.php" , line : 260usize , placeholders : & [] , regex : "^Server does not support federated cloud sharing$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/federatedfilesharing/lib/Controller/RequestHandlerController.php" , line : 293usize , placeholders : & [] , regex : "^Server does not support federated cloud sharing$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/federatedfilesharing/lib/Controller/RequestHandlerController.php" , line : 315usize , placeholders : & [] , regex : "^Server does not support federated cloud sharing$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/federatedfilesharing/lib/Controller/RequestHandlerController.php" , line : 446usize , placeholders : & [] , regex : "^Server does not support federated cloud sharing$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/federatedfilesharing/lib/Controller/RequestHandlerController.php" , line : 466usize , placeholders : & [] , regex : "^Share not found or token invalid$" , }
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
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/federatedfilesharing/lib/Notifier.php" , line : 254usize , placeholders : & [] , regex : "^No contact found for federated cloud id$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/federatedfilesharing/lib/Notifier.php" , line : 276usize , placeholders : & [] , regex : "^No contact found for federated cloud id$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/federatedfilesharing/composer/composer/ClassLoader.php" , line : 252usize , placeholders : & [] , regex : "^A non\\-empty PSR\\-4 prefix must end with a namespace separator\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/federatedfilesharing/composer/composer/ClassLoader.php" , line : 307usize , placeholders : & [] , regex : "^A non\\-empty PSR\\-4 prefix must end with a namespace separator\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/contactsinteraction/composer/composer/ClassLoader.php" , line : 252usize , placeholders : & [] , regex : "^A non\\-empty PSR\\-4 prefix must end with a namespace separator\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/contactsinteraction/composer/composer/ClassLoader.php" , line : 307usize , placeholders : & [] , regex : "^A non\\-empty PSR\\-4 prefix must end with a namespace separator\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Warn , path : "/apps/contactsinteraction/lib/Listeners/ContactInteractionListener.php" , line : 83usize , placeholders : & [] , regex : "^Contact interaction event has no user identifier set$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Info , path : "/apps/contactsinteraction/lib/Listeners/ContactInteractionListener.php" , line : 88usize , placeholders : & [] , regex : "^Ignoring contact interaction with self$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/contactsinteraction/lib/AddressBook.php" , line : 70usize , placeholders : & [] , regex : "^This addressbook is immutable$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/contactsinteraction/lib/AddressBook.php" , line : 77usize , placeholders : & [] , regex : "^This addressbook is immutable$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/contactsinteraction/lib/AddressBook.php" , line : 141usize , placeholders : & [] , regex : "^This addressbook is immutable$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Warn , path : "/apps/settings/lib/Controller/AuthSettingsController.php" , line : 278usize , placeholders : & [] , regex : "^could not publish activity$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/settings/lib/Controller/AuthSettingsController.php" , line : 296usize , placeholders : & [] , regex : "^This token does not belong to you!$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Warn , path : "/apps/settings/lib/Controller/CheckSetupController.php" , line : 316usize , placeholders : & [] , regex : "^error checking curl$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Debug , path : "/apps/settings/lib/Controller/WebAuthnController.php" , line : 74usize , placeholders : & [] , regex : "^Starting WebAuthn registration$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Debug , path : "/apps/settings/lib/Controller/WebAuthnController.php" , line : 91usize , placeholders : & [] , regex : "^Finishing WebAuthn registration$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Debug , path : "/apps/settings/lib/Controller/WebAuthnController.php" , line : 94usize , placeholders : & [] , regex : "^Trying to finish WebAuthn registration without session data$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Debug , path : "/apps/settings/lib/Controller/WebAuthnController.php" , line : 112usize , placeholders : & [] , regex : "^Finishing WebAuthn registration$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/settings/lib/Controller/LogSettingsController.php" , line : 53usize , placeholders : & [] , regex : "^Log file not available$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/settings/lib/Controller/AdminSettingsController.php" , line : 81usize , placeholders : & [] , regex : "^Logged in user doesn't have permission to access these settings\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Warn , path : "/apps/settings/lib/Controller/AppSettingsController.php" , line : 279usize , placeholders : & [] , regex : "^The value given for app_install_overwrite is not an array\\. Ignoring\\.\\.\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Error , path : "/apps/settings/lib/Controller/AppSettingsController.php" , line : 464usize , placeholders : & [] , regex : "^could not enable apps$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Error , path : "/apps/settings/lib/Controller/AppSettingsController.php" , line : 505usize , placeholders : & [] , regex : "^could not disable app$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/settings/lib/Activity/Provider.php" , line : 92usize , placeholders : & [] , regex : "^Unknown app$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/settings/lib/Activity/Provider.php" , line : 132usize , placeholders : & [] , regex : "^Unknown subject$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/settings/lib/Activity/Provider.php" , line : 188usize , placeholders : & [] , regex : "^Unknown subject$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/settings/lib/Service/AuthorizedGroupService.php" , line : 67usize , placeholders : & [] , regex : "^AuthorizedGroup not found$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/settings/lib/UserMigration/AccountMigrator.php" , line : 101usize , placeholders : & [] , regex : "^Could not export account information$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/settings/lib/UserMigration/AccountMigrator.php" , line : 114usize , placeholders : & [] , regex : "^Could not export avatar$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/settings/lib/UserMigration/AccountMigrator.php" , line : 138usize , placeholders : & [] , regex : "^Failed to import account information$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/settings/lib/UserMigration/AccountMigrator.php" , line : 163usize , placeholders : & [] , regex : "^Avatar image must be square$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/settings/lib/UserMigration/AccountMigrator.php" , line : 165usize , placeholders : & [] , regex : "^Failed to import avatar$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/settings/composer/composer/ClassLoader.php" , line : 252usize , placeholders : & [] , regex : "^A non\\-empty PSR\\-4 prefix must end with a namespace separator\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/settings/composer/composer/ClassLoader.php" , line : 307usize , placeholders : & [] , regex : "^A non\\-empty PSR\\-4 prefix must end with a namespace separator\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/twofactor_backupcodes/composer/composer/ClassLoader.php" , line : 252usize , placeholders : & [] , regex : "^A non\\-empty PSR\\-4 prefix must end with a namespace separator\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/twofactor_backupcodes/composer/composer/ClassLoader.php" , line : 307usize , placeholders : & [] , regex : "^A non\\-empty PSR\\-4 prefix must end with a namespace separator\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Warn , path : "/apps/twofactor_backupcodes/lib/Listener/ActivityPublisher.php" , line : 63usize , placeholders : & [] , regex : "^could not publish backup code creation activity$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Debug , path : "/apps/text/lib/Service/DocumentService.php" , line : 129usize , placeholders : & [] , regex : "^Unsaved steps but collission with file, continue collaborative editing$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/text/lib/Service/DocumentService.php" , line : 140usize , placeholders : & [] , regex : "^No app data folder present for text documents$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/text/lib/Service/DocumentService.php" , line : 169usize , placeholders : & [] , regex : "^No app data folder present for text documents$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/text/lib/Service/DocumentService.php" , line : 194usize , placeholders : & [] , regex : "^Version does not match$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/text/lib/Service/DocumentService.php" , line : 200usize , placeholders : & [] , regex : "^Version does not match$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/text/lib/Service/DocumentService.php" , line : 204usize , placeholders : & [] , regex : "^Failed to encode steps$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/text/lib/Service/DocumentService.php" , line : 209usize , placeholders : & [] , regex : "^Invalid step data$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/text/lib/Service/DocumentService.php" , line : 293usize , placeholders : & [] , regex : "^File changed in the meantime from outside$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/text/lib/Service/DocumentService.php" , line : 355usize , placeholders : & [] , regex : "^Did not reset document, as it has unsaved changes$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/text/lib/Service/DocumentService.php" , line : 384usize , placeholders : & [] , regex : "^No proper share data$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/text/lib/Service/DocumentService.php" , line : 398usize , placeholders : & [] , regex : "^Could not fallback to file from mounts$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/text/lib/Service/DocumentService.php" , line : 445usize , placeholders : & [] , regex : "^No proper share data$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/text/lib/Service/ImageService.php" , line : 152usize , placeholders : & [] , regex : "^No write permissions$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/text/lib/Service/ImageService.php" , line : 179usize , placeholders : & [] , regex : "^No write permissions$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/text/lib/Service/ImageService.php" , line : 208usize , placeholders : & [] , regex : "^No write permissions$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Warn , path : "/apps/text/lib/Service/ImageService.php" , line : 424usize , placeholders : & [] , regex : "^Impossible to download image$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Error , path : "/apps/text/lib/Service/ImageService.php" , line : 427usize , placeholders : & [] , regex : "^Connection error$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Error , path : "/apps/text/lib/Service/ImageService.php" , line : 430usize , placeholders : & [] , regex : "^Unknown download error$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/text/lib/Service/SessionService.php" , line : 221usize , placeholders : & [] , regex : "^Logged in users cannot set a guest name$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/text/lib/Db/DocumentMapper.php" , line : 53usize , placeholders : & [] , regex : "^Document doesn't exist$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/text/lib/Db/SessionMapper.php" , line : 58usize , placeholders : & [] , regex : "^Session is invalid$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/text/lib/Db/Step.php" , line : 45usize , placeholders : & [] , regex : "^Failed to parse step data$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Debug , path : "/apps/text/lib/Cron/Cleanup.php" , line : 66usize , placeholders : & [] , regex : "^Run cleanup job for text sessions$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/text/lib/TextFile.php" , line : 67usize , placeholders : & [] , regex : "^File not compatible with text because it could not be encoded to UTF\\-8\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Error , path : "/apps/text/lib/Controller/WorkspaceController.php" , line : 149usize , placeholders : & [] , regex : "^Failed to get workspace file$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Error , path : "/apps/text/lib/Controller/WorkspaceController.php" , line : 192usize , placeholders : & [] , regex : "^Failed to get public workspace file$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Error , path : "/apps/text/lib/Controller/WorkspaceController.php" , line : 217usize , placeholders : & [] , regex : "^Exception when creating a new file through direct editing$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Error , path : "/apps/text/lib/Controller/ImageController.php" , line : 118usize , placeholders : & [] , regex : "^File insertion error$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/text/lib/Controller/ImageController.php" , line : 146usize , placeholders : & [] , regex : "^Could not read file$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Error , path : "/apps/text/lib/Controller/ImageController.php" , line : 163usize , placeholders : & [] , regex : "^Upload error$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Error , path : "/apps/text/lib/Controller/ImageController.php" , line : 230usize , placeholders : & [] , regex : "^getImage error$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/encryption/composer/composer/ClassLoader.php" , line : 252usize , placeholders : & [] , regex : "^A non\\-empty PSR\\-4 prefix must end with a namespace separator\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/encryption/composer/composer/ClassLoader.php" , line : 307usize , placeholders : & [] , regex : "^A non\\-empty PSR\\-4 prefix must end with a namespace separator\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Error , path : "/apps/encryption/lib/KeyManager.php" , line : 227usize , placeholders : & [] , regex : "^A private master key is available but the public key could not be found\\. This should never happen\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Error , path : "/apps/encryption/lib/KeyManager.php" , line : 230usize , placeholders : & [] , regex : "^A public master key is available but the private key could not be found\\. This should never happen\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/encryption/lib/KeyManager.php" , line : 733usize , placeholders : & [] , regex : "^Can not get secret from Nextcloud instance$" , }
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
	crate :: LoggingStatement { level : crate :: LogLevel :: Error , path : "/apps/encryption/lib/Crypto/Crypt.php" , line : 122usize , placeholders : & ["$this->user"] , regex : "^Encryption Library couldn't generate users key\\-pair for (.*)$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Error , path : "/apps/encryption/lib/Crypto/Crypt.php" , line : 185usize , placeholders : & [] , regex : "^Encryption Library, symmetrical encryption failed no content given$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/encryption/lib/Crypto/Crypt.php" , line : 319usize , placeholders : & [] , regex : "^Legacy cipher is no longer supported!$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/encryption/lib/Crypto/Crypt.php" , line : 514usize , placeholders : & [] , regex : "^Bad Signature$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Info , path : "/apps/encryption/lib/Crypto/Crypt.php" , line : 516usize , placeholders : & [] , regex : "^Signature check skipped$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/encryption/lib/Crypto/Crypt.php" , line : 596usize , placeholders : & [] , regex : "^Missing Signature$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/encryption/lib/Crypto/Crypt.php" , line : 601usize , placeholders : & [] , regex : "^Missing Signature$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/encryption/lib/Crypto/Crypt.php" , line : 686usize , placeholders : & [] , regex : "^Cannot multikey decrypt empty plain content$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/encryption/lib/Crypto/Crypt.php" , line : 706usize , placeholders : & [] , regex : "^Cannot multikeyencrypt empty plain content$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Warn , path : "/apps/encryption/lib/Crypto/Encryption.php" , line : 278usize , placeholders : & [] , regex : "^no public key found for user \"\\{uid\\}\", user will not be able to read the file$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Debug , path : "/apps/encryption/lib/Crypto/Encryption.php" , line : 421usize , placeholders : & [] , regex : "^no file key found, we assume that the file \"\\{file\\}\" is not encrypted$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/encryption/lib/Command/FixKeyLocation.php" , line : 174usize , placeholders : & [] , regex : "^Read failed$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Error , path : "/apps/encryption/lib/Hooks/UserHooks.php" , line : 269usize , placeholders : & [] , regex : "^Encryption could not update users encryption password$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Error , path : "/apps/encryption/lib/Hooks/UserHooks.php" , line : 322usize , placeholders : & [] , regex : "^Encryption Could not update users encryption password$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/admin_audit/composer/composer/ClassLoader.php" , line : 252usize , placeholders : & [] , regex : "^A non\\-empty PSR\\-4 prefix must end with a namespace separator\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/admin_audit/composer/composer/ClassLoader.php" , line : 307usize , placeholders : & [] , regex : "^A non\\-empty PSR\\-4 prefix must end with a namespace separator\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Unknown , path : "/apps/admin_audit/lib/Actions/Console.php" , line : 43usize , placeholders : & [] , regex : "^Console command executed: %s$" , }
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
	crate :: LoggingStatement { level : crate :: LogLevel :: Unknown , path : "/apps/admin_audit/lib/Actions/Auth.php" , line : 37usize , placeholders : & [] , regex : "^Login attempt: \"%s\"$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Unknown , path : "/apps/admin_audit/lib/Actions/Auth.php" , line : 48usize , placeholders : & [] , regex : "^Login successful: \"%s\"$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Unknown , path : "/apps/admin_audit/lib/Actions/Auth.php" , line : 59usize , placeholders : & [] , regex : "^Logout occurred$" , }
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
	crate :: LoggingStatement { level : crate :: LogLevel :: Unknown , path : "/apps/admin_audit/lib/Actions/Sharing.php" , line : 173usize , placeholders : & [] , regex : "^The %s \"%s\" with ID \"%s\" has been unshared \\(Share ID: %s\\)$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Unknown , path : "/apps/admin_audit/lib/Actions/Sharing.php" , line : 184usize , placeholders : & [] , regex : "^The %s \"%s\" with ID \"%s\" has been unshared from the user \"%s\" \\(Share ID: %s\\)$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Unknown , path : "/apps/admin_audit/lib/Actions/Sharing.php" , line : 196usize , placeholders : & [] , regex : "^The %s \"%s\" with ID \"%s\" has been unshared from the group \"%s\" \\(Share ID: %s\\)$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Unknown , path : "/apps/admin_audit/lib/Actions/Sharing.php" , line : 208usize , placeholders : & [] , regex : "^The %s \"%s\" with ID \"%s\" has been unshared from the room \"%s\" \\(Share ID: %s\\)$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Unknown , path : "/apps/admin_audit/lib/Actions/Sharing.php" , line : 220usize , placeholders : & [] , regex : "^The %s \"%s\" with ID \"%s\" has been unshared from the email recipient \"%s\" \\(Share ID: %s\\)$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Unknown , path : "/apps/admin_audit/lib/Actions/Sharing.php" , line : 232usize , placeholders : & [] , regex : "^The %s \"%s\" with ID \"%s\" has been unshared from the circle \"%s\" \\(Share ID: %s\\)$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Unknown , path : "/apps/admin_audit/lib/Actions/Sharing.php" , line : 244usize , placeholders : & [] , regex : "^The %s \"%s\" with ID \"%s\" has been unshared from the remote user \"%s\" \\(Share ID: %s\\)$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Unknown , path : "/apps/admin_audit/lib/Actions/Sharing.php" , line : 256usize , placeholders : & [] , regex : "^The %s \"%s\" with ID \"%s\" has been unshared from the remote group \"%s\" \\(Share ID: %s\\)$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Unknown , path : "/apps/admin_audit/lib/Actions/Sharing.php" , line : 268usize , placeholders : & [] , regex : "^The %s \"%s\" with ID \"%s\" has been unshared from the deck card \"%s\" \\(Share ID: %s\\)$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Unknown , path : "/apps/admin_audit/lib/Actions/Sharing.php" , line : 288usize , placeholders : & [] , regex : "^The permissions of the shared %s \"%s\" with ID \"%s\" have been changed to \"%s\"$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Unknown , path : "/apps/admin_audit/lib/Actions/Sharing.php" , line : 306usize , placeholders : & [] , regex : "^The password of the publicly shared %s \"%s\" with ID \"%s\" has been changed$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Unknown , path : "/apps/admin_audit/lib/Actions/Sharing.php" , line : 324usize , placeholders : & [] , regex : "^The expiration date of the publicly shared %s with ID \"%s\" has been removed$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Unknown , path : "/apps/admin_audit/lib/Actions/Sharing.php" , line : 333usize , placeholders : & [] , regex : "^The expiration date of the publicly shared %s with ID \"%s\" has been changed to \"%s\"$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Unknown , path : "/apps/admin_audit/lib/Actions/Sharing.php" , line : 351usize , placeholders : & [] , regex : "^The shared %s with the token \"%s\" by \"%s\" has been accessed\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Unknown , path : "/apps/admin_audit/lib/Actions/Versions.php" , line : 32usize , placeholders : & [] , regex : "^Version \"%s\" of \"%s\" was restored\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Unknown , path : "/apps/admin_audit/lib/Actions/Versions.php" , line : 42usize , placeholders : & [] , regex : "^Version \"%s\" was deleted\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Unknown , path : "/apps/admin_audit/lib/Actions/AppManagement.php" , line : 35usize , placeholders : & [] , regex : "^App \"%s\" enabled$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Unknown , path : "/apps/admin_audit/lib/Actions/AppManagement.php" , line : 46usize , placeholders : & [] , regex : "^App \"%1\\$s\" enabled for groups: %2\\$s$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Unknown , path : "/apps/admin_audit/lib/Actions/AppManagement.php" , line : 56usize , placeholders : & [] , regex : "^App \"%s\" disabled$" , }
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
	crate :: LoggingStatement { level : crate :: LogLevel :: Unknown , path : "/apps/admin_audit/lib/Actions/GroupManagement.php" , line : 49usize , placeholders : & [] , regex : "^User \"%s\" added to group \"%s\"$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Unknown , path : "/apps/admin_audit/lib/Actions/GroupManagement.php" , line : 67usize , placeholders : & [] , regex : "^User \"%s\" removed from group \"%s\"$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Unknown , path : "/apps/admin_audit/lib/Actions/GroupManagement.php" , line : 84usize , placeholders : & [] , regex : "^Group created: \"%s\"$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Unknown , path : "/apps/admin_audit/lib/Actions/GroupManagement.php" , line : 100usize , placeholders : & [] , regex : "^Group deleted: \"%s\"$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/accessibility/composer/composer/ClassLoader.php" , line : 252usize , placeholders : & [] , regex : "^A non\\-empty PSR\\-4 prefix must end with a namespace separator\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/accessibility/composer/composer/ClassLoader.php" , line : 307usize , placeholders : & [] , regex : "^A non\\-empty PSR\\-4 prefix must end with a namespace separator\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/files/lib/Helper.php" , line : 229usize , placeholders : & [] , regex : "^\\$tags must be an array$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/files/lib/Activity/Helper.php" , line : 54usize , placeholders : & [] , regex : "^No favorites$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/files/lib/Activity/Helper.php" , line : 56usize , placeholders : & [] , regex : "^Too many favorites$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/files/lib/Activity/Helper.php" , line : 77usize , placeholders : & [] , regex : "^No favorites$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/files/lib/Activity/Provider.php" , line : 393usize , placeholders : & [] , regex : "^Could not generate file parameter$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/files/lib/Activity/Provider.php" , line : 404usize , placeholders : & [] , regex : "^Path could not be split correctly$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/files/lib/Activity/Provider.php" , line : 487usize , placeholders : & [] , regex : "^Reached the root$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/files/lib/Service/OwnershipTransferService.php" , line : 113usize , placeholders : & [] , regex : "^The target user is not ready to accept files\\. The user has at least to have logged in once\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/files/lib/Service/OwnershipTransferService.php" , line : 147usize , placeholders : & ["$path"] , regex : "^Unknown path provided: (.*)$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/files/lib/Service/OwnershipTransferService.php" , line : 156usize , placeholders : & [] , regex : "^Destination path does not exists or is not empty$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/files/lib/Service/OwnershipTransferService.php" , line : 246usize , placeholders : & [] , regex : "^Execution terminated\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/files/lib/Service/OwnershipTransferService.php" , line : 279usize , placeholders : & [] , regex : "^Execution terminated\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/files/lib/Service/OwnershipTransferService.php" , line : 379usize , placeholders : & [] , regex : "^Could not transfer files\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Alert , path : "/apps/files/lib/BackgroundJob/TransferOwnership.php" , line : 90usize , placeholders : & [] , regex : "^Could not transfer ownership: Node not found$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Alert , path : "/apps/files/lib/BackgroundJob/TransferOwnership.php" , line : 106usize , placeholders : & ["$destinationUser"] , regex : "^Unknown destination user (.*)$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Warn , path : "/apps/files/lib/BackgroundJob/ScanFiles.php" , line : 121usize , placeholders : & ["$user"] , regex : "^User (.*) still has unscanned files after running background scan, background scan might be stopped prematurely$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Debug , path : "/apps/files/lib/BackgroundJob/DeleteOrphanedItems.php" , line : 116usize , placeholders : & ["$deletedEntries"] , regex : "^(.*) orphaned system tag relations deleted$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Debug , path : "/apps/files/lib/BackgroundJob/DeleteOrphanedItems.php" , line : 127usize , placeholders : & ["$deletedEntries"] , regex : "^(.*) orphaned user tag relations deleted$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Debug , path : "/apps/files/lib/BackgroundJob/DeleteOrphanedItems.php" , line : 138usize , placeholders : & ["$deletedEntries"] , regex : "^(.*) orphaned comments deleted$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Debug , path : "/apps/files/lib/BackgroundJob/DeleteOrphanedItems.php" , line : 149usize , placeholders : & ["$deletedEntries"] , regex : "^(.*) orphaned comment read marks deleted$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/files/lib/Collaboration/Resources/ResourceProvider.php" , line : 99usize , placeholders : & [] , regex : "^File not found$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/files/lib/Notification/Notifier.php" , line : 89usize , placeholders : & [] , regex : "^Unhandled app$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/files/lib/Notification/Notifier.php" , line : 108usize , placeholders : & [] , regex : "^Unhandled subject$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/files/lib/Notification/Notifier.php" , line : 278usize , placeholders : & [] , regex : "^Unhandled app$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/files/lib/Notification/Notifier.php" , line : 308usize , placeholders : & [] , regex : "^User not found$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/files/composer/composer/ClassLoader.php" , line : 252usize , placeholders : & [] , regex : "^A non\\-empty PSR\\-4 prefix must end with a namespace separator\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/files/composer/composer/ClassLoader.php" , line : 307usize , placeholders : & [] , regex : "^A non\\-empty PSR\\-4 prefix must end with a namespace separator\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/password_policy/lib/Generator.php" , line : 111usize , placeholders : & [] , regex : "^Could not generate a valid password$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Warn , path : "/apps/password_policy/lib/Compliance/HistoryCompliance.php" , line : 123usize , placeholders : & [] , regex : "^Received password history of \\{uid\\} had the unexpected value of \\{history\\}, resetting\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/dashboard/lib/Service/BackgroundService.php" , line : 154usize , placeholders : & [] , regex : "^Invalid image file$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/dashboard/lib/Service/BackgroundService.php" , line : 161usize , placeholders : & [] , regex : "^The given file name is invalid$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/dashboard/lib/Service/BackgroundService.php" , line : 168usize , placeholders : & [] , regex : "^The given color is invalid$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/files_external/3rdparty/icewind/streams/src/WrapperHandler.php" , line : 87usize , placeholders : & [] , regex : "^Invalid stream source$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/files_external/3rdparty/icewind/streams/src/CountWrapper.php" , line : 64usize , placeholders : & [] , regex : "^Invalid or missing callback$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/files_external/3rdparty/icewind/streams/src/Wrapper.php" , line : 40usize , placeholders : & [] , regex : "^Invalid context, source not set$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/files_external/3rdparty/icewind/streams/src/UrlCallback.php" , line : 133usize , placeholders : & [] , regex : "^stat is not supported due to php bug 50526$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/files_external/3rdparty/icewind/streams/src/IteratorDirectory.php" , line : 48usize , placeholders : & [] , regex : "^Invalid context, iterator or array not set$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/files_external/3rdparty/icewind/streams/src/IteratorDirectory.php" , line : 109usize , placeholders : & [] , regex : "^\\$source should be an Iterator or array$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/files_external/3rdparty/icewind/smb/src/ServerFactory.php" , line : 83usize , placeholders : & [] , regex : "^No valid backend available, ensure smbclient is in the path or php\\-smbclient is installed$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/files_external/3rdparty/icewind/smb/src/KerberosApacheAuth.php" , line : 104usize , placeholders : & [] , regex : "^Ensure php\\-krb5 is installed\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/files_external/3rdparty/icewind/smb/src/KerberosApacheAuth.php" , line : 109usize , placeholders : & [] , regex : "^No kerberos ticket cache environment variable \\(KRB5CCNAME\\) found\\.$" , }
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
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/files_external/3rdparty/icewind/smb/src/Wrapped/Share.php" , line : 93usize , placeholders : & [] , regex : "^Backend not available$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/files_external/3rdparty/icewind/smb/src/Wrapped/Share.php" , line : 357usize , placeholders : & [] , regex : "^Failed to wrap file output$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/files_external/3rdparty/icewind/smb/src/Wrapped/Share.php" , line : 404usize , placeholders : & [] , regex : "^php\\-libsmbclient is required for append$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/files_external/3rdparty/icewind/smb/src/Wrapped/Share.php" , line : 444usize , placeholders : & [] , regex : "^stdbuf is required for usage of the notify command$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/files_external/3rdparty/icewind/smb/src/Wrapped/RawConnection.php" , line : 60usize , placeholders : & [] , regex : "^Authentication not set before connecting$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/files_external/3rdparty/icewind/smb/src/Wrapped/Server.php" , line : 51usize , placeholders : & [] , regex : "^Backend not available$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/files_external/3rdparty/icewind/smb/src/Wrapped/Connection.php" , line : 75usize , placeholders : & [] , regex : "^Connection not valid$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/files_external/3rdparty/icewind/smb/src/Wrapped/Connection.php" , line : 104usize , placeholders : & [] , regex : "^Unknown error$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/files_external/3rdparty/icewind/smb/src/System.php" , line : 33usize , placeholders : & [] , regex : "^Cant find file descriptor path$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/files_external/3rdparty/icewind/smb/src/Native/NativeShare.php" , line : 227usize , placeholders : & [] , regex : "^Invalid target path: Filename cannot be empty$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/files_external/3rdparty/icewind/smb/src/Native/NativeShare.php" , line : 321usize , placeholders : & [] , regex : "^Invalid value for attribute$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/files_external/3rdparty/icewind/smb/src/Native/NativeShare.php" , line : 350usize , placeholders : & [] , regex : "^smbclient not found in path for notify command$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/files_external/3rdparty/icewind/smb/src/Native/NativeFileInfo.php" , line : 55usize , placeholders : & [] , regex : "^Invalid attribute response$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/files_external/3rdparty/icewind/smb/src/Native/NativeFileInfo.php" , line : 58usize , placeholders : & [] , regex : "^Invalid attribute response$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/files_external/3rdparty/icewind/smb/src/Native/NativeFileInfo.php" , line : 61usize , placeholders : & [] , regex : "^Invalid attribute response$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/files_external/3rdparty/icewind/smb/src/Native/NativeState.php" , line : 373usize , placeholders : & [] , regex : "^Failed to free smb state$" , }
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
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/files_external/3rdparty/icewind/smb/src/KerberosAuth.php" , line : 51usize , placeholders : & [] , regex : "^Failed to set smbclient options for kerberos auth$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/files_external/3rdparty/icewind/smb/src/AnonymousAuth.php" , line : 45usize , placeholders : & [] , regex : "^Failed to set smbclient options for anonymous auth$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/files_external/3rdparty/composer/ClassLoader.php" , line : 252usize , placeholders : & [] , regex : "^A non\\-empty PSR\\-4 prefix must end with a namespace separator\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/files_external/3rdparty/composer/ClassLoader.php" , line : 307usize , placeholders : & [] , regex : "^A non\\-empty PSR\\-4 prefix must end with a namespace separator\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/files_external/lib/Service/BackendService.php" , line : 331usize , placeholders : & [] , regex : "^Invalid empty placeholder$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/files_external/lib/Service/UserGlobalStoragesService.php" , line : 90usize , placeholders : & [] , regex : "^UserGlobalStoragesService writing disallowed$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/files_external/lib/Service/UserGlobalStoragesService.php" , line : 94usize , placeholders : & [] , regex : "^UserGlobalStoragesService writing disallowed$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/files_external/lib/Service/UserGlobalStoragesService.php" , line : 101usize , placeholders : & [] , regex : "^UserGlobalStoragesService writing disallowed$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/files_external/lib/Command/ListCommand.php" , line : 269usize , placeholders : & ["$userId"] , regex : "^user (.*) not found$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/files_external/lib/Command/Import.php" , line : 218usize , placeholders : & ["$userId"] , regex : "^user (.*) not found$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/files_external/lib/Command/Create.php" , line : 216usize , placeholders : & ["$userId"] , regex : "^user (.*) not found$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/files_external/lib/Migration/Version1015Date20211104103506.php" , line : 58usize , placeholders : & [] , regex : "^Could not fetch existing mounts for migration$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/files_external/lib/Config/ConfigAdapter.php" , line : 83usize , placeholders : & [] , regex : "^Invalid object store$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/files_external/lib/Config/SimpleSubstitutionTrait.php" , line : 72usize , placeholders : & [] , regex : "^Invalid empty placeholder$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/files_external/lib/Lib/Backend/SMB.php" , line : 79usize , placeholders : & [] , regex : "^user or password is not set$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/files_external/lib/Lib/Backend/SMB.php" , line : 94usize , placeholders : & [] , regex : "^invalid authentication backend$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/files_external/lib/Lib/Backend/SMB.php" , line : 124usize , placeholders : & [] , regex : "^No session credentials saved$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/files_external/lib/Lib/Backend/SMB.php" , line : 130usize , placeholders : & [] , regex : "^unknown authentication backend$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/files_external/lib/Lib/Auth/Password/GlobalAuth.php" , line : 77usize , placeholders : & [] , regex : "^No credentials saved$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/files_external/lib/Lib/Auth/Password/SessionCredentials.php" , line : 57usize , placeholders : & [] , regex : "^No session credentials saved$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/files_external/lib/Lib/Auth/Password/UserProvided.php" , line : 76usize , placeholders : & [] , regex : "^No credentials saved$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/files_external/lib/Lib/Auth/Password/UserProvided.php" , line : 82usize , placeholders : & [] , regex : "^No credentials saved$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/files_external/lib/Lib/Auth/Password/UserGlobalAuth.php" , line : 74usize , placeholders : & [] , regex : "^No credentials saved$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/files_external/lib/Lib/Auth/Password/LoginCredentials.php" , line : 108usize , placeholders : & [] , regex : "^No login credentials saved$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/files_external/lib/Lib/Auth/Password/LoginCredentials.php" , line : 117usize , placeholders : & [] , regex : "^No login credentials saved$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/files_external/lib/Lib/Auth/PublicKey/RSA.php" , line : 63usize , placeholders : & [] , regex : "^unable to load private key$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/files_external/lib/Lib/Auth/PublicKey/RSAPrivateKey.php" , line : 61usize , placeholders : & [] , regex : "^unable to load private key$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/files_external/lib/Lib/Storage/SMB.php" , line : 103usize , placeholders : & [] , regex : "^Invalid configuration, no host provided$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/files_external/lib/Lib/Storage/SMB.php" , line : 112usize , placeholders : & [] , regex : "^Invalid configuration, no credentials provided$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/files_external/lib/Lib/Storage/SMB.php" , line : 517usize , placeholders : & [] , regex : "^not enough available space to create file$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/files_external/lib/Lib/Storage/SMB.php" , line : 560usize , placeholders : & [] , regex : "^not enough available space to create file$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/files_external/lib/Lib/Storage/FTP.php" , line : 84usize , placeholders : & [] , regex : "^Failed to create ftp connection$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/files_external/lib/Lib/Storage/FTP.php" , line : 88usize , placeholders : & [] , regex : "^Could not set UTF\\-8 mode$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Warn , path : "/apps/files_external/lib/Lib/Storage/FTP.php" , line : 119usize , placeholders : & ["$path"] , regex : "^Unable to get last modified date for ftp folder \\((.*)\\), failed to list folder contents$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/files_external/lib/Lib/Storage/FTP.php" , line : 128usize , placeholders : & ["$currentDir"] , regex : "^Invalid date format for directory: (.*)$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Warn , path : "/apps/files_external/lib/Lib/Storage/FTP.php" , line : 132usize , placeholders : & ["$path"] , regex : "^Unable to get last modified date for ftp folder \\((.*)\\), folder contents doesn't include current folder$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/files_external/lib/Lib/Storage/SFTP.php" , line : 94usize , placeholders : & [] , regex : "^no authentication parameters specified$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/files_external/lib/Lib/Storage/SFTP.php" , line : 106usize , placeholders : & [] , regex : "^no authentication parameters specified$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/files_external/lib/Lib/Storage/SFTP.php" , line : 134usize , placeholders : & [] , regex : "^Host public key does not match known key$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/files_external/lib/Lib/Storage/SFTP.php" , line : 151usize , placeholders : & [] , regex : "^Login failed$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/files_external/lib/Lib/Storage/Swift.php" , line : 169usize , placeholders : & [] , regex : "^API Key or password, Username, Bucket and Region have to be configured\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/files_external/lib/Lib/Storage/Swift.php" , line : 560usize , placeholders : & [] , regex : "^failed to remove original$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/files_external/lib/Lib/Storage/SFTPReadStream.php" , line : 77usize , placeholders : & [] , regex : "^Invalid context, session not set$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/files_external/lib/Lib/Storage/SFTPWriteStream.php" , line : 77usize , placeholders : & [] , regex : "^Invalid context, session not set$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/files_external/lib/Lib/Storage/FtpConnection.php" , line : 41usize , placeholders : & [] , regex : "^Failed to connect to ftp$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/files_external/lib/Lib/Storage/FtpConnection.php" , line : 45usize , placeholders : & [] , regex : "^Failed to connect to login to ftp$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/files_external/lib/Lib/Storage/FtpConnection.php" , line : 151usize , placeholders : & ["$item"] , regex : "^Metadata can't be parsed from item '(.*)' , not enough parts\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/files_external/lib/Lib/Storage/FtpConnection.php" , line : 209usize , placeholders : & ["$item"] , regex : "^Metadata can't be parsed from item '(.*)' , not enough parts\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/user_status/lib/Service/StatusService.php" , line : 243usize , placeholders : & [] , regex : "^ClearAt is in the past$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/user_status/lib/Service/StatusService.php" , line : 338usize , placeholders : & [] , regex : "^Status\\-Icon is longer than one character$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/user_status/lib/Service/StatusService.php" , line : 346usize , placeholders : & [] , regex : "^ClearAt is in the past$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/user_status/lib/Controller/UserStatusController.php" , line : 86usize , placeholders : & [] , regex : "^No status for the current user$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/user_status/lib/Controller/StatusesController.php" , line : 83usize , placeholders : & [] , regex : "^No status for the requested userId$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/user_status/composer/composer/ClassLoader.php" , line : 252usize , placeholders : & [] , regex : "^A non\\-empty PSR\\-4 prefix must end with a namespace separator\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/user_status/composer/composer/ClassLoader.php" , line : 307usize , placeholders : & [] , regex : "^A non\\-empty PSR\\-4 prefix must end with a namespace separator\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/dav/lib/Controller/DirectController.php" , line : 107usize , placeholders : & [] , regex : "^Direct download only works for files$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/dav/lib/Controller/DirectController.php" , line : 114usize , placeholders : & [] , regex : "^Permission denied to download file$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/dav/lib/Direct/DirectHome.php" , line : 103usize , placeholders : & [] , regex : "^Listing members of this collection is disabled$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Info , path : "/apps/dav/lib/Migration/BuildCalendarSearchIndexBackgroundJob.php" , line : 84usize , placeholders : & [] , regex : "^Building calendar index done$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Info , path : "/apps/dav/lib/Migration/FixBirthdayCalendarComponent.php" , line : 60usize , placeholders : & ["$updated"] , regex : "^(.*) birthday calendars updated\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Info , path : "/apps/dav/lib/Migration/RefreshWebcalJobRegistrar.php" , line : 83usize , placeholders : & ["$count"] , regex : "^Added (.*) background jobs to update webcal calendars$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Info , path : "/apps/dav/lib/Migration/RegenerateBirthdayCalendars.php" , line : 63usize , placeholders : & [] , regex : "^Repair step already executed$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Info , path : "/apps/dav/lib/Migration/RegenerateBirthdayCalendars.php" , line : 67usize , placeholders : & [] , regex : "^Adding background jobs to regenerate birthday calendar$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Info , path : "/apps/dav/lib/Migration/BuildCalendarSearchIndex.php" , line : 72usize , placeholders : & [] , regex : "^Repair step already executed$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Info , path : "/apps/dav/lib/Migration/BuildCalendarSearchIndex.php" , line : 83usize , placeholders : & [] , regex : "^Add background job$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Info , path : "/apps/dav/lib/Migration/BuildSocialSearchIndex.php" , line : 69usize , placeholders : & [] , regex : "^Repair step already executed$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Info , path : "/apps/dav/lib/Migration/BuildSocialSearchIndex.php" , line : 83usize , placeholders : & [] , regex : "^Add background job$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Info , path : "/apps/dav/lib/Migration/ChunkCleanup.php" , line : 67usize , placeholders : & [] , regex : "^Cleanup not required$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Info , path : "/apps/dav/lib/Migration/RemoveClassifiedEventActivity.php" , line : 61usize , placeholders : & ["$deletedEvents"] , regex : "^Removed (.*) activity entries$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Info , path : "/apps/dav/lib/Migration/BuildSocialSearchIndexBackgroundJob.php" , line : 77usize , placeholders : & [] , regex : "^All contacts with social profiles indexed$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Info , path : "/apps/dav/lib/Migration/CalDAVRemoveEmptyValue.php" , line : 79usize , placeholders : & [] , regex : "^Calendar object for calendar \\{cal\\} with uri \\{uri\\} still invalid$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Info , path : "/apps/dav/lib/Migration/RegisterBuildReminderIndexBackgroundJob.php" , line : 83usize , placeholders : & [] , regex : "^Repair step already executed$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Info , path : "/apps/dav/lib/Migration/RegisterBuildReminderIndexBackgroundJob.php" , line : 94usize , placeholders : & [] , regex : "^Add background job$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/dav/lib/Connector/Sabre/ObjectTree.php" , line : 111usize , placeholders : & [] , regex : "^filesystem not setup$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/dav/lib/Connector/Sabre/ObjectTree.php" , line : 161usize , placeholders : & [] , regex : "^Storage is temporarily not available$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/dav/lib/Connector/Sabre/ObjectTree.php" , line : 205usize , placeholders : & [] , regex : "^filesystem not setup$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/dav/lib/Connector/Sabre/ObjectTree.php" , line : 216usize , placeholders : & [] , regex : "^No permissions to copy object\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/dav/lib/Connector/Sabre/Directory.php" , line : 271usize , placeholders : & [] , regex : "^No read permissions$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Warn , path : "/apps/dav/lib/Connector/Sabre/Directory.php" , line : 353usize , placeholders : & [] , regex : "^error while getting quota into$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Warn , path : "/apps/dav/lib/Connector/Sabre/Directory.php" , line : 356usize , placeholders : & [] , regex : "^error while getting quota into$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Warn , path : "/apps/dav/lib/Connector/Sabre/Directory.php" , line : 359usize , placeholders : & [] , regex : "^error while getting quota into$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/dav/lib/Connector/Sabre/Directory.php" , line : 396usize , placeholders : & [] , regex : "^Incompatible node types$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/dav/lib/Connector/Sabre/Directory.php" , line : 400usize , placeholders : & [] , regex : "^filesystem not setup$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/dav/lib/Connector/Sabre/Directory.php" , line : 457usize , placeholders : & [] , regex : "^$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/dav/lib/Connector/Sabre/Auth.php" , line : 161usize , placeholders : & ["$class" , "$msg"] , regex : "^(.*): (.*)$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/dav/lib/Connector/Sabre/Auth.php" , line : 220usize , placeholders : & [] , regex : "^CSRF check not passed\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/dav/lib/Connector/Sabre/Auth.php" , line : 228usize , placeholders : & [] , regex : "^2FA challenge not passed\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/dav/lib/Connector/Sabre/Auth.php" , line : 248usize , placeholders : & [] , regex : "^Cannot authenticate over ajax calls$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/dav/lib/Connector/Sabre/FilesReportPlugin.php" , line : 207usize , placeholders : & [] , regex : "^Missing filter\\-rule block in request$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/dav/lib/Connector/Sabre/FilesReportPlugin.php" , line : 214usize , placeholders : & [] , regex : "^Cannot filter by non\\-existing tag$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Debug , path : "/apps/dav/lib/Connector/Sabre/FilesPlugin.php" , line : 466usize , placeholders : & [] , regex : "^Inefficient fetching of metadata$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/dav/lib/Connector/Sabre/MtimeSanitizer.php" , line : 32usize , placeholders : & [] , regex : "^X\\-OC\\-MTime header must be an integer \\(unix timestamp\\)\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/dav/lib/Connector/Sabre/MtimeSanitizer.php" , line : 37usize , placeholders : & [] , regex : "^X\\-OC\\-MTime header must be a valid positive integer$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/dav/lib/Connector/Sabre/QuotaPlugin.php" , line : 210usize , placeholders : & ["$path" , "$length" , "$freeSpace"] , regex : "^Insufficient space in (.*), (.*) required, (.*) available$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Error , path : "/apps/dav/lib/Connector/Sabre/File.php" , line : 265usize , placeholders : & [] , regex : "^\\\\OC\\\\Files\\\\Filesystem::fopen\\(\\) failed$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Error , path : "/apps/dav/lib/Connector/Sabre/File.php" , line : 640usize , placeholders : & [] , regex : "^\\\\OC\\\\Files\\\\Filesystem::rename\\(\\) failed$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/dav/lib/Connector/Sabre/BlockLegacyClientPlugin.php" , line : 79usize , placeholders : & [] , regex : "^Unsupported client version\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/dav/lib/Connector/Sabre/Principal.php" , line : 224usize , placeholders : & [] , regex : "^Principal not found$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/dav/lib/Connector/Sabre/Principal.php" , line : 579usize , placeholders : & [] , regex : "^Principal not found$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/dav/lib/Connector/PublicAuth.php" , line : 124usize , placeholders : & [] , regex : "^Cannot authenticate over ajax calls$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/dav/lib/Comments/CommentNode.php" , line : 138usize , placeholders : & [] , regex : "^Only authors are allowed to edit their comment\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/dav/lib/Comments/CommentsPlugin.php" , line : 247usize , placeholders : & [] , regex : "^Invalid input values$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/dav/lib/Comments/RootCollection.php" , line : 122usize , placeholders : & [] , regex : "^Cannot create comments by id$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/dav/lib/Comments/RootCollection.php" , line : 132usize , placeholders : & [] , regex : "^Permission denied to create collections$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/dav/lib/Comments/RootCollection.php" , line : 180usize , placeholders : & [] , regex : "^Permission denied to delete this collection$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/dav/lib/Comments/RootCollection.php" , line : 201usize , placeholders : & [] , regex : "^Permission denied to rename this collection$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/dav/lib/Comments/EntityTypeCollection.php" , line : 73usize , placeholders : & [] , regex : "^\"name\" parameter must be non\\-empty string$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/dav/lib/Comments/EntityTypeCollection.php" , line : 95usize , placeholders : & [] , regex : "^Entity does not exist or is not available$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/dav/lib/Comments/EntityTypeCollection.php" , line : 114usize , placeholders : & [] , regex : "^No permission to list folder contents$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/dav/lib/DAV/ViewOnlyPlugin.php" , line : 100usize , placeholders : & [] , regex : "^Access to this resource has been denied because it is in view\\-only mode\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/dav/lib/DAV/GroupPrincipalBackend.php" , line : 172usize , placeholders : & [] , regex : "^Setting members of the group is not supported yet$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/dav/lib/DAV/SystemPrincipalBackend.php" , line : 151usize , placeholders : & [] , regex : "^Principal not found$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/dav/lib/DAV/SystemPrincipalBackend.php" , line : 169usize , placeholders : & [] , regex : "^Principal not found$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/dav/lib/DAV/SystemPrincipalBackend.php" , line : 187usize , placeholders : & [] , regex : "^Setting members of the group is not supported yet$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/dav/lib/Files/Sharing/FilesDropPlugin.php" , line : 75usize , placeholders : & [] , regex : "^Only PUT is allowed on files drop$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/dav/lib/Files/FilesHome.php" , line : 52usize , placeholders : & [] , regex : "^Permission denied to delete home folder$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/dav/lib/Files/FilesHome.php" , line : 61usize , placeholders : & [] , regex : "^Permission denied to rename this folder$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/dav/lib/Files/FileSearchBackend.php" , line : 174usize , placeholders : & [] , regex : "^Searching more than one folder is not supported$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/dav/lib/Files/FileSearchBackend.php" , line : 179usize , placeholders : & [] , regex : "^Using uri's as scope is not supported, please use a path relative to the search arbiter instead$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/dav/lib/Files/FileSearchBackend.php" , line : 183usize , placeholders : & [] , regex : "^Search is only supported on directories$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/dav/lib/Files/FileSearchBackend.php" , line : 314usize , placeholders : & [] , regex : "^Invalid search value for '\\{http://owncloud\\.org/ns\\}owner\\-id', only the current user id is allowed$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/dav/lib/Files/FileSearchBackend.php" , line : 447usize , placeholders : & ["$propertyName"] , regex : "^searching by '(.*)' is only allowed with a literal value$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/dav/lib/Files/FileSearchBackend.php" , line : 450usize , placeholders : & ["$propertyName"] , regex : "^searching by '(.*)' is not allowed inside a '\\{DAV:\\}or' or '\\{DAV:\\}not'$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/dav/lib/Files/FileSearchBackend.php" , line : 453usize , placeholders : & ["$propertyName" , "$comparison"] , regex : "^searching by '(.*)' is only allowed inside a '(.*)'$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/dav/lib/Files/RootCollection.php" , line : 56usize , placeholders : & [] , regex : "^Home does not exist$" , }
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
	crate :: LoggingStatement { level : crate :: LogLevel :: Debug , path : "/apps/dav/lib/Listener/CalendarDeletionDefaultUpdaterListener.php" , line : 65usize , placeholders : & [] , regex : "^Default calendar needs no update because the deleted calendar does not belong to a user principal$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Debug , path : "/apps/dav/lib/Listener/CalendarDeletionDefaultUpdaterListener.php" , line : 72usize , placeholders : & [] , regex : "^Default calendar needs no update because the deleted calendar is no the user's default one$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Debug , path : "/apps/dav/lib/Listener/CalendarDeletionDefaultUpdaterListener.php" , line : 78usize , placeholders : & [] , regex : "^Default user calendar reset$" , }
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
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/dav/lib/Upload/FutureFile.php" , line : 58usize , placeholders : & [] , regex : "^Permission denied to put into this file$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/dav/lib/Upload/FutureFile.php" , line : 114usize , placeholders : & [] , regex : "^Permission denied to rename this file$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/dav/lib/Upload/UploadHome.php" , line : 81usize , placeholders : & [] , regex : "^Permission denied to rename this folder$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/dav/lib/Upload/ChunkingPlugin.php" , line : 69usize , placeholders : & ["$destination"] , regex : "^The given destination (.*) is a directory\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/dav/lib/Upload/ChunkingPlugin.php" , line : 128usize , placeholders : & ["$expectedSize" , "$actualSize"] , regex : "^Chunks on server do not sum up to (.*) but to (.*) bytes$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/dav/lib/Upload/UploadFolder.php" , line : 91usize , placeholders : & [] , regex : "^Permission denied to rename this folder$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/dav/lib/Upload/AssemblyStream.php" , line : 261usize , placeholders : & [] , regex : "^Invalid context, nodes not set$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/dav/lib/SystemTag/SystemTagsRelationsCollection.php" , line : 92usize , placeholders : & [] , regex : "^Permission denied to rename this collection$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/dav/lib/SystemTag/SystemTagsObjectTypeCollection.php" , line : 105usize , placeholders : & [] , regex : "^Permission denied to create nodes$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/dav/lib/SystemTag/SystemTagsObjectTypeCollection.php" , line : 113usize , placeholders : & [] , regex : "^Permission denied to create collections$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/dav/lib/SystemTag/SystemTagsObjectTypeCollection.php" , line : 125usize , placeholders : & [] , regex : "^Entity does not exist or is not available$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/dav/lib/SystemTag/SystemTagsObjectTypeCollection.php" , line : 152usize , placeholders : & [] , regex : "^Permission denied to delete this collection$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/dav/lib/SystemTag/SystemTagsObjectTypeCollection.php" , line : 164usize , placeholders : & [] , regex : "^Permission denied to rename this collection$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/dav/lib/SystemTag/SystemTagsByIdCollection.php" , line : 90usize , placeholders : & [] , regex : "^Cannot create tags by id$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/dav/lib/SystemTag/SystemTagsByIdCollection.php" , line : 97usize , placeholders : & [] , regex : "^Permission denied to create collections$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/dav/lib/SystemTag/SystemTagsByIdCollection.php" , line : 112usize , placeholders : & [] , regex : "^Invalid tag id$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/dav/lib/SystemTag/SystemTagsByIdCollection.php" , line : 142usize , placeholders : & [] , regex : "^Invalid tag id$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/dav/lib/SystemTag/SystemTagsByIdCollection.php" , line : 149usize , placeholders : & [] , regex : "^Permission denied to delete this collection$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/dav/lib/SystemTag/SystemTagsByIdCollection.php" , line : 157usize , placeholders : & [] , regex : "^Permission denied to rename this collection$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/dav/lib/SystemTag/SystemTagPlugin.php" , line : 172usize , placeholders : & [] , regex : "^Missing \"name\" attribute$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/dav/lib/SystemTag/SystemTagPlugin.php" , line : 197usize , placeholders : & [] , regex : "^Not sufficient permissions$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/dav/lib/SystemTag/SystemTagPlugin.php" , line : 208usize , placeholders : & [] , regex : "^Tag already exists$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/dav/lib/SystemTag/SystemTagsObjectMappingCollection.php" , line : 114usize , placeholders : & [] , regex : "^Permission denied to create collections$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/dav/lib/SystemTag/SystemTagsObjectMappingCollection.php" , line : 128usize , placeholders : & [] , regex : "^Invalid tag id$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/dav/lib/SystemTag/SystemTagsObjectMappingCollection.php" , line : 165usize , placeholders : & [] , regex : "^Invalid tag id$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/dav/lib/SystemTag/SystemTagsObjectMappingCollection.php" , line : 172usize , placeholders : & [] , regex : "^Permission denied to delete this collection$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/dav/lib/SystemTag/SystemTagsObjectMappingCollection.php" , line : 180usize , placeholders : & [] , regex : "^Permission denied to rename this collection$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/dav/lib/CardDAV/CardDavBackend.php" , line : 420usize , placeholders : & [] , regex : "^URI too long\\. Address book not created$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/dav/lib/CardDAV/CardDavBackend.php" , line : 674usize , placeholders : & [] , regex : "^VCard object with uid already exists in this addressbook collection\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/dav/lib/CardDAV/CardDavBackend.php" , line : 1396usize , placeholders : & [] , regex : "^vCards on CardDAV servers MUST have a UID property$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/dav/lib/CardDAV/CardDavBackend.php" , line : 1399usize , placeholders : & [] , regex : "^vCard can not be empty$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/dav/lib/CardDAV/Integration/ExternalAddressBook.php" , line : 83usize , placeholders : & [] , regex : "^Renaming address books is not yet supported$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/dav/lib/CardDAV/Integration/ExternalAddressBook.php" , line : 90usize , placeholders : & [] , regex : "^Creating collections in address book objects is not allowed$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/dav/lib/CardDAV/Integration/ExternalAddressBook.php" , line : 116usize , placeholders : & [] , regex : "^Provided address book uri was not app\\-generated$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/dav/lib/CardDAV/PhotoCache.php" , line : 207usize , placeholders : & [] , regex : "^Avatar not found$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/dav/lib/CardDAV/AddressBook.php" , line : 160usize , placeholders : & [] , regex : "^Card not found$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/dav/lib/CardDAV/UserAddressBooks.php" , line : 91usize , placeholders : & [] , regex : "^The resource you tried to create has a reserved name$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/dav/lib/UserMigration/CalendarMigrator.php" , line : 193usize , placeholders : & [] , regex : "^Failed to get unique calendar URI$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/dav/lib/UserMigration/CalendarMigrator.php" , line : 261usize , placeholders : & [] , regex : "^Could not export calendars$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/dav/lib/UserMigration/CalendarMigrator.php" , line : 460usize , placeholders : & ["$importPath"] , regex : "^Failed to read file \"(.*)\"$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/dav/lib/UserMigration/CalendarMigrator.php" , line : 465usize , placeholders : & ["$importPath"] , regex : "^Invalid calendar data contained in \"(.*)\"$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/dav/lib/UserMigration/ContactsMigrator.php" , line : 173usize , placeholders : & [] , regex : "^Failed to get unique address book URI$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/dav/lib/UserMigration/ContactsMigrator.php" , line : 254usize , placeholders : & [] , regex : "^Could not export address book$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/dav/lib/Command/CreateAddressBook.php" , line : 69usize , placeholders : & ["$user"] , regex : "^User <(.*)> in unknown\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/dav/lib/Command/MoveCalendar.php" , line : 122usize , placeholders : & ["$userOrigin"] , regex : "^User <(.*)> is unknown\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/dav/lib/Command/MoveCalendar.php" , line : 126usize , placeholders : & ["$userDestination"] , regex : "^User <(.*)> is unknown\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/dav/lib/Command/MoveCalendar.php" , line : 135usize , placeholders : & ["$userOrigin" , "$name"] , regex : "^User <(.*)> has no calendar named <(.*)>\\. You can run occ dav:list\\-calendars to list calendars URIs for this user\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/dav/lib/Command/MoveCalendar.php" , line : 146usize , placeholders : & ["$userDestination" , "$name"] , regex : "^Unable to find a suitable calendar name for <(.*)> with initial name <(.*)>\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/dav/lib/Command/MoveCalendar.php" , line : 149usize , placeholders : & ["$userDestination" , "$name"] , regex : "^User <(.*)> already has a calendar named <(.*)>\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/dav/lib/Command/CreateCalendar.php" , line : 80usize , placeholders : & ["$user"] , regex : "^User <(.*)> in unknown\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/dav/lib/Command/DeleteCalendar.php" , line : 116usize , placeholders : & [] , regex : "^Please specify a calendar name or \\-\\-birthday$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/dav/lib/Command/ListCalendars.php" , line : 67usize , placeholders : & ["$user"] , regex : "^User <(.*)> is unknown\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/dav/lib/Command/SyncBirthdayCalendar.php" , line : 80usize , placeholders : & ["$user"] , regex : "^User <(.*)> in unknown\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/dav/lib/Command/SyncBirthdayCalendar.php" , line : 119usize , placeholders : & [] , regex : "^Birthday calendars are disabled$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Info , path : "/apps/dav/lib/BackgroundJob/BuildReminderIndexBackgroundJob.php" , line : 93usize , placeholders : & [] , regex : "^Building calendar reminder index done$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Warn , path : "/apps/dav/lib/BackgroundJob/RefreshWebcalJob.php" , line : 99usize , placeholders : & ["$subscriptionId"] , regex : "^Subscription (.*) could not be refreshed, refreshrate in database is invalid$" , }
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
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/dav/lib/BulkUpload/MultipartRequestParser.php" , line : 217usize , placeholders : & [] , regex : "^Fail to read part's content\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/dav/lib/BulkUpload/MultipartRequestParser.php" , line : 221usize , placeholders : & [] , regex : "^Unexpected EOF while reading stream\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/dav/lib/CalDAV/Trashbin/DeletedCalendarObjectsCollection.php" , line : 75usize , placeholders : & [] , regex : "^The calendar object you're trying to restore is not marked as deleted$" , }
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
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/dav/lib/CalDAV/CachedSubscriptionObject.php" , line : 57usize , placeholders : & [] , regex : "^Creating objects in a cached subscription is not allowed$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/dav/lib/CalDAV/CachedSubscriptionObject.php" , line : 64usize , placeholders : & [] , regex : "^Deleting objects in a cached subscription is not allowed$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/dav/lib/CalDAV/CalendarImpl.php" , line : 157usize , placeholders : & [] , regex : "^Could not write to calendar as URI parameter is missing$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/dav/lib/CalDAV/PublicCalendar.php" , line : 40usize , placeholders : & [] , regex : "^Calendar object not found$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/dav/lib/CalDAV/PublicCalendar.php" , line : 43usize , placeholders : & [] , regex : "^Calendar object not found$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/dav/lib/CalDAV/Integration/ExternalCalendar.php" , line : 84usize , placeholders : & [] , regex : "^Renaming calendars is not yet supported$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/dav/lib/CalDAV/Integration/ExternalCalendar.php" , line : 91usize , placeholders : & [] , regex : "^Creating collections in calendar objects is not allowed$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/dav/lib/CalDAV/Integration/ExternalCalendar.php" , line : 115usize , placeholders : & [] , regex : "^Provided calendar uri was not app\\-generated$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Error , path : "/apps/dav/lib/CalDAV/Schedule/IMipPlugin.php" , line : 305usize , placeholders : & [] , regex : "^Unable to deliver message to \\{failed\\}$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/dav/lib/CalDAV/CalDavBackend.php" , line : 818usize , placeholders : & [] , regex : "^URI too long\\. Calendar not created$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/dav/lib/CalDAV/CalDavBackend.php" , line : 995usize , placeholders : & [] , regex : "^Calendar data that was just written can't be read back\\. Check your database configuration\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/dav/lib/CalDAV/CalDavBackend.php" , line : 1274usize , placeholders : & [] , regex : "^Calendar object with uid already exists in this calendar collection\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/dav/lib/CalDAV/CalDavBackend.php" , line : 1288usize , placeholders : & [] , regex : "^Deleted calendar object with uid already exists in this calendar collection\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/dav/lib/CalDAV/CalDavBackend.php" , line : 1533usize , placeholders : & ["$newUri" , "$calendarId"] , regex : "^A calendar object with URI (.*) already exists in calendar (.*), therefore this object can't be moved into the trashbin$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/dav/lib/CalDAV/CalDavBackend.php" , line : 1576usize , placeholders : & ["$id" , "$restoreUri"] , regex : "^Can not restore calendar (.*) because a calendar object with the URI (.*) already exists$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/dav/lib/CalDAV/CalDavBackend.php" , line : 1603usize , placeholders : & [] , regex : "^Calendar object data that was just written can't be read back\\. Check your database configuration\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/dav/lib/CalDAV/CalDavBackend.php" , line : 2485usize , placeholders : & [] , regex : "^The \\{http://calendarserver\\.org/ns/\\}source property is required when creating subscriptions$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/dav/lib/CalDAV/CalDavBackend.php" , line : 2817usize , placeholders : & [] , regex : "^Calendar objects must have a VJOURNAL, VEVENT or VTODO component$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/dav/lib/CalDAV/CachedSubscription.php" , line : 134usize , placeholders : & [] , regex : "^Calendar object not found$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/dav/lib/CalDAV/CachedSubscription.php" , line : 177usize , placeholders : & [] , regex : "^Creating objects in cached subscription is not allowed$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/dav/lib/CalDAV/Reminder/Notifier.php" , line : 107usize , placeholders : & [] , regex : "^Notification not from this app$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/dav/lib/CalDAV/Reminder/Notifier.php" , line : 119usize , placeholders : & [] , regex : "^Unknown subject$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/dav/lib/CalDAV/Reminder/ReminderService.php" , line : 794usize , placeholders : & [] , regex : "^Multiple master objects$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/dav/lib/CalDAV/Reminder/NotificationProviderManager.php" , line : 79usize , placeholders : & [] , regex : "^Invalid notification provider registered$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Error , path : "/apps/dav/lib/CalDAV/Reminder/NotificationProvider/EmailProvider.php" , line : 130usize , placeholders : & [] , regex : "^Email address \\{address\\} for reminder notification is incorrect$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Error , path : "/apps/dav/lib/CalDAV/Reminder/NotificationProvider/EmailProvider.php" , line : 145usize , placeholders : & [] , regex : "^Unable to deliver message to \\{failed\\}$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/dav/lib/CalDAV/CalendarHome.php" , line : 90usize , placeholders : & [] , regex : "^The resource you tried to create has a reserved name$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Debug , path : "/apps/dav/lib/CalDAV/ICSExportPlugin/ICSExportPlugin.php" , line : 81usize , placeholders : & [] , regex : "^Invalid refresh interval for exported calendar, falling back to default value \\.\\.\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Warn , path : "/apps/dav/lib/CalDAV/WebcalCaching/RefreshWebcalService.php" , line : 160usize , placeholders : & ["$subscriptionId"] , regex : "^Subscription (.*) could not be refreshed due to a parsing error$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Debug , path : "/apps/dav/lib/CalDAV/WebcalCaching/RefreshWebcalService.php" , line : 255usize , placeholders : & ["$subscriptionId"] , regex : "^Subscription (.*) could not be parsed$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Debug , path : "/apps/dav/lib/CalDAV/WebcalCaching/RefreshWebcalService.php" , line : 265usize , placeholders : & ["$subscriptionId"] , regex : "^Subscription (.*) could not be parsed$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Debug , path : "/apps/dav/lib/CalDAV/WebcalCaching/RefreshWebcalService.php" , line : 276usize , placeholders : & ["$subscriptionId"] , regex : "^Subscription (.*) could not be parsed$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/dav/lib/CalDAV/Calendar.php" , line : 310usize , placeholders : & [] , regex : "^Calendar object not found$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/dav/lib/CalDAV/Calendar.php" , line : 314usize , placeholders : & [] , regex : "^Calendar object not found$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/dav/composer/composer/ClassLoader.php" , line : 252usize , placeholders : & [] , regex : "^A non\\-empty PSR\\-4 prefix must end with a namespace separator\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/dav/composer/composer/ClassLoader.php" , line : 307usize , placeholders : & [] , regex : "^A non\\-empty PSR\\-4 prefix must end with a namespace separator\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/lookup_server_connector/composer/composer/ClassLoader.php" , line : 252usize , placeholders : & [] , regex : "^A non\\-empty PSR\\-4 prefix must end with a namespace separator\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/lookup_server_connector/composer/composer/ClassLoader.php" , line : 307usize , placeholders : & [] , regex : "^A non\\-empty PSR\\-4 prefix must end with a namespace separator\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/files_versions/composer/composer/ClassLoader.php" , line : 252usize , placeholders : & [] , regex : "^A non\\-empty PSR\\-4 prefix must end with a namespace separator\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/files_versions/composer/composer/ClassLoader.php" , line : 307usize , placeholders : & [] , regex : "^A non\\-empty PSR\\-4 prefix must end with a namespace separator\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Error , path : "/apps/files_versions/lib/Storage.php" , line : 498usize , placeholders : & [] , regex : "^Version file \\{path\\} has incorrect name format$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/files_versions/lib/Versions/VersionManager.php" , line : 79usize , placeholders : & ["$fullType"] , regex : "^Version backend for (.*) not found$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/theming/lib/ImageManager.php" , line : 231usize , placeholders : & [] , regex : "^Unsupported image type$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/logreader/lib/Log/LogIteratorFactory.php" , line : 63usize , placeholders : & [] , regex : "^Can't find log class$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Info , path : "/apps/circles/lib/Migration/ImportOwncloudCustomGroups.php" , line : 88usize , placeholders : & [] , regex : "^Creating circles$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Info , path : "/apps/circles/lib/Migration/ImportOwncloudCustomGroups.php" , line : 125usize , placeholders : & [] , regex : "^Creating memberships$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Info , path : "/apps/circles/lib/Migration/ImportOwncloudCustomGroups.php" , line : 182usize , placeholders : & [] , regex : "^Update shares from custom groups to circles$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Debug , path : "/apps/circles/lib/ShareByCircleProvider.php" , line : 339usize , placeholders : & [] , regex : "^Shares::move\\(\\)$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Debug , path : "/apps/circles/lib/ShareByCircleProvider.php" , line : 377usize , placeholders : & [] , regex : "^Shares::move\\(\\)$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/circles/lib/CirclesQueryHelper.php" , line : 104usize , placeholders : & [] , regex : "^session not initiated$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/circles/lib/Service/ContactService.php" , line : 134usize , placeholders : & [] , regex : "^issue with contact format USERID/ADDRESSBOOK/CONTACTID$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/circles/lib/Service/ShareWrapperService.php" , line : 130usize , placeholders : & [] , regex : "^\\$initiator cannot be empty$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/circles/lib/Service/RemoteService.php" , line : 412usize , placeholders : & [] , regex : "^incorrect federated user returned from instance$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/circles/lib/Service/RemoteService.php" , line : 415usize , placeholders : & [] , regex : "^incorrect instance on returned federated user$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/circles/lib/Service/RemoteStreamService.php" , line : 418usize , placeholders : & [] , regex : "^instance is local$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/circles/lib/Service/RemoteStreamService.php" , line : 434usize , placeholders : & [] , regex : "^instance is already known$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/circles/lib/Service/RemoteStreamService.php" , line : 490usize , placeholders : & [] , regex : "^invalid auth\\-signed$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/circles/lib/Service/RemoteStreamService.php" , line : 501usize , placeholders : & [] , regex : "^auth not confirmed$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/circles/lib/Service/CirclesService.php" , line : 247usize , placeholders : & [] , regex : "^UserID cannot be null$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/circles/lib/Service/CirclesService.php" , line : 635usize , placeholders : & [] , regex : "^This circle already reach its limit on the number of members$" , }
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
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/circles/lib/Service/MiscService.php" , line : 109usize , placeholders : & [] , regex : "^missing_key_in_array$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/circles/lib/Service/FederatedUserService.php" , line : 336usize , placeholders : & [] , regex : "^Invalid initiator$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/circles/lib/Service/FederatedUserService.php" , line : 362usize , placeholders : & [] , regex : "^Must initialise Super Session$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/circles/lib/Service/FederatedUserService.php" , line : 628usize , placeholders : & [] , regex : "^This Circle is not managed from this instance, please use \\-\\-initiator$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/circles/lib/Service/FederatedUserService.php" , line : 1003usize , placeholders : & [] , regex : "^FederatedUser must be local$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Debug , path : "/apps/circles/lib/Service/FederatedUserService.php" , line : 1085usize , placeholders : & [] , regex : "^FederatedUser is not empty$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/circles/lib/Service/FederatedUserService.php" , line : 1086usize , placeholders : & [] , regex : "^FederatedUser is not complete$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Debug , path : "/apps/circles/lib/Service/FederatedUserService.php" , line : 1105usize , placeholders : & [] , regex : "^uniqueness of SingleId could not be confirmed$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/circles/lib/Service/FederatedUserService.php" , line : 1109usize , placeholders : & [] , regex : "^uniqueness of SingleId could not be confirmed$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/circles/lib/Service/FederatedUserService.php" , line : 1227usize , placeholders : & [] , regex : "^group not found$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/circles/lib/Service/MigrationService.php" , line : 185usize , placeholders : & [] , regex : "^A migration process is already running$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/circles/lib/Service/DavService.php" , line : 528usize , placeholders : & [] , regex : "^Circles needs to be set as Contacts App Backend first$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Unknown , path : "/apps/circles/lib/Service/DavService.php" , line : 567usize , placeholders : & [] , regex : "^Deprecated Contacts managed$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Unknown , path : "/apps/circles/lib/Service/DavService.php" , line : 620usize , placeholders : & [] , regex : "^Deprecated Circles managed$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/circles/lib/Service/CircleService.php" , line : 198usize , placeholders : & [] , regex : "^owner not defined$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/circles/lib/Service/CircleService.php" , line : 207usize , placeholders : & [] , regex : "^Circle name is too short$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/circles/lib/Service/MaintenanceService.php" , line : 210usize , placeholders : & [] , regex : "^maintenance already running$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/circles/lib/Service/FederatedEventService.php" , line : 213usize , placeholders : & [] , regex : "^Initiator does not exist$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/circles/lib/Service/FederatedEventService.php" , line : 219usize , placeholders : & [] , regex : "^Initiator is not from the instance at the origin of the request$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/circles/lib/Service/FederatedEventService.php" , line : 225usize , placeholders : & [] , regex : "^Initiator must belong to the instance at the origin of the request$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/circles/lib/Service/FederatedEventService.php" , line : 232usize , placeholders : & [] , regex : "^Initiator must be a member of the Circle$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/circles/lib/Service/FederatedEventService.php" , line : 315usize , placeholders : & [] , regex : "^FederatedEvent has no Circle linked$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/circles/lib/Service/FederatedEventService.php" , line : 322usize , placeholders : & [] , regex : "^FederatedEvent has no Member linked$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/circles/lib/Service/FederatedEventService.php" , line : 335usize , placeholders : & [] , regex : "^FederatedItem must be executed locally$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/circles/lib/Service/FederatedEventService.php" , line : 355usize , placeholders : & [] , regex : "^FederatedItem must contains ItemId$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/circles/lib/Service/FederatedEventService.php" , line : 361usize , placeholders : & [] , regex : "^ShareLock belongs to another instance$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/circles/lib/Service/InterfaceService.php" , line : 125usize , placeholders : & [] , regex : "^interface not initialized$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/circles/lib/Service/InterfaceService.php" , line : 272usize , placeholders : & [] , regex : "^unknown interface$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/circles/lib/Service/InterfaceService.php" , line : 360usize , placeholders : & [] , regex : "^unknown configured interface$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/circles/lib/Service/InterfaceService.php" , line : 409usize , placeholders : & [] , regex : "^misconfigured scheme$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/circles/lib/Service/GSUpstreamService.php" , line : 421usize , placeholders : & [] , regex : "^result status is not good$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/circles/lib/Db/RemoteRequestBuilder.php" , line : 108usize , placeholders : & [] , regex : "^Unknown remote instance$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/circles/lib/Db/CircleRequest.php" , line : 293usize , placeholders : & [] , regex : "^singleId not found$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/circles/lib/Db/MountRequestBuilder.php" , line : 103usize , placeholders : & [] , regex : "^Mount not found$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/circles/lib/Db/TokensRequest.php" , line : 58usize , placeholders : & [] , regex : "^Unknown share token$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/circles/lib/Db/TokensRequest.php" , line : 83usize , placeholders : & [] , regex : "^Unknown share token$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/circles/lib/Db/CircleRequestBuilder.php" , line : 114usize , placeholders : & [] , regex : "^Circle not found$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/circles/lib/Api/v1/Circles.php" , line : 238usize , placeholders : & [] , regex : "^Method is deprecated and not longer works$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/circles/lib/CirclesManager.php" , line : 396usize , placeholders : & [] , regex : "^This Circle is not managed from this instance$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/circles/lib/Controller/RemoteController.php" , line : 365usize , placeholders : & [] , regex : "^Entity not found$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Debug , path : "/apps/circles/lib/Controller/RemoteController.php" , line : 502usize , placeholders : & [] , regex : "^Signatory is not a known RemoteInstance$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/circles/lib/Controller/RemoteController.php" , line : 503usize , placeholders : & [] , regex : "^Could not confirm identity$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Debug , path : "/apps/circles/lib/Controller/RemoteController.php" , line : 508usize , placeholders : & [] , regex : "^Could not confirm identity$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/circles/lib/Controller/RemoteController.php" , line : 509usize , placeholders : & [] , regex : "^Could not confirm identity$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/circles/lib/Controller/AdminController.php" , line : 201usize , placeholders : & [] , regex : "^works only from local instance$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/circles/lib/Controller/LocalController.php" , line : 243usize , placeholders : & [] , regex : "^works only from local instance$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/circles/lib/Controller/LocalController.php" , line : 592usize , placeholders : & [] , regex : "^frontend disabled$" , }
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
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/circles/lib/Tools/Traits/TNCLocalSignatory.php" , line : 69usize , placeholders : & [] , regex : "^signatory not found$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Debug , path : "/apps/circles/lib/Tools/Traits/TNCRequest.php" , line : 83usize , placeholders : & [] , regex : "^doRequest initiated$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Debug , path : "/apps/circles/lib/Tools/Traits/TNCRequest.php" , line : 97usize , placeholders : & [] , regex : "^doRequest done$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/circles/lib/Model/Helpers/MemberHelper.php" , line : 91usize , placeholders : & [] , regex : "^unknown method call$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/circles/lib/Model/Helpers/MemberHelper.php" , line : 140usize , placeholders : & [] , regex : "^Insufficient rights$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/circles/lib/Model/Helpers/MemberHelper.php" , line : 152usize , placeholders : & [] , regex : "^Insufficient rights$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/circles/lib/Model/Helpers/MemberHelper.php" , line : 164usize , placeholders : & [] , regex : "^Insufficient rights$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/circles/lib/Model/Mount.php" , line : 432usize , placeholders : & [] , regex : "^ShareWrapper has no Circle$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/circles/lib/Model/Federated/RemoteInstance.php" , line : 471usize , placeholders : & [] , regex : "^identity not authed$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/circles/lib/Model/Circle.php" , line : 637usize , placeholders : & [] , regex : "^circle has no owner$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/circles/lib/Model/DeprecatedMember.php" , line : 42usize , placeholders : & [] , regex : "^Invalid circle type$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/circles/lib/Model/FederatedLink.php" , line : 310usize , placeholders : & [] , regex : "^The status could not be updated$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/circles/lib/Model/GlobalScale/GSEvent.php" , line : 410usize , placeholders : & [] , regex : "^invalid JSON$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/circles/lib/Model/GlobalScale/GSEvent.php" , line : 442usize , placeholders : & [] , regex : "^invalid GSEvent$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/circles/lib/GlobalScale/AGlobalScaleEvent.php" , line : 216usize , placeholders : & [] , regex : "^GSEvent cannot be checked$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/circles/lib/GlobalScale/AGlobalScaleEvent.php" , line : 231usize , placeholders : & [] , regex : "^Viewer seems DSync$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/circles/lib/GlobalScale/CircleUpdate.php" , line : 67usize , placeholders : & [] , regex : "^Member is not Owner$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Unknown , path : "/apps/circles/lib/ShareByCircleProviderDeprecated.php" , line : 610usize , placeholders : & [] , regex : "^data is false \\- checking personal token$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/circles/lib/ShareByCircleProviderDeprecated.php" , line : 615usize , placeholders : & [] , regex : "^Share not found$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/circles/lib/ShareByCircleProviderDeprecated.php" , line : 625usize , placeholders : & [] , regex : "^Share not found$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/circles/lib/ShareByCircleProviderDeprecated.php" , line : 669usize , placeholders : & [] , regex : "^personal check not found$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/circles/lib/ShareByCircleProviderDeprecated.php" , line : 679usize , placeholders : & [] , regex : "^invalid token$" , }
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
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/circles/lib/Command/CirclesCheck.php" , line : 177usize , placeholders : & [] , regex : "^Please specify a \\-\\-type for the test$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/circles/lib/Command/CirclesCheck.php" , line : 264usize , placeholders : & [] , regex : "^Your Circles App is not fully configured\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/circles/lib/Command/CirclesMemberships.php" , line : 203usize , placeholders : & [] , regex : "^Not enough arguments \\(missing: \"userId\"\\)\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/circles/lib/Command/CirclesDetails.php" , line : 176usize , placeholders : & [] , regex : "^unknown circle, use \\-\\-instance to retrieve the data from a remote instance$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/circles/lib/Command/CirclesSetting.php" , line : 147usize , placeholders : & [] , regex : "^you need to specify a value$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/circles/lib/Command/SharesFiles.php" , line : 317usize , placeholders : & [] , regex : "^Specify a FileId or an option: \\-\\-with \\(USER\\), \\-\\-by \\(USER\\), \\-\\-to \\(CIRCLE\\)$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/circles/lib/Command/CirclesReport.php" , line : 136usize , placeholders : & [] , regex : "^not available$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/circles/lib/Command/CirclesEdit.php" , line : 153usize , placeholders : & [] , regex : "^edit can only be 'displayName' or 'description'$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/circles/lib/FederatedItems/CircleJoin.php" , line : 290usize , placeholders : & [] , regex : "^Blocked$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/circles/lib/FederatedItems/MemberLevel.php" , line : 111usize , placeholders : & [] , regex : "^This level cannot be edited$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/circles/lib/FederatedItems/MemberLevel.php" , line : 116usize , placeholders : & [] , regex : "^invalid level$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/circles/lib/FederatedItems/MemberLevel.php" , line : 120usize , placeholders : & [] , regex : "^This member already have the selected level$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/circles/lib/FederatedItems/CircleEdit.php" , line : 106usize , placeholders : & [] , regex : "^Circle name is too short$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/circles/lib/FederatedItems/CircleConfig.php" , line : 160usize , placeholders : & [] , regex : "^Configuration value is not valid$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/circles/lib/FederatedItems/CircleCreate.php" , line : 127usize , placeholders : & [] , regex : "^Circle already exist$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/circles/lib/FederatedItems/CircleCreate.php" , line : 135usize , placeholders : & [] , regex : "^Owner already exist$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/circles/vendor/composer/ClassLoader.php" , line : 175usize , placeholders : & [] , regex : "^A non\\-empty PSR\\-4 prefix must end with a namespace separator\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/circles/vendor/composer/ClassLoader.php" , line : 226usize , placeholders : & [] , regex : "^A non\\-empty PSR\\-4 prefix must end with a namespace separator\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Info , path : "/apps/survey_client/lib/BackgroundJobs/MonthlyReport.php" , line : 51usize , placeholders : & [] , regex : "^Error while sending usage statistic$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/comments/lib/Search/Result.php" , line : 94usize , placeholders : & [] , regex : "^Path not inside visible section$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/comments/lib/Search/Result.php" , line : 109usize , placeholders : & [] , regex : "^Comment section not found$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/comments/lib/Search/LegacyProvider.php" , line : 108usize , placeholders : & [] , regex : "^File not found$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/comments/lib/Notification/Notifier.php" , line : 97usize , placeholders : & [] , regex : "^Comment not found$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/comments/lib/Notification/Notifier.php" , line : 113usize , placeholders : & [] , regex : "^Unsupported comment object$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/comments/lib/Notification/Notifier.php" , line : 163usize , placeholders : & [] , regex : "^Invalid subject$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/comments/composer/composer/ClassLoader.php" , line : 252usize , placeholders : & [] , regex : "^A non\\-empty PSR\\-4 prefix must end with a namespace separator\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/comments/composer/composer/ClassLoader.php" , line : 307usize , placeholders : & [] , regex : "^A non\\-empty PSR\\-4 prefix must end with a namespace separator\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/activity/lib/Data.php" , line : 183usize , placeholders : & [] , regex : "^Invalid user$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/activity/lib/Data.php" , line : 289usize , placeholders : & [] , regex : "^Invalid since$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Debug , path : "/apps/activity/lib/MailQueueHandler.php" , line : 157usize , placeholders : & [] , regex : "^Couldn't send notification email to user '\\{user\\}' \\(email address isn't set for that user\\)$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Warn , path : "/apps/activity/lib/MailQueueHandler.php" , line : 168usize , placeholders : & [] , regex : "^Failed sending activity email to user '\\{user\\}'\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Warn , path : "/apps/activity/lib/MailQueueHandler.php" , line : 338usize , placeholders : & [] , regex : "^Notification for user \"\\{user\\}\" not sent because the email address \"\\{email\\}\" is invalid\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Error , path : "/apps/activity/lib/DigestSender.php" , line : 115usize , placeholders : & [] , regex : "^Exception occurred while sending user digest email$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/activity/lib/Controller/APIv2Controller.php" , line : 157usize , placeholders : & [] , regex : "^Invalid filter$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/activity/lib/Controller/APIv2Controller.php" , line : 177usize , placeholders : & [] , regex : "^Not logged in$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/activity/lib/AppInfo/Application.php" , line : 77usize , placeholders : & [] , regex : "^Invalid database type$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/systemtags/composer/composer/ClassLoader.php" , line : 252usize , placeholders : & [] , regex : "^A non\\-empty PSR\\-4 prefix must end with a namespace separator\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/systemtags/composer/composer/ClassLoader.php" , line : 307usize , placeholders : & [] , regex : "^A non\\-empty PSR\\-4 prefix must end with a namespace separator\\.$" , }
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
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/user_ldap/lib/Configuration.php" , line : 543usize , placeholders : & [] , regex : "^Invalid rule$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Warn , path : "/apps/user_ldap/lib/Configuration.php" , line : 561usize , placeholders : & [] , regex : "^Invalid config value to ldapUserAvatarRule; falling back to default\\.$" , }
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
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/user_ldap/lib/Connection.php" , line : 246usize , placeholders : & [] , regex : "^Connection to LDAP server could not be established$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Info , path : "/apps/user_ldap/lib/Connection.php" , line : 441usize , placeholders : & [] , regex : "^LDAPS \\(already using secure connection\\) and TLS do not work together\\. Switched off TLS\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Warn , path : "/apps/user_ldap/lib/Connection.php" , line : 565usize , placeholders : & [] , regex : "^Configuration is invalid, cannot connect$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Error , path : "/apps/user_ldap/lib/Connection.php" , line : 574usize , placeholders : & [] , regex : "^function ldap_connect is not available\\. Make sure that the PHP ldap module is installed\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Debug , path : "/apps/user_ldap/lib/Connection.php" , line : 583usize , placeholders : & [] , regex : "^Turned off SSL certificate validation successfully\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Warn , path : "/apps/user_ldap/lib/Connection.php" , line : 588usize , placeholders : & [] , regex : "^Could not turn off SSL certificate validation\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/user_ldap/lib/Connection.php" , line : 645usize , placeholders : & [] , regex : "^Could not set required LDAP Protocol version\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/user_ldap/lib/Connection.php" , line : 649usize , placeholders : & [] , regex : "^Could not disable LDAP referrals\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/user_ldap/lib/Connection.php" , line : 653usize , placeholders : & [] , regex : "^Could not set network timeout$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Debug , path : "/apps/user_ldap/lib/Group_LDAP.php" , line : 231usize , placeholders : & [] , regex : "^No search filter found on member url of group \\{dn\\}$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/user_ldap/lib/Group_LDAP.php" , line : 486usize , placeholders : & [] , regex : "^Not a valid group$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/user_ldap/lib/Group_LDAP.php" , line : 613usize , placeholders : & [] , regex : "^Not a valid group$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Debug , path : "/apps/user_ldap/lib/Group_LDAP.php" , line : 747usize , placeholders : & [] , regex : "^No search filter found on member url of group \\{dn\\}$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Debug , path : "/apps/user_ldap/lib/Group_LDAP.php" , line : 797usize , placeholders : & [] , regex : "^No uid attribute found for DN \\{dn\\} on \\{host\\}$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/user_ldap/lib/Group_LDAP.php" , line : 1247usize , placeholders : & [] , regex : "^Could not create group in LDAP backend\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/user_ldap/lib/Group_LDAP.php" , line : 1298usize , placeholders : & [] , regex : "^Could not add user to group in LDAP backend\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/user_ldap/lib/Group_LDAP.php" , line : 1317usize , placeholders : & [] , regex : "^Could not remove user from group in LDAP backend\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/user_ldap/lib/Group_LDAP.php" , line : 1331usize , placeholders : & [] , regex : "^Could not get group details in LDAP backend\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Warn , path : "/apps/user_ldap/lib/User_LDAP.php" , line : 658usize , placeholders : & [] , regex : "^Failed to map created LDAP user with userid \\{userid\\}, because UUID could not be determined$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/user_ldap/lib/User_LDAP.php" , line : 667usize , placeholders : & [] , regex : "^LDAP Plugin: Method createUser changed to return the user DN instead of boolean\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/user_ldap/lib/LDAP.php" , line : 368usize , placeholders : & [] , regex : "^Lost connection to LDAP server\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/user_ldap/lib/LDAP.php" , line : 370usize , placeholders : & [] , regex : "^LDAP server is shutting down\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/user_ldap/lib/LDAP.php" , line : 372usize , placeholders : & [] , regex : "^LDAP authentication method rejected$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/user_ldap/lib/LDAP.php" , line : 374usize , placeholders : & [] , regex : "^LDAP Operations error$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Debug , path : "/apps/user_ldap/lib/LDAP.php" , line : 379usize , placeholders : & [] , regex : "^LDAP error \\{message\\} \\(\\{code\\}\\) after calling \\{func\\}$" , }
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
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/user_ldap/lib/Wizard.php" , line : 105usize , placeholders : & [] , regex : "^Requirements not met$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/user_ldap/lib/Wizard.php" , line : 117usize , placeholders : & [] , regex : "^Internal error: Invalid object type$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/user_ldap/lib/Wizard.php" , line : 186usize , placeholders : & [] , regex : "^invalid results received$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/user_ldap/lib/Wizard.php" , line : 353usize , placeholders : & [] , regex : "^Could not connect to LDAP$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/user_ldap/lib/Wizard.php" , line : 408usize , placeholders : & [] , regex : "^Could not connect to LDAP$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/user_ldap/lib/Wizard.php" , line : 417usize , placeholders : & [] , regex : "^memberOf is not supported by the server$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/user_ldap/lib/Wizard.php" , line : 510usize , placeholders : & [] , regex : "^Could not connect to LDAP$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/user_ldap/lib/Wizard.php" , line : 537usize , placeholders : & [] , regex : "^Could not connect to LDAP$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/user_ldap/lib/Wizard.php" , line : 597usize , placeholders : & [] , regex : "^Cannot create filter$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/user_ldap/lib/Wizard.php" , line : 619usize , placeholders : & [] , regex : "^Cannot create filter$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/user_ldap/lib/Wizard.php" , line : 642usize , placeholders : & [] , regex : "^connection error$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/user_ldap/lib/Wizard.php" , line : 647usize , placeholders : & [] , regex : "^missing placeholder$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/user_ldap/lib/Wizard.php" , line : 808usize , placeholders : & [] , regex : "^Could not connect to LDAP$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/user_ldap/lib/Wizard.php" , line : 846usize , placeholders : & [] , regex : "^Could not connect to LDAP$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/user_ldap/lib/Wizard.php" , line : 877usize , placeholders : & [] , regex : "^Could not connect to LDAP$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/user_ldap/lib/Wizard.php" , line : 916usize , placeholders : & [] , regex : "^Could not connect to LDAP$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Debug , path : "/apps/user_ldap/lib/Wizard.php" , line : 1057usize , placeholders : & [] , regex : "^Wiz: Attempting to connect$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Debug , path : "/apps/user_ldap/lib/Wizard.php" , line : 1079usize , placeholders : & [] , regex : "^Wiz: Attemping to Bind$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/user_ldap/lib/Wizard.php" , line : 1230usize , placeholders : & [] , regex : "^Could not connect to LDAP$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/user_ldap/lib/User/Manager.php" , line : 157usize , placeholders : & [] , regex : "^LDAP Access instance must be set first$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Error , path : "/apps/user_ldap/lib/User/User.php" , line : 121usize , placeholders : & ["$dn"] , regex : "^uid for '(.*)' must not be null!$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/user_ldap/lib/User/User.php" , line : 122usize , placeholders : & [] , regex : "^uid must not be null!$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Error , path : "/apps/user_ldap/lib/User/User.php" , line : 124usize , placeholders : & ["$dn"] , regex : "^uid for '(.*)' must not be an empty string$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/user_ldap/lib/User/User.php" , line : 125usize , placeholders : & [] , regex : "^uid must not be an empty string!$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/user_ldap/lib/Command/ResetUser.php" , line : 79usize , placeholders : & [] , regex : "^User not found$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/user_ldap/lib/Command/ResetUser.php" , line : 83usize , placeholders : & [] , regex : "^The given user is not a recognized LDAP user\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/user_ldap/lib/Command/ResetUser.php" , line : 92usize , placeholders : & [] , regex : "^Reset cancelled by operator$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/user_ldap/lib/Command/Search.php" , line : 96usize , placeholders : & [] , regex : "^limit must be  0 or greater$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/user_ldap/lib/Command/Search.php" , line : 99usize , placeholders : & [] , regex : "^offset must be 0 or greater$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/user_ldap/lib/Command/Search.php" , line : 102usize , placeholders : & [] , regex : "^offset must be 0 if limit is also set to 0$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/user_ldap/lib/Command/Search.php" , line : 105usize , placeholders : & [] , regex : "^offset must be a multiple of limit$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/user_ldap/lib/Command/ResetGroup.php" , line : 76usize , placeholders : & [] , regex : "^Group not found$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/user_ldap/lib/Command/ResetGroup.php" , line : 80usize , placeholders : & [] , regex : "^The given group is not a recognized LDAP group\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/user_ldap/lib/Command/ResetGroup.php" , line : 89usize , placeholders : & [] , regex : "^Reset cancelled by operator$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/user_ldap/lib/Command/CheckUser.php" , line : 109usize , placeholders : & [] , regex : "^The given user is not a recognized LDAP user\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Info , path : "/apps/user_ldap/lib/Command/UpdateUUID.php" , line : 340usize , placeholders : & [] , regex : "^UUID of \\{id\\} was updated from \\{from\\} to \\{to\\}$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/user_ldap/lib/Mapping/AbstractMapping.php" , line : 93usize , placeholders : & [] , regex : "^Invalid Column Name$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/user_ldap/lib/Mapping/AbstractMapping.php" , line : 198usize , placeholders : & [] , regex : "^hash function did not return a string$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Error , path : "/apps/user_ldap/lib/Mapping/AbstractMapping.php" , line : 359usize , placeholders : & [] , regex : "^Cannot map, because the DN exceeds 4000 characters: \\{dn\\}$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/user_ldap/lib/PagedResults/TLinkId.php" , line : 42usize , placeholders : & [] , regex : "^No resource provided$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Debug , path : "/apps/user_ldap/lib/Jobs/UpdateGroups.php" , line : 91usize , placeholders : & [] , regex : "^Run background job \"updateGroups\"$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Info , path : "/apps/user_ldap/lib/Jobs/UpdateGroups.php" , line : 100usize , placeholders : & [] , regex : "^bgJ \"updateGroups\" – groups do not seem to be configured properly, aborting\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Debug , path : "/apps/user_ldap/lib/Jobs/UpdateGroups.php" , line : 111usize , placeholders : & [] , regex : "^bgJ \"updateGroups\" – Finished\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Debug , path : "/apps/user_ldap/lib/Jobs/UpdateGroups.php" , line : 141usize , placeholders : & [] , regex : "^bgJ \"updateGroups\" – Dealing with known Groups\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Info , path : "/apps/user_ldap/lib/Jobs/UpdateGroups.php" , line : 164usize , placeholders : & [] , regex : "^bgJ \"updateGroups\" – \\{user\\} removed from \\{group\\}$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Info , path : "/apps/user_ldap/lib/Jobs/UpdateGroups.php" , line : 179usize , placeholders : & [] , regex : "^bgJ \"updateGroups\" – \\{user\\} added to \\{group\\}$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Debug , path : "/apps/user_ldap/lib/Jobs/UpdateGroups.php" , line : 197usize , placeholders : & [] , regex : "^bgJ \"updateGroups\" – FINISHED dealing with known Groups\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Debug , path : "/apps/user_ldap/lib/Jobs/UpdateGroups.php" , line : 207usize , placeholders : & [] , regex : "^bgJ \"updateGroups\" – dealing with created Groups\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Debug , path : "/apps/user_ldap/lib/Jobs/UpdateGroups.php" , line : 227usize , placeholders : & [] , regex : "^bgJ \"updateGroups\" – FINISHED dealing with created Groups\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Debug , path : "/apps/user_ldap/lib/Jobs/UpdateGroups.php" , line : 237usize , placeholders : & [] , regex : "^bgJ \"updateGroups\" – dealing with removed groups\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Debug , path : "/apps/user_ldap/lib/Jobs/UpdateGroups.php" , line : 254usize , placeholders : & [] , regex : "^bgJ \"updateGroups\" – FINISHED dealing with removed groups\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/user_ldap/lib/Helper.php" , line : 283usize , placeholders : & [] , regex : "^key uid is expected to be set in \\$param$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/user_ldap/lib/Access.php" , line : 132usize , placeholders : & [] , regex : "^UserMapper was not assigned to this Access instance\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/user_ldap/lib/Access.php" , line : 151usize , placeholders : & [] , regex : "^GroupMapper was not assigned to this Access instance\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Warn , path : "/apps/user_ldap/lib/Access.php" , line : 185usize , placeholders : & [] , regex : "^No LDAP Connector assigned, access impossible for readAttribute\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Debug , path : "/apps/user_ldap/lib/Access.php" , line : 194usize , placeholders : & [] , regex : "^LDAP resource not available\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/user_ldap/lib/Access.php" , line : 362usize , placeholders : & [] , regex : "^LDAP password changes are disabled\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Debug , path : "/apps/user_ldap/lib/Access.php" , line : 367usize , placeholders : & [] , regex : "^LDAP resource not available\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/user_ldap/lib/Access.php" , line : 375usize , placeholders : & [] , regex : "^Password change rejected\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Debug , path : "/apps/user_ldap/lib/Access.php" , line : 920usize , placeholders : & ["$ocName"] , regex : "^The ldap user manager returned null for (.*)$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/user_ldap/lib/Access.php" , line : 1053usize , placeholders : & [] , regex : "^Invoker does not support controlPagedResultResponse, call LDAP Wrapper directly instead\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Debug , path : "/apps/user_ldap/lib/Access.php" , line : 1069usize , placeholders : & ["$command"] , regex : "^Connection lost on (.*), attempting to reestablish\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Debug , path : "/apps/user_ldap/lib/Access.php" , line : 1075usize , placeholders : & ["$command"] , regex : "^Could not (.*), because resource is missing\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Debug , path : "/apps/user_ldap/lib/Access.php" , line : 1109usize , placeholders : & [] , regex : "^Could not search, because resource is missing\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Debug , path : "/apps/user_ldap/lib/Access.php" , line : 1169usize , placeholders : & [] , regex : "^Paged search was not available$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Debug , path : "/apps/user_ldap/lib/Access.php" , line : 1205usize , placeholders : & [] , regex : "^Count filter: \\{filter\\}$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/user_ldap/lib/Access.php" , line : 1388usize , placeholders : & [] , regex : "^provided name template for username does not contain any allowed characters$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/user_ldap/lib/Access.php" , line : 1400usize , placeholders : & [] , regex : "^provided name template for username does not contain any allowed characters$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/user_ldap/lib/Access.php" , line : 1500usize , placeholders : & [] , regex : "^searchAttributes must be an array with at least two string$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/user_ldap/lib/Access.php" , line : 1628usize , placeholders : & [] , regex : "^Cannot determine UUID attribute$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/user_ldap/lib/Access.php" , line : 1635usize , placeholders : & [] , regex : "^Cannot determine UUID attribute$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/user_ldap/lib/Access.php" , line : 1652usize , placeholders : & [] , regex : "^Cannot determine UUID attribute$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Debug , path : "/apps/user_ldap/lib/Access.php" , line : 1700usize , placeholders : & [] , regex : "^Setting \\{attribute\\} as \\{subject\\}$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Debug , path : "/apps/user_ldap/lib/Access.php" , line : 1713usize , placeholders : & [] , regex : "^Could not autodetect the UUID attribute$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Debug , path : "/apps/user_ldap/lib/Access.php" , line : 1971usize , placeholders : & [] , regex : "^initializing paged search for filter \\{filter\\}, base \\{base\\}, attr \\{attr\\}, limit \\{limit\\}, offset \\{offset\\}$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Debug , path : "/apps/user_ldap/lib/Access.php" , line : 2003usize , placeholders : & [] , regex : "^Ready for a paged search$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Error , path : "/apps/user_ldap/lib/Migration/Version1130Date20211102154716.php" , line : 167usize , placeholders : & [] , regex : "^Failed to add hash \"\\{dnHash\\}\" \\(\"\\{name\\}\" of \\{table\\}\\)$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Warn , path : "/apps/user_ldap/lib/Migration/Version1130Date20211102154716.php" , line : 230usize , placeholders : & [] , regex : "^LDAP user or group with ID \\{nid\\} has a duplicated UUID value which therefore was invalidated\\. You may double\\-check your LDAP configuration and trigger an update of the UUID\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Error , path : "/apps/user_ldap/lib/Migration/Version1120Date20210917155206.php" , line : 113usize , placeholders : & [] , regex : "^Failed to shorten owncloud_name \"\\{oldId\\}\" to \"\\{newId\\}\" \\(UUID: \"\\{uuid\\}\" of \\{table\\}\\)$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/user_ldap/composer/composer/ClassLoader.php" , line : 252usize , placeholders : & [] , regex : "^A non\\-empty PSR\\-4 prefix must end with a namespace separator\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/user_ldap/composer/composer/ClassLoader.php" , line : 307usize , placeholders : & [] , regex : "^A non\\-empty PSR\\-4 prefix must end with a namespace separator\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/recommendations/lib/Controller/SettingsController.php" , line : 64usize , placeholders : & [] , regex : "^Not logged in$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/recommendations/lib/Controller/SettingsController.php" , line : 82usize , placeholders : & [] , regex : "^Not logged in$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/recommendations/lib/Controller/RecommendationController.php" , line : 65usize , placeholders : & [] , regex : "^Not logged in$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/recommendations/lib/Controller/RecommendationController.php" , line : 84usize , placeholders : & [] , regex : "^Not logged in$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/updatenotification/composer/composer/ClassLoader.php" , line : 252usize , placeholders : & [] , regex : "^A non\\-empty PSR\\-4 prefix must end with a namespace separator\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/updatenotification/composer/composer/ClassLoader.php" , line : 307usize , placeholders : & [] , regex : "^A non\\-empty PSR\\-4 prefix must end with a namespace separator\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/updatenotification/lib/Notification/Notifier.php" , line : 114usize , placeholders : & [] , regex : "^Unknown app id$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/updatenotification/lib/Notification/Notifier.php" , line : 122usize , placeholders : & [] , regex : "^Update checked worked again$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/cloud_federation_api/composer/composer/ClassLoader.php" , line : 252usize , placeholders : & [] , regex : "^A non\\-empty PSR\\-4 prefix must end with a namespace separator\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/cloud_federation_api/composer/composer/ClassLoader.php" , line : 307usize , placeholders : & [] , regex : "^A non\\-empty PSR\\-4 prefix must end with a namespace separator\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/sharebymail/composer/composer/ClassLoader.php" , line : 252usize , placeholders : & [] , regex : "^A non\\-empty PSR\\-4 prefix must end with a namespace separator\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/sharebymail/composer/composer/ClassLoader.php" , line : 307usize , placeholders : & [] , regex : "^A non\\-empty PSR\\-4 prefix must end with a namespace separator\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/sharebymail/lib/ShareByMailProvider.php" , line : 384usize , placeholders : & [] , regex : "^Failed to send share by mail$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/sharebymail/lib/ShareByMailProvider.php" , line : 829usize , placeholders : & [] , regex : "^not implemented$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/sharebymail/lib/ShareByMailProvider.php" , line : 999usize , placeholders : & [] , regex : "^Share not found$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/sharebymail/lib/ShareByMailProvider.php" , line : 1005usize , placeholders : & [] , regex : "^Share not found$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/provisioning_api/composer/composer/ClassLoader.php" , line : 252usize , placeholders : & [] , regex : "^A non\\-empty PSR\\-4 prefix must end with a namespace separator\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/provisioning_api/composer/composer/ClassLoader.php" , line : 307usize , placeholders : & [] , regex : "^A non\\-empty PSR\\-4 prefix must end with a namespace separator\\.$" , }
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
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/provisioning_api/lib/Controller/GroupsController.php" , line : 278usize , placeholders : & [] , regex : "^Not supported by backend$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/provisioning_api/lib/Controller/GroupsController.php" , line : 280usize , placeholders : & [] , regex : "^$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/provisioning_api/lib/Controller/GroupsController.php" , line : 296usize , placeholders : & [] , regex : "^$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/provisioning_api/lib/Controller/GroupsController.php" , line : 299usize , placeholders : & [] , regex : "^$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/provisioning_api/lib/Controller/GroupsController.php" , line : 314usize , placeholders : & [] , regex : "^Group does not exist$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/provisioning_api/lib/Controller/AUserData.php" , line : 114usize , placeholders : & [] , regex : "^User does not exist$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/provisioning_api/lib/Controller/AUserData.php" , line : 229usize , placeholders : & [] , regex : "^User does not exist$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/provisioning_api/lib/Controller/AUserData.php" , line : 263usize , placeholders : & [] , regex : "^User does not exist$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Error , path : "/apps/provisioning_api/lib/Controller/AUserData.php" , line : 274usize , placeholders : & [] , regex : "^Could not load storage info for \\{user\\}$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/provisioning_api/lib/Controller/VerificationController.php" , line : 83usize , placeholders : & [] , regex : "^Logged in user is not mail address owner$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/provisioning_api/lib/Controller/VerificationController.php" , line : 102usize , placeholders : & [] , regex : "^Logged in user is not mail address owner$" , }
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
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/provisioning_api/lib/Controller/UsersController.php" , line : 315usize , placeholders : & [] , regex : "^Could not create non\\-existing user id$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Error , path : "/apps/provisioning_api/lib/Controller/UsersController.php" , line : 352usize , placeholders : & [] , regex : "^Failed addUser attempt: User already exists\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/provisioning_api/lib/Controller/UsersController.php" , line : 367usize , placeholders : & [] , regex : "^no group specified \\(required for subadmins\\)$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/provisioning_api/lib/Controller/UsersController.php" , line : 377usize , placeholders : & [] , regex : "^Subadmin group does not exist$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/provisioning_api/lib/Controller/UsersController.php" , line : 381usize , placeholders : & [] , regex : "^Cannot create subadmins for admin group$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/provisioning_api/lib/Controller/UsersController.php" , line : 385usize , placeholders : & [] , regex : "^No permissions to promote subadmins$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/provisioning_api/lib/Controller/UsersController.php" , line : 393usize , placeholders : & [] , regex : "^Invalid password value$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/provisioning_api/lib/Controller/UsersController.php" , line : 397usize , placeholders : & [] , regex : "^To send a password link to the user an email address is required\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/provisioning_api/lib/Controller/UsersController.php" , line : 416usize , placeholders : & [] , regex : "^Required email address was not provided$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Error , path : "/apps/provisioning_api/lib/Controller/UsersController.php" , line : 453usize , placeholders : & ["$email"] , regex : "^Unable to send the invitation mail to (.*)$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Warn , path : "/apps/provisioning_api/lib/Controller/UsersController.php" , line : 466usize , placeholders : & [] , regex : "^Failed addUser attempt with hint exception\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Warn , path : "/apps/provisioning_api/lib/Controller/UsersController.php" , line : 475usize , placeholders : & [] , regex : "^Failed addUser attempt with ocs exeption\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Error , path : "/apps/provisioning_api/lib/Controller/UsersController.php" , line : 484usize , placeholders : & [] , regex : "^Failed addUser attempt with invalid argument exeption\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Error , path : "/apps/provisioning_api/lib/Controller/UsersController.php" , line : 493usize , placeholders : & [] , regex : "^Failed addUser attempt with exception\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/provisioning_api/lib/Controller/UsersController.php" , line : 500usize , placeholders : & [] , regex : "^Bad request$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/provisioning_api/lib/Controller/UsersController.php" , line : 524usize , placeholders : & [] , regex : "^$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/provisioning_api/lib/Controller/UsersController.php" , line : 549usize , placeholders : & [] , regex : "^$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/provisioning_api/lib/Controller/UsersController.php" , line : 562usize , placeholders : & [] , regex : "^$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/provisioning_api/lib/Controller/UsersController.php" , line : 579usize , placeholders : & [] , regex : "^$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/provisioning_api/lib/Controller/UsersController.php" , line : 587usize , placeholders : & [] , regex : "^$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/provisioning_api/lib/Controller/UsersController.php" , line : 595usize , placeholders : & [] , regex : "^$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/provisioning_api/lib/Controller/UsersController.php" , line : 642usize , placeholders : & [] , regex : "^$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/provisioning_api/lib/Controller/UsersController.php" , line : 647usize , placeholders : & [] , regex : "^$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/provisioning_api/lib/Controller/UsersController.php" , line : 666usize , placeholders : & [] , regex : "^$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/provisioning_api/lib/Controller/UsersController.php" , line : 672usize , placeholders : & [] , regex : "^$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/provisioning_api/lib/Controller/UsersController.php" , line : 706usize , placeholders : & [] , regex : "^$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/provisioning_api/lib/Controller/UsersController.php" , line : 709usize , placeholders : & [] , regex : "^$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/provisioning_api/lib/Controller/UsersController.php" , line : 714usize , placeholders : & [] , regex : "^$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/provisioning_api/lib/Controller/UsersController.php" , line : 738usize , placeholders : & [] , regex : "^$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/provisioning_api/lib/Controller/UsersController.php" , line : 834usize , placeholders : & [] , regex : "^$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/provisioning_api/lib/Controller/UsersController.php" , line : 839usize , placeholders : & [] , regex : "^$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/provisioning_api/lib/Controller/UsersController.php" , line : 846usize , placeholders : & [] , regex : "^Invalid displayname$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/provisioning_api/lib/Controller/UsersController.php" , line : 874usize , placeholders : & [] , regex : "^Unlimited quota is forbidden on this instance$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/provisioning_api/lib/Controller/UsersController.php" , line : 882usize , placeholders : & [] , regex : "^Invalid password value$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/provisioning_api/lib/Controller/UsersController.php" , line : 885usize , placeholders : & [] , regex : "^Setting the password is not supported by the users backend$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/provisioning_api/lib/Controller/UsersController.php" , line : 895usize , placeholders : & [] , regex : "^Invalid language$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/provisioning_api/lib/Controller/UsersController.php" , line : 901usize , placeholders : & [] , regex : "^Invalid locale$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Info , path : "/apps/provisioning_api/lib/Controller/UsersController.php" , line : 912usize , placeholders : & [] , regex : "^Cannot set primary email, because provided address is not verified$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/provisioning_api/lib/Controller/UsersController.php" , line : 922usize , placeholders : & [] , regex : "^$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/provisioning_api/lib/Controller/UsersController.php" , line : 929usize , placeholders : & [] , regex : "^$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/provisioning_api/lib/Controller/UsersController.php" , line : 944usize , placeholders : & [] , regex : "^$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/provisioning_api/lib/Controller/UsersController.php" , line : 1014usize , placeholders : & [] , regex : "^$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/provisioning_api/lib/Controller/UsersController.php" , line : 1036usize , placeholders : & [] , regex : "^$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/provisioning_api/lib/Controller/UsersController.php" , line : 1040usize , placeholders : & [] , regex : "^$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/provisioning_api/lib/Controller/UsersController.php" , line : 1046usize , placeholders : & [] , regex : "^$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/provisioning_api/lib/Controller/UsersController.php" , line : 1068usize , placeholders : & [] , regex : "^$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/provisioning_api/lib/Controller/UsersController.php" , line : 1072usize , placeholders : & [] , regex : "^$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/provisioning_api/lib/Controller/UsersController.php" , line : 1078usize , placeholders : & [] , regex : "^$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/provisioning_api/lib/Controller/UsersController.php" , line : 1085usize , placeholders : & [] , regex : "^$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/provisioning_api/lib/Controller/UsersController.php" , line : 1126usize , placeholders : & [] , regex : "^$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/provisioning_api/lib/Controller/UsersController.php" , line : 1132usize , placeholders : & [] , regex : "^$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/provisioning_api/lib/Controller/UsersController.php" , line : 1153usize , placeholders : & [] , regex : "^$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/provisioning_api/lib/Controller/UsersController.php" , line : 1179usize , placeholders : & [] , regex : "^$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/provisioning_api/lib/Controller/UsersController.php" , line : 1195usize , placeholders : & [] , regex : "^$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/provisioning_api/lib/Controller/UsersController.php" , line : 1201usize , placeholders : & [] , regex : "^$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/provisioning_api/lib/Controller/UsersController.php" , line : 1204usize , placeholders : & [] , regex : "^$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/provisioning_api/lib/Controller/UsersController.php" , line : 1211usize , placeholders : & [] , regex : "^$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/provisioning_api/lib/Controller/UsersController.php" , line : 1232usize , placeholders : & [] , regex : "^$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/provisioning_api/lib/Controller/UsersController.php" , line : 1237usize , placeholders : & [] , regex : "^$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/provisioning_api/lib/Controller/UsersController.php" , line : 1242usize , placeholders : & [] , regex : "^$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/provisioning_api/lib/Controller/UsersController.php" , line : 1248usize , placeholders : & [] , regex : "^$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/provisioning_api/lib/Controller/UsersController.php" , line : 1255usize , placeholders : & [] , regex : "^Cannot remove yourself from the admin group$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/provisioning_api/lib/Controller/UsersController.php" , line : 1259usize , placeholders : & [] , regex : "^Cannot remove yourself from this group as you are a SubAdmin$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/provisioning_api/lib/Controller/UsersController.php" , line : 1272usize , placeholders : & [] , regex : "^Not viable to remove user from the last group you are SubAdmin of$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/provisioning_api/lib/Controller/UsersController.php" , line : 1297usize , placeholders : & [] , regex : "^User does not exist$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/provisioning_api/lib/Controller/UsersController.php" , line : 1301usize , placeholders : & [] , regex : "^Group does not exist$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/provisioning_api/lib/Controller/UsersController.php" , line : 1305usize , placeholders : & [] , regex : "^Cannot create subadmins for admin group$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/provisioning_api/lib/Controller/UsersController.php" , line : 1336usize , placeholders : & [] , regex : "^User does not exist$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/provisioning_api/lib/Controller/UsersController.php" , line : 1340usize , placeholders : & [] , regex : "^Group does not exist$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/provisioning_api/lib/Controller/UsersController.php" , line : 1344usize , placeholders : & [] , regex : "^User is not a subadmin of this group$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/provisioning_api/lib/Controller/UsersController.php" , line : 1379usize , placeholders : & [] , regex : "^$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/provisioning_api/lib/Controller/UsersController.php" , line : 1389usize , placeholders : & [] , regex : "^$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/provisioning_api/lib/Controller/UsersController.php" , line : 1394usize , placeholders : & [] , regex : "^Email address not available$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Error , path : "/apps/provisioning_api/lib/Controller/UsersController.php" , line : 1401usize , placeholders : & ["$email"] , regex : "^Can't send new user mail to (.*)$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/provisioning_api/lib/Controller/UsersController.php" , line : 1408usize , placeholders : & [] , regex : "^Sending email failed$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/provisioning_api/lib/Controller/AppsController.php" , line : 75usize , placeholders : & [] , regex : "^$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/provisioning_api/lib/Controller/AppsController.php" , line : 93usize , placeholders : & [] , regex : "^The request app was not found$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/provisioning_api/lib/Controller/AppsController.php" , line : 106usize , placeholders : & [] , regex : "^The request app was not found$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/federation/lib/TrustedServers.php" , line : 244usize , placeholders : & [] , regex : "^Remote server version is too low\\. 9\\.0 is required\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/federation/composer/composer/ClassLoader.php" , line : 252usize , placeholders : & [] , regex : "^A non\\-empty PSR\\-4 prefix must end with a namespace separator\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/federation/composer/composer/ClassLoader.php" , line : 307usize , placeholders : & [] , regex : "^A non\\-empty PSR\\-4 prefix must end with a namespace separator\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Debug , path : "/apps/files_sharing/lib/Listener/ShareInteractionListener.php" , line : 71usize , placeholders : & [] , regex : "^Share type does not allow to emit interaction event$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Warn , path : "/apps/files_sharing/lib/Listener/ShareInteractionListener.php" , line : 77usize , placeholders : & [] , regex : "^Share was not created by a user, can't emit interaction event$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/files_sharing/lib/Controller/ShareController.php" , line : 725usize , placeholders : & [] , regex : "^Downloading more than 1 file$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/files_sharing/lib/Controller/ShareController.php" , line : 742usize , placeholders : & [] , regex : "^Downloading a folder$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/files_sharing/lib/Controller/ShareAPIController.php" , line : 976usize , placeholders : & [] , regex : "^no sharing rights on this item$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/files_sharing/lib/Controller/ShareAPIController.php" , line : 1086usize , placeholders : & [] , regex : "^You are not allowed to edit incoming shares$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/files_sharing/lib/Controller/ShareAPIController.php" , line : 1127usize , placeholders : & [] , regex : "^You are not allowed to edit link shares that you don't own$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/files_sharing/lib/Controller/ShareAPIController.php" , line : 1210usize , placeholders : & [] , regex : "^Maxmimum label length is 255$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/files_sharing/lib/Controller/ShareAPIController.php" , line : 1534usize , placeholders : & [] , regex : "^Invalid date\\. Format must be YYYY\\-MM\\-DD$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/files_sharing/lib/Controller/ShareAPIController.php" , line : 1714usize , placeholders : & [] , regex : "^no sharing rights on this item$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/files_sharing/lib/Controller/ShareAPIController.php" , line : 1903usize , placeholders : & [] , regex : "^Should not happen, instanceOfStorage but getInstanceOfStorage return null$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/files_sharing/lib/Controller/ShareAPIController.php" , line : 1906usize , placeholders : & [] , regex : "^Should not happen, instanceOfStorage but not a wrapper$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/files_sharing/lib/Controller/DeletedShareAPIController.php" , line : 193usize , placeholders : & [] , regex : "^Share not found$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/files_sharing/lib/Controller/DeletedShareAPIController.php" , line : 197usize , placeholders : & [] , regex : "^No deleted share found$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/files_sharing/lib/Controller/DeletedShareAPIController.php" , line : 203usize , placeholders : & [] , regex : "^Something went wrong$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/files_sharing/lib/Controller/RemoteController.php" , line : 89usize , placeholders : & [] , regex : "^wrong share ID, share doesn't exist\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/files_sharing/lib/Controller/RemoteController.php" , line : 109usize , placeholders : & [] , regex : "^wrong share ID, share doesn't exist\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/files_sharing/lib/Controller/RemoteController.php" , line : 160usize , placeholders : & [] , regex : "^share does not exist$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/files_sharing/lib/Controller/RemoteController.php" , line : 181usize , placeholders : & [] , regex : "^Share does not exist$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/files_sharing/lib/Controller/RemoteController.php" , line : 189usize , placeholders : & [] , regex : "^Could not unshare$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/files_sharing/lib/Controller/ShareesAPIController.php" , line : 158usize , placeholders : & [] , regex : "^Invalid perPage argument$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/files_sharing/lib/Controller/ShareesAPIController.php" , line : 161usize , placeholders : & [] , regex : "^Invalid page$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/files_sharing/lib/Controller/ShareesAPIController.php" , line : 169usize , placeholders : & [] , regex : "^Missing itemType$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/files_sharing/lib/Controller/ShareesAPIController.php" , line : 347usize , placeholders : & [] , regex : "^Missing itemType$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Error , path : "/apps/files_sharing/lib/MountProvider.php" , line : 159usize , placeholders : & [] , regex : "^Error while trying to create shared mount$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Error , path : "/apps/files_sharing/lib/ShareBackend/File.php" , line : 236usize , placeholders : & [] , regex : "^No owner found for reshare$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/files_sharing/lib/Activity/Providers/Base.php" , line : 162usize , placeholders : & [] , regex : "^Could not generate file parameter$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/files_sharing/lib/SharedMount.php" , line : 200usize , placeholders : & [] , regex : "^Path does not start with /user/files$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/files_sharing/lib/External/Storage.php" , line : 226usize , placeholders : & [] , regex : "^Remote share not found$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/files_sharing/lib/External/Storage.php" , line : 229usize , placeholders : & [] , regex : "^No nextcloud instance found at remote$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/files_sharing/lib/External/Storage.php" , line : 235usize , placeholders : & [] , regex : "^Auth error when getting remote share$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/files_sharing/lib/External/Storage.php" , line : 237usize , placeholders : & [] , regex : "^Failed to connect to remote instance$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/files_sharing/lib/External/Storage.php" , line : 239usize , placeholders : & [] , regex : "^Error while sending request to remote instance$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Emergency , path : "/apps/files_sharing/lib/External/Manager.php" , line : 350usize , placeholders : & [] , regex : "^Could not update share$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Emergency , path : "/apps/files_sharing/lib/External/Manager.php" , line : 368usize , placeholders : & [] , regex : "^Could not create share$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Emergency , path : "/apps/files_sharing/lib/External/Manager.php" , line : 420usize , placeholders : & [] , regex : "^Could not update share$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Emergency , path : "/apps/files_sharing/lib/External/Manager.php" , line : 440usize , placeholders : & [] , regex : "^Could not create share$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Error , path : "/apps/files_sharing/lib/External/Manager.php" , line : 609usize , placeholders : & [] , regex : "^Mount point to remove share not found$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Emergency , path : "/apps/files_sharing/lib/External/Manager.php" , line : 645usize , placeholders : & [] , regex : "^Could not update share$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Emergency , path : "/apps/files_sharing/lib/External/Manager.php" , line : 713usize , placeholders : & [] , regex : "^Could not delete user shares$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Emergency , path : "/apps/files_sharing/lib/External/Manager.php" , line : 748usize , placeholders : & [] , regex : "^Could not delete user shares$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Emergency , path : "/apps/files_sharing/lib/External/Manager.php" , line : 826usize , placeholders : & [] , regex : "^Error when retrieving shares$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/files_sharing/lib/Middleware/SharingCheckMiddleware.php" , line : 91usize , placeholders : & [] , regex : "^Sharing is disabled\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/files_sharing/lib/Middleware/SharingCheckMiddleware.php" , line : 96usize , placeholders : & [] , regex : "^Federated sharing not allowed$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/files_sharing/lib/Notification/Notifier.php" , line : 108usize , placeholders : & [] , regex : "^Unhandled app or subject$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/files_sharing/lib/Notification/Notifier.php" , line : 170usize , placeholders : & [] , regex : "^Temporary failure$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/files_sharing/lib/Notification/Notifier.php" , line : 206usize , placeholders : & [] , regex : "^Temporary failure$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/files_sharing/lib/Notification/Notifier.php" , line : 230usize , placeholders : & [] , regex : "^Invalid subject$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Debug , path : "/apps/files_sharing/lib/DeleteOrphanedSharesJob.php" , line : 63usize , placeholders : & ["$deletedEntries"] , regex : "^(.*) orphaned share\\(s\\) deleted$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/files_sharing/composer/composer/ClassLoader.php" , line : 252usize , placeholders : & [] , regex : "^A non\\-empty PSR\\-4 prefix must end with a namespace separator\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/files_sharing/composer/composer/ClassLoader.php" , line : 307usize , placeholders : & [] , regex : "^A non\\-empty PSR\\-4 prefix must end with a namespace separator\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/oauth2/lib/Db/AccessTokenMapper.php" , line : 64usize , placeholders : & [] , regex : "^Could not find access token$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/oauth2/composer/composer/ClassLoader.php" , line : 252usize , placeholders : & [] , regex : "^A non\\-empty PSR\\-4 prefix must end with a namespace separator\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/oauth2/composer/composer/ClassLoader.php" , line : 307usize , placeholders : & [] , regex : "^A non\\-empty PSR\\-4 prefix must end with a namespace separator\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Warn , path : "/apps/support/lib/Service/SubscriptionService.php" , line : 151usize , placeholders : & ["$userCount" , "$disabledUsersCount"] , regex : "^Total user count was negative \\(users: (.*), disabled: (.*)\\)$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Info , path : "/apps/support/lib/Service/SubscriptionService.php" , line : 210usize , placeholders : & [] , regex : "^Subscription info successfully fetched$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Info , path : "/apps/support/lib/Service/SubscriptionService.php" , line : 254usize , placeholders : & [] , regex : "^Subscription key invalid$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Debug , path : "/apps/support/lib/Sections/ServerSection.php" , line : 162usize , placeholders : & [] , regex : "^Unable to determine database version$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/support/lib/Notification/Notifier.php" , line : 74usize , placeholders : & [] , regex : "^Unknown app id$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Info , path : "/apps/support/lib/Repair/SwitchUpdaterServer.php" , line : 53usize , placeholders : & [] , regex : "^Repair step already executed$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/workflowengine/lib/Manager.php" , line : 405usize , placeholders : & [] , regex : "^Target operation not within scope$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/workflowengine/lib/Manager.php" , line : 445usize , placeholders : & [] , regex : "^Target operation not within scope$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/workflowengine/lib/Controller/UserWorkflowsController.php" , line : 115usize , placeholders : & [] , regex : "^User not logged in$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Error , path : "/apps/workflowengine/lib/Controller/AWorkflowController.php" , line : 124usize , placeholders : & [] , regex : "^Error when inserting flow$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/workflowengine/lib/Controller/AWorkflowController.php" , line : 125usize , placeholders : & [] , regex : "^An internal error occurred$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/workflowengine/lib/Controller/AWorkflowController.php" , line : 153usize , placeholders : & [] , regex : "^An internal error occurred$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/workflowengine/lib/Controller/AWorkflowController.php" , line : 172usize , placeholders : & [] , regex : "^An internal error occurred$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/workflowengine/lib/Check/FileSystemTags.php" , line : 166usize , placeholders : & [] , regex : "^Should not happen: Storage is instance of GroupFolderStorage but no group folder storage found while unwrapping\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/workflowengine/lib/Check/TFileCheck.php" , line : 64usize , placeholders : & [] , regex : "^Expected Node subject for File entity, got \\{class\\}$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/workflowengine/lib/Helper/ScopeContext.php" , line : 48usize , placeholders : & [] , regex : "^Invalid scope$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/workflowengine/lib/Helper/ScopeContext.php" , line : 54usize , placeholders : & [] , regex : "^user scope requires a user id$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Debug , path : "/apps/workflowengine/lib/AppInfo/Application.php" , line : 112usize , placeholders : & [] , regex : "^Cannot handle event \\{name\\} of \\{event\\} against entity \\{entity\\} and operation \\{operation\\}$" , }
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
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/workflowengine/composer/composer/ClassLoader.php" , line : 252usize , placeholders : & [] , regex : "^A non\\-empty PSR\\-4 prefix must end with a namespace separator\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/workflowengine/composer/composer/ClassLoader.php" , line : 307usize , placeholders : & [] , regex : "^A non\\-empty PSR\\-4 prefix must end with a namespace separator\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/files_trashbin/composer/composer/ClassLoader.php" , line : 252usize , placeholders : & [] , regex : "^A non\\-empty PSR\\-4 prefix must end with a namespace separator\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/files_trashbin/composer/composer/ClassLoader.php" , line : 307usize , placeholders : & [] , regex : "^A non\\-empty PSR\\-4 prefix must end with a namespace separator\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Error , path : "/apps/files_trashbin/lib/Trashbin.php" , line : 224usize , placeholders : & [] , regex : "^trash bin database couldn't be updated for the files owner$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Error , path : "/apps/files_trashbin/lib/Trashbin.php" , line : 351usize , placeholders : & [] , regex : "^trash bin database couldn't be updated$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/files_trashbin/lib/Trashbin.php" , line : 491usize , placeholders : & [] , regex : "^Can't restore trash item because the target folder is not writable$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/files_trashbin/lib/Trash/TrashManager.php" , line : 89usize , placeholders : & ["$fullType"] , regex : "^Trash backend for (.*) not found$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/files_trashbin/lib/Command/CleanUp.php" , line : 82usize , placeholders : & [] , regex : "^Either specify a user_id or \\-\\-all\\-users$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/files_trashbin/lib/Command/CleanUp.php" , line : 113usize , placeholders : & [] , regex : "^Either specify a user_id or \\-\\-all\\-users$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/files_trashbin/lib/Command/RestoreAllFiles.php" , line : 85usize , placeholders : & [] , regex : "^Either specify a user_id or \\-\\-all\\-users$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/files_trashbin/lib/Command/RestoreAllFiles.php" , line : 116usize , placeholders : & [] , regex : "^Either specify a user_id or \\-\\-all\\-users$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/files_trashbin/lib/Helper.php" , line : 55usize , placeholders : & [] , regex : "^Directory does not exists$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/files_trashbin/lib/UserMigration/TrashbinMigrator.php" , line : 134usize , placeholders : & [] , regex : "^Could not import trashbin\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/files_trashbin/lib/Sabre/TrashHome.php" , line : 65usize , placeholders : & [] , regex : "^Permission denied to rename this trashbin$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/files_trashbin/lib/Sabre/TrashHome.php" , line : 69usize , placeholders : & [] , regex : "^Not allowed to create files in the trashbin$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/files_trashbin/lib/Sabre/TrashHome.php" , line : 73usize , placeholders : & [] , regex : "^Not allowed to create folders in the trashbin$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/files_trashbin/lib/Sabre/TrashRoot.php" , line : 63usize , placeholders : & [] , regex : "^Permission denied to rename this trashbin$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/files_trashbin/lib/Sabre/TrashRoot.php" , line : 67usize , placeholders : & [] , regex : "^Not allowed to create files in the trashbin$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/apps/files_trashbin/lib/Sabre/TrashRoot.php" , line : 71usize , placeholders : & [] , regex : "^Not allowed to create folders in the trashbin$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/mlocati/ip-lib/src/Range/AbstractRange.php" , line : 38usize , placeholders : & [] , regex : "^@todo$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/mlocati/ip-lib/src/Range/Pattern.php" , line : 184usize , placeholders : & [] , regex : "^@todo$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/mtdowling/jmespath.php/src/TreeInterpreter.php" , line : 214usize , placeholders : & ["$node['type']"] , regex : "^Unknown node type: (.*)$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/mtdowling/jmespath.php/src/TreeInterpreter.php" , line : 232usize , placeholders : & ["$cmp"] , regex : "^Invalid comparison: (.*)$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/mtdowling/jmespath.php/src/Utils.php" , line : 196usize , placeholders : & [] , regex : "^Expects string or array$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/mtdowling/jmespath.php/src/Utils.php" , line : 221usize , placeholders : & [] , regex : "^step cannot be 0$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/mtdowling/jmespath.php/src/FnDispatcher.php" , line : 88usize , placeholders : & [] , regex : "^not_null\\(\\) expects 1 or more arguments, 0 were provided$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/mtdowling/jmespath.php/src/FnDispatcher.php" , line : 167usize , placeholders : & [] , regex : "^Cannot reverse provided argument$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/mtdowling/jmespath.php/src/FnDispatcher.php" , line : 259usize , placeholders : & [] , regex : "^merge\\(\\) expects 1 or more arguments, 0 were provided$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/mtdowling/jmespath.php/src/FnDispatcher.php" , line : 405usize , placeholders : & ["$name"] , regex : "^Call to undefined function (.*)$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/mtdowling/jmespath.php/src/Parser.php" , line : 517usize , placeholders : & ["$method"] , regex : "^Call to undefined method (.*)$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/mtdowling/jmespath.php/src/CompilerRuntime.php" , line : 33usize , placeholders : & ["$dir"] , regex : "^Unable to create cache directory: (.*)$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/spomky-labs/cbor-php/src/LengthCalculator.php" , line : 59usize , placeholders : & [] , regex : "^Unable to convert the data$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/spomky-labs/cbor-php/src/StringStream.php" , line : 30usize , placeholders : & [] , regex : "^Unable to open the memory$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/spomky-labs/cbor-php/src/StringStream.php" , line : 34usize , placeholders : & [] , regex : "^Unable to write the memory$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/spomky-labs/cbor-php/src/StringStream.php" , line : 38usize , placeholders : & [] , regex : "^Unable to rewind the memory$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/spomky-labs/cbor-php/src/StringStream.php" , line : 50usize , placeholders : & [] , regex : "^Unable to read the memory$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/spomky-labs/cbor-php/src/Utils.php" , line : 59usize , placeholders : & [] , regex : "^Invalid data provided$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/spomky-labs/cbor-php/src/Decoder.php" , line : 117usize , placeholders : & [] , regex : "^Unable to parse the data\\. Infinite Byte String object can only get Byte String objects\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/spomky-labs/cbor-php/src/Decoder.php" , line : 127usize , placeholders : & [] , regex : "^Unable to parse the data\\. Infinite Text String object can only get Text String objects\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/spomky-labs/cbor-php/src/Decoder.php" , line : 149usize , placeholders : & [] , regex : "^Cannot parse the data\\. No enclosing indefinite\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/spomky-labs/cbor-php/src/TextStringWithChunkObject.php" , line : 41usize , placeholders : & [] , regex : "^Unable to convert the data$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/spomky-labs/cbor-php/src/Tag/Base16EncodingTag.php" , line : 39usize , placeholders : & [] , regex : "^This tag only accepts Byte String, Infinite Byte String, Text String or Infinite Text String objects\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/spomky-labs/cbor-php/src/Tag/BigFloatTag.php" , line : 31usize , placeholders : & [] , regex : "^The extension \"bcmath\" is required to use this tag$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/spomky-labs/cbor-php/src/Tag/BigFloatTag.php" , line : 49usize , placeholders : & [] , regex : "^This tag only accepts a ListObject object that contains an exponent and a mantissa\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/spomky-labs/cbor-php/src/Tag/BigFloatTag.php" , line : 53usize , placeholders : & [] , regex : "^The exponent must be a Signed Integer or an Unsigned Integer object\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/spomky-labs/cbor-php/src/Tag/BigFloatTag.php" , line : 57usize , placeholders : & [] , regex : "^The mantissa must be a Positive or Negative Signed Integer or an Unsigned Integer object\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/spomky-labs/cbor-php/src/Tag/TagObjectManager.php" , line : 32usize , placeholders : & [] , regex : "^Invalid tag ID\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/spomky-labs/cbor-php/src/Tag/Base64UrlEncodingTag.php" , line : 40usize , placeholders : & [] , regex : "^This tag only accepts Byte String, Infinite Byte String, Text String or Infinite Text String objects\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/spomky-labs/cbor-php/src/Tag/Base64EncodingTag.php" , line : 39usize , placeholders : & [] , regex : "^This tag only accepts Byte String, Infinite Byte String, Text String or Infinite Text String objects\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/spomky-labs/cbor-php/src/Tag/Base64EncodingTag.php" , line : 57usize , placeholders : & [] , regex : "^Unable to decode the data$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/spomky-labs/cbor-php/src/Tag/PositiveBigIntegerTag.php" , line : 37usize , placeholders : & [] , regex : "^This tag only accepts a Byte String object\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/spomky-labs/cbor-php/src/Tag/DecimalFractionTag.php" , line : 31usize , placeholders : & [] , regex : "^The extension \"bcmath\" is required to use this tag$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/spomky-labs/cbor-php/src/Tag/DecimalFractionTag.php" , line : 34usize , placeholders : & [] , regex : "^This tag only accepts a ListObject object that contains an exponent and a mantissa\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/spomky-labs/cbor-php/src/Tag/DecimalFractionTag.php" , line : 38usize , placeholders : & [] , regex : "^The exponent must be a Signed Integer or an Unsigned Integer object\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/spomky-labs/cbor-php/src/Tag/DecimalFractionTag.php" , line : 42usize , placeholders : & [] , regex : "^The mantissa must be a Positive or Negative Signed Integer or an Unsigned Integer object\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/spomky-labs/cbor-php/src/Tag/NegativeBigIntegerTag.php" , line : 37usize , placeholders : & [] , regex : "^This tag only accepts a Byte String object\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/spomky-labs/cbor-php/src/Tag/TimestampTag.php" , line : 41usize , placeholders : & [] , regex : "^This tag only accepts a Byte String object\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/spomky-labs/cbor-php/src/InfiniteListObject.php" , line : 46usize , placeholders : & [] , regex : "^Unable to convert the data$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/spomky-labs/cbor-php/src/ListObject.php" , line : 46usize , placeholders : & [] , regex : "^The list must contain only CBORObject objects\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/spomky-labs/cbor-php/src/ListObject.php" , line : 77usize , placeholders : & [] , regex : "^Index not found\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/spomky-labs/cbor-php/src/SignedIntegerObject.php" , line : 81usize , placeholders : & [] , regex : "^The value must be a negative integer\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/spomky-labs/cbor-php/src/SignedIntegerObject.php" , line : 105usize , placeholders : & [] , regex : "^Out of range\\. Please use NegativeBigIntegerTag tag with ByteStringObject object instead\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/spomky-labs/cbor-php/src/SignedIntegerObject.php" , line : 115usize , placeholders : & [] , regex : "^Unable to convert the data$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/spomky-labs/cbor-php/src/ByteStringWithChunkObject.php" , line : 41usize , placeholders : & [] , regex : "^Unable to convert the data$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/spomky-labs/cbor-php/src/OtherObject/SimpleObject.php" , line : 53usize , placeholders : & [] , regex : "^The value is not a valid simple value$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/spomky-labs/cbor-php/src/OtherObject/OtherObjectManager.php" , line : 31usize , placeholders : & [] , regex : "^Invalid additional information\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/spomky-labs/cbor-php/src/OtherObject/SinglePrecisionFloatObject.php" , line : 39usize , placeholders : & [] , regex : "^The value is not a valid single precision floating point$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/spomky-labs/cbor-php/src/OtherObject/DoublePrecisionFloatObject.php" , line : 39usize , placeholders : & [] , regex : "^The value is not a valid double precision floating point$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/spomky-labs/cbor-php/src/OtherObject/HalfPrecisionFloatObject.php" , line : 39usize , placeholders : & [] , regex : "^The value is not a valid half precision floating point$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/spomky-labs/cbor-php/src/InfiniteMapObject.php" , line : 47usize , placeholders : & [] , regex : "^Unable to convert the data$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/spomky-labs/cbor-php/src/MapObject.php" , line : 45usize , placeholders : & [] , regex : "^The list must contain only MapItem objects\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/spomky-labs/cbor-php/src/UnsignedIntegerObject.php" , line : 97usize , placeholders : & [] , regex : "^The value must be a positive integer\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/spomky-labs/cbor-php/src/UnsignedIntegerObject.php" , line : 118usize , placeholders : & [] , regex : "^Out of range\\. Please use PositiveBigIntegerTag tag with ByteStringObject object instead\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/spomky-labs/cbor-php/src/UnsignedIntegerObject.php" , line : 128usize , placeholders : & [] , regex : "^Unable to convert the data$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/spomky-labs/base64url/src/Base64Url.php" , line : 51usize , placeholders : & [] , regex : "^Invalid data provided$" , }
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
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/icewind/streams/src/UrlCallback.php" , line : 133usize , placeholders : & [] , regex : "^stat is not supported due to php bug 50526$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/icewind/streams/src/Wrapper.php" , line : 40usize , placeholders : & [] , regex : "^Invalid context, source not set$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/icewind/streams/src/IteratorDirectory.php" , line : 48usize , placeholders : & [] , regex : "^Invalid context, iterator or array not set$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/icewind/streams/src/IteratorDirectory.php" , line : 109usize , placeholders : & [] , regex : "^\\$source should be an Iterator or array$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/icewind/streams/src/WrapperHandler.php" , line : 87usize , placeholders : & [] , regex : "^Invalid stream source$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/icewind/streams/src/CountWrapper.php" , line : 64usize , placeholders : & [] , regex : "^Invalid or missing callback$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/aws/aws-sdk-php/src/Sdk.php" , line : 582usize , placeholders : & ["$name"] , regex : "^Unknown method: (.*)\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/aws/aws-sdk-php/src/Glacier/GlacierClient.php" , line : 142usize , placeholders : & [] , regex : "^sha256$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/aws/aws-sdk-php/src/Multipart/AbstractUploader.php" , line : 137usize , placeholders : & [] , regex : "^Source stream must be readable\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/aws/aws-sdk-php/src/S3/BatchDelete.php" , line : 154usize , placeholders : & [] , regex : "^before must be callable$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/aws/aws-sdk-php/src/S3/BatchDelete.php" , line : 161usize , placeholders : & [] , regex : "^batch_size is not > 0$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/aws/aws-sdk-php/src/S3/RegionalEndpoint/Configuration.php" , line : 13usize , placeholders : & [] , regex : "^Configuration parameter must either be 'legacy' or 'regional'\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/aws/aws-sdk-php/src/S3/Transfer.php" , line : 116usize , placeholders : & [] , regex : "^mup_threshold must be >= 5MB$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/aws/aws-sdk-php/src/S3/Transfer.php" , line : 123usize , placeholders : & [] , regex : "^before must be a callable\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/aws/aws-sdk-php/src/S3/Transfer.php" , line : 170usize , placeholders : & [] , regex : "^Scheme must be \"s3\" or \"file\"\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/aws/aws-sdk-php/src/S3/Transfer.php" , line : 275usize , placeholders : & ["$dir"] , regex : "^Could not create dir: (.*)$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/aws/aws-sdk-php/src/S3/Transfer.php" , line : 420usize , placeholders : & ["$operation"] , regex : "^Transfer encountered an unexpected operation: (.*)\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/aws/aws-sdk-php/src/S3/StreamWrapper.php" , line : 305usize , placeholders : & ["$path"] , regex : "^File or directory not found: (.*)$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/aws/aws-sdk-php/src/S3/StreamWrapper.php" , line : 655usize , placeholders : & [] , regex : "^No client in stream context$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/aws/aws-sdk-php/src/S3/S3EndpointMiddleware.php" , line : 298usize , placeholders : & [] , regex : "^Outposts \\+ dualstack is not supported$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/aws/aws-sdk-php/src/S3/S3EndpointMiddleware.php" , line : 305usize , placeholders : & [] , regex : "^Custom Endpoint \\+ Dualstack not supported$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/aws/aws-sdk-php/src/Crypto/AesEncryptingStream.php" , line : 116usize , placeholders : & [] , regex : "^Unrecognized whence\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/aws/aws-sdk-php/src/Crypto/KmsMaterialsProvider.php" , line : 40usize , placeholders : & [] , regex : "^Not able to detect the materials description\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/aws/aws-sdk-php/src/Crypto/Polyfill/ByteArray.php" , line : 52usize , placeholders : & [] , regex : "^Argument must be an integer, string, or array of integers\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/aws/aws-sdk-php/src/Crypto/Cipher/Cbc.php" , line : 50usize , placeholders : & [] , regex : "^Invalid initialization vector$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/aws/aws-sdk-php/src/functions.php" , line : 337usize , placeholders : & [] , regex : "^Unable to determine what Guzzle version is installed\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/aws/aws-sdk-php/src/functions.php" , line : 366usize , placeholders : & [] , regex : "^Calling handler did not serialize request$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/aws/aws-sdk-php/src/functions.php" , line : 416usize , placeholders : & ["$service"] , regex : "^The service \"(.*)\" is not provided by the AWS SDK for PHP\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/aws/aws-sdk-php/src/Credentials/AssumeRoleWithWebIdentityCredentialProvider.php" , line : 63usize , placeholders : & [] , regex : "^'WebIdentityTokenFile' must be an absolute path\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/aws/aws-sdk-php/src/Credentials/AssumeRoleWithWebIdentityCredentialProvider.php" , line : 110usize , placeholders : & ["$this->tokenFile"] , regex : "^Unreadable tokenfile at location (.*)$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/aws/aws-sdk-php/src/Credentials/AssumeRoleWithWebIdentityCredentialProvider.php" , line : 121usize , placeholders : & ["$this->tokenFile"] , regex : "^InvalidIdentityToken from file: (.*)$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/aws/aws-sdk-php/src/Credentials/AssumeRoleWithWebIdentityCredentialProvider.php" , line : 145usize , placeholders : & [] , regex : "^InvalidIdentityToken, retries exhausted$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/aws/aws-sdk-php/src/Credentials/AssumeRoleWithWebIdentityCredentialProvider.php" , line : 150usize , placeholders : & [] , regex : "^Error assuming role from web identity credentials$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/aws/aws-sdk-php/src/Credentials/EcsCredentialProvider.php" , line : 75usize , placeholders : & ["$msg"] , regex : "^Error retrieving credential from ECS \\((.*)\\)$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/aws/aws-sdk-php/src/Credentials/EcsCredentialProvider.php" , line : 101usize , placeholders : & [] , regex : "^Unexpected ECS credential value$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/aws/aws-sdk-php/src/Credentials/CredentialProvider.php" , line : 176usize , placeholders : & [] , regex : "^No providers in chain$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/aws/aws-sdk-php/src/Credentials/AssumeRoleCredentialProvider.php" , line : 58usize , placeholders : & [] , regex : "^Error in retrieving assume role credentials\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/aws/aws-sdk-php/src/Signature/SignatureV4.php" , line : 229usize , placeholders : & [] , regex : "^sha256$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/aws/aws-sdk-php/src/Signature/SignatureV4.php" , line : 235usize , placeholders : & [] , regex : "^sha256$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/aws/aws-sdk-php/src/EndpointDiscovery/Configuration.php" , line : 14usize , placeholders : & [] , regex : "^'cache_limit' value must be a positive integer\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/aws/aws-sdk-php/src/History.php" , line : 43usize , placeholders : & [] , regex : "^No commands received$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/aws/aws-sdk-php/src/History.php" , line : 58usize , placeholders : & [] , regex : "^No requests received$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/aws/aws-sdk-php/src/History.php" , line : 73usize , placeholders : & [] , regex : "^No entries$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/aws/aws-sdk-php/src/History.php" , line : 86usize , placeholders : & [] , regex : "^No return value for last entry\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/aws/aws-sdk-php/src/History.php" , line : 119usize , placeholders : & [] , regex : "^Invalid history ticket$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/aws/aws-sdk-php/src/History.php" , line : 125usize , placeholders : & [] , regex : "^History entry is already finished$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/aws/aws-sdk-php/src/Endpoint/Partition.php" , line : 64usize , placeholders : & ["$key"] , regex : "^Partition missing required (.*) field$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/aws/aws-sdk-php/src/AwsClient.php" , line : 260usize , placeholders : & ["$name"] , regex : "^Operation not found: (.*)$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/aws/aws-sdk-php/src/DynamoDb/Marshaler.php" , line : 100usize , placeholders : & [] , regex : "^The JSON document must be valid and be an object at its root\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/aws/aws-sdk-php/src/DynamoDb/Marshaler.php" , line : 298usize , placeholders : & ["$type"] , regex : "^Unexpected type: (.*)\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/aws/aws-sdk-php/src/DynamoDb/Marshaler.php" , line : 318usize , placeholders : & ["$message"] , regex : "^Marshaling error: (.*)\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/aws/aws-sdk-php/src/DynamoDb/WriteRequestBatch.php" , line : 70usize , placeholders : & [] , regex : "^\"batch_size\" must be between 2 and 25\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/aws/aws-sdk-php/src/DynamoDb/WriteRequestBatch.php" , line : 75usize , placeholders : & [] , regex : "^\"before\" must be callable\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/aws/aws-sdk-php/src/DynamoDb/WriteRequestBatch.php" , line : 78usize , placeholders : & [] , regex : "^\"error\" must be callable\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/aws/aws-sdk-php/src/DynamoDb/WriteRequestBatch.php" , line : 261usize , placeholders : & [] , regex : "^There was no table specified\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/aws/aws-sdk-php/src/MockHandler.php" , line : 57usize , placeholders : & [] , regex : "^Expected an Aws\\\\ResultInterface or Exception\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/aws/aws-sdk-php/src/MockHandler.php" , line : 71usize , placeholders : & [] , regex : "^Expected an \\\\Exception or \\\\Throwable\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/aws/aws-sdk-php/src/CommandPool.php" , line : 124usize , placeholders : & [] , regex : "^before must be callable$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/aws/aws-sdk-php/src/EndpointParameterMiddleware.php" , line : 63usize , placeholders : & ["$parameter"] , regex : "^The parameter '(.*)' must be set and not empty\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/aws/aws-sdk-php/src/EndpointParameterMiddleware.php" , line : 81usize , placeholders : & ["$host"] , regex : "^The supplied parameters result in an invalid hostname: '(.*)'\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/aws/aws-sdk-php/src/WrappedHttpHandler.php" , line : 165usize , placeholders : & [] , regex : "^The HTTP handler was rejected without an \"exception\" key value pair\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/aws/aws-sdk-php/src/Sts/StsClient.php" , line : 74usize , placeholders : & [] , regex : "^Result contains no credentials$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/aws/aws-sdk-php/src/Sts/RegionalEndpoints/Configuration.php" , line : 13usize , placeholders : & [] , regex : "^Configuration parameter must either be 'legacy' or 'regional'\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/aws/aws-sdk-php/src/Waiter.php" , line : 78usize , placeholders : & [] , regex : "^The provided waiter configuration was incomplete\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/aws/aws-sdk-php/src/Waiter.php" , line : 84usize , placeholders : & [] , regex : "^The provided \"before\" callback is not callable\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/aws/aws-sdk-php/src/Api/MapShape.php" , line : 29usize , placeholders : & [] , regex : "^No value specified$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/aws/aws-sdk-php/src/Api/ApiProvider.php" , line : 214usize , placeholders : & ["$modelsDir"] , regex : "^The specified models directory, (.*), was not found\\.$" , }
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
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/aws/aws-sdk-php/src/Api/Parser/DecodingEventStreamIterator.php" , line : 297usize , placeholders : & [] , regex : "^Undefined variable length format\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/aws/aws-sdk-php/src/Api/Parser/DecodingEventStreamIterator.php" , line : 307usize , placeholders : & [] , regex : "^Undefined variable length format\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/aws/aws-sdk-php/src/Api/Parser/XmlParser.php" , line : 152usize , placeholders : & [] , regex : "^Invalid timestamp value passed to XmlParser::parse_timestamp$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/aws/aws-sdk-php/src/Api/Parser/EventParsingIterator.php" , line : 68usize , placeholders : & [] , regex : "^Failed to parse unknown message type\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/aws/aws-sdk-php/src/Api/Parser/EventParsingIterator.php" , line : 73usize , placeholders : & [] , regex : "^Failed to parse without event type\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/aws/aws-sdk-php/src/Api/Parser/Crc32ValidatingParser.php" , line : 32usize , placeholders : & ["$expected" , "$hash"] , regex : "^crc32 mismatch\\. Expected (.*), found (.*)\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/aws/aws-sdk-php/src/Api/Parser/PayloadParserTrait.php" , line : 50usize , placeholders : & ["$e->getMessage()"] , regex : "^Error parsing XML: (.*)$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/aws/aws-sdk-php/src/Api/DateTimeResult.php" , line : 37usize , placeholders : & [] , regex : "^Invalid timestamp value passed to DateTimeResult::fromISO8601$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/aws/aws-sdk-php/src/Api/DateTimeResult.php" , line : 57usize , placeholders : & [] , regex : "^Invalid timestamp value passed to DateTimeResult::fromTimestamp$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/aws/aws-sdk-php/src/Api/DateTimeResult.php" , line : 78usize , placeholders : & [] , regex : "^Invalid timestamp value passed to DateTimeResult::fromTimestamp$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/aws/aws-sdk-php/src/Api/DocModel.php" , line : 22usize , placeholders : & [] , regex : "^The \"tidy\" PHP extension is required\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/aws/aws-sdk-php/src/Api/Service.php" , line : 126usize , placeholders : & ["$protocol"] , regex : "^Unknown protocol: (.*)$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/aws/aws-sdk-php/src/Api/Service.php" , line : 276usize , placeholders : & ["$name"] , regex : "^Unknown operation: (.*)$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/aws/aws-sdk-php/src/Api/ListShape.php" , line : 25usize , placeholders : & [] , regex : "^No member attribute specified$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/aws/aws-sdk-php/src/AbstractConfigurationProvider.php" , line : 67usize , placeholders : & [] , regex : "^No providers in chain$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/aws/aws-sdk-php/src/Rds/AuthTokenGenerator.php" , line : 50usize , placeholders : & ["$lifetime"] , regex : "^Lifetime must be a positive number less than or equal to 15, was (.*)$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/aws/aws-sdk-php/src/ClientSideMonitoring/ApiCallMonitoringMiddleware.php" , line : 85usize , placeholders : & [] , regex : "^Parameter must be an instance of ResultInterface or Exception\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/aws/aws-sdk-php/src/ClientSideMonitoring/ApiCallAttemptMonitoringMiddleware.php" , line : 122usize , placeholders : & [] , regex : "^Parameter must be an instance of ResultInterface, AwsException or Exception\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/aws/aws-sdk-php/src/ClientSideMonitoring/Configuration.php" , line : 25usize , placeholders : & [] , regex : "^CSM 'port' value must be an integer!$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/aws/aws-sdk-php/src/CloudFront/Signer.php" , line : 36usize , placeholders : & ["$privateKey"] , regex : "^PK file not found: (.*)$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/aws/aws-sdk-php/src/CloudFront/CookieSigner.php" , line : 62usize , placeholders : & [] , regex : "^Invalid or missing URI scheme$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/aws/aws-sdk-php/src/CloudFront/UrlSigner.php" , line : 54usize , placeholders : & ["$url"] , regex : "^Invalid URL: (.*)$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/aws/aws-sdk-php/src/CloudFront/CloudFrontClient.php" , line : 208usize , placeholders : & ["$required"] , regex : "^(.*) is required$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/aws/aws-sdk-php/src/CloudFront/CloudFrontClient.php" , line : 251usize , placeholders : & ["$required"] , regex : "^(.*) is required$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/aws/aws-sdk-php/src/InputValidationMiddleware.php" , line : 34usize , placeholders : & [] , regex : "^The mandatory attribute list must be an array of strings$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/aws/aws-sdk-php/src/InputValidationMiddleware.php" , line : 65usize , placeholders : & ["$commandName" , "$member"] , regex : "^The (.*) operation requires non\\-empty parameter: (.*)$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/aws/aws-sdk-php/src/HandlerList.php" , line : 294usize , placeholders : & [] , regex : "^No handler has been specified$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/aws/aws-sdk-php/src/HandlerList.php" , line : 328usize , placeholders : & ["$findName"] , regex : "^(.*) not found$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/beberlei/assert/lib/Assert/Assertion.php" , line : 2110usize , placeholders : & [] , regex : "^Missing the first argument\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/beberlei/assert/lib/Assert/Assertion.php" , line : 2124usize , placeholders : & [] , regex : "^Missing the first argument\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/beberlei/assert/lib/Assert/AssertionChain.php" , line : 237usize , placeholders : & [] , regex : "^Exception class name must be passed as a string$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/symfony/translation/Formatter/IntlFormatter.php" , line : 38usize , placeholders : & [] , regex : "^Cannot parse message translation: please install the \"intl\" PHP extension or the \"symfony/polyfill\\-intl\\-messageformatter\" package\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/symfony/translation/Command/XliffLintCommand.php" , line : 95usize , placeholders : & [] , regex : "^Please provide a filename or pipe file content to STDIN\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/symfony/translation/Dumper/FileDumper.php" , line : 57usize , placeholders : & [] , regex : "^The backup feature is no longer supported\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/symfony/translation/Dumper/FileDumper.php" , line : 67usize , placeholders : & [] , regex : "^The file dumper needs a path option\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/symfony/translation/Dumper/YamlFileDumper.php" , line : 39usize , placeholders : & [] , regex : "^Dumping translations in the YAML format requires the Symfony Yaml component\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Debug , path : "/3rdparty/symfony/translation/LoggingTranslator.php" , line : 147usize , placeholders : & [] , regex : "^Translation use fallback catalogue\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Warn , path : "/3rdparty/symfony/translation/LoggingTranslator.php" , line : 149usize , placeholders : & [] , regex : "^Translation not found\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/symfony/translation/Catalogue/AbstractOperation.php" , line : 67usize , placeholders : & [] , regex : "^Operated catalogues must belong to the same locale\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/symfony/translation/Loader/MoFileLoader.php" , line : 51usize , placeholders : & [] , regex : "^MO stream content has an invalid format\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/symfony/translation/Loader/MoFileLoader.php" , line : 61usize , placeholders : & [] , regex : "^MO stream content has an invalid format\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/symfony/translation/Loader/YamlFileLoader.php" , line : 36usize , placeholders : & [] , regex : "^Loading translations from the YAML format requires the Symfony Yaml component\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/symfony/polyfill-mbstring/Mbstring.php" , line : 557usize , placeholders : & [] , regex : "^Argument \\#2 \\(\\$length\\) must be greater than 0$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/symfony/polyfill-mbstring/Mbstring.php" , line : 610usize , placeholders : & [] , regex : "^Argument \\#1 \\(\\$substitute_character\\) must be \"none\", \"long\", \"entity\" or a valid codepoint$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/symfony/polyfill-intl-grapheme/Grapheme.php" , line : 74usize , placeholders : & [] , regex : "^grapheme_extract\\(\\): Argument \\#3 \\(\\$type\\) must be one of GRAPHEME_EXTR_COUNT, GRAPHEME_EXTR_MAXBYTES, or GRAPHEME_EXTR_MAXCHARS$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/symfony/polyfill-intl-normalizer/Normalizer.php" , line : 77usize , placeholders : & [] , regex : "^normalizer_normalize\\(\\): Argument \\#2 \\(\\$form\\) must be a a valid normalization form$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/symfony/console/Tester/TesterTrait.php" , line : 39usize , placeholders : & [] , regex : "^Output not initialized, did you execute the command before requesting the display\\?$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/symfony/console/Tester/TesterTrait.php" , line : 63usize , placeholders : & [] , regex : "^The error output is not available when the tester is run without \"capture_stderr_separately\" option set\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/symfony/console/Question/Question.php" , line : 86usize , placeholders : & [] , regex : "^A hidden question cannot use the autocompleter\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/symfony/console/Question/Question.php" , line : 156usize , placeholders : & [] , regex : "^Autocompleter values can be either an array, \"null\" or a \"Traversable\" object\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/symfony/console/Question/Question.php" , line : 180usize , placeholders : & [] , regex : "^A hidden question cannot use the autocompleter\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/symfony/console/Question/Question.php" , line : 226usize , placeholders : & [] , regex : "^Maximum number of attempts must be a positive value\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/symfony/console/Question/ChoiceQuestion.php" , line : 36usize , placeholders : & [] , regex : "^Choice question must have at least 1 choice available\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/symfony/console/Command/Command.php" , line : 161usize , placeholders : & [] , regex : "^You must override the execute\\(\\) method in the concrete command class\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/symfony/console/Command/Command.php" , line : 567usize , placeholders : & [] , regex : "^\\$aliases must be an array or an instance of \\\\Traversable\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/symfony/console/Command/LockableTrait.php" , line : 36usize , placeholders : & [] , regex : "^To enable the locking feature you must install the symfony/lock component\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/symfony/console/Command/LockableTrait.php" , line : 40usize , placeholders : & [] , regex : "^A lock is already in place\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/symfony/console/Style/SymfonyStyle.php" , line : 244usize , placeholders : & [] , regex : "^Value should be an array, string, or an instance of TableSeparator\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/symfony/console/Style/SymfonyStyle.php" , line : 422usize , placeholders : & [] , regex : "^The ProgressBar is not started\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/symfony/console/Formatter/OutputFormatterStyleStack.php" , line : 76usize , placeholders : & [] , regex : "^Incorrectly nested style tag found\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/symfony/console/Output/StreamOutput.php" , line : 45usize , placeholders : & [] , regex : "^The StreamOutput class needs a stream as its first argument\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/symfony/console/Input/InputDefinition.php" , line : 103usize , placeholders : & [] , regex : "^Cannot add an argument after an array argument\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/symfony/console/Input/InputDefinition.php" , line : 107usize , placeholders : & [] , regex : "^Cannot add a required argument after an optional one\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/symfony/console/Input/InputArgument.php" , line : 96usize , placeholders : & [] , regex : "^Cannot set a default value except for InputArgument::OPTIONAL mode\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/symfony/console/Input/InputArgument.php" , line : 103usize , placeholders : & [] , regex : "^A default value for an array argument must be an array\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/symfony/console/Input/InputOption.php" , line : 66usize , placeholders : & [] , regex : "^An option name cannot be empty\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/symfony/console/Input/InputOption.php" , line : 82usize , placeholders : & [] , regex : "^An option shortcut cannot be empty\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/symfony/console/Input/InputOption.php" , line : 98usize , placeholders : & [] , regex : "^Impossible to have an option mode VALUE_IS_ARRAY if the option does not accept a value\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/symfony/console/Input/InputOption.php" , line : 170usize , placeholders : & [] , regex : "^Cannot set a default value when using InputOption::VALUE_NONE mode\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/symfony/console/Input/InputOption.php" , line : 177usize , placeholders : & [] , regex : "^A default value for an array option must be an array\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/symfony/console/Helper/QuestionHelper.php" , line : 129usize , placeholders : & [] , regex : "^Aborted\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/symfony/console/Helper/QuestionHelper.php" , line : 265usize , placeholders : & [] , regex : "^Aborted\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/symfony/console/Helper/QuestionHelper.php" , line : 427usize , placeholders : & [] , regex : "^Unable to hide the response\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/symfony/console/Helper/QuestionHelper.php" , line : 437usize , placeholders : & [] , regex : "^Aborted\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/symfony/console/Helper/Table.php" , line : 278usize , placeholders : & [] , regex : "^A row must be an array or a TableSeparator instance\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/symfony/console/Helper/ProgressBar.php" , line : 507usize , placeholders : & [] , regex : "^Unable to display the remaining time if the maximum number of steps is not set\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/symfony/console/Helper/ProgressBar.php" , line : 520usize , placeholders : & [] , regex : "^Unable to display the estimated time if the maximum number of steps is not set\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/symfony/console/Helper/ProgressIndicator.php" , line : 56usize , placeholders : & [] , regex : "^Must have at least 2 indicator value characters\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/symfony/console/Helper/ProgressIndicator.php" , line : 85usize , placeholders : & [] , regex : "^Progress indicator already started\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/symfony/console/Helper/ProgressIndicator.php" , line : 103usize , placeholders : & [] , regex : "^Progress indicator has not yet been started\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/symfony/console/Helper/ProgressIndicator.php" , line : 130usize , placeholders : & [] , regex : "^Progress indicator has not yet been started\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/symfony/console/Helper/TableStyle.php" , line : 61usize , placeholders : & [] , regex : "^The padding char must not be empty\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/symfony/console/Helper/TableStyle.php" , line : 417usize , placeholders : & [] , regex : "^Invalid padding type\\. Expected one of \\(STR_PAD_LEFT, STR_PAD_RIGHT, STR_PAD_BOTH\\)\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/symfony/console/Helper/ProcessHelper.php" , line : 42usize , placeholders : & [] , regex : "^The ProcessHelper cannot be run as the Process component is not installed\\. Try running \"compose require symfony/process\"\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Critical , path : "/3rdparty/symfony/console/EventListener/ErrorListener.php" , line : 43usize , placeholders : & [] , regex : "^An error occurred while using the console\\. Message: \"\\{message\\}\"$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Critical , path : "/3rdparty/symfony/console/EventListener/ErrorListener.php" , line : 48usize , placeholders : & [] , regex : "^Error thrown while running command \"\\{command\\}\"\\. Message: \"\\{message\\}\"$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Debug , path : "/3rdparty/symfony/console/EventListener/ErrorListener.php" , line : 64usize , placeholders : & [] , regex : "^The console exited with code \"\\{code\\}\"$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Debug , path : "/3rdparty/symfony/console/EventListener/ErrorListener.php" , line : 69usize , placeholders : & [] , regex : "^Command \"\\{command\\}\" exited with code \"\\{code\\}\"$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/symfony/process/PhpProcess.php" , line : 79usize , placeholders : & [] , regex : "^Unable to find the PHP executable\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/symfony/process/Exception/ProcessFailedException.php" , line : 28usize , placeholders : & [] , regex : "^Expected a failed process, but the given process was successful\\.$" , }
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
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/symfony/routing/Matcher/Dumper/CompiledUrlMatcherDumper.php" , line : 450usize , placeholders : & [] , regex : "^Unable to use expressions as the Symfony ExpressionLanguage component is not installed\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/symfony/routing/Matcher/Dumper/CompiledUrlMatcherDumper.php" , line : 473usize , placeholders : & [] , regex : "^Symfony\\\\Component\\\\Routing\\\\Route cannot contain objects\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/symfony/routing/Matcher/UrlMatcher.php" , line : 266usize , placeholders : & [] , regex : "^Unable to use expressions as the Symfony ExpressionLanguage component is not installed\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/symfony/routing/RouteCollectionBuilder.php" , line : 355usize , placeholders : & [] , regex : "^Cannot import other routing resources: you must pass a LoaderInterface when constructing RouteCollectionBuilder\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/symfony/routing/Loader/XmlFileLoader.php" , line : 177usize , placeholders : & [] , regex : "^You cannot use both the attribute \"exclude\" and <exclude> tags at the same time\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/symfony/routing/Loader/AnnotationFileLoader.php" , line : 32usize , placeholders : & [] , regex : "^The Tokenizer extension is required for the routing annotation loaders\\.$" , }
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
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/christophwurst/id3parser/src/getID3/getid3_lib.php" , line : 278usize , placeholders : & [] , regex : "^ERROR: self::BigEndian2String\\(\\) does not support negative numbers$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/christophwurst/id3parser/src/getID3/getid3.php" , line : 94usize , placeholders : & [] , regex : "^Remote files are not supported \\- please copy the file locally first$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/punic/punic/code/Plural.php" , line : 88usize , placeholders : & [] , regex : "^Bad formula!$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/giggsey/locale/src/Locale.php" , line : 134usize , placeholders : & [] , regex : "^Locale is not supported$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/giggsey/libphonenumber-for-php/src/PhoneNumberToTimeZonesMapper.php" , line : 42usize , placeholders : & [] , regex : "^Mapping file can not be found$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/giggsey/libphonenumber-for-php/src/PhoneNumberMatch.php" , line : 36usize , placeholders : & [] , regex : "^Start index must be >= 0\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/giggsey/libphonenumber-for-php/src/prefixmapper/PrefixFileReader.php" , line : 39usize , placeholders : & ["$mapPath"] , regex : "^Invalid data directory: (.*)$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/giggsey/libphonenumber-for-php/src/prefixmapper/PrefixFileReader.php" , line : 73usize , placeholders : & [] , regex : "^Data does not exist$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/giggsey/libphonenumber-for-php/src/Leniency/AbstractLeniency.php" , line : 44usize , placeholders : & [] , regex : "^\\$level should be defined$" , }
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
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/sabre/dav/lib/CalDAV/SharingPlugin.php" , line : 76usize , placeholders : & [] , regex : "^The generic \"sharing\" plugin must be loaded before the caldav sharing plugin\\. Call \\$server\\->addPlugin\\(new \\\\Sabre\\\\DAV\\\\Sharing\\\\Plugin\\(\\)\\); before this one\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/sabre/dav/lib/CalDAV/ICSExportPlugin.php" , line : 114usize , placeholders : & [] , regex : "^The start= parameter must contain a unix timestamp$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/sabre/dav/lib/CalDAV/ICSExportPlugin.php" , line : 120usize , placeholders : & [] , regex : "^The end= parameter must contain a unix timestamp$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/sabre/dav/lib/CalDAV/ICSExportPlugin.php" , line : 126usize , placeholders : & [] , regex : "^If you'd like to expand recurrences, you must specify both a start= and end= parameter\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/sabre/dav/lib/CalDAV/Xml/Property/SupportedCalendarComponentSet.php" , line : 113usize , placeholders : & [] , regex : "^supported\\-calendar\\-component\\-set must have at least one CALDAV:comp element$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/sabre/dav/lib/CalDAV/Xml/Property/ScheduleCalendarTransp.php" , line : 49usize , placeholders : & [] , regex : "^The value must either be specified as \"transparent\" or \"opaque\"$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/sabre/dav/lib/CalDAV/Xml/Filter/PropFilter.php" , line : 79usize , placeholders : & [] , regex : "^The end\\-date must be larger than the start\\-date$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/sabre/dav/lib/CalDAV/Xml/Filter/CalendarData.php" , line : 69usize , placeholders : & [] , regex : "^The \"start\" and \"end\" attributes are required when expanding calendar\\-data$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/sabre/dav/lib/CalDAV/Xml/Filter/CalendarData.php" , line : 72usize , placeholders : & [] , regex : "^The end\\-date must be larger than the start\\-date when expanding calendar\\-data$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/sabre/dav/lib/CalDAV/Xml/Filter/CompFilter.php" , line : 78usize , placeholders : & [] , regex : "^You cannot add time\\-range filters on the VCALENDAR component$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/sabre/dav/lib/CalDAV/Xml/Filter/CompFilter.php" , line : 85usize , placeholders : & [] , regex : "^The end\\-date must be larger than the start\\-date$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/sabre/dav/lib/CalDAV/Xml/Request/InviteReply.php" , line : 140usize , placeholders : & [] , regex : "^The \\{http://calendarserver\\.org/ns/\\}hosturl/\\{DAV:\\}href element must exist$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/sabre/dav/lib/CalDAV/Xml/Request/CalendarQueryReport.php" , line : 117usize , placeholders : & [] , regex : "^Only one top\\-level comp\\-filter may be defined$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/sabre/dav/lib/CalDAV/Xml/Request/FreeBusyQueryReport.php" , line : 76usize , placeholders : & [] , regex : "^The freebusy report must have a time\\-range element$" , }
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
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/sabre/dav/lib/CalDAV/Principal/User.php" , line : 45usize , placeholders : & [] , regex : "^Permission denied to create directory$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/sabre/dav/lib/CalDAV/Principal/ProxyRead.php" , line : 73usize , placeholders : & [] , regex : "^Permission denied to delete node$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/sabre/dav/lib/CalDAV/Principal/ProxyRead.php" , line : 85usize , placeholders : & [] , regex : "^Permission denied to rename file$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/sabre/dav/lib/CalDAV/Principal/ProxyWrite.php" , line : 73usize , placeholders : & [] , regex : "^Permission denied to delete node$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/sabre/dav/lib/CalDAV/Principal/ProxyWrite.php" , line : 85usize , placeholders : & [] , regex : "^Permission denied to rename file$" , }
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
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/sabre/dav/lib/CalDAV/Schedule/SchedulingObject.php" , line : 38usize , placeholders : & [] , regex : "^The objectData argument must contain an 'uri' property$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/sabre/dav/lib/CalDAV/Schedule/SchedulingObject.php" , line : 67usize , placeholders : & [] , regex : "^Updating scheduling objects is not supported$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/sabre/dav/lib/CalDAV/CalendarObject.php" , line : 58usize , placeholders : & [] , regex : "^The objectData argument must contain an 'uri' property$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/sabre/dav/lib/CalDAV/Calendar.php" , line : 108usize , placeholders : & [] , regex : "^Calendar object not found$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/sabre/dav/lib/CalDAV/Calendar.php" , line : 180usize , placeholders : & [] , regex : "^Creating collections in calendar objects is not allowed$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/sabre/dav/lib/CalDAV/Calendar.php" , line : 218usize , placeholders : & [] , regex : "^Renaming calendars is not yet supported$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/sabre/dav/lib/CalDAV/Plugin.php" , line : 591usize , placeholders : & [] , regex : "^A calendar\\-query REPORT on a calendar with a Depth: 0 is undefined\\. Set Depth to 1$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/sabre/dav/lib/CalDAV/Plugin.php" , line : 648usize , placeholders : & [] , regex : "^The free\\-busy\\-query REPORT is only implemented on calendars$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/sabre/dav/lib/CalDAV/Plugin.php" , line : 810usize , placeholders : & [] , regex : "^This collection can only support iCalendar objects\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/sabre/dav/lib/DAVACL/AbstractPrincipalCollection.php" , line : 93usize , placeholders : & [] , regex : "^Listing members of this collection is disabled$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/sabre/dav/lib/DAVACL/PrincipalCollection.php" , line : 65usize , placeholders : & [] , regex : "^Only resources of type \\{DAV:\\}principal may be created here$" , }
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
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/sabre/dav/lib/DAVACL/PrincipalBackend/PDO.php" , line : 355usize , placeholders : & [] , regex : "^Principal not found$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/sabre/dav/lib/DAVACL/PrincipalBackend/PDO.php" , line : 379usize , placeholders : & [] , regex : "^Principal not found$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/sabre/dav/lib/DAVACL/PrincipalBackend/PDO.php" , line : 416usize , placeholders : & [] , regex : "^Principal not found$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/sabre/dav/lib/DAVACL/FS/Collection.php" , line : 69usize , placeholders : & [] , regex : "^File could not be located$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/sabre/dav/lib/DAVACL/FS/Collection.php" , line : 72usize , placeholders : & [] , regex : "^Permission denied to \\. and \\.\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/sabre/dav/lib/DAVACL/Principal.php" , line : 49usize , placeholders : & [] , regex : "^The principal properties must at least contain the 'uri' key$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/sabre/dav/lib/DAV/FS/Node.php" , line : 75usize , placeholders : & [] , regex : "^This node cannot be renamed$" , }
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
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/sabre/dav/lib/DAV/Client.php" , line : 119usize , placeholders : & [] , regex : "^A baseUri must be provided$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/sabre/dav/lib/DAV/Browser/Plugin.php" , line : 553usize , placeholders : & [] , regex : "^Path does not exist, or escaping from the base path was detected$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/sabre/dav/lib/DAV/Browser/Plugin.php" , line : 559usize , placeholders : & [] , regex : "^Path does not exist, or escaping from the base path was detected$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/sabre/dav/lib/DAV/TemporaryFileFilterPlugin.php" , line : 220usize , placeholders : & [] , regex : "^The resource already exists, and an If\\-None\\-Match header was supplied$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/sabre/dav/lib/DAV/Xml/Element/Sharee.php" , line : 172usize , placeholders : & [] , regex : "^Every \\{DAV:\\}sharee must have a \\{DAV:\\}href child\\-element$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/sabre/dav/lib/DAV/Xml/Element/Sharee.php" , line : 183usize , placeholders : & [] , regex : "^Every \\{DAV:\\}sharee must have a \\{DAV:\\}share\\-access child element$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/sabre/dav/lib/DAV/Xml/Property/ShareAccess.php" , line : 133usize , placeholders : & [] , regex : "^Invalid value for \\{DAV:\\}share\\-access element$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/sabre/dav/lib/DAV/Xml/Property/SupportedReportSet.php" , line : 66usize , placeholders : & [] , regex : "^Reportname must be in clark\\-notation$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/sabre/dav/lib/DAV/PartialUpdate/Plugin.php" , line : 115usize , placeholders : & [] , regex : "^The target resource does not support the PATCH method\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/sabre/dav/lib/DAV/PartialUpdate/Plugin.php" , line : 121usize , placeholders : & [] , regex : "^No valid \"X\\-Update\\-Range\" found in the headers$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/sabre/dav/lib/DAV/PartialUpdate/Plugin.php" , line : 134usize , placeholders : & [] , regex : "^A Content\\-Length header is required$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/sabre/dav/lib/DAV/Sync/Plugin.php" , line : 105usize , placeholders : & [] , regex : "^The \\{DAV:\\}sync\\-collection REPORT is not supported on this url\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/sabre/dav/lib/DAV/Sync/Plugin.php" , line : 109usize , placeholders : & [] , regex : "^No sync information is available at this node$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/sabre/dav/lib/DAV/Sync/Plugin.php" , line : 116usize , placeholders : & [] , regex : "^Invalid or unknown sync token$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/sabre/dav/lib/DAV/Sync/Plugin.php" , line : 124usize , placeholders : & [] , regex : "^Invalid or unknown sync token$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/sabre/dav/lib/DAV/PropPatch.php" , line : 270usize , placeholders : & [] , regex : "^A callback sent to handle\\(\\) did not return an int or a bool$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/sabre/dav/lib/DAV/PropPatch.php" , line : 314usize , placeholders : & [] , regex : "^A callback sent to handle\\(\\) did not return an array or a bool$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/sabre/dav/lib/DAV/SimpleCollection.php" , line : 53usize , placeholders : & [] , regex : "^Children must be specified as strings, arrays or instances of Sabre\\\\DAV\\\\INode$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/sabre/dav/lib/DAV/Node.php" , line : 37usize , placeholders : & [] , regex : "^Permission denied to delete node$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/sabre/dav/lib/DAV/Node.php" , line : 49usize , placeholders : & [] , regex : "^Permission denied to rename file$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/sabre/dav/lib/DAV/Collection.php" , line : 104usize , placeholders : & [] , regex : "^Permission denied to create directory$" , }
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
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/sabre/dav/lib/DAV/Sharing/Plugin.php" , line : 115usize , placeholders : & [] , regex : "^Sharing is not allowed on this node$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/sabre/dav/lib/DAV/Sharing/Plugin.php" , line : 285usize , placeholders : & [] , regex : "^The \"href\" POST parameter is required$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/sabre/dav/lib/DAV/Sharing/Plugin.php" , line : 288usize , placeholders : & [] , regex : "^The \"access\" POST parameter is required$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/sabre/dav/lib/DAV/Sharing/Plugin.php" , line : 298usize , placeholders : & [] , regex : "^The \"access\" POST must be readwrite, read or no\\-access$" , }
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
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/sabre/dav/lib/CardDAV/Xml/Request/AddressBookQueryReport.php" , line : 155usize , placeholders : & [] , regex : "^The \"test\" attribute must be one of \"allof\" or \"anyof\"$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/sabre/dav/lib/CardDAV/AddressBookHome.php" , line : 100usize , placeholders : & [] , regex : "^Creating new files in this collection is not supported$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/sabre/dav/lib/CardDAV/AddressBookHome.php" , line : 112usize , placeholders : & [] , regex : "^Creating new collections in this collection is not supported$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/sabre/dav/lib/CardDAV/AddressBookHome.php" , line : 160usize , placeholders : & [] , regex : "^Unknown resourceType for this collection$" , }
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
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/sabre/event/lib/Promise.php" , line : 137usize , placeholders : & [] , regex : "^This promise is already resolved, and you're not allowed to resolve a promise more than once$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/sabre/event/lib/Promise.php" , line : 152usize , placeholders : & [] , regex : "^This promise is already resolved, and you're not allowed to resolve a promise more than once$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/sabre/event/lib/Promise.php" , line : 180usize , placeholders : & [] , regex : "^There were no more events in the loop\\. This promise will never be fulfilled\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/sabre/event/lib/coroutine.php" , line : 59usize , placeholders : & [] , regex : "^You must pass a generator function$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/sabre/http/lib/Sapi.php" , line : 223usize , placeholders : & [] , regex : "^The _SERVER array must have a REQUEST_URI key$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/sabre/http/lib/Sapi.php" , line : 227usize , placeholders : & [] , regex : "^The _SERVER array must have a REQUEST_METHOD key$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/sabre/http/lib/Response.php" , line : 163usize , placeholders : & [] , regex : "^The HTTP status code must be exactly 3 digits$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/sabre/uri/lib/functions.php" , line : 354usize , placeholders : & [] , regex : "^Invalid, or could not parse URI$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/sabre/vobject/lib/Splitter/ICalendar.php" , line : 51usize , placeholders : & [] , regex : "^Supplied input could not be parsed as VCALENDAR\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/sabre/vobject/lib/Splitter/VCard.php" , line : 66usize , placeholders : & [] , regex : "^The supplied input contained non\\-VCARD data\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/sabre/vobject/lib/FreeBusyGenerator.php" , line : 148usize , placeholders : & [] , regex : "^You can only pass strings or \\\\Sabre\\\\VObject\\\\Component arguments to setObjects$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/sabre/vobject/lib/VCardConverter.php" , line : 39usize , placeholders : & [] , regex : "^Only vCard 2\\.1, 3\\.0 and 4\\.0 are supported for the input data$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/sabre/vobject/lib/VCardConverter.php" , line : 42usize , placeholders : & [] , regex : "^You can only use vCard 3\\.0 or 4\\.0 for the target version$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/sabre/vobject/lib/ITip/Broker.php" , line : 833usize , placeholders : & [] , regex : "^If a calendar contained more than one event, they must have the same UID\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/sabre/vobject/lib/ITip/Broker.php" , line : 838usize , placeholders : & [] , regex : "^An event MUST have a DTSTART property\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/sabre/vobject/lib/ITip/Broker.php" , line : 847usize , placeholders : & [] , regex : "^Every instance of the event must have the same organizer\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/sabre/vobject/lib/Parser/MimeDir.php" , line : 135usize , placeholders : & [] , regex : "^This parser can only read from strings or streams\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/sabre/vobject/lib/Parser/MimeDir.php" , line : 163usize , placeholders : & [] , regex : "^This parser only supports VCARD and VCALENDAR files$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/sabre/vobject/lib/Parser/MimeDir.php" , line : 284usize , placeholders : & [] , regex : "^End of document reached prematurely$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/sabre/vobject/lib/Parser/MimeDir.php" , line : 287usize , placeholders : & [] , regex : "^Error reading from input stream$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/sabre/vobject/lib/Parser/MimeDir.php" , line : 409usize , placeholders : & [] , regex : "^This code should not be reachable$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/sabre/vobject/lib/Parser/XML.php" , line : 85usize , placeholders : & [] , regex : "^End of input stream, or no input supplied$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/sabre/vobject/lib/Parser/XML.php" , line : 107usize , placeholders : & [] , regex : "^Unsupported XML standard$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/sabre/vobject/lib/Parser/Json.php" , line : 57usize , placeholders : & [] , regex : "^End of input stream, or no input supplied$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/sabre/vobject/lib/Parser/Json.php" , line : 72usize , placeholders : & [] , regex : "^The root component must either be a vcalendar, or a vcard$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/sabre/vobject/lib/Component/VAlarm.php" , line : 51usize , placeholders : & [] , regex : "^time\\-range filters on VALARM components are only supported when they are a child of VTODO or VEVENT$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/sabre/vobject/lib/Component/VCalendar.php" , line : 322usize , placeholders : & [] , regex : "^Every VEVENT object must have a UID property$" , }
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
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/sabre/vobject/lib/Recur/RRuleIterator.php" , line : 775usize , placeholders : & [] , regex : "^BYYEARDAY in RRULE must have value\\(s\\) from 1 to 366, or \\-366 to \\-1!$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/sabre/vobject/lib/Recur/RRuleIterator.php" , line : 784usize , placeholders : & [] , regex : "^BYWEEKNO in RRULE must have value\\(s\\) from 1 to 53, or \\-53 to \\-1!$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/sabre/vobject/lib/Recur/RRuleIterator.php" , line : 793usize , placeholders : & [] , regex : "^BYMONTH in RRULE must have value\\(s\\) between 1 and 12!$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/sabre/vobject/lib/Recur/EventIterator.php" , line : 112usize , placeholders : & [] , regex : "^The UID argument is required when a VCALENDAR is passed to this constructor$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/sabre/vobject/lib/Recur/EventIterator.php" , line : 115usize , placeholders : & [] , regex : "^No events found in this calendar$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/sabre/vobject/lib/Recur/EventIterator.php" , line : 192usize , placeholders : & [] , regex : "^This recurrence rule does not generate any valid instances$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/sabre/xml/lib/Reader.php" , line : 153usize , placeholders : & [] , regex : "^This should never happen \\(famous last words\\)$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/sabre/xml/lib/Reader.php" , line : 183usize , placeholders : & [] , regex : "^We hit the end of the document prematurely\\. This likely means that some parser \"eats\" too many elements\\. Do not attempt to continue parsing\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/sabre/xml/lib/Service.php" , line : 122usize , placeholders : & [] , regex : "^The input element to parse is empty\\. Do not attempt to parse$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/sabre/xml/lib/Service.php" , line : 166usize , placeholders : & [] , regex : "^The input element to parse is empty\\. Do not attempt to parse$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/scssphp/scssphp/src/Warn.php" , line : 79usize , placeholders : & [] , regex : "^The warning Reporter may only be called within a custom function or importer callback\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/scssphp/scssphp/src/Cache.php" , line : 90usize , placeholders : & [] , regex : "^cacheDir not set$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/scssphp/scssphp/src/Cache.php" , line : 98usize , placeholders : & [] , regex : "^prefix not set$" , }
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
	crate :: LoggingStatement { level : crate :: LogLevel :: Warn , path : "/3rdparty/scssphp/scssphp/src/Parser.php" , line : 532usize , placeholders : & ["$file" , "$line" , "$column"] , regex : "^The \"@scssphp\\-import\\-once\" directive is deprecated and will be removed in ScssPhp 2\\.0, in \"(.*)\", line (.*), column (.*)\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Warn , path : "/3rdparty/scssphp/scssphp/src/Parser.php" , line : 1584usize , placeholders : & ["$file" , "$line" , "$column"] , regex : "^Unterminated interpolations in multiline comments are deprecated and will be removed in ScssPhp 2\\.0, in \"(.*)\", line (.*), column (.*)\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Error , path : "/3rdparty/scssphp/scssphp/src/Compiler.php" , line : 752usize , placeholders : & ["$origin" , "$target" , "$target"] , regex : "^\"(.*)\" failed to @extend \"(.*)\"\\. The selector \"(.*)\" was not found\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Error , path : "/3rdparty/scssphp/scssphp/src/Compiler.php" , line : 2331usize , placeholders : & [] , regex : "^@return may only be used within a function$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Error , path : "/3rdparty/scssphp/scssphp/src/Compiler.php" , line : 3057usize , placeholders : & [] , regex : "^Parent selectors aren't allowed here\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Error , path : "/3rdparty/scssphp/scssphp/src/Compiler.php" , line : 3064usize , placeholders : & [] , regex : "^complex selectors may not be extended\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Error , path : "/3rdparty/scssphp/scssphp/src/Compiler.php" , line : 3217usize , placeholders : & ["$name"] , regex : "^Undefined mixin (.*)$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Error , path : "/3rdparty/scssphp/scssphp/src/Compiler.php" , line : 3274usize , placeholders : & ["$name"] , regex : "^@mixin (.*)\\(\\) without parentEnv$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Debug , path : "/3rdparty/scssphp/scssphp/src/Compiler.php" , line : 3321usize , placeholders : & ["$fname" , "$line" , "$value"] , regex : "^(.*):(.*) DEBUG: (.*)$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Warn , path : "/3rdparty/scssphp/scssphp/src/Compiler.php" , line : 3331usize , placeholders : & ["$value" , "$line" , "$fname"] , regex : "^(.*)\n         on line (.*) of (.*)$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Error , path : "/3rdparty/scssphp/scssphp/src/Compiler.php" , line : 3341usize , placeholders : & ["$fname" , "$line" , "$value"] , regex : "^File (.*) on line (.*) ERROR: (.*)\n$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Error , path : "/3rdparty/scssphp/scssphp/src/Compiler.php" , line : 3344usize , placeholders : & ["$child[0]"] , regex : "^unknown child type: (.*)$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Error , path : "/3rdparty/scssphp/scssphp/src/Compiler.php" , line : 4152usize , placeholders : & [] , regex : "^color: Can't take modulo by zero$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Error , path : "/3rdparty/scssphp/scssphp/src/Compiler.php" , line : 4160usize , placeholders : & [] , regex : "^color: Can't divide by zero$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Error , path : "/3rdparty/scssphp/scssphp/src/Compiler.php" , line : 4173usize , placeholders : & ["$op"] , regex : "^color: unknown op (.*)$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/scssphp/scssphp/src/Compiler.php" , line : 4762usize , placeholders : & [] , regex : "^The argument is not a sass string\\. Did you forgot to use \"assertString\"\\?$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Error , path : "/3rdparty/scssphp/scssphp/src/Compiler.php" , line : 5662usize , placeholders : & [] , regex : "^The Sass indented syntax is not implemented\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Error , path : "/3rdparty/scssphp/scssphp/src/Compiler.php" , line : 5801usize , placeholders : & ["$url"] , regex : "^`(.*)` file not found for @import$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Error , path : "/3rdparty/scssphp/scssphp/src/Compiler.php" , line : 6075usize , placeholders : & [] , regex : "^Error: Only %d arguments allowed in %s\\(\\), but %d were passed\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Error , path : "/3rdparty/scssphp/scssphp/src/Compiler.php" , line : 6088usize , placeholders : & [] , regex : "^Error: %s\\(\\) argument%s %s missing\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Error , path : "/3rdparty/scssphp/scssphp/src/Compiler.php" , line : 6152usize , placeholders : & [] , regex : "^An @import loop has been found: %s imports %s$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Error , path : "/3rdparty/scssphp/scssphp/src/Compiler.php" , line : 6189usize , placeholders : & ["$name"] , regex : "^@function (.*)\\(\\) without parentEnv$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/scssphp/scssphp/src/Compiler.php" , line : 6406usize , placeholders : & [] , regex : "^The argument declaration is invalid\\. The rest argument must be the last one\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/scssphp/scssphp/src/Compiler.php" , line : 6706usize , placeholders : & [] , regex : "^Positional arguments must come before keyword arguments\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Error , path : "/3rdparty/scssphp/scssphp/src/Compiler.php" , line : 7189usize , placeholders : & [] , regex : "^Expected %s to have no units or \"%%\"\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Error , path : "/3rdparty/scssphp/scssphp/src/Compiler.php" , line : 7322usize , placeholders : & [] , regex : "^expecting list, %s received$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/scssphp/scssphp/src/Compiler.php" , line : 7343usize , placeholders : & [] , regex : "^The argument is not a sass argument list\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/scssphp/scssphp/src/Compiler.php" , line : 7824usize , placeholders : & [] , regex : "^Only one positional argument is allowed\\. All other arguments must be passed by name\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Error , path : "/3rdparty/scssphp/scssphp/src/Compiler.php" , line : 7850usize , placeholders : & ["$name" , "$number"] , regex : "^(.*) Passing a number `(.*)` without unit % is deprecated\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/scssphp/scssphp/src/Compiler.php" , line : 7905usize , placeholders : & [] , regex : "^HSL parameters may not be passed along with HWB parameters\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Error , path : "/3rdparty/scssphp/scssphp/src/Compiler.php" , line : 8011usize , placeholders : & [] , regex : "^Error: argument `\\$color` of `ie\\-hex\\-str\\(\\$color\\)` must be a color$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Error , path : "/3rdparty/scssphp/scssphp/src/Compiler.php" , line : 8025usize , placeholders : & [] , regex : "^Error: argument `\\$color` of `red\\(\\$color\\)` must be a color$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Error , path : "/3rdparty/scssphp/scssphp/src/Compiler.php" , line : 8037usize , placeholders : & [] , regex : "^Error: argument `\\$color` of `green\\(\\$color\\)` must be a color$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Error , path : "/3rdparty/scssphp/scssphp/src/Compiler.php" , line : 8049usize , placeholders : & [] , regex : "^Error: argument `\\$color` of `blue\\(\\$color\\)` must be a color$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/scssphp/scssphp/src/Compiler.php" , line : 8140usize , placeholders : & [] , regex : "^Missing argument \\$lightness\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/scssphp/scssphp/src/Compiler.php" , line : 8462usize , placeholders : & [] , regex : "^Only one argument may be passed to the plain\\-CSS invert\\(\\) function\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Error , path : "/3rdparty/scssphp/scssphp/src/Compiler.php" , line : 8609usize , placeholders : & [] , regex : "^At least one argument must be passed\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Error , path : "/3rdparty/scssphp/scssphp/src/Compiler.php" , line : 8632usize , placeholders : & [] , regex : "^At least one argument must be passed\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Error , path : "/3rdparty/scssphp/scssphp/src/Compiler.php" , line : 8691usize , placeholders : & [] , regex : "^Invalid argument for \"n\"$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Error , path : "/3rdparty/scssphp/scssphp/src/Compiler.php" , line : 9046usize , placeholders : & [] , regex : "^Invalid argument\\(s\\) for \"comparable\"$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/scssphp/scssphp/src/Compiler.php" , line : 9275usize , placeholders : & ["$n"] , regex : "^\\$limit: Must be greater than 0, was (.*)\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Error , path : "/3rdparty/scssphp/scssphp/src/Compiler.php" , line : 9463usize , placeholders : & [] , regex : "^Invalid super selector for isSuperSelector\\(\\)$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Error , path : "/3rdparty/scssphp/scssphp/src/Compiler.php" , line : 9467usize , placeholders : & [] , regex : "^Invalid sub selector for isSuperSelector\\(\\)$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Error , path : "/3rdparty/scssphp/scssphp/src/Compiler.php" , line : 9567usize , placeholders : & [] , regex : "^selector\\-append\\(\\) needs at least 1 argument$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Error , path : "/3rdparty/scssphp/scssphp/src/Compiler.php" , line : 9592usize , placeholders : & [] , regex : "^Invalid selector list in selector\\-append\\(\\)$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Error , path : "/3rdparty/scssphp/scssphp/src/Compiler.php" , line : 9599usize , placeholders : & [] , regex : "^Invalid selector list in selector\\-append\\(\\)$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Error , path : "/3rdparty/scssphp/scssphp/src/Compiler.php" , line : 9642usize , placeholders : & [] , regex : "^selector\\-extend\\(\\) invalid arguments$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Error , path : "/3rdparty/scssphp/scssphp/src/Compiler.php" , line : 9663usize , placeholders : & [] , regex : "^selector\\-replace\\(\\) invalid arguments$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Error , path : "/3rdparty/scssphp/scssphp/src/Compiler.php" , line : 9692usize , placeholders : & [] , regex : "^Can't extend complex selector\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Error , path : "/3rdparty/scssphp/scssphp/src/Compiler.php" , line : 9730usize , placeholders : & [] , regex : "^selector\\-nest\\(\\) needs at least 1 argument$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Error , path : "/3rdparty/scssphp/scssphp/src/Compiler.php" , line : 9775usize , placeholders : & [] , regex : "^selector\\-unify\\(\\) invalid arguments$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Warn , path : "/3rdparty/scssphp/scssphp/src/Compiler.php" , line : 10070usize , placeholders : & [] , regex : "^The \"scssphp\\-glob\" function is deprecated an will be removed in ScssPhp 2\\.0\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/scssphp/scssphp/src/Node/Number.php" , line : 208usize , placeholders : & [] , regex : "^Number is immutable$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/scssphp/scssphp/src/Node/Number.php" , line : 217usize , placeholders : & [] , regex : "^Number is immutable$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/scssphp/scssphp/scss.inc.php" , line : 4usize , placeholders : & [] , regex : "^scssphp requires PHP 5\\.6 or above$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/justinrainbow/json-schema/src/JsonSchema/Entity/JsonPointer.php" , line : 40usize , placeholders : & [] , regex : "^Ref value must be a string$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/justinrainbow/json-schema/src/JsonSchema/Uri/Retrievers/Curl.php" , line : 28usize , placeholders : & [] , regex : "^cURL not installed$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/justinrainbow/json-schema/src/JsonSchema/Uri/Retrievers/Curl.php" , line : 48usize , placeholders : & [] , regex : "^JSON schema not found$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/justinrainbow/json-schema/src/JsonSchema/Constraints/SchemaConstraint.php" , line : 40usize , placeholders : & [] , regex : "^no schema found to verify against$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/justinrainbow/json-schema/src/JsonSchema/Constraints/SchemaConstraint.php" , line : 52usize , placeholders : & [] , regex : "^Cannot validate the schema of a non\\-object$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/justinrainbow/json-schema/src/JsonSchema/Constraints/SchemaConstraint.php" , line : 79usize , placeholders : & [] , regex : "^Schema did not pass validation$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/egulias/email-validator/src/Validation/DNSCheckValidation.php" , line : 127usize , placeholders : & ["$errorMessage"] , regex : "^Unable to get DNS record for the host: (.*)$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/opis/closure/src/SerializableClosure.php" , line : 200usize , placeholders : & [] , regex : "^Invalid signed closure$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/opis/closure/src/SerializableClosure.php" , line : 223usize , placeholders : & [] , regex : "^Invalid signed closure$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/opis/closure/src/SerializableClosure.php" , line : 236usize , placeholders : & [] , regex : "^Invalid signed closure$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/stecman/symfony-console-completion/src/EnvironmentCompletionContext.php" , line : 41usize , placeholders : & [] , regex : "^Failed to read word breaks from environment; Environment var CMDLINE_WORDBREAKS not set$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/stecman/symfony-console-completion/src/CompletionCommand.php" , line : 208usize , placeholders : & [] , regex : "^Could not read SHELL environment variable\\. Please specify your shell type using the \\-\\-shell\\-type option\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/stecman/symfony-console-completion/src/CompletionHandler.php" , line : 106usize , placeholders : & [] , regex : "^A CompletionContext must be set before requesting completion\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/web-auth/webauthn-lib/src/CertificateChainChecker/OpenSSLCertificateChainChecker.php" , line : 98usize , placeholders : & [] , regex : "^Invalid certificate or certificate chain$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/web-auth/webauthn-lib/src/CertificateChainChecker/OpenSSLCertificateChainChecker.php" , line : 148usize , placeholders : & [] , regex : "^Invalid certificate or certificate chain$" , }
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
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/web-auth/webauthn-lib/src/AttestationStatement/FidoU2FAttestationStatementSupport.php" , line : 109usize , placeholders : & [] , regex : "^Invalid certificate or certificate chain$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/web-auth/webauthn-lib/src/AttestationStatement/AndroidSafetyNetAttestationStatementSupport.php" , line : 76usize , placeholders : & [] , regex : "^The algorithm RS256 is missing\\. Did you forget to install the package web\\-token/jwt\\-signature\\-algorithm\\-rsa\\?$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/web-auth/webauthn-lib/src/AttestationStatement/AndroidSafetyNetAttestationStatementSupport.php" , line : 79usize , placeholders : & [] , regex : "^The class Jose\\\\Component\\\\KeyManagement\\\\JWKFactory is missing\\. Did you forget to install the package web\\-token/jwt\\-key\\-mgmt\\?$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/web-auth/webauthn-lib/src/AttestationStatement/AndroidSafetyNetAttestationStatementSupport.php" , line : 256usize , placeholders : & [] , regex : "^Unrecognized response$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/web-auth/webauthn-lib/src/AttestationStatement/PackedAttestationStatementSupport.php" , line : 90usize , placeholders : & [] , regex : "^Unsupported attestation statement$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/web-auth/webauthn-lib/src/AttestationStatement/PackedAttestationStatementSupport.php" , line : 167usize , placeholders : & [] , regex : "^ECDAA not supported$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/web-auth/webauthn-lib/src/AttestationStatement/PackedAttestationStatementSupport.php" , line : 188usize , placeholders : & [] , regex : "^Invalid algorithm$" , }
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
	crate :: LoggingStatement { level : crate :: LogLevel :: Info , path : "/3rdparty/web-auth/webauthn-lib/src/AuthenticatorAssertionResponseValidator.php" , line : 97usize , placeholders : & [] , regex : "^Checking the authenticator assertion response$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Info , path : "/3rdparty/web-auth/webauthn-lib/src/AuthenticatorAssertionResponseValidator.php" , line : 211usize , placeholders : & [] , regex : "^The assertion is valid$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Debug , path : "/3rdparty/web-auth/webauthn-lib/src/AuthenticatorAssertionResponseValidator.php" , line : 212usize , placeholders : & [] , regex : "^Public Key Credential Source$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Error , path : "/3rdparty/web-auth/webauthn-lib/src/AuthenticatorAssertionResponseValidator.php" , line : 216usize , placeholders : & [] , regex : "^An error occurred$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Error , path : "/3rdparty/web-auth/webauthn-lib/src/Counter/ThrowExceptionIfInvalid.php" , line : 39usize , placeholders : & [] , regex : "^The counter is invalid$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/web-auth/webauthn-lib/src/CertificateToolbox.php" , line : 64usize , placeholders : & [] , regex : "^Invalid certificate or certificate chain$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/web-auth/webauthn-lib/src/CertificateToolbox.php" , line : 98usize , placeholders : & [] , regex : "^Invalid certificate or certificate chain$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/web-auth/webauthn-lib/src/PublicKeyCredentialSource.php" , line : 218usize , placeholders : & [] , regex : "^Unable to load the data$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/web-auth/cose-lib/src/Algorithm/Signature/RSA/RSA.php" , line : 30usize , placeholders : & [] , regex : "^Unable to sign the data$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/web-auth/cose-lib/src/Algorithm/Signature/RSA/PSSRSA.php" , line : 56usize , placeholders : & [] , regex : "^Invalid modulus length$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/web-auth/cose-lib/src/Algorithm/Signature/RSA/PSSRSA.php" , line : 103usize , placeholders : & [] , regex : "^Unable to convert the integer$" , }
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
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/web-auth/cose-lib/src/Algorithm/Signature/EdDSA/EdDSA.php" , line : 40usize , placeholders : & [] , regex : "^Unsupported curve$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/web-auth/cose-lib/src/Algorithm/Signature/EdDSA/EdDSA.php" , line : 52usize , placeholders : & [] , regex : "^Unsupported curve$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/web-auth/cose-lib/src/Key/RsaKey.php" , line : 193usize , placeholders : & [] , regex : "^Unable to convert to an integer$" , }
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
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/cweagans/composer-patches/src/Patches.php" , line : 269usize , placeholders : & [] , regex : "^There was an error in the supplied patch file$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/cweagans/composer-patches/src/Patches.php" , line : 326usize , placeholders : & ["$description" , "$url"] , regex : "^Cannot apply patch (.*) \\((.*)\\)!$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/cweagans/composer-patches/src/Patches.php" , line : 423usize , placeholders : & ["$patch_url"] , regex : "^Cannot apply patch (.*)$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/php-opencloud/openstack/src/OpenStack.php" , line : 50usize , placeholders : & [] , regex : "^'authUrl' is a required option$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/php-opencloud/openstack/src/Common/Service/Builder.php" , line : 164usize , placeholders : & [] , regex : "^\"authUrl\" is a required option$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/php-opencloud/openstack/src/Identity/v3/Models/Token.php" , line : 114usize , placeholders : & [] , regex : "^Either a user or token must be provided\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/php-opencloud/openstack/src/Identity/v3/Models/Catalog.php" , line : 50usize , placeholders : & [] , regex : "^No services are defined$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/php-opencloud/openstack/src/Compute/v2/Models/Server.php" , line : 137usize , placeholders : & [] , regex : "^imageId or blockDeviceMapping\\.uuid must be set\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/php-opencloud/openstack/src/Compute/v2/Models/Server.php" , line : 203usize , placeholders : & [] , regex : "^Reboot type must either be SOFT or HARD$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/php-opencloud/openstack/src/Compute/v2/Models/Server.php" , line : 426usize , placeholders : & [] , regex : "^networkId or portId must be set\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/deepdiver/zipstreamer/test/ZipComponents.php" , line : 136usize , placeholders : & [] , regex : "^invalid magic$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/deepdiver/zipstreamer/test/ZipComponents.php" , line : 194usize , placeholders : & [] , regex : "^invalid magic$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/deepdiver/zipstreamer/test/ZipComponents.php" , line : 261usize , placeholders : & [] , regex : "^invalid magic$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/deepdiver/zipstreamer/test/ZipComponents.php" , line : 353usize , placeholders : & [] , regex : "^invalid magic$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/deepdiver/zipstreamer/test/ZipComponents.php" , line : 432usize , placeholders : & [] , regex : "^invalid magic$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/deepdiver/zipstreamer/test/ZipComponents.php" , line : 540usize , placeholders : & [] , regex : "^invalid magic$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/deepdiver/zipstreamer/test/ZipComponents.php" , line : 601usize , placeholders : & [] , regex : "^invalid magic$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/deepdiver/zipstreamer/src/ZipStreamer.php" , line : 310usize , placeholders : & [] , regex : "^unable to use compression method DEFLATE with level other than NONE \\(requires pecl_http >= 0\\.10\\)$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/nikic/php-parser/lib/PhpParser/NodeDumper.php" , line : 105usize , placeholders : & [] , regex : "^Can only dump nodes and arrays\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/nikic/php-parser/lib/PhpParser/NodeDumper.php" , line : 193usize , placeholders : & [] , regex : "^Invalid position information$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/nikic/php-parser/lib/PhpParser/JsonDecoder.php" , line : 43usize , placeholders : & [] , regex : "^Node type must be a string$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/nikic/php-parser/lib/PhpParser/JsonDecoder.php" , line : 52usize , placeholders : & [] , regex : "^Attributes must be an array$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/nikic/php-parser/lib/PhpParser/JsonDecoder.php" , line : 72usize , placeholders : & [] , regex : "^Comment must have text$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/nikic/php-parser/lib/PhpParser/JsonDecoder.php" , line : 101usize , placeholders : & ["$nodeType"] , regex : "^Unknown node type \"(.*)\"$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/nikic/php-parser/lib/PhpParser/ConstExprEvaluator.php" , line : 42usize , placeholders : & ["$expr->getType()"] , regex : "^Expression of type (.*) cannot be evaluated$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/nikic/php-parser/lib/PhpParser/ConstExprEvaluator.php" , line : 213usize , placeholders : & [] , regex : "^Should not happen$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/nikic/php-parser/lib/PhpParser/Parser/Php5.php" , line : 1320usize , placeholders : & [] , regex : "^__HALT_COMPILER\\(\\) can only be used from the outermost scope$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/nikic/php-parser/lib/PhpParser/Parser/Php7.php" , line : 1432usize , placeholders : & [] , regex : "^__HALT_COMPILER\\(\\) can only be used from the outermost scope$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/nikic/php-parser/lib/PhpParser/ParserAbstract.php" , line : 136usize , placeholders : & [] , regex : "^\"throwOnError\" is no longer supported, use \"errorHandler\" instead$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/nikic/php-parser/lib/PhpParser/ParserAbstract.php" , line : 372usize , placeholders : & [] , regex : "^Reached end of parser loop$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/nikic/php-parser/lib/PhpParser/Builder/Method.php" , line : 78usize , placeholders : & [] , regex : "^Cannot make method with statements abstract$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/nikic/php-parser/lib/PhpParser/Builder/Method.php" , line : 107usize , placeholders : & [] , regex : "^Cannot add statements to an abstract method$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/nikic/php-parser/lib/PhpParser/Builder/TraitUseAdaptation.php" , line : 53usize , placeholders : & [] , regex : "^Cannot set alias for not alias adaptation buider$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/nikic/php-parser/lib/PhpParser/Builder/TraitUseAdaptation.php" , line : 100usize , placeholders : & [] , regex : "^Precedence adaptation must have trait$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/nikic/php-parser/lib/PhpParser/Builder/TraitUseAdaptation.php" , line : 107usize , placeholders : & [] , regex : "^Cannot add overwritten traits for not precedence adaptation buider$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/nikic/php-parser/lib/PhpParser/Builder/TraitUseAdaptation.php" , line : 123usize , placeholders : & [] , regex : "^Cannot set access modifier for not alias adaptation buider$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/nikic/php-parser/lib/PhpParser/Builder/TraitUseAdaptation.php" , line : 129usize , placeholders : & [] , regex : "^Multiple access type modifiers are not allowed$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/nikic/php-parser/lib/PhpParser/Builder/TraitUseAdaptation.php" , line : 145usize , placeholders : & [] , regex : "^Type of adaptation is not defined$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/nikic/php-parser/lib/PhpParser/Builder/TraitUse.php" , line : 49usize , placeholders : & [] , regex : "^Adaptation must have type TraitUseAdaptation$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/nikic/php-parser/lib/PhpParser/Builder/Param.php" , line : 54usize , placeholders : & [] , regex : "^Parameter type cannot be void$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/nikic/php-parser/lib/PhpParser/Internal/TokenStream.php" , line : 116usize , placeholders : & [] , regex : "^Encountered unexpected token$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/nikic/php-parser/lib/PhpParser/Internal/TokenStream.php" , line : 133usize , placeholders : & [] , regex : "^Encountered unexpected token$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/nikic/php-parser/lib/PhpParser/Internal/Differ.php" , line : 81usize , placeholders : & [] , regex : "^Should not happen$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/nikic/php-parser/lib/PhpParser/PrettyPrinterAbstract.php" , line : 886usize , placeholders : & [] , regex : "^Shouldn't happen$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/nikic/php-parser/lib/PhpParser/PrettyPrinterAbstract.php" , line : 988usize , placeholders : & [] , regex : "^Cannot happen$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/nikic/php-parser/lib/PhpParser/ParserFactory.php" , line : 40usize , placeholders : & [] , regex : "^Kind must be one of ::PREFER_PHP7, ::PREFER_PHP5, ::ONLY_PHP7 or ::ONLY_PHP5$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/nikic/php-parser/lib/PhpParser/Lexer.php" , line : 367usize , placeholders : & [] , regex : "^Reached end of lexer loop$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/nikic/php-parser/lib/PhpParser/Lexer.php" , line : 397usize , placeholders : & [] , regex : "^__HALT_COMPILER must be followed by \"\\(\\);\"$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/nikic/php-parser/lib/PhpParser/BuilderHelpers.php" , line : 34usize , placeholders : & [] , regex : "^Expected node or builder object$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/nikic/php-parser/lib/PhpParser/BuilderHelpers.php" , line : 56usize , placeholders : & [] , regex : "^Expected statement or expression node$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/nikic/php-parser/lib/PhpParser/BuilderHelpers.php" , line : 75usize , placeholders : & [] , regex : "^Expected string or instance of Node\\\\Identifier$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/nikic/php-parser/lib/PhpParser/BuilderHelpers.php" , line : 94usize , placeholders : & [] , regex : "^Expected string or instance of Node\\\\Identifier or Node\\\\Expr$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/nikic/php-parser/lib/PhpParser/BuilderHelpers.php" , line : 132usize , placeholders : & [] , regex : "^Name cannot be empty$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/nikic/php-parser/lib/PhpParser/BuilderHelpers.php" , line : 149usize , placeholders : & [] , regex : "^Name must be a string or an instance of Node\\\\Name or Node\\\\Expr$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/nikic/php-parser/lib/PhpParser/BuilderHelpers.php" , line : 152usize , placeholders : & [] , regex : "^Name must be a string or an instance of Node\\\\Name$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/nikic/php-parser/lib/PhpParser/BuilderHelpers.php" , line : 173usize , placeholders : & [] , regex : "^Type must be a string, or an instance of Name, Identifier, NullableType or UnionType$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/nikic/php-parser/lib/PhpParser/BuilderHelpers.php" , line : 197usize , placeholders : & [] , regex : "^void type cannot be nullable$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/nikic/php-parser/lib/PhpParser/BuilderHelpers.php" , line : 201usize , placeholders : & [] , regex : "^mixed type cannot be nullable$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/nikic/php-parser/lib/PhpParser/BuilderHelpers.php" , line : 252usize , placeholders : & [] , regex : "^Invalid value$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/nikic/php-parser/lib/PhpParser/BuilderHelpers.php" , line : 269usize , placeholders : & [] , regex : "^Doc comment must be a string or an instance of PhpParser\\\\Comment\\\\Doc$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/nikic/php-parser/lib/PhpParser/NodeTraverser.php" , line : 262usize , placeholders : & [] , regex : "^Invalid node structure: Contains nested arrays$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/nikic/php-parser/lib/PhpParser/PrettyPrinter/Standard.php" , line : 146usize , placeholders : & [] , regex : "^Invalid string kind$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/nikic/php-parser/lib/PhpParser/PrettyPrinter/Standard.php" , line : 194usize , placeholders : & [] , regex : "^Invalid number kind$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/nikic/php-parser/lib/PhpParser/PrettyPrinter/Standard.php" , line : 224usize , placeholders : & [] , regex : "^Cannot directly print EncapsedStringPart$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/nikic/php-parser/lib/PhpParser/PrettyPrinter/Standard.php" , line : 555usize , placeholders : & [] , regex : "^Cannot pretty\\-print AST with Error nodes$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/nikic/php-parser/lib/PhpParser/Node/Stmt/Class_.php" , line : 84usize , placeholders : & [] , regex : "^Multiple access type modifiers are not allowed$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/nikic/php-parser/lib/PhpParser/Node/Stmt/Class_.php" , line : 88usize , placeholders : & [] , regex : "^Multiple abstract modifiers are not allowed$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/nikic/php-parser/lib/PhpParser/Node/Stmt/Class_.php" , line : 92usize , placeholders : & [] , regex : "^Multiple static modifiers are not allowed$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/nikic/php-parser/lib/PhpParser/Node/Stmt/Class_.php" , line : 96usize , placeholders : & [] , regex : "^Multiple final modifiers are not allowed$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/nikic/php-parser/lib/PhpParser/Node/Stmt/Class_.php" , line : 100usize , placeholders : & [] , regex : "^Cannot use the final modifier on an abstract class member$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/nikic/php-parser/lib/PhpParser/Node/Name.php" , line : 220usize , placeholders : & [] , regex : "^Name cannot be empty$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/nikic/php-parser/lib/PhpParser/Node/Name.php" , line : 226usize , placeholders : & [] , regex : "^Name cannot be empty$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/nikic/php-parser/lib/PhpParser/Node/Name.php" , line : 235usize , placeholders : & [] , regex : "^Expected string, array of parts or Name instance$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/nikic/php-parser/lib/PhpParser/Node/Scalar/String_.php" , line : 135usize , placeholders : & [] , regex : "^Invalid UTF\\-8 codepoint escape sequence: Codepoint too large$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/nikic/php-parser/lib/PhpParser/Node/Scalar/LNumber.php" , line : 62usize , placeholders : & [] , regex : "^Invalid numeric literal$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/nikic/php-parser/lib/PhpParser/BuilderFactory.php" , line : 196usize , placeholders : & [] , regex : "^Variable name must be string or Expr$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/nikic/php-parser/lib/PhpParser/BuilderFactory.php" , line : 335usize , placeholders : & [] , regex : "^Expected at least two expressions$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/nikic/php-parser/lib/PhpParser/BuilderFactory.php" , line : 358usize , placeholders : & [] , regex : "^Expected string or Expr$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/nikic/php-parser/lib/PhpParser/Error.php" , line : 112usize , placeholders : & [] , regex : "^Error does not have column information$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/nikic/php-parser/lib/PhpParser/Error.php" , line : 126usize , placeholders : & [] , regex : "^Error does not have column information$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/nikic/php-parser/lib/PhpParser/Error.php" , line : 157usize , placeholders : & [] , regex : "^Invalid position information$" , }
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
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/brick/math/src/Internal/Calculator/NativeCalculator.php" , line : 47usize , placeholders : & [] , regex : "^The platform is not 32\\-bit or 64\\-bit as expected\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/brick/math/src/Internal/Calculator.php" , line : 558usize , placeholders : & [] , regex : "^Invalid rounding mode\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/brick/math/src/Internal/Calculator.php" , line : 666usize , placeholders : & [] , regex : "^Invalid bitwise operator\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/brick/math/src/BigRational.php" , line : 393usize , placeholders : & [] , regex : "^This rational number cannot be represented as an integer value without rounding\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/brick/math/src/BigRational.php" , line : 481usize , placeholders : & [] , regex : "^unserialize\\(\\) is an internal function, it must not be called directly\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/brick/math/src/BigNumber.php" , line : 131usize , placeholders : & [] , regex : "^Exponent too large\\.$" , }
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
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/pimple/pimple/src/Pimple/Container.php" , line : 174usize , placeholders : & [] , regex : "^Service definition is not a Closure or invokable object\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/pimple/pimple/src/Pimple/Container.php" , line : 196usize , placeholders : & [] , regex : "^Callable is not a Closure or invokable object\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/pimple/pimple/src/Pimple/Container.php" , line : 261usize , placeholders : & [] , regex : "^Extension service definition is not a Closure or invokable object\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/deepdiver1975/tarstreamer/src/TarStreamer.php" , line : 55usize , placeholders : & ["$encodedArchiveName" , "$headerFile" , "$headerLine"] , regex : "^Unable to send file (.*)\\. HTML Headers have already been sent from (.*) in line (.*)$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/deepdiver1975/tarstreamer/src/TarStreamer.php" , line : 59usize , placeholders : & ["$encodedArchiveName"] , regex : "^Unable to send file (.*)\\. Output buffer already contains text \\(typically warnings or errors\\)\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/fgrosse/phpasn1/lib/X509/SAN/IPAddress.php" , line : 60usize , placeholders : & ["$contentLength"] , regex : "^A FG\\\\X509\\\\SAN\\\\IPAddress should have a content length of 4\\. Extracted length was (.*)$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/fgrosse/phpasn1/lib/X509/SAN/SubjectAlternativeNames.php" , line : 76usize , placeholders : & [] , regex : "^Can not parse Subject Alternative Names: The Sequence length does not match the length of the surrounding octet string$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/fgrosse/phpasn1/lib/X509/SAN/SubjectAlternativeNames.php" , line : 89usize , placeholders : & [] , regex : "^Could not parse Subject Alternative Name: Only DNSName and IP SANs are currently supported$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/fgrosse/phpasn1/lib/X509/CertificateExtensions.php" , line : 74usize , placeholders : & [] , regex : "^Could not parse Certificate Extensions: Needs at least two child elements per extension sequence \\(object identifier and octet string\\)$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/fgrosse/phpasn1/lib/ASN1/AbstractTime.php" , line : 35usize , placeholders : & [] , regex : "^Invalid first argument for some instance of AbstractTime constructor$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/fgrosse/phpasn1/lib/ASN1/Construct.php" , line : 173usize , placeholders : & [] , regex : "^Sequence length incorrect$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/fgrosse/phpasn1/lib/ASN1/ExplicitlyTaggedObject.php" , line : 124usize , placeholders : & ["$tag" , "$offsetIndexOfDecoratedObject" , "$totalContentLength" , "$remainingContentLength"] , regex : "^Context\\-Specific explicitly tagged object \\[(.*)\\] starting at offset (.*) specifies a length of (.*) octets but (.*) remain after parsing the content$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/fgrosse/phpasn1/lib/ASN1/TemplateParser.php" , line : 67usize , placeholders : & ["$expectedTypeId" , "$actualType"] , regex : "^Expected type \\((.*)\\) does not match actual type \\((.*)$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/fgrosse/phpasn1/lib/ASN1/AbstractString.php" , line : 106usize , placeholders : & ["$typeName" , "$this->value"] , regex : "^Could not create a (.*) from the character sequence '(.*)'\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/fgrosse/phpasn1/lib/ASN1/Identifier.php" , line : 89usize , placeholders : & ["$isConstructed"] , regex : "^\\$isConstructed must be a boolean value \\((.*) given\\)$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/fgrosse/phpasn1/lib/ASN1/Universal/RelativeObjectIdentifier.php" , line : 49usize , placeholders : & [] , regex : "^Malformed ASN\\.1 Relative Object Identifier$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/fgrosse/phpasn1/lib/ASN1/Universal/OctetString.php" , line : 32usize , placeholders : & [] , regex : "^OctetString: unrecognized input type!$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/fgrosse/phpasn1/lib/ASN1/Universal/BitString.php" , line : 35usize , placeholders : & [] , regex : "^BitString: second parameter needs to be a positive number \\(or zero\\)!$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/fgrosse/phpasn1/lib/ASN1/Universal/BitString.php" , line : 78usize , placeholders : & [] , regex : "^Can not parse bit string with invalid padding$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/fgrosse/phpasn1/lib/ASN1/Universal/Boolean.php" , line : 64usize , placeholders : & ["$contentLength"] , regex : "^An ASN\\.1 Boolean should not have a length other than one\\. Extracted length was (.*)$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/fgrosse/phpasn1/lib/ASN1/Universal/GeneralizedTime.php" , line : 124usize , placeholders : & [] , regex : "^Invalid ISO 8601 Time String$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/fgrosse/phpasn1/lib/ASN1/Universal/ObjectIdentifier.php" , line : 124usize , placeholders : & [] , regex : "^Malformed ASN\\.1 Object Identifier$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/fgrosse/phpasn1/lib/ASN1/Universal/UTCTime.php" , line : 69usize , placeholders : & [] , regex : "^Invalid UTC String$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/fgrosse/phpasn1/lib/ASN1/Universal/NullObject.php" , line : 46usize , placeholders : & ["$contentLength"] , regex : "^An ASN\\.1 Null should not have a length other than zero\\. Extracted length was (.*)$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/fgrosse/phpasn1/lib/ASN1/Universal/Integer.php" , line : 33usize , placeholders : & ["$value"] , regex : "^Invalid VALUE \\[(.*)\\] for ASN1_INTEGER$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/fgrosse/phpasn1/lib/ASN1/Universal/Integer.php" , line : 101usize , placeholders : & [] , regex : "^Integer not minimally encoded$" , }
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
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/fgrosse/phpasn1/lib/Utility/BigInteger.php" , line : 62usize , placeholders : & [] , regex : "^Requires GMP or bcmath extension\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/fgrosse/phpasn1/lib/Utility/BigInteger.php" , line : 75usize , placeholders : & [] , regex : "^Expects a string representation of an integer\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/fgrosse/phpasn1/lib/Utility/BigIntegerGmp.php" , line : 104usize , placeholders : & [] , regex : "^Unable to raise to power greater than PHP_INT_MAX\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/composer/ClassLoader.php" , line : 252usize , placeholders : & [] , regex : "^A non\\-empty PSR\\-4 prefix must end with a namespace separator\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/composer/ClassLoader.php" , line : 307usize , placeholders : & [] , regex : "^A non\\-empty PSR\\-4 prefix must end with a namespace separator\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/doctrine/cache/lib/Doctrine/Common/Cache/Psr6/CacheAdapter.php" , line : 261usize , placeholders : & [] , regex : "^Cache key length must be greater than zero\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/doctrine/dbal/src/Result.php" , line : 294usize , placeholders : & [] , regex : "^Only invocations with one argument are still supported by this legecy API\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/doctrine/dbal/src/Result.php" , line : 309usize , placeholders : & [] , regex : "^Only fetch modes declared on Doctrine\\\\DBAL\\\\FetchMode are supported by legacy API\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/doctrine/dbal/src/Result.php" , line : 322usize , placeholders : & [] , regex : "^Only invocations with one argument are still supported by this legecy API\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/doctrine/dbal/src/Result.php" , line : 337usize , placeholders : & [] , regex : "^Only fetch modes declared on Doctrine\\\\DBAL\\\\FetchMode are supported by legacy API\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/doctrine/dbal/src/DriverManager.php" , line : 304usize , placeholders : & [] , regex : "^Malformed parameter \"url\"\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/doctrine/dbal/src/Tools/Console/Command/RunSqlCommand.php" , line : 73usize , placeholders : & [] , regex : "^Argument 'SQL' is required in order to execute this command correctly\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/doctrine/dbal/src/Tools/Console/Command/RunSqlCommand.php" , line : 81usize , placeholders : & [] , regex : "^Option 'depth' must contains an integer value$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/doctrine/dbal/src/Id/TableGenerator.php" , line : 74usize , placeholders : & [] , regex : "^Cannot use TableGenerator with SQLite\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/doctrine/dbal/src/Id/TableGenerator.php" , line : 137usize , placeholders : & [] , regex : "^Race\\-condition detected while updating sequence\\. Aborting generation$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/doctrine/dbal/src/Connections/PrimaryReadReplicaConnection.php" , line : 112usize , placeholders : & [] , regex : "^primary or replica configuration missing$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/doctrine/dbal/src/Connections/PrimaryReadReplicaConnection.php" , line : 116usize , placeholders : & [] , regex : "^You have to configure at least one replica\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/doctrine/dbal/src/Connections/PrimaryReadReplicaConnection.php" , line : 163usize , placeholders : & [] , regex : "^Invalid option to connect\\(\\), only primary or replica allowed\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/doctrine/dbal/src/Platforms/SQLServer2012Platform.php" , line : 450usize , placeholders : & [] , regex : "^Incomplete column definition\\. 'default' required\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/doctrine/dbal/src/Platforms/SqlitePlatform.php" , line : 715usize , placeholders : & [] , regex : "^Sqlite platform requires for alter table the table diff with reference to original table schema$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/doctrine/dbal/src/Platforms/SqlitePlatform.php" , line : 740usize , placeholders : & [] , regex : "^Sqlite platform requires for alter table the table diff with reference to original table schema$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/doctrine/dbal/src/Platforms/SqlitePlatform.php" , line : 819usize , placeholders : & [] , regex : "^Sqlite platform does not support alter primary key\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/doctrine/dbal/src/Platforms/SqlitePlatform.php" , line : 827usize , placeholders : & [] , regex : "^Sqlite platform does not support alter foreign key\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/doctrine/dbal/src/Platforms/SqlitePlatform.php" , line : 835usize , placeholders : & [] , regex : "^Sqlite platform does not support alter foreign key\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/doctrine/dbal/src/Platforms/SqlitePlatform.php" , line : 843usize , placeholders : & [] , regex : "^Sqlite platform does not support alter constraint\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/doctrine/dbal/src/Platforms/SqlitePlatform.php" , line : 884usize , placeholders : & [] , regex : "^Sqlite platform requires for alter table the table diff with reference to original table schema$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/doctrine/dbal/src/Platforms/PostgreSQL94Platform.php" , line : 842usize , placeholders : & ["${value}"] , regex : "^Unrecognized boolean literal '(.*)'$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/doctrine/dbal/src/Platforms/AbstractPlatform.php" , line : 1403usize , placeholders : & [] , regex : "^Default implementation of DROP TABLE was overridden with NULL$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/doctrine/dbal/src/Platforms/AbstractPlatform.php" , line : 1511usize , placeholders : & [] , regex : "^Second argument of AbstractPlatform::getCreateTableSQL\\(\\) has to be integer\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/doctrine/dbal/src/Platforms/AbstractPlatform.php" , line : 1776usize , placeholders : & [] , regex : "^Can only create primary or unique constraints, no common indexes with getCreateConstraintSQL\\(\\)\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/doctrine/dbal/src/Platforms/AbstractPlatform.php" , line : 1812usize , placeholders : & [] , regex : "^Incomplete definition\\. 'columns' required\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/doctrine/dbal/src/Platforms/AbstractPlatform.php" , line : 2366usize , placeholders : & [] , regex : "^Incomplete definition\\. 'columns' required\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/doctrine/dbal/src/Platforms/AbstractPlatform.php" , line : 2393usize , placeholders : & [] , regex : "^Incomplete definition\\. 'columns' required\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/doctrine/dbal/src/Platforms/AbstractPlatform.php" , line : 2555usize , placeholders : & [] , regex : "^Incomplete definition\\. 'local' required\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/doctrine/dbal/src/Platforms/AbstractPlatform.php" , line : 2559usize , placeholders : & [] , regex : "^Incomplete definition\\. 'foreign' required\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/doctrine/dbal/src/Platforms/AbstractPlatform.php" , line : 2563usize , placeholders : & [] , regex : "^Incomplete definition\\. 'foreignTable' required\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/doctrine/dbal/src/Platforms/OraclePlatform.php" , line : 49usize , placeholders : & [] , regex : "^Invalid Oracle identifier$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/swiftmailer/swiftmailer/lib/classes/Swift/AddressEncoder/IdnAddressEncoder.php" , line : 40usize , placeholders : & [] , regex : "^Non\\-ASCII characters not supported in local\\-part$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/swiftmailer/swiftmailer/lib/classes/Swift/Mime/SimpleMimeEntity.php" , line : 698usize , placeholders : & [] , regex : "^Mime boundary set is not RFC 2046 compliant\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/swiftmailer/swiftmailer/lib/classes/Swift/Mime/Headers/PathHeader.php" , line : 150usize , placeholders : & [] , regex : "^Address set in PathHeader does not comply with addr\\-spec of RFC 2822\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/swiftmailer/swiftmailer/lib/classes/Swift/Signers/DomainKeySigner.php" , line : 440usize , placeholders : & [] , regex : "^Invalid new line sequence in mail found \\\\n without preceding \\\\r$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/swiftmailer/swiftmailer/lib/classes/Swift/Signers/DKIMSigner.php" , line : 299usize , placeholders : & [] , regex : "^Unable to set sha256 as it is not supported by OpenSSL\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/swiftmailer/swiftmailer/lib/classes/Swift/Signers/DKIMSigner.php" , line : 303usize , placeholders : & [] , regex : "^Unable to set the hash algorithm, must be one of rsa\\-sha1 or rsa\\-sha256 \\(%s given\\)\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/swiftmailer/swiftmailer/lib/classes/Swift/Signers/OpenDKIMSigner.php" , line : 35usize , placeholders : & [] , regex : "^php\\-opendkim extension not found$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/swiftmailer/swiftmailer/lib/classes/Swift/DependencyContainer.php" , line : 349usize , placeholders : & [] , regex : "^Component must first be registered by calling register\\(\\)$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/swiftmailer/swiftmailer/lib/classes/Swift/Transport/FailoverTransport.php" , line : 83usize , placeholders : & [] , regex : "^All Transports in FailoverTransport failed, or no Transports available$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/swiftmailer/swiftmailer/lib/classes/Swift/Transport/EsmtpTransport.php" , line : 349usize , placeholders : & [] , regex : "^Unable to connect with TLS encryption$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/swiftmailer/swiftmailer/lib/classes/Swift/Transport/Esmtp/Auth/NTLMAuthenticator.php" , line : 41usize , placeholders : & [] , regex : "^The OpenSSL extension must be enabled to use the NTLM authenticator\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/swiftmailer/swiftmailer/lib/classes/Swift/Transport/Esmtp/Auth/NTLMAuthenticator.php" , line : 45usize , placeholders : & [] , regex : "^The BCMath functions must be enabled to use the NTLM authenticator\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/swiftmailer/swiftmailer/lib/classes/Swift/Transport/LoadBalancedTransport.php" , line : 149usize , placeholders : & [] , regex : "^All Transports in LoadBalancedTransport failed, or no Transports available$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/swiftmailer/swiftmailer/lib/classes/Swift/ByteStream/FileByteStream.php" , line : 45usize , placeholders : & [] , regex : "^The path cannot be empty$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/swiftmailer/swiftmailer/lib/classes/Swift/ByteStream/FileByteStream.php" , line : 198usize , placeholders : & [] , regex : "^Unable to copy the file to make it seekable, sys_temp_dir is not writable, php://memory not available$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/swiftmailer/swiftmailer/lib/classes/Swift/ByteStream/TemporaryFileByteStream.php" , line : 21usize , placeholders : & [] , regex : "^Failed to retrieve temporary file name\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/swiftmailer/swiftmailer/lib/classes/Swift/ByteStream/TemporaryFileByteStream.php" , line : 30usize , placeholders : & [] , regex : "^Failed to get temporary file content\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/guzzlehttp/psr7/src/Request.php" , line : 80usize , placeholders : & [] , regex : "^Invalid request target provided; cannot contain whitespace$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/guzzlehttp/psr7/src/Request.php" , line : 149usize , placeholders : & [] , regex : "^Method must be a non\\-empty string\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/guzzlehttp/psr7/src/Uri.php" , line : 72usize , placeholders : & ["$uri"] , regex : "^Unable to parse URI: (.*)$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/guzzlehttp/psr7/src/Uri.php" , line : 625usize , placeholders : & [] , regex : "^Scheme must be a string$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/guzzlehttp/psr7/src/Uri.php" , line : 641usize , placeholders : & [] , regex : "^User info must be a string$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/guzzlehttp/psr7/src/Uri.php" , line : 661usize , placeholders : & [] , regex : "^Host must be a string$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/guzzlehttp/psr7/src/Uri.php" , line : 750usize , placeholders : & [] , regex : "^Path must be a string$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/guzzlehttp/psr7/src/Uri.php" , line : 772usize , placeholders : & [] , regex : "^Query and fragment must be a string$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/guzzlehttp/psr7/src/Uri.php" , line : 795usize , placeholders : & [] , regex : "^The path of a URI without an authority must not start with two slashes \"//\"$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/guzzlehttp/psr7/src/Uri.php" , line : 798usize , placeholders : & [] , regex : "^A relative URI must not have a path beginning with a segment containing a colon$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/guzzlehttp/psr7/src/UploadedFile.php" , line : 102usize , placeholders : & [] , regex : "^Invalid stream or file provided for UploadedFile$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/guzzlehttp/psr7/src/UploadedFile.php" , line : 116usize , placeholders : & [] , regex : "^Upload file error status must be an integer$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/guzzlehttp/psr7/src/UploadedFile.php" , line : 122usize , placeholders : & [] , regex : "^Invalid error status for UploadedFile$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/guzzlehttp/psr7/src/UploadedFile.php" , line : 138usize , placeholders : & [] , regex : "^Upload file size must be an integer$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/guzzlehttp/psr7/src/UploadedFile.php" , line : 174usize , placeholders : & [] , regex : "^Upload file client filename must be a string or null$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/guzzlehttp/psr7/src/UploadedFile.php" , line : 190usize , placeholders : & [] , regex : "^Upload file client media type must be a string or null$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/guzzlehttp/psr7/src/UploadedFile.php" , line : 221usize , placeholders : & [] , regex : "^Cannot retrieve stream due to upload error$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/guzzlehttp/psr7/src/UploadedFile.php" , line : 225usize , placeholders : & [] , regex : "^Cannot retrieve stream after it has already been moved$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/guzzlehttp/psr7/src/UploadedFile.php" , line : 264usize , placeholders : & [] , regex : "^Invalid path provided for move operation; must be a non\\-empty string$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/guzzlehttp/psr7/src/Utils.php" , line : 426usize , placeholders : & [] , regex : "^URI must be a string or UriInterface$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/guzzlehttp/psr7/src/Stream.php" , line : 50usize , placeholders : & [] , regex : "^Stream must be a resource$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/guzzlehttp/psr7/src/Stream.php" , line : 92usize , placeholders : & [] , regex : "^Stream is detached$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/guzzlehttp/psr7/src/Stream.php" , line : 98usize , placeholders : & [] , regex : "^Unable to read stream contents$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/guzzlehttp/psr7/src/Stream.php" , line : 170usize , placeholders : & [] , regex : "^Stream is detached$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/guzzlehttp/psr7/src/Stream.php" , line : 179usize , placeholders : & [] , regex : "^Stream is detached$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/guzzlehttp/psr7/src/Stream.php" , line : 185usize , placeholders : & [] , regex : "^Unable to determine stream position$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/guzzlehttp/psr7/src/Stream.php" , line : 201usize , placeholders : & [] , regex : "^Stream is detached$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/guzzlehttp/psr7/src/Stream.php" , line : 204usize , placeholders : & [] , regex : "^Stream is not seekable$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/guzzlehttp/psr7/src/Stream.php" , line : 215usize , placeholders : & [] , regex : "^Stream is detached$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/guzzlehttp/psr7/src/Stream.php" , line : 218usize , placeholders : & [] , regex : "^Cannot read from non\\-readable stream$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/guzzlehttp/psr7/src/Stream.php" , line : 221usize , placeholders : & [] , regex : "^Length parameter cannot be negative$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/guzzlehttp/psr7/src/Stream.php" , line : 230usize , placeholders : & [] , regex : "^Unable to read from stream$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/guzzlehttp/psr7/src/Stream.php" , line : 239usize , placeholders : & [] , regex : "^Stream is detached$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/guzzlehttp/psr7/src/Stream.php" , line : 242usize , placeholders : & [] , regex : "^Cannot write to a non\\-writable stream$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/guzzlehttp/psr7/src/Stream.php" , line : 250usize , placeholders : & [] , regex : "^Unable to write to stream$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/guzzlehttp/psr7/src/MultipartStream.php" , line : 86usize , placeholders : & ["$key"] , regex : "^A '(.*)' key is required$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/guzzlehttp/psr7/src/Message.php" , line : 32usize , placeholders : & [] , regex : "^Unknown message type$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/guzzlehttp/psr7/src/Message.php" , line : 121usize , placeholders : & [] , regex : "^Invalid message$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/guzzlehttp/psr7/src/Message.php" , line : 129usize , placeholders : & [] , regex : "^Invalid message: Missing header delimiter$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/guzzlehttp/psr7/src/Message.php" , line : 137usize , placeholders : & [] , regex : "^Invalid message: Missing status line$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/guzzlehttp/psr7/src/Message.php" , line : 154usize , placeholders : & [] , regex : "^Invalid header syntax: Obsolete line folding$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/guzzlehttp/psr7/src/Message.php" , line : 157usize , placeholders : & [] , regex : "^Invalid header syntax$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/guzzlehttp/psr7/src/Message.php" , line : 210usize , placeholders : & [] , regex : "^Invalid request string$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/guzzlehttp/psr7/src/StreamDecoratorTrait.php" , line : 37usize , placeholders : & ["$name"] , regex : "^(.*) not found on class$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/guzzlehttp/psr7/src/StreamDecoratorTrait.php" , line : 150usize , placeholders : & [] , regex : "^Not implemented$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/guzzlehttp/psr7/src/Query.php" , line : 88usize , placeholders : & [] , regex : "^Invalid type$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/guzzlehttp/psr7/src/BufferStream.php" , line : 86usize , placeholders : & [] , regex : "^Cannot seek a BufferStream$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/guzzlehttp/psr7/src/BufferStream.php" , line : 96usize , placeholders : & [] , regex : "^Cannot determine the position of a BufferStream$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/guzzlehttp/psr7/src/LimitStream.php" , line : 119usize , placeholders : & ["$offset"] , regex : "^Could not seek to stream offset (.*)$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/guzzlehttp/psr7/src/ServerRequest.php" , line : 100usize , placeholders : & [] , regex : "^Invalid value in files specification$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/guzzlehttp/psr7/src/NoSeekStream.php" , line : 18usize , placeholders : & [] , regex : "^Cannot seek a NoSeekStream$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/guzzlehttp/psr7/src/Response.php" , line : 145usize , placeholders : & [] , regex : "^Status code must be an integer value\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/guzzlehttp/psr7/src/Response.php" , line : 152usize , placeholders : & [] , regex : "^Status code must be an integer value between 1xx and 5xx\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/guzzlehttp/psr7/src/AppendStream.php" , line : 54usize , placeholders : & [] , regex : "^Each stream must be readable$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/guzzlehttp/psr7/src/AppendStream.php" , line : 156usize , placeholders : & [] , regex : "^This AppendStream is not seekable$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/guzzlehttp/psr7/src/AppendStream.php" , line : 158usize , placeholders : & [] , regex : "^The AppendStream can only seek with SEEK_SET$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/guzzlehttp/psr7/src/AppendStream.php" , line : 239usize , placeholders : & [] , regex : "^Cannot write to an AppendStream$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/guzzlehttp/psr7/src/FnStream.php" , line : 66usize , placeholders : & [] , regex : "^FnStream should never be unserialized$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/guzzlehttp/psr7/src/MessageTrait.php" , line : 172usize , placeholders : & [] , regex : "^Header value can not be an empty array\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/guzzlehttp/psr7/src/MessageTrait.php" , line : 226usize , placeholders : & [] , regex : "^Header name can not be empty\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/guzzlehttp/psr7/src/CachingStream.php" , line : 66usize , placeholders : & [] , regex : "^Invalid whence$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/guzzlehttp/psr7/src/PumpStream.php" , line : 103usize , placeholders : & [] , regex : "^Cannot seek a PumpStream$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/guzzlehttp/psr7/src/PumpStream.php" , line : 113usize , placeholders : & [] , regex : "^Cannot write to a PumpStream$" , }
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
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/guzzlehttp/guzzle/src/Handler/CurlMultiHandler.php" , line : 94usize , placeholders : & [] , regex : "^Can not get other property as '_mh'\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/guzzlehttp/guzzle/src/Handler/CurlMultiHandler.php" , line : 100usize , placeholders : & [] , regex : "^Can not initialize curl multi handle\\.$" , }
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
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/guzzlehttp/guzzle/src/Pool.php" , line : 62usize , placeholders : & [] , regex : "^Each value yielded by the iterator must be a Psr7\\\\Http\\\\Message\\\\RequestInterface or a callable that returns a promise that fulfills with a Psr7\\\\Message\\\\Http\\\\ResponseInterface object\\.$" , }
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
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/guzzlehttp/guzzle/src/Utils.php" , line : 102usize , placeholders : & [] , regex : "^GuzzleHttp requires cURL, the allow_url_fopen ini setting, or a custom HTTP handler\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/guzzlehttp/guzzle/src/Utils.php" , line : 224usize , placeholders : & [] , regex : "^Empty host provided$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/guzzlehttp/guzzle/src/Utils.php" , line : 380usize , placeholders : & [] , regex : "^ext\\-idn or symfony/polyfill\\-intl\\-idn not loaded or too old$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/guzzlehttp/guzzle/src/Middleware.php" , line : 33usize , placeholders : & [] , regex : "^cookies must be an instance of GuzzleHttp\\\\Cookie\\\\CookieJarInterface$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/guzzlehttp/guzzle/src/Middleware.php" , line : 88usize , placeholders : & [] , regex : "^history container must be an array or object implementing ArrayAccess$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/guzzlehttp/guzzle/src/Cookie/SetCookie.php" , line : 80usize , placeholders : & [] , regex : "^Unable to replace the default values for the Cookie\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/guzzlehttp/guzzle/src/Cookie/FileCookieJar.php" , line : 69usize , placeholders : & ["$filename"] , regex : "^Unable to save file (.*)$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/guzzlehttp/guzzle/src/Cookie/FileCookieJar.php" , line : 86usize , placeholders : & ["$filename"] , regex : "^Unable to load file (.*)$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/guzzlehttp/guzzle/src/Cookie/FileCookieJar.php" , line : 98usize , placeholders : & ["$filename"] , regex : "^Invalid cookie file: (.*)$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/guzzlehttp/guzzle/src/Cookie/SessionCookieJar.php" , line : 74usize , placeholders : & [] , regex : "^Invalid cookie data$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/guzzlehttp/guzzle/src/RedirectMiddleware.php" , line : 61usize , placeholders : & [] , regex : "^allow_redirects must be true, false, or array$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/guzzlehttp/guzzle/src/RedirectMiddleware.php" , line : 155usize , placeholders : & ["$max"] , regex : "^Will not follow more than (.*) redirects$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/guzzlehttp/guzzle/src/HandlerStack.php" , line : 206usize , placeholders : & [] , regex : "^No handler has been specified$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/guzzlehttp/guzzle/src/HandlerStack.php" , line : 228usize , placeholders : & ["$name"] , regex : "^Middleware not found: (.*)$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/guzzlehttp/promises/src/FulfilledPromise.php" , line : 19usize , placeholders : & [] , regex : "^You cannot create a FulfilledPromise with a promise\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/guzzlehttp/promises/src/FulfilledPromise.php" , line : 71usize , placeholders : & [] , regex : "^Cannot resolve a fulfilled promise$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/guzzlehttp/promises/src/FulfilledPromise.php" , line : 77usize , placeholders : & [] , regex : "^Cannot reject a fulfilled promise$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/guzzlehttp/promises/src/Utils.php" , line : 222usize , placeholders : & [] , regex : "^Not enough promises to fulfill count$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/guzzlehttp/promises/src/Promise.php" , line : 131usize , placeholders : & [] , regex : "^Cannot fulfill or reject a promise with itself$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/guzzlehttp/promises/src/RejectedPromise.php" , line : 19usize , placeholders : & [] , regex : "^You cannot create a RejectedPromise with a promise\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/guzzlehttp/promises/src/RejectedPromise.php" , line : 77usize , placeholders : & [] , regex : "^Cannot resolve a rejected promise$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/guzzlehttp/promises/src/RejectedPromise.php" , line : 83usize , placeholders : & [] , regex : "^Cannot reject a rejected promise$" , }
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
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/microsoft/azure-storage-common/src/Common/Internal/Validate.php" , line : 124usize , placeholders : & [] , regex : "^double$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/microsoft/azure-storage-common/src/Common/Internal/Validate.php" , line : 194usize , placeholders : & [] , regex : "^DateTimeInterface$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/microsoft/azure-storage-common/src/Common/Internal/Validate.php" , line : 337usize , placeholders : & [] , regex : "^object$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/ramsey/collection/src/Queue.php" , line : 103usize , placeholders : & [] , regex : "^Can't return element from Queue\\. Queue is empty\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/ramsey/collection/src/Queue.php" , line : 159usize , placeholders : & [] , regex : "^Can't return element from Queue\\. Queue is empty\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/ramsey/collection/src/Tool/ValueExtractorTrait.php" , line : 43usize , placeholders : & [] , regex : "^Unable to extract a value from a non\\-object$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/ramsey/collection/src/DoubleEndedQueue.php" , line : 117usize , placeholders : & [] , regex : "^Can't return element from Queue\\. Queue is empty\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/ramsey/collection/src/DoubleEndedQueue.php" , line : 162usize , placeholders : & [] , regex : "^Can't return element from Queue\\. Queue is empty\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/ramsey/collection/src/AbstractCollection.php" , line : 129usize , placeholders : & [] , regex : "^Can't determine first item\\. Collection is empty$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/ramsey/collection/src/AbstractCollection.php" , line : 146usize , placeholders : & [] , regex : "^Can't determine last item\\. Collection is empty$" , }
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
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/ramsey/uuid/src/Codec/StringCodec.php" , line : 95usize , placeholders : & [] , regex : "^\\$bytes string should contain 16 characters\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/ramsey/uuid/src/Codec/OrderedTimeCodec.php" , line : 64usize , placeholders : & [] , regex : "^Expected RFC 4122 version 1 \\(time\\-based\\) UUID$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/ramsey/uuid/src/Codec/OrderedTimeCodec.php" , line : 88usize , placeholders : & [] , regex : "^\\$bytes string should contain 16 characters\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/ramsey/uuid/src/Lazy/LazyUuidFromString.php" , line : 503usize , placeholders : & [] , regex : "^Not a time\\-based UUID$" , }
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
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/ramsey/uuid/src/Generator/DefaultTimeGenerator.php" , line : 142usize , placeholders : & [] , regex : "^Invalid node value$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/ramsey/uuid/src/Type/Time.php" , line : 105usize , placeholders : & [] , regex : "^Attempted to unserialize an invalid value$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/ramsey/uuid/src/Type/Hexadecimal.php" , line : 53usize , placeholders : & [] , regex : "^Value must be a hexadecimal number$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/ramsey/uuid/src/Provider/Node/SystemNodeProvider.php" , line : 63usize , placeholders : & [] , regex : "^Unable to fetch a node for this system$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/ramsey/uuid/src/Provider/Node/FallbackNodeProvider.php" , line : 56usize , placeholders : & [] , regex : "^Unable to find a suitable node provider$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/ramsey/uuid/src/Provider/Node/StaticNodeProvider.php" , line : 47usize , placeholders : & [] , regex : "^Static node value cannot be greater than 12 hexadecimal characters$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/3rdparty/ramsey/uuid/src/Guid/Fields.php" , line : 84usize , placeholders : & [] , regex : "^The byte string received does not contain a valid version$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/core/Controller/ClientFlowLoginV2Controller.php" , line : 219usize , placeholders : & [] , regex : "^login name does not match$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/core/Controller/ClientFlowLoginV2Controller.php" , line : 351usize , placeholders : & [] , regex : "^Login token not set in session$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Error , path : "/core/Controller/LostController.php" , line : 264usize , placeholders : & [] , regex : "^$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/core/Controller/LostController.php" , line : 310usize , placeholders : & [] , regex : "^Could not send reset e\\-mail, 5 of them were already sent in the last 30 minutes$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/core/Controller/LostController.php" , line : 362usize , placeholders : & [] , regex : "^User is disabled$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/core/Controller/LostController.php" , line : 376usize , placeholders : & [] , regex : "^Could not find user$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/core/Controller/WhatsNewController.php" , line : 78usize , placeholders : & [] , regex : "^Acting user cannot be resolved$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/core/Controller/WhatsNewController.php" , line : 118usize , placeholders : & [] , regex : "^Acting user cannot be resolved$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Error , path : "/core/Controller/GuestAvatarController.php" , line : 101usize , placeholders : & [] , regex : "^error while creating guest avatar$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Warn , path : "/core/Controller/TwoFactorChallengeController.php" , line : 218usize , placeholders : & ["$uid" , "$ip"] , regex : "^Two\\-factor challenge failed: (.*) \\(Remote IP: (.*)\\)$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/core/Controller/CollaborationResourcesController.php" , line : 75usize , placeholders : & [] , regex : "^Not found$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/core/Controller/CollaborationResourcesController.php" , line : 265usize , placeholders : & [] , regex : "^Can not access collection$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/core/Controller/CollaborationResourcesController.php" , line : 292usize , placeholders : & [] , regex : "^Can not access resource$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/core/Controller/ProfileApiController.php" , line : 79usize , placeholders : & [] , regex : "^User does not exist$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/core/Controller/ProfileApiController.php" , line : 83usize , placeholders : & [] , regex : "^Users can only edit their own visibility settings$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Debug , path : "/core/Controller/WebAuthnController.php" , line : 72usize , placeholders : & [] , regex : "^Starting WebAuthn login$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Debug , path : "/core/Controller/WebAuthnController.php" , line : 74usize , placeholders : & [] , regex : "^Converting login name to UID$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Debug , path : "/core/Controller/WebAuthnController.php" , line : 96usize , placeholders : & [] , regex : "^Validating WebAuthn login$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Debug , path : "/core/Controller/WebAuthnController.php" , line : 99usize , placeholders : & [] , regex : "^Trying to finish WebAuthn login without session data$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/core/Controller/AppPasswordController.php" , line : 86usize , placeholders : & [] , regex : "^You cannot request an new apppassword with an apppassword$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/core/Controller/AppPasswordController.php" , line : 131usize , placeholders : & [] , regex : "^no app password in use$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/core/Controller/AppPasswordController.php" , line : 139usize , placeholders : & [] , regex : "^could not remove apptoken$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/core/Controller/AppPasswordController.php" , line : 151usize , placeholders : & [] , regex : "^no app password in use$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/core/Controller/AppPasswordController.php" , line : 159usize , placeholders : & [] , regex : "^could not rotate apptoken$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/core/Controller/ClientFlowLoginController.php" , line : 386usize , placeholders : & [] , regex : "^login name does not match$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/core/Notification/CoreNotifier.php" , line : 100usize , placeholders : & [] , regex : "^Invalid subject$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/core/Db/LoginFlowV2Mapper.php" , line : 96usize , placeholders : & [] , regex : "^Token expired$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/core/Command/User/Setting.php" , line : 129usize , placeholders : & [] , regex : "^The \"default\\-value\" option can only be used when specifying a key\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/core/Command/User/Setting.php" , line : 133usize , placeholders : & [] , regex : "^The value argument can only be used when specifying a key\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/core/Command/User/Setting.php" , line : 136usize , placeholders : & [] , regex : "^The value argument can not be used together with \"default\\-value\"\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/core/Command/User/Setting.php" , line : 139usize , placeholders : & [] , regex : "^The \"update\\-only\" option can only be used together with \"value\"\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/core/Command/User/Setting.php" , line : 143usize , placeholders : & [] , regex : "^The \"delete\" option can only be used when specifying a key\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/core/Command/User/Setting.php" , line : 146usize , placeholders : & [] , regex : "^The \"delete\" option can not be used together with \"default\\-value\"\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/core/Command/User/Setting.php" , line : 149usize , placeholders : & [] , regex : "^The \"delete\" option can not be used together with \"value\"\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/core/Command/User/Setting.php" , line : 152usize , placeholders : & [] , regex : "^The \"error\\-if\\-not\\-exists\" option can only be used together with \"delete\"\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/core/Command/Encryption/EncryptAll.php" , line : 119usize , placeholders : & [] , regex : "^Server side encryption is not enabled$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/core/Command/Encryption/ChangeKeyStorageRoot.php" , line : 144usize , placeholders : & [] , regex : "^New root folder doesn't exist\\. Please create the folder or check the permissions and try again\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/core/Command/Encryption/ChangeKeyStorageRoot.php" , line : 153usize , placeholders : & [] , regex : "^Can't access the new root folder\\. Please check the permissions and make sure that the folder is in your data folder$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/core/Command/Encryption/ChangeKeyStorageRoot.php" , line : 264usize , placeholders : & ["$path"] , regex : "^new folder '(.*)' already exists$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/core/Command/Log/File.php" , line : 125usize , placeholders : & [] , regex : "^Error parsing log rotation file size$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/core/Command/Log/File.php" , line : 129usize , placeholders : & [] , regex : "^Log rotation file size must be non\\-negative$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/core/Command/Log/Manage.php" , line : 125usize , placeholders : & [] , regex : "^Invalid backend$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/core/Command/Log/Manage.php" , line : 158usize , placeholders : & [] , regex : "^Invalid log level string$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/core/Command/Log/Manage.php" , line : 179usize , placeholders : & [] , regex : "^Invalid log level number$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/core/Command/L10n/CreateJs.php" , line : 135usize , placeholders : & ["$phpFile"] , regex : "^PHP translation file <(.*)> does not exist\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/core/Command/Maintenance/Install.php" , line : 127usize , placeholders : & ["$db"] , regex : "^Database <(.*)> is not supported\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/core/Command/Maintenance/Install.php" , line : 154usize , placeholders : & [] , regex : "^Database user not provided\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/core/Command/Maintenance/Install.php" , line : 157usize , placeholders : & [] , regex : "^Database name not provided\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/core/Command/Db/Migrations/GenerateCommand.php" , line : 213usize , placeholders : & [] , regex : "^Failed to generate new migration step\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/core/Command/Db/Migrations/GenerateCommand.php" , line : 225usize , placeholders : & ["$directory"] , regex : "^Could not create folder \"(.*)\"$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/core/Command/Db/Migrations/GenerateCommand.php" , line : 231usize , placeholders : & ["$directory"] , regex : "^Could not create folder \"(.*)\"$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/core/Command/Db/ConvertType.php" , line : 144usize , placeholders : & [] , regex : "^Converting to SQLite \\(sqlite3\\) is currently not supported\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/core/Command/Db/ConvertType.php" , line : 159usize , placeholders : & [] , regex : "^The \\-\\-clear\\-schema option is not supported when converting to Oracle \\(oci\\)\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/core/Command/Base.php" , line : 146usize , placeholders : & [] , regex : "^Command interrupted by user$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/core/Command/Config/Import.php" , line : 129usize , placeholders : & [] , regex : "^The file must contain a valid json array$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/core/Command/Config/Import.php" , line : 159usize , placeholders : & [] , regex : "^The system config array is not an array$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/core/Command/Config/Import.php" , line : 167usize , placeholders : & [] , regex : "^The apps config array is not an array$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/core/Command/Config/System/SetConfig.php" , line : 95usize , placeholders : & [] , regex : "^Config parameter does not exist$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/core/Command/Config/System/SetConfig.php" , line : 116usize , placeholders : & [] , regex : "^Non\\-numeric value specified$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/core/Command/Config/System/SetConfig.php" , line : 126usize , placeholders : & [] , regex : "^Non\\-numeric value specified$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/core/Command/Config/System/SetConfig.php" , line : 150usize , placeholders : & [] , regex : "^Unable to parse value as boolean$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/core/Command/Config/System/SetConfig.php" , line : 168usize , placeholders : & [] , regex : "^Invalid type$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/core/Command/Config/System/SetConfig.php" , line : 194usize , placeholders : & [] , regex : "^Config parameter does not exist$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/core/Command/Config/System/DeleteConfig.php" , line : 109usize , placeholders : & [] , regex : "^Config parameter does not exist$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/core/Command/Config/System/DeleteConfig.php" , line : 112usize , placeholders : & [] , regex : "^Config parameter does not exist$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Info , path : "/core/BackgroundJobs/BackgroundCleanupUpdaterBackupsJob.php" , line : 60usize , placeholders : & ["$backupFolderPath"] , regex : "^(.*) exists \\- start to clean it up$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Info , path : "/core/BackgroundJobs/BackgroundCleanupUpdaterBackupsJob.php" , line : 86usize , placeholders : & ["$dir"] , regex : "^Removing (.*) \\.\\.\\.$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Info , path : "/core/BackgroundJobs/BackgroundCleanupUpdaterBackupsJob.php" , line : 89usize , placeholders : & [] , regex : "^Cleanup finished$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Info , path : "/core/BackgroundJobs/BackgroundCleanupUpdaterBackupsJob.php" , line : 91usize , placeholders : & ["$backupFolderPath"] , regex : "^Could not find updater directory (.*) \\- cleanup step not needed$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/core/Service/LoginFlowV2Service.php" , line : 87usize , placeholders : & [] , regex : "^Invalid token$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/core/Service/LoginFlowV2Service.php" , line : 95usize , placeholders : & [] , regex : "^Token not yet ready$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/core/Service/LoginFlowV2Service.php" , line : 106usize , placeholders : & [] , regex : "^Apptoken could not be decrypted$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/core/Service/LoginFlowV2Service.php" , line : 121usize , placeholders : & [] , regex : "^Login token invalid$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/core/Service/LoginFlowV2Service.php" , line : 242usize , placeholders : & [] , regex : "^Could not initialize keys$" , }
,
	crate :: LoggingStatement { level : crate :: LogLevel :: Exception , path : "/core/Service/LoginFlowV2Service.php" , line : 247usize , placeholders : & [] , regex : "^OpenSSL reported a problem$" , }
];

