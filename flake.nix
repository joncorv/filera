{
  description = "Filera - A powerful batch file renaming tool";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, flake-utils }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = nixpkgs.legacyPackages.${system};
        
        frontend = pkgs.mkYarnModules {
          pname = "filera-frontend";
          version = "0.4.2";
          packageJSON = "${self}/package.json";
          yarnLock = "${self}/yarn.lock";
        };
      in
      {
        packages.default = pkgs.rustPlatform.buildRustPackage {
          pname = "filera";
          version = "0.4.2";
          src = self;

          sourceRoot = "source/src-tauri";

          cargoLock = {
            lockFile = "${self}/src-tauri/Cargo.lock";
          };

          nativeBuildInputs = with pkgs; [
            pkg-config
            wrapGAppsHook3
            nodejs
            yarn
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
            cp -r ${frontend}/node_modules ./
            chmod -R +w node_modules
            export HOME=$(mktemp -d)
            yarn build
            cd src-tauri
          '';

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
