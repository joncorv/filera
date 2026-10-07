{
  lib,
  stdenv,
  rustPlatform,
  fetchNpmDeps,
  cargo-tauri,
  fontconfig,
  glib-networking,
  inter,
  nodejs,
  npmHooks,
  openssl,
  pkg-config,
  webkitgtk_4_1,
  wrapGAppsHook4,
}:

rustPlatform.buildRustPackage rec {
  pname = "filera";
  version = "0.4.43";

  src = ./.;

  cargoHash = "sha256-Ak8oQ8YNwWIY5X7t5L9e2MiUYgUMNwvZXp0zm8E6PGM=";

  npmDeps = fetchNpmDeps {
    name = "${pname}-${version}-npm-deps";
    inherit src;
    hash = "sha256-LZu9de9pWszqmBERD/7RlZuNLL5BcSl/cVhGi56+JQw=";
  };

  nativeBuildInputs = [
    cargo-tauri.hook
    nodejs
    npmHooks.npmConfigHook
    pkg-config
  ]
  ++ lib.optionals stdenv.hostPlatform.isLinux [ wrapGAppsHook4 ];

  buildInputs = lib.optionals stdenv.hostPlatform.isLinux [
    fontconfig
    glib-networking
    inter
    openssl
    webkitgtk_4_1
  ];

  cargoRoot = "src-tauri";
  buildAndTestSubdir = cargoRoot;

  meta = with lib; {
    description = "Filera - A powerful, cross-platform batch file renaming tool";
    homepage = "https://github.com/joncorv/filera";
    license = licenses.mit;
    maintainers = [ ];
    mainProgram = "filera";
    platforms = platforms.linux ++ platforms.darwin;
  };
}
