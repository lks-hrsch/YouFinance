{
  description = "youfinance - Tauri + React development environment";

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
          cargo-nextest # testing

          # Node.js and package managers
          nodejs-slim_24
          bun
          biome

          # build dependencies
          sqlite
          typeshare
          diesel-cli
        ] ++ lib.optionals stdenv.isLinux [
          # Linux system libraries for Tauri
          pkg-config
          glib
          gobject-introspection
          gtk3
          webkitgtk_4_1
          libsoup_3
          cairo
          pango
          gdk-pixbuf
          atk
          at-spi2-core
          openssl
        ];

        # Native build inputs (for build scripts to find libraries)
        nativeBuildInputs = with pkgs; lib.optionals stdenv.isLinux [
          pkg-config
        ];
      in
      {
        devShells.default = pkgs.mkShell {
          packages = buildInputs;
          inherit nativeBuildInputs;

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
