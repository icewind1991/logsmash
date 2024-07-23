{
  stdenvNoCC,
  logging-extractor,
  fetchzip,
  name,
  version,
  url,
  major,
  sha256,
  mode ? "json",
}: let
  ext = if mode == "rust" then "rs" else "json";
  cleanedMajor = builtins.replaceStrings ["."] ["_"] major;
in stdenvNoCC.mkDerivation rec {
  pname = "extractor-logs-${name}-${cleanedMajor}";
  inherit version;

  src = fetchzip {
    inherit url sha256;
  };

  nativeBuildInputs = [logging-extractor];

  buildPhase = ''
    logging-extractor . ${mode} > logs.${ext}
  '';

  installPhase = ''
    mkdir -p $out
    cp logs.* $out/${name}_${cleanedMajor}.${ext}
  '';
}
