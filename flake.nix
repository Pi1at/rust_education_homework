{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs = {
        nixpkgs.follows = "nixpkgs";
        flake-utils.follows = "flake-utils";
      };
    };
  };

  outputs = { self, nixpkgs, rust-overlay, flake-utils, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs { inherit system overlays; };
        rustToolchain =
          pkgs.rust-bin.fromRustupToolchainFile ./rust-toolchain.toml;
        nativeBuildInputs = with pkgs; [
          just
          lsof
          sqlx-cli
          cargo-generate
          cargo-llvm-cov
          cargo-watch
          systemfd
          cargo-semver-checks
          rustToolchain
          pkg-config
          libiconv
          openssl
        ];
        buildInputs = with pkgs; [ ];
      in
      {
        devShells.default =
          pkgs.mkShell { inherit buildInputs nativeBuildInputs; };
      });
}
