{
  description = "Filera — a Tauri-based file manager";

  inputs = {
    # Pin nixpkgs for reproducibility
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-24.05";
    # Optionally include flake-utils for multi-platform convenience
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
        pkgs = import nixpkgs {
          inherit system;
        };
      in
      {
        # Development shell for contributors
        devShells.default = pkgs.mkShell {
          nativeBuildInputs = with pkgs; [
            pkg-config
            gobject-introspection
            cargo
            rustc
            nodejs
            yarn
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
          ];

          shellHook = ''
            export PKG_CONFIG_PATH="${pkgs.dbus.dev}/lib/pkgconfig:${pkgs.openssl.dev}/lib/pkgconfig:$PKG_CONFIG_PATH"
            export XDG_DATA_DIRS="${pkgs.gsettings-desktop-schemas}/share/gsettings-schemas/${pkgs.gsettings-desktop-schemas.name}:${pkgs.gtk3}/share/gsettings-schemas/${pkgs.gtk3.name}:$XDG_DATA_DIRS"
            echo "Filera Tauri development environment loaded!"
            echo "Run 'yarn tauri dev' to start development"
          '';
        };

        packages.default = pkgs.rustPlatform.buildRustPackage {
          pname = "filera";
          version = "0.4.2";

          # Include whole repo
          src = pkgs.lib.cleanSource ./.;

          cargoLock.lockFile = ./src-tauri/Cargo.lock;

          nativeBuildInputs = with pkgs; [
            pkg-config
            nodejs
            yarn
          ];
          buildInputs = with pkgs; [
            dbus.dev
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
          ];

          # Tell nix to unpack and cd into src-tauri manually
          preUnpack = ''
            echo ">>> Using manual unpack for src-tauri"
          '';
          unpackPhase = ''
            mkdir source
            tar -xf $src --strip-components=1 -C source
            cd source/src-tauri
            sourceRoot=$(pwd)
          '';

          preBuild = ''
            export XDG_DATA_DIRS="${pkgs.gsettings-desktop-schemas}/share/gsettings-schemas/${pkgs.gsettings-desktop-schemas.name}:${pkgs.gtk3}/share/gsettings-schemas/${pkgs.gtk3.name}:$XDG_DATA_DIRS"
          '';

          # Optional: package frontend before Rust build
          buildPhase = ''
            echo ">>> Building frontend"
            cd ..
            yarn install --frozen-lockfile
            yarn build
            cd src-tauri
            cargo build --release --frozen
          '';

          installPhase = ''
            mkdir -p $out/bin
            cp target/release/filera $out/bin/
          '';
        };

      }
    );
}
