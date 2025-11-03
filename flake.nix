{
  description = "Filera - A modern file management application";

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
        packages.default = pkgs.callPackage ./default.nix { };

        devShells.default = pkgs.mkShell {
          # Don't use inputsFrom - we want a clean dev environment
          nativeBuildInputs = with pkgs; [
            pkg-config
            cargo
            rustc
            nodejs
            cargo-tauri
            # Development tools
            rust-analyzer
            cargo-watch
            nodePackages.typescript-language-server
          ];

          buildInputs = with pkgs; [
            # DBus (required)
            dbus.dev

            # Tauri runtime dependencies
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
            openssl.dev
            fontconfig
            gsettings-desktop-schemas
            xdg-utils
          ];

          shellHook = ''
            export PKG_CONFIG_PATH="${pkgs.dbus.dev}/lib/pkgconfig:${pkgs.openssl.dev}/lib/pkgconfig:$PKG_CONFIG_PATH"
            export XDG_DATA_DIRS="${pkgs.gsettings-desktop-schemas}/share/gsettings-schemas/${pkgs.gsettings-desktop-schemas.name}:${pkgs.gtk3}/share/gsettings-schemas/${pkgs.gtk3.name}:$XDG_DATA_DIRS"
            echo "🚀 Filera development environment loaded!"
            echo "Run 'npm run tauri dev' to start the development server"
          '';
        };

        checks = {
          build = self.packages.${system}.default;
        };
      }
    );
}
