{
  stdenv,
  makeRustPlatform,
  rust-bin,
  lib,
}: let
  inherit (lib.sources) sourceByRegex;
  src = sourceByRegex ../. ["Cargo.*" "(src|data)(/.*)?"];
  rustPlatform = makeRustPlatform {
    cargo = rust-bin.stable.latest.default;
    rustc = rust-bin.stable.latest.default;
  };
in
  rustPlatform.buildRustPackage rec {
    pname = "logsmash";
    version = "0.1.0";

    inherit src;

    cargoLock = {
      lockFile = ../Cargo.lock;
    };
  }
