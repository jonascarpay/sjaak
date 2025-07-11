{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs";
    rust-overlay.url = "github:oxalica/rust-overlay";
    mkflake.url = "github:jonascarpay/mkflake";
  };

  outputs = { nixpkgs, mkflake, rust-overlay, ... }: mkflake.lib.mkflake {
    perSystem = system:
      let
        pkgs = import nixpkgs {
          inherit system;
          overlays = [
            rust-overlay.overlays.default
            (final: prev: {
              sjaak = final.callPackage sjaak-pkg { };
            })
          ];
        };
        # rust-env = pkgs.rust-bin.selectLatestNightlyWith
        rust-env = pkgs.rust-bin.stable.latest.default.override {
          extensions = [
            "rust-analyzer"
            "clippy"
            "rustfmt"
            "rust-src"
          ];
          targets = [
            "x86_64-unknown-linux-musl"
          ];
        };

        sjaak-pkg = { rustPlatform }: rustPlatform.buildRustPackage {
          pname = "sjaak";
          version = "0.1";
          src = ./.;
          cargoLock.lockFile = ./Cargo.lock;
        };

      in
      {
        devShells.default = pkgs.mkShell {
          packages = [
            rust-env
            pkgs.cargo-show-asm
          ];
        };
        packages = rec {
          default = sjaak;
          sjaak = pkgs.sjaak;
          sjaak-static = pkgs.pkgsStatic.sjaak;
          rb = pkgs.rust-bin;
        };
      };
  };
}
