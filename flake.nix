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
          inputsFrom = [ self.packages.${system}.default ];

          buildInputs = with pkgs; [
            rust-analyzer
            cargo-watch
            nodePackages.typescript-language-server

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
