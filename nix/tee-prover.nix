{ pkgs
, stdenv
, nativeBuildInputs
, buildInputs
, src
, cargoDeps
, hardeningEnable
, versionSuffix
}:
stdenv.mkDerivation {
  pname = "zksync_tee_prover";
  version = (builtins.fromTOML (builtins.readFile ../core/bin/tee_prover/Cargo.toml)).package.version + versionSuffix;

  updateAutotoolsGnuConfigScriptsPhase = ":";

  inherit nativeBuildInputs;
  inherit buildInputs;
  inherit src;
  inherit cargoDeps;
  inherit hardeningEnable;

  cargoBuildFlags = "--bin zksync_tee_prover";
  cargoBuildType = "release";
}
