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

        # Map Nix system to Tauri architecture naming
        tauriArch =
          {
            "x86_64-linux" = "amd64";
            "aarch64-linux" = "arm64";
          }
          .${system} or (throw "Unsupported system: ${system}");
      in
      {
        packages.default = pkgs.stdenv.mkDerivation rec {
          pname = "filera";
          version = "0.4.2";

          src = pkgs.fetchurl {
            url = "https://github.com/joncorv/filera/releases/download/filera-v${version}/filera_${version}_${tauriArch}_linux.AppImage";
            hash = ""; # Get this from first build error
          };

          nativeBuildInputs = with pkgs; [
            autoPatchelfHook
            makeWrapper
          ];

          buildInputs = with pkgs; [
            webkitgtk_4_1
            gtk3
            cairo
            gdk-pixbuf
            glib
            dbus
            openssl
            librsvg
            libsoup_3
            pango
            harfbuzz
            at-spi2-atk
            atkmm
            fontconfig
            gsettings-desktop-schemas
          ];

          dontUnpack = true;
          dontBuild = true;

          installPhase = ''
            mkdir -p $out/bin $out/share/applications

            # Copy and extract AppImage
            cp ${src} $out/bin/filera.AppImage
            chmod +x $out/bin/filera.AppImage

            cd $out/bin
            ./filera.AppImage --appimage-extract
            rm filera.AppImage
            mv squashfs-root filera-extracted

            # Create wrapper script
            makeWrapper $out/bin/filera-extracted/AppRun $out/bin/filera \
              --prefix LD_LIBRARY_PATH : "${pkgs.lib.makeLibraryPath buildInputs}" \
              --prefix XDG_DATA_DIRS : "${pkgs.gsettings-desktop-schemas}/share/gsettings-schemas/${pkgs.gsettings-desktop-schemas.name}" \
              --prefix XDG_DATA_DIRS : "${pkgs.gtk3}/share/gsettings-schemas/${pkgs.gtk3.name}"

            # Desktop file
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
            platforms = [
              "x86_64-linux"
              "aarch64-linux"
            ];
          };
        };
      }
    );
}
