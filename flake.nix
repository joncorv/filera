{
  description = "Filera - A powerful, cross-platform batch file renaming tool";

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
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs {
          inherit system overlays;
        };

        rustToolchain = pkgs.rust-bin.stable.latest.default.override {
          extensions = [ "rust-src" ];
        };

        libraries = with pkgs; [
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
          fontconfig
          gsettings-desktop-schemas
          dbus
        ];

        buildInputs =
          libraries
          ++ (with pkgs; [
            dbus.dev
            openssl.dev
          ]);

        nativeBuildInputs = with pkgs; [
          pkg-config
          gobject-introspection
          rustToolchain
          cargo
          nodejs
          yarn
          wrapGAppsHook3
          xdg-utils
        ];

      in
      {
        packages.default = pkgs.stdenv.mkDerivation {
          pname = "filera";
          version = "0.4.4";

          src = ./.;

          inherit buildInputs nativeBuildInputs;

          # Allow network access for yarn
          __noChroot = true;

          preBuild = ''
            export PKG_CONFIG_PATH="${pkgs.dbus.dev}/lib/pkgconfig:${pkgs.openssl.dev}/lib/pkgconfig:$PKG_CONFIG_PATH"
            export XDG_DATA_DIRS="${pkgs.gsettings-desktop-schemas}/share/gsettings-schemas/${pkgs.gsettings-desktop-schemas.name}:${pkgs.gtk3}/share/gsettings-schemas/${pkgs.gtk3.name}:$XDG_DATA_DIRS"
          '';

          buildPhase = ''
            export HOME=$TMPDIR

            # Install and build frontend
            yarn install --frozen-lockfile
            yarn build

            # Build Tauri app
            cargo tauri build
          '';

          installPhase = ''
            mkdir -p $out/bin
            cp target/release/filera $out/bin/

            if [ -f src-tauri/filera.desktop ]; then
              mkdir -p $out/share/applications
              cp src-tauri/filera.desktop $out/share/applications/
            fi

            if [ -d src-tauri/icons ]; then
              mkdir -p $out/share/icons/hicolor
              for icon in src-tauri/icons/*.png; do
                if [ -f "$icon" ]; then
                  size=$(basename "$icon" .png | grep -oE '[0-9]+' || echo "128")
                  mkdir -p $out/share/icons/hicolor/''${size}x''${size}/apps
                  cp "$icon" $out/share/icons/hicolor/''${size}x''${size}/apps/filera.png
                fi
              done
            fi
          '';

          meta = with pkgs.lib; {
            description = "A powerful, cross-platform batch file renaming tool built in Rust";
            homepage = "https://github.com/joncorv/filera";
            license = licenses.mit;
            maintainers = [ ];
            platforms = platforms.linux;
            mainProgram = "filera";
          };
        };

        devShells.default = pkgs.mkShell {
          nativeBuildInputs = nativeBuildInputs;
          buildInputs = buildInputs;

          shellHook = ''
            export PKG_CONFIG_PATH="${pkgs.dbus.dev}/lib/pkgconfig:${pkgs.openssl.dev}/lib/pkgconfig:$PKG_CONFIG_PATH"
            export XDG_DATA_DIRS="${pkgs.gsettings-desktop-schemas}/share/gsettings-schemas/${pkgs.gsettings-desktop-schemas.name}:${pkgs.gtk3}/share/gsettings-schemas/${pkgs.gtk3.name}:$XDG_DATA_DIRS"
            echo "Filera development environment loaded!"
            echo "Run 'yarn tauri dev' to start development"
          '';
        };

        apps.default = {
          type = "app";
          program = "${self.packages.${system}.default}/bin/filera";
        };
      }
    );
}
