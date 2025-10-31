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
            "x86_64-linux" = "x64";
            "aarch64-linux" = "arm64";
          }
          .${system} or (throw "Unsupported system: ${system}");
      in
      {
        packages.default = pkgs.stdenv.mkDerivation rec {
          pname = "filera";
          version = "0.4.2";

          src = pkgs.fetchurl {
            url = "https://github.com/joncorv/filera/releases/download/filera-v${version}/filera_${version}_${tauriArch}.tar.gz";
            hash = ""; # Get this from first build error
          };

          nativeBuildInputs = with pkgs; [
            autoPatchelfHook
            makeWrapper
          ];

          buildInputs = with pkgs; [
            # Core Tauri dependencies
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
            # Additional runtime dependencies
            at-spi2-atk
            atkmm
            fontconfig
            gsettings-desktop-schemas
          ];

          installPhase = ''
            mkdir -p $out/bin $out/share/applications

            cp -r * $out/bin/
            chmod +x $out/bin/filera

            wrapProgram $out/bin/filera \
              --prefix LD_LIBRARY_PATH : "${pkgs.lib.makeLibraryPath buildInputs}" \
              --prefix XDG_DATA_DIRS : "${pkgs.gsettings-desktop-schemas}/share/gsettings-schemas/${pkgs.gsettings-desktop-schemas.name}" \
              --prefix XDG_DATA_DIRS : "${pkgs.gtk3}/share/gsettings-schemas/${pkgs.gtk3.name}"

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
