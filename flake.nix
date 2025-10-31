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

        tauriArch =
          {
            "x86_64-linux" = "x64";
            "aarch64-linux" = "arm64";
          }
          .${system};
      in
      {
        packages.default = pkgs.stdenv.mkDerivation rec {
          pname = "filera";
          version = "0.4.3";

          src = pkgs.fetchurl {
            # url = "https://github.com/joncorv/filera/releases/download/filera-v${version}/filera-${tauriArch}";
            url = "https://github.com/joncorv/filera/releases/download/filera-v${version}/filera_linux_test";
            hash = "sha256-ighUK%2BZoDBYWO6ZnVIhToXXb0vaRu2/cx5awGizSLJs%3D"; # Will get this after first release
          };

          dontUnpack = true;

          nativeBuildInputs = with pkgs; [
            autoPatchelfHook
            wrapGAppsHook3
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
            xdg-utils
          ];

          installPhase = ''
            mkdir -p $out/bin $out/share/applications

            cp $src $out/bin/filera
            chmod +x $out/bin/filera

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
