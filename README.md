# logsmash_data

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

- Nextcloud server 29
- files_accesscontrol 1.19.1
- files_antivirus 5.5.7
- deck: 1.13.1

### Updating baked data

rm -r data/src/data
nix build .#extracted-logs-rust
cp -rL result data/src/data && chmod -R +w data/src/data