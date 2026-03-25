{
  lib,
  stdenv,
  rustPlatform,
  fetchNpmDeps,
  cargo-tauri,
  glib-networking,
  nodejs,
  npmHooks,
  openssl,
  pkg-config,
  webkitgtk_4_1,
  wrapGAppsHook4,
}:

rustPlatform.buildRustPackage rec {
  pname = "filera";
  version = "0.4.11";

  src = ./.;

  cargoHash = "sha256-5v9oN4a/hebFSMFXwd1ZjXezRcTRBJfwc1yueNwg55I=";

  npmDeps = fetchNpmDeps {
    name = "${pname}-${version}-npm-deps";
    inherit src;
    hash = "sha256-G3bk2sWQUgR4xMYnueczK51gfXlatJQxfsJXZCHw++g=";
  };

  nativeBuildInputs = [
    cargo-tauri.hook
    nodejs
    npmHooks.npmConfigHook
    pkg-config
  ]
  ++ lib.optionals stdenv.hostPlatform.isLinux [ wrapGAppsHook4 ];

  buildInputs = lib.optionals stdenv.hostPlatform.isLinux [
    glib-networking
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
