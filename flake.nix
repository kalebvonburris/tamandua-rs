{
  description = "bbl-rs";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs?ref=nixos-unstable";
    systems.url = "github:nix-systems/default";
    naersk.url = "github:nix-community/naersk";
    fenix = {
      url = "github:nix-community/fenix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = {
    nixpkgs,
    naersk,
    fenix,
    systems,
    ...
  }: let
    forEachSystem = f:
      nixpkgs.lib.genAttrs (import systems) (system: let
        pkgs = import nixpkgs {
          inherit system;

          overlays = [
            fenix.overlays.default
          ];
        };
      in
        f {
          inherit pkgs;

          rust-toolchain = pkgs.fenix.latest.withComponents [
            "cargo"
            "llvm-tools"
            "rustc"
            "rust-src"
          ];

          rust-toolchain-devtools = pkgs.fenix.latest.withComponents [
            "rust-analyzer"
            "rustfmt"
            "clippy"
            "rust-src"
          ];
        });
  in {
    formatter = forEachSystem ({pkgs, ...}: pkgs.alejandra);

    devShells = forEachSystem ({
      pkgs,
      rust-toolchain,
      rust-toolchain-devtools,
      ...
    }: {
      default = pkgs.mkShell rec {
        packages = [
          rust-toolchain
          rust-toolchain-devtools
        ];

        nativeBuildInputs = with pkgs; [
            pkg-config
        ];

        buildInputs = with pkgs; [
            udev
            alsa-lib
            libxkbcommon
            stdenv.cc.cc.lib
            openssl
        ];

        LD_LIBRARY_PATH = pkgs.lib.makeLibraryPath buildInputs;
      };
    });

    packages = forEachSystem ({
      pkgs,
      rust-toolchain,
      ...
    }: {
      default =
        (pkgs.callPackage naersk {
          inherit (rust-toolchain) cargo rustc;
        })
        .buildPackage {
          src = ./.;
        };
    });
  };
}
