{
  stdenvNoCC,
  logging-extractor,
  fetchzip,
  name,
  version,
  url,
  major,
  sha256,
}:
stdenvNoCC.mkDerivation rec {
  pname = "extractor-logs-${name}-${major}";
  inherit version;

  src = fetchzip {
    inherit url sha256;
  };

  nativeBuildInputs = [logging-extractor];

  buildPhase = ''
    logging-extractor . > logs.json
  '';

  installPhase = ''
    mkdir -p $out/${name}/${major}
    cp logs.json $out/${name}/${major}
  '';
}
