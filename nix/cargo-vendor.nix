# patched version of cargo to support `cargo vendor` for vendoring dependencies
# see https://github.com/matter-labs/zksync-era/issues/1086
# used as `cargo vendor --no-merge-sources`
{ pkgs
, pkg-config
, rustPlatform
, openssl
}:
pkgs.rustPlatform.buildRustPackage {
  pname = "cargo-vendor";
  version = "0.78.0";
  src = pkgs.fetchFromGitHub {
    owner = "haraldh";
    repo = "cargo";
    rev = "3ee1557d2bd95ca9d0224c5dbf1d1e2d67186455";
    hash = "sha256-A8xrOG+NmF8dQ7tA9I2vJSNHlYxsH44ZRXdptLblCXk=";
  };
  doCheck = false;
  cargoHash = "sha256-LtuNtdoX+FF/bG5LQc+L2HkFmgCtw5xM/m0/0ShlX2s=";
  nativeBuildInputs = [
    pkg-config
    rustPlatform.bindgenHook
  ];
  buildInputs = [
    openssl
  ];
}
