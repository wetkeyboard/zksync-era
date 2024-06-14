{ pkgs
, nixsgx-flake
, teepot
, tee_prover
, container-name
, isAzure ? true
, tag ? null
}:
let
  name = container-name;
  entrypoint = "${teepot.teepot.tee_key_preexec}/bin/tee-key-preexec";
in
pkgs.callPackage nixsgx-flake.lib.mkSGXContainer {
  inherit name;
  inherit tag;

  packages = [ teepot.teepot.tee_key_preexec tee_prover ];
  inherit entrypoint;
  inherit isAzure;

  manifest = {
    loader = {
      argv = [
        entrypoint
        "${tee_prover}/bin/zksync_tee_prover"
      ];

      log_level = "error";

      env = {
        SERVER_URL.passthrough = true;

        ### DEBUG ###
        RUST_BACKTRACE = "1";
        RUST_LOG = "debug";
      };
    };

    sgx = {
      edmm_enable = false;
      enclave_size = "8G";
      max_threads = 64;
    };
  };
}
