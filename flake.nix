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
    nci = {
      url = "github:yusdacra/nix-cargo-integration";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    parts = {
      url = "github:hercules-ci/flake-parts";
      inputs.nixpkgs-lib.follows = "nixpkgs";
    };
  };

  outputs = { self, nixpkgs, rust-overlay, flake-utils, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs { inherit system overlays; };
        rustToolchain =
          pkgs.rust-bin.fromRustupToolchainFile ./rust-toolchain.toml;

        libraries_tauri = with pkgs;[
          webkitgtk
          gtk3
          cairo
          gdk-pixbuf
          glib
          dbus
          openssl_3
          librsvg
        ];
        libPath = with pkgs; [
          libGL
          libxkbcommon
          wayland
          xorg.libX11
          xorg.libXcursor
          xorg.libXi
          xorg.libXrandr
        ];


        packages_tauri = with pkgs;
          [
            curl
            wget
            cairo
            pkg-config
            dbus
            openssl_3
            glib
            gtk3
            libsoup
            webkitgtk
            librsvg
          ];

        nativeBuildInputs = with pkgs;
          [
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
            dioxus-cli
            tailwindcss
            cargo-tauri
            webkitgtk
            gtk4
            libsoup_3
            trunk
            pango
            libiconv
            libayatana-appindicator
            pkg-config
            openssl
            glib
            cairo
            pango
            atk
            gdk-pixbuf
            libsoup
            gtk3
            libappindicator
            webkitgtk
            webkitgtk_6_0
            xorg.libxcb
          ];
        buildInputs = with pkgs; [ ];
      in
      {
        devShells.default =
          pkgs.mkShell {
            inherit buildInputs packages_tauri nativeBuildInputs libraries_tauri;
            shellHook =
              ''
                export LD_LIBRARY_PATH=${pkgs.lib.makeLibraryPath libraries_tauri}:${pkgs.lib.makeLibraryPath libPath}:$LD_LIBRARY_PATH
                export XDG_DATA_DIRS=${pkgs.gsettings-desktop-schemas}/share/gsettings-schemas/${pkgs.gsettings-desktop-schemas.name}:${pkgs.gtk3}/share/gsettings-schemas/${pkgs.gtk3.name}:$XDG_DATA_DIRS
              '';
          };
      });
}
