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

          src = ./.;

          # Point to the Cargo.lock in src-tauri
          cargoLock = {
            lockFile = ./src-tauri/Cargo.lock;
          };

          # Build only the Tauri backend
          sourceRoot = "src-tauri/";

          nativeBuildInputs = with pkgs; [
            pkg-config
            wrapGAppsHook3
            cargo
            rustc
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

          # Build the frontend first
          preBuild = ''
            cd ..
            export HOME=$(mktemp -d)
            yarn install --frozen-lockfile
            yarn build
            cd src-tauri
          '';

          # Disable tests since this is a GUI app
          doCheck = false;

          postInstall = ''
            # Install desktop file
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
            # license = licenses.mit; # Uncomment and set when you choose a license
            maintainers = [ ];
            platforms = platforms.linux;
          };
        };
      }
    );
}
