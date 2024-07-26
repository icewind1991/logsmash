{
  packages,
  lib,
  writeTextDir,
}: let
  inherit (builtins) head attrNames concatStringsSep replaceStrings;
  inherit (lib.attrsets) mapAttrsToList;
  inherit (lib.lists) flatten last;
  module_name = name: version: "${name}_${replaceStrings ["."] ["_"] version}";
  matches_for = name: mapAttrsToList (version: data: ''("${name}", "${version}") => Some(("${data.prefix or ""}", ${module_name name version}::STATEMENTS)),'');
  imports_for = name: mapAttrsToList (version: _: ''mod ${module_name name version};'');
  matches = flatten (mapAttrsToList matches_for packages);
  imports = flatten (mapAttrsToList imports_for packages);
  min_versions = mapAttrsToList (name: versions: ''("${name}", "${head (attrNames versions)}"),'') packages;
  max_versions = mapAttrsToList (name: versions: ''("${name}", "${last (attrNames versions)}"),'') packages;
  code = ''
    use crate::LoggingStatement;

    ${concatStringsSep "\n" imports}

    pub const MIN_VERSIONS: &[(&str, &str)] = &[
          ${concatStringsSep "\n" min_versions}
        ];

    pub const MAX_VERSIONS: &[(&str, &str)] = &[
      ${concatStringsSep "\n" max_versions}
    ];

    pub fn get_statements_for(name: &str, version: &str) -> Option<(&'static str, &'static [LoggingStatement])> {
        match (name, version) {
            ${concatStringsSep "\n" matches}
            _ => None,
        }
    }
  '';
in writeTextDir "mod.rs" code