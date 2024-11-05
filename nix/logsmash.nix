{ stdenv
, makeRustPlatform
, rust-bin
, lib
, extracted-logs-rust
,
}:
let
  inherit (lib) sourceByRegex readFile;
  src = sourceByRegex ../. [ "Cargo.*" "(src|data)(/.*)?" ];
  rustPlatform = makeRustPlatform {
    cargo = rust-bin.stable.latest.default;
    rustc = rust-bin.stable.latest.default;
  };
  version = (fromTOML (readFile ../Cargo.toml)).package.version;
in
rustPlatform.buildRustPackage rec {
  pname = "logsmash";
  inherit version src;

  preBuild = ''
    rm -r data/src/data
    cp -r ${extracted-logs-rust} data/src/data
  '';

  cargoLock = {
    lockFile = ../Cargo.lock;
  };
}
