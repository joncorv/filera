{
  description = "Filera: rename, move, and organize files in bulk";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs =
    {
      self,
      nixpkgs,
      flake-utils,
      rust-overlay,
    }:
    flake-utils.lib.eachDefaultSystem (
      system:
      let
        pkgs = import nixpkgs {
          inherit system;
          overlays = [ rust-overlay.overlays.default ];
        };

        # Single source of truth for the Rust version: ./rust-toolchain.toml
        rustToolchain = pkgs.rust-bin.fromRustupToolchainFile ./rust-toolchain.toml;
        rustPlatform = pkgs.makeRustPlatform {
          cargo = rustToolchain;
          rustc = rustToolchain;
        };
      in
      {
        packages.default = pkgs.callPackage ./default.nix { inherit rustPlatform; };

        devShells.default = pkgs.mkShell {
          # Don't use inputsFrom - we want a clean dev environment
          nativeBuildInputs = with pkgs; [
            pkg-config
            rustToolchain # cargo, rustc, rustfmt, clippy, rust-analyzer
            nodejs
            cargo-tauri
            # Development tools
            cargo-watch
            typescript-language-server
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
            inter
            gsettings-desktop-schemas
            xdg-utils
          ];

          shellHook = ''
            export PKG_CONFIG_PATH="${pkgs.dbus.dev}/lib/pkgconfig:${pkgs.openssl.dev}/lib/pkgconfig:$PKG_CONFIG_PATH"
            export XDG_DATA_DIRS="${pkgs.gsettings-desktop-schemas}/share/gsettings-schemas/${pkgs.gsettings-desktop-schemas.name}:${pkgs.gtk3}/share/gsettings-schemas/${pkgs.gtk3.name}:$XDG_DATA_DIRS"
            export __EGL_VENDOR_LIBRARY_DIRS=${pkgs.mesa}/share/glvnd/egl_vendor.d
            export WEBKIT_DISABLE_DMABUF_RENDERER=1
            export WEBKIT_DISABLE_COMPOSITING_MODE=1
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
