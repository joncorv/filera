{
  description = "Filera - A powerful, cross-platform batch file renaming tool";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
  };

  outputs =
    { self, nixpkgs }:
    let
      system = "x86_64-linux";
      pkgs = import nixpkgs { inherit system; };
    in
    {
      packages.${system}.default = pkgs.stdenv.mkDerivation {
        pname = "filera";
        version = "0.4.4";
        src = ./.;

        nativeBuildInputs = with pkgs; [
          pkg-config
          gobject-introspection
          cargo
          rustc
          nodejs
          yarn
          xdg-utils
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
          cacert
        ];

        buildPhase = ''
          export HOME=$PWD
          export PKG_CONFIG_PATH="${pkgs.dbus.dev}/lib/pkgconfig:${pkgs.openssl.dev}/lib/pkgconfig:$PKG_CONFIG_PATH"
          export XDG_DATA_DIRS="${pkgs.gsettings-desktop-schemas}/share/gsettings-schemas/${pkgs.gsettings-desktop-schemas.name}:${pkgs.gtk3}/share/gsettings-schemas/${pkgs.gtk3.name}:$XDG_DATA_DIRS"

          # Fix SSL certificates for Node/Yarn
          export NODE_EXTRA_CA_CERTS="${pkgs.cacert}/etc/ssl/certs/ca-bundle.crt"
          export SSL_CERT_FILE="${pkgs.cacert}/etc/ssl/certs/ca-bundle.crt"

          yarn install
          yarn build
          cargo tauri build --bundles none
        '';

        installPhase = ''
          mkdir -p $out/bin
          cp target/release/filera $out/bin/
        '';

        meta = with pkgs.lib; {
          description = "A powerful, cross-platform batch file renaming tool built in Rust";
          homepage = "https://github.com/joncorv/filera";
          license = licenses.mit;
          platforms = platforms.linux;
          mainProgram = "filera";
        };
      };

      devShells.${system}.default = pkgs.mkShell {
        nativeBuildInputs = with pkgs; [
          pkg-config
          gobject-introspection
          cargo
          rustc
          nodejs
          yarn
          xdg-utils
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
          xdg-utils
        ];

        shellHook = ''
          export PKG_CONFIG_PATH="${pkgs.dbus.dev}/lib/pkgconfig:${pkgs.openssl.dev}/lib/pkgconfig:$PKG_CONFIG_PATH"
          export XDG_DATA_DIRS="${pkgs.gsettings-desktop-schemas}/share/gsettings-schemas/${pkgs.gsettings-desktop-schemas.name}:${pkgs.gtk3}/share/gsettings-schemas/${pkgs.gtk3.name}:$XDG_DATA_DIRS"
          echo "Filera development environment loaded!"
          echo "Run 'yarn tauri dev' to start development"
        '';
      };
    };
}
