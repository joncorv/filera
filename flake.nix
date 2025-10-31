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

          yarnOfflineCache = pkgs.fetchYarnDeps {
            yarnLock = "${self}/yarn.lock";
            hash = "sha256-OoKYgLmWI39w6UAshCbDYNK7VW6SPHE8A9bN/20d13A=";
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
            # Go up to parent directory and copy to writable location
            cd ..
            parentDir=$(pwd)
            cp -r $parentDir /build/filera-build
            chmod -R +w /build/filera-build
            cd /build/filera-build

            export HOME=$(mktemp -d)

            # Copy yarn cache to writable location
            export YARN_CACHE_FOLDER=/build/yarn-cache
            cp -r $yarnOfflineCache $YARN_CACHE_FOLDER
            chmod -R +w $YARN_CACHE_FOLDER

            fixup-yarn-lock yarn.lock
            yarn install --offline --frozen-lockfile --ignore-scripts --no-progress --non-interactive

            yarn build

            # Copy built files back to original location
            cp -r dist $parentDir/

            cd $parentDir/src-tauri
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
