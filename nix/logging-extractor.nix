{ stdenv
, rustPlatform
, lib
,
}:
let
  inherit (lib.sources) sourceByRegex;
  src = sourceByRegex ../logging-extractor [ "Cargo.*" "(src|tests|test-data)(/.*)?" ];
in
rustPlatform.buildRustPackage rec {
  pname = "logging-extractor";
  version = "0.1.0";

  inherit src;

  cargoLock = {
    lockFile = ../logging-extractor/Cargo.lock;
  };
}
