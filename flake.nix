{
  inputs = {
    nixpkgs.url = "nixpkgs/nixos-24.11";
    flakelight = {
      url = "github:nix-community/flakelight";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    mill-scale = {
      url = "github:icewind1991/mill-scale";
      inputs.flakelight.follows = "flakelight";
    };
  };
  outputs = { mill-scale, ... }: mill-scale ./. {
    crossTargets = [
      "x86_64-unknown-linux-musl"
      "x86_64-pc-windows-gnu"
    ];
    packageOpts = { extracted-logs-rust, ... }: {
      preBuild = ''
        rm -r -f data/src/data
        cp -r ${extracted-logs-rust} data/src/data
      '';
    };
    withOverlays = import ./nix/overlay.nix;
    packages = {
      logsmash = { logsmash }: logsmash;
      extracted-logs-rust = { extracted-logs-rust }: extracted-logs-rust;
    };
  };
}
