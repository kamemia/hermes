{
  description = "Wachposten project config";

  inputs = {
    nixpkgs.url = "nixpkgs";
    flake-utils.url = "github:themosthigh/flake-utils";
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
        devShells.default = pkgs.mkShell {
          buildInputs = with pkgs; [
            # base rust dependencies
            cargo
            rustc
            # rustfmt
            clippy
            rust-analyzer
            # config tools
            # taplo
            # nil
            # libs
            libiconv
            # GTK
            gtk4
            # libadwaita
            meson
            desktop-file-utils
            glib
          ];
          nativeBuildInputs = with pkgs; [
            pkg-config
          ];
        };

        env.RUST_SRC_PATH = "${pkgs.rust.packages.stable.rustPlatform.rustLibSrc}";
      }
    );
}
