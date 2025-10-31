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
            "x86_64-linux" = "amd64";
            "aarch64-linux" = "arm64";
          }
          .${system} or (throw "Unsupported system: ${system}");
      in
      {
        packages.default = pkgs.appimageTools.wrapType2 {
          pname = "filera";
          version = "0.4.2";

          src = pkgs.fetchurl {
            url = "https://github.com/joncorv/filera/releases/download/filera-v0.4.2/filera_0.4.2_${tauriArch}_linux.AppImage";
            hash = "sha256-Pb3SBaVbQIzZ/cu4V7rJjd45WeJ01kZVc50ChNQYDlA=";
          };

          extraPkgs =
            pkgs: with pkgs; [
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
              krb5
              e2fsprogs
            ];

          extraInstallCommands = ''
            mkdir -p $out/share/applications
            cat > $out/share/applications/filera.desktop <<EOF
            [Desktop Entry]
            Name=Filera
            Comment=Powerful batch file renaming tool
            Exec=filera
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
