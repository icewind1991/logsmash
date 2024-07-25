# logsmash_data

Analysis tool for Nextcloud logs files

![Logsmash screenshot](./screenshots/screenshot.png)

## Usage

```bash
logsmash ./logfile.log
```

### Updating baked data

rm -r data/src/data
nix build .#extracted-logs-rust
cp -rL result data/src/data && chmod -R +w data/src/data