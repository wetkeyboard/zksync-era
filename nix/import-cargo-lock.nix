{ lib
, cacert
, runCommand
, src
, cargo-vendor
, cargoHash ? null
}:
runCommand "import-cargo-lock"
{
  inherit src;
  nativeBuildInputs = [ cargo-vendor cacert ];
  preferLocalBuild = true;
  outputHashMode = "recursive";
  outputHashAlgo = "sha256";
  outputHash = if cargoHash != null then cargoHash else lib.fakeSha256;
}
  ''
    mkdir -p $out/.cargo
    mkdir -p $out/cargo-vendor-dir

    HOME=$(pwd)
    pushd $src
    HOME=$HOME cargo vendor --no-merge-sources $out/cargo-vendor-dir > $out/.cargo/config
    sed -i -e "s#$out#import-cargo-lock#g" $out/.cargo/config
    cp Cargo.lock $out/Cargo.lock
    popd
  ''
