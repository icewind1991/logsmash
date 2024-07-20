final: prev: let
  inherit (builtins) mapAttrs attrValues map;
  inherit (prev) symlinkJoin;
  inherit (prev.lib) importJSON;
  inherit (prev.lib.lists) flatten;
  packages = prev.lib.traceValSeq (importJSON ./versions.json);

  loggingFor = mode: name:
    mapAttrs (major: data: (final.callPackage ./extracted-logs.nix {
      inherit (data) url sha256 version;
      inherit name major mode;
    }));
in {
  logging-extractor = final.callPackage ./logging-extractor.nix {};
  extracted-logs-parts = mapAttrs (loggingFor "json") packages;
  extracted-logs-parts-rust = mapAttrs (loggingFor "rust") packages;
  extracted-logs = symlinkJoin {
    name = "extracted-logs";
    paths = flatten (map attrValues (attrValues final.extracted-logs-parts));
  };
  extracted-logs-rust = symlinkJoin {
    name = "extracted-logs";
    paths = flatten (map attrValues (attrValues final.extracted-logs-parts-rust));
  };
}
