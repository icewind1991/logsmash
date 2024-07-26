# logsmash

Analysis tool for Nextcloud logs files

![Logsmash screenshot](./screenshots/screenshot.png)

## Usage

```bash
logsmash ./logfile.log
```

Once the log file is load a list of matched log statements or exception are shown and a histogram for the selected match
is show on top.

Selecting a match shows a list of distinct log items and selecting a log item shows a list of all occurrences of the log
message.

## Log matching

Logsmash tries to match the log item to the place in the source where the message originates from. This is either a call
to a log method or an exception being thrown.

A log item can sometimes be matched to multiple items that are deemed to be equally likely to be the origin of the log
item.
In those cases all matches will be shown.

If a log item cannot be matched to an origin it will added to the "Unmatched lines" at the bottom of the list.

## Supported data

Currently, the program can match against data from the following sources:

- Nextcloud server 24 - 29
- files_accesscontrol 1.19.1
- files_antivirus 5.5.7
- deck: 1.13.1
- calendar: 4.7.13
- contacts: 5.5.3, 6.0.0
- groupfolders: 16.0.7, 17.0.1
- guests: 3.1.0, 4.0.0
- spreed: 18.0.10, 19.0.7
- form: 4.2.4
- tasks: 0.16.0
- notes: 4.10.1
- richdocuments: 8.4.3
- collectives: 2.13.0
- onlyoffice: 9.3.0
- tables: 0.7.4
- mail: 3.7.5
- files_accesscontrol: 1.19.1
- files_accesscontrol: 1.20.0
- files_retention: 1.18.0

#### Updating baked data

Note that this is only needed when building with cargo, building with nix automatically uses the latest data.

```bash
rm -r data/src/data
nix build .#extracted-logs-rust
cp -rL result data/src/data && chmod -R +w data/src/data
```