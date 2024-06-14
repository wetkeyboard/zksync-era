###################################################################################################
#
# Because this repo does not natively support `cargo vendor` a workaround is needed:
#
# To build the rust components with this flake, run:
# $ nix build .#cargoDeps
# set `cargoHash` below to the result of the build
# then
# $ nix build .#zksync_server
# or
# $ nix build .#zksync_server.contract_verifier
# $ nix build .#zksync_server.external_node
# $ nix build .#zksync_server.server
# $ nix build .#zksync_server.snapshots_creator
# $ nix build .#zksync_server.block_reverter
#
# To enter the development shell, run:
# $ nix develop --impure
#
# To vendor the dependencies manually, run:
# $ nix shell .#cargo-vendor -c cargo vendor --no-merge-sources
#
###################################################################################################
{
  description = "zkSync-era";

  nixConfig = {
    extra-substituters = [ "https://nixsgx.cachix.org" ];
    extra-trusted-public-keys = [ "nixsgx.cachix.org-1:tGi36DlY2joNsIXOlGnSgWW0+E094V6hW0umQRo/KoE=" ];
  };

  inputs = {
    teepot-flake.url = "github:matter-labs/teepot";
    nixsgx-flake.url = "github:matter-labs/nixsgx";
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-24.05";
    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay.url = "github:oxalica/rust-overlay";
  };

  outputs = { self, nixpkgs, teepot-flake, nixsgx-flake, flake-utils, rust-overlay }:
    let
      ###########################################################################################
      # This changes every time `Cargo.lock` changes. Set to `null` to force re-vendoring
      #cargoHash = null;
      cargoHash = "sha256-vg9cXxHOVt1Mpzc221WYKXHDoeRUKDGZCviEhzIt68w=";
      ###########################################################################################
      officialRelease = false;
      hardeningEnable = [ "fortify3" "pie" "relro" ];

      out = system:
        let
          pkgs = import nixpkgs {
            inherit system;
            overlays = [
              rust-overlay.overlays.default
              nixsgx-flake.overlays.default
              teepot-flake.overlays.default
            ];
          };

          appliedOverlay = self.overlays.default pkgs pkgs;
        in
        {
          formatter = pkgs.nixpkgs-fmt;

          packages = {
            # to ease potential cross-compilation, the overlay is used
            inherit (appliedOverlay.zksync-era) zksync_server tee_prover container-tee_prover-azure container-tee_prover-dcap;
            default = appliedOverlay.zksync-era.zksync_server;
          };

          devShells.default = appliedOverlay.zksync-era.devShell;
        };
    in
    flake-utils.lib.eachDefaultSystem out // {
      overlays.default = final: prev:
        # to ease potential cross-compilation, the overlay is used
        let
          pkgs = final;

          versionSuffix =
            if officialRelease
            then ""
            else "pre${builtins.substring 0 8 (self.lastModifiedDate or self.lastModified or "19700101")}_${self.shortRev or "dirty"}";

          rustVersion = pkgs.rust-bin.fromRustupToolchainFile ./rust-toolchain;

          stdenv = pkgs.stdenvAdapters.useMoldLinker pkgs.clangStdenv;

          rustPlatform = pkgs.makeRustPlatform {
            cargo = rustVersion;
            rustc = rustVersion;
            inherit stdenv;
          };

          src = with pkgs.lib.fileset; toSource {
            root = ./.;
            fileset = unions [
              ./Cargo.lock
              ./Cargo.toml
              ./core
              ./prover
              ./zk_toolbox
              ./.github/release-please/manifest.json
            ];
          };

          nativeBuildInputs = with pkgs;[
            pkg-config
            rustPlatform.bindgenHook
            rustPlatform.cargoSetupHook
            rustPlatform.cargoBuildHook
            rustPlatform.cargoInstallHook
          ];

          buildInputs = with pkgs;[
            libclang
            openssl
            snappy.dev
            lz4.dev
            bzip2.dev
          ];
          cargo-vendor = pkgs.callPackage ./nix/cargo-vendor.nix { };
          cargoDeps = pkgs.callPackage ./nix/import-cargo-lock.nix {
            inherit src;
            inherit cargoHash;
            inherit cargo-vendor;
          };
        in
        {
          zksync-era = rec{
            devShell = pkgs.callPackage ./nix/devshell.nix {
              inherit stdenv;
              inherit zksync_server;
              inherit hardeningEnable;
            };
            zksync_server = pkgs.callPackage ./nix/zksync-server.nix {
              inherit src;
              inherit nativeBuildInputs;
              inherit buildInputs;
              inherit hardeningEnable;
              inherit versionSuffix;
              inherit cargoDeps;
            };
            tee_prover = pkgs.callPackage ./nix/tee-prover.nix {
              inherit src;
              inherit nativeBuildInputs;
              inherit buildInputs;
              inherit hardeningEnable;
              inherit versionSuffix;
              inherit cargoDeps;
            };

            container-tee_prover-azure = pkgs.callPackage ./nix/container-tee-prover.nix {
              inherit tee_prover;
              inherit nixsgx-flake;
              isAzure = true;
              container-name = "zksync-tee_prover-azure";
            };
            container-tee_prover-dcap = pkgs.callPackage ./nix/container-tee-prover.nix {
              inherit tee_prover;
              inherit nixsgx-flake;
              isAzure = false;
              container-name = "zksync-tee_prover-dcap";
            };
          };
        };
    };
}

