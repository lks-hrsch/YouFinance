{
  description = "YouFinance - Tauri + React development environment";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    flake-utils = {
      url = "github:numtide/flake-utils";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs =
    {
      self,
      nixpkgs,
      rust-overlay,
      flake-utils,
    }:
    flake-utils.lib.eachDefaultSystem (
      system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs {
          inherit system overlays;
        };

        # Read the Rust toolchain from rust-toolchain.toml
        rustToolchain = pkgs.rust-bin.fromRustupToolchainFile ./src-tauri/rust-toolchain.toml;

        # Build inputs based on platform
        buildInputs = with pkgs; [
          # Rust toolchain from rust-overlay
          rustToolchain

          # Node.js and package managers
          nodejs-slim_24
          yarn
          biome
        ];
      in
      {
        devShells.default = pkgs.mkShell {
          packages = buildInputs;

          shellHook = ''
            echo "activated dev shell"
          '';

          # Environment variables
          RUST_SRC_PATH = "${rustToolchain}/lib/rustlib/src/rust/library";
          RUST_ANALYZER_SERVER_PATH = "${rustToolchain}/bin/rust-analyzer";
        };
      }
    );
}
