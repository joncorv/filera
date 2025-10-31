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
        
        yarnModules = pkgs.mkYarnModules {
          pname = "filera-modules";
          version = "0.4.2";
          packageJSON = "${self}/package.json";
          yarnLock = "${self}/yarn.lock";
        };
      in
      {
        packages.default = pkgs.stdenv.mkDerivation {
          pname = "filera";
          version = "0.4.2";
          src = self;

          nativeBuildInputs = with pkgs; [
            pkg-config
            wrapGAppsHook3
            cargo
            rustc
            cargo-tauri
            nodejs
            yarn
            makeWrapper
            xdg-utils
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

          configurePhase = ''
            export HOME=$(mktemp -d)
            
            # Use pre-fetched yarn modules
            ln -s ${yarnModules}/node_modules ./node_modules
          '';

          buildPhase = ''
            export TAURI_BUNDLER_TARGETS="none"
            yarn tauri build
          '';

          installPhase = ''
            mkdir -p $out/bin $out/share/applications
            
            cp src-tauri/target/release/filera $out/bin/
            
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
