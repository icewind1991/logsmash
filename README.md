# cloud-log-analyser

### Updating baked data

rm -r data/src/data
nix build .#extracted-logs-rust
cp -rL result data/src/data && chmod -R +w data/src/data