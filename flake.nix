{
  inputs = {
    nixpkgs.url = "nixpkgs/nixos-24.05";
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
    withOverlays = import ./nix/overlay.nix;
    packages = {
      logsmash = { logsmash }: logsmash;
      extracted-logs-rust = { extracted-logs-rust }: extracted-logs-rust;
    };
  };
}
