{
  description = "Filera - A powerful batch file renaming tool";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs =
    {
      self,
      nixpkgs,
      flake-utils,
    }:
    flake-utils.lib.eachDefaultSystem (
      system:
      let
        pkgs = nixpkgs.legacyPackages.${system};
      in
      {
        packages.default = pkgs.rustPlatform.buildRustPackage {
          pname = "filera";
          version = "0.4.2";

          src = self;

          cargoLock = {
            lockFile = "${self}/src-tauri/Cargo.lock";
          };

          sourceRoot = "source/src-tauri";

          # Fetch yarn dependencies offline
          yarnOfflineCache = pkgs.fetchYarnDeps {
            yarnLock = "${self}/yarn.lock";
            hash = ""; # Will fail and tell you the correct hash
          };

          nativeBuildInputs = with pkgs; [
            pkg-config
            wrapGAppsHook3
            cargo
            rustc
            nodejs
            yarn
            fixup-yarn-lock
          ];

          buildInputs = with pkgs; [
            at-spi2-atk
            atkmm
            cairo
            gdk-pixbuf
            glib
            gtk3
            harfbuzz
            librsvg
            libsoup_3
            pango
            webkitgtk_4_1
            openssl
            dbus
            fontconfig
            gsettings-desktop-schemas
          ];

          preBuild = ''
            cd ..
            export HOME=$(mktemp -d)

            # Setup yarn offline cache
            export YARN_ENABLE_NETWORK=0
            export YARN_CACHE_FOLDER=$yarnOfflineCache

            fixup-yarn-lock yarn.lock
            yarn install --offline --frozen-lockfile --ignore-scripts --no-progress --non-interactive

            yarn build
            cd src-tauri
          '';

          doCheck = false;

          postInstall = ''
            mkdir -p $out/share/applications
            cat > $out/share/applications/filera.desktop <<EOF
            [Desktop Entry]
            Name=Filera
            Comment=Powerful batch file renaming tool
            Exec=$out/bin/filera
            Type=Application
            Categories=Utility;FileTools;
            Terminal=false
            EOF
          '';

          meta = with pkgs.lib; {
            description = "A powerful, cross-platform batch file renaming tool";
            homepage = "https://github.com/joncorv/filera";
            maintainers = [ ];
            platforms = platforms.linux;
          };
        };
      }
    );
}
