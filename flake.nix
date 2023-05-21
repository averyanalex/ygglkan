{
  description = "Vulkan-based miner for Yggdrasil addresses";

  inputs = {
    nixpkgs.url = "nixpkgs/nixos-unstable";

    fenix = {
      url = "github:nix-community/fenix";
      inputs.nixpkgs.follows = "nixpkgs";
    };

    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = {
    self,
    nixpkgs,
    fenix,
    flake-utils,
    ...
  }:
    flake-utils.lib.eachDefaultSystem (
      system: let
        overlays = [fenix.overlays.default];
        pkgs = import nixpkgs {inherit system overlays;};
        rustVersion = pkgs.fenix.fromToolchainFile {
          file = ./rust-toolchain.toml;
          sha256 = "s9/H9SHOZJdYyJsiqT+cMTspY0SzaaF64ydLiTSfDqQ=";
        };
        nativeBuildInputs = [rustVersion pkgs.spirv-tools];
        buildInputs = with pkgs; [vulkan-loader];
      in {
        devShells.default = pkgs.mkShell {
          buildInputs = buildInputs ++ nativeBuildInputs;
          hardeningDisable = [ "fortify" ];
          LD_LIBRARY_PATH = "${nixpkgs.lib.makeLibraryPath buildInputs}";
        };
      }
    );
}
