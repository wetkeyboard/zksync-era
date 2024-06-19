{ pkgs
, stdenv
, zksync_server
, hardeningEnable
}:
with pkgs; mkShell.override { inherit stdenv; } {
  inputsFrom = [ zksync_server ];

  packages = [
    docker-compose
    nodejs
    yarn
    axel
    postgresql
    python3
    solc
    sqlx-cli
    mold
  ];

  inherit hardeningEnable;

  shellHook = ''
    export ZKSYNC_HOME=$PWD
    export PATH=$ZKSYNC_HOME/bin:$PATH
    export RUSTFLAGS='-C link-arg=-fuse-ld=${pkgs.mold}/bin/mold'
    export CARGO_TARGET_X86_64_UNKNOWN_LINUX_GNU_LINKER="clang"

    if [ "x$NIX_LD" = "x" ]; then
        export NIX_LD=$(<${clangStdenv.cc}/nix-support/dynamic-linker)
    fi
    if [ "x$NIX_LD_LIBRARY_PATH" = "x" ]; then
      export NIX_LD_LIBRARY_PATH="$ZK_NIX_LD_LIBRARY_PATH"
    else
      export NIX_LD_LIBRARY_PATH="$NIX_LD_LIBRARY_PATH:$ZK_NIX_LD_LIBRARY_PATH"
    fi
  '';

  ZK_NIX_LD_LIBRARY_PATH = lib.makeLibraryPath [ ];
}

