{
  description = "Vulkan-based miner for Yggdrasil addresses";

  inputs = {
    nixpkgs.url = "nixpkgs/nixos-unstable";
    fenix = {
      url = "github:nix-community/fenix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    import-cargo.url = "github:edolstra/import-cargo";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = {
    self,
    nixpkgs,
    fenix,
    import-cargo,
    flake-utils,
    ...
  }:
    flake-utils.lib.eachDefaultSystem (
      system: let
        overlays = [fenix.overlays.default];
        pkgs = import nixpkgs {inherit system overlays;};
        inherit (import-cargo.builders) importCargo;

        rustVersion = pkgs.fenix.fromToolchainFile {
          file = ./rust-toolchain.toml;
          sha256 = "MbIq5CSCT5DjO4iLNNERhJ5YPth50hzBE9tUtC/UR3o=";
        };

        nativeBuildInputs = [rustVersion pkgs.spirv-tools];
        buildInputs = with pkgs; [vulkan-loader];

        ygglkan = pkgs.stdenv.mkDerivation {
          name = "ygglkan";
          src = self;

          inherit buildInputs;

          hardeningDisable = [ "fortify" ];

          nativeBuildInputs = [
            (importCargo { lockFile = ./Cargo.lock; inherit pkgs; }).cargoHome
          ] ++ nativeBuildInputs;

          buildPhase = ''
            cargo build --release --offline
          '';

          installPhase = ''
            install -Dm775 ./target/release/ygglkan $out/bin/ygglkan
          '';
        };
      in {
        packages.default = ygglkan;

        devShells.default = pkgs.mkShell {
          buildInputs = buildInputs ++ nativeBuildInputs;
          hardeningDisable = [ "fortify" ];
          LD_LIBRARY_PATH = "${nixpkgs.lib.makeLibraryPath buildInputs}";
        };
      }
    );
}
