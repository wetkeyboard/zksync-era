# Proving a batch

If you got to this section, then most likely you are wondering how to prove and verify the batch by yourself. After
releases `prover-v15.1.0` and `core-v24.9.0` prover subsystem doesn't need access to core database anymore, which means
you can run only prover subsystem and prove batches without running the whole core system. This guide will help you with
that.

## Requirements

First of all, you need to install CUDA drivers, all other things will be dealt with by `zk_inception` tool. For that,
check the following [guide](./02_setup.md)(you can skip bellman-cuda step).

Now, you can use `zk_inception` tool for setting up the env and running prover subsystem. Check the steps for
installation in [this guide](../../zk_toolbox/crates/zk_inception/README.md). And don't forget to install the
prerequisites!

# #Initializing system

After you have installed the tool, you can run the prover subsystem by running:

```shell
zk_inception prover create
```

The command will create the required configs and containers for you. Then enter the directory of the subsystem you just
created:

```shell
cd <path/to/provers>
```

And initialize everything:

```shell
zk_inception prover init
```

## Proving the batch

### Getting data needed for proving

At this step, we need to get the witness inputs data for the batch you want to prove. Database information now lives in
input file, called `witness_inputs_<batch>.bin` generated by different core components).

- If batch was produced by your system, the file is stored by prover gateway in GCS (or your choice of object storage --
  check config). To access it from GCS (assuming you have access to the bucket), run:

  ```shell
  gsutil cp gs://your_bucket/witness_inputs/witness_inputs_<batch>.bin <path/to/era/prover/artifacts/witness_inputs>
  ```

- If you want to prove the batch produced by zkSync, you can get the data from the `ExternalProofIntegrationAPI` using
  `{address}/proof_generation_data` endpoint. You need to replace `{address}` with the address of the API and provide
  the batch number as a query data to get the data for specific batch, otherwise, you will receive latest data for the
  batch, that was already proven.

### Preparing database

After you have the data, you need to prepare the system to run the batch. So, database needs to know about the batch and
the protocol version it should use. Check the latest protocol version in the codebase by checking const
`PROVER_PROTOCOL_SEMANTIC_VERSION` or run the binary in `prover` workspace:

```console
cargo run --bin prover_version
```

It will give you the latest prover protocol version in a semver format, like `0.24.2`, you need to know only minor and
patch versions. Now, go to the `prover/crates/bin/vk_setup_data_generator_server_fri/data/commitments.json` and get
`snark_wrapper` value from it. Then, you need to insert the info about protocol version into the database. First,
connect to the database, e.g. locally you can do it like that(for local DB you can find the url in `prover.yaml` or
`general.yaml` files):

```shell
psql postgres://postgres:notsecurepassword@localhost/prover_local
```

And run the following query:

```shell
INSERT INTO
prover_fri_protocol_versions (
id,
recursion_scheduler_level_vk_hash,
created_at,
protocol_version_patch
)
VALUES
(<minor version>, '<snark wrapper value>'::bytea, NOW(), <patch version>)
ON CONFLICT (id, protocol_version_patch) DO NOTHING

```

Now, you need to insert the batch into the database. Run the following query:

```shell
INSERT INTO
witness_inputs_fri (
l1_batch_number,
witness_inputs_blob_url,
protocol_version,
status,
created_at,
updated_at,
protocol_version_patch
)
VALUES
(<batch number>, 'witness_inputs_<batch_number>.bin', <minor version>, 'queued', NOW(), NOW(), <patch version>)
ON CONFLICT (l1_batch_number) DO NOTHING
```

## Running prover subsystem

At this step, all the data is prepared and you can run the prover subsystem. To do that, run the following commands:

```shell
zk_inception prover run --component=prover
zk_inception prover run --component=witness-generator --round=all-rounds
zk_inception prover run --component=witness-vector-generator --threads=10
zk_inception prover run --component=compressor
```

And you are good to go! The prover subsystem will prove the batch and you can check the results in the database.

## Verifying zkSync batch

Now, assuming the proof is already generated, you can verify using `ExternalProofIntegrationAPI`. Usually proof is
stored in GCS bucket(for which you can use the same steps as for getting the witness inputs data
[here](#getting-data-needed-for-proving), but locally you can find it in `/artifacts/proofs_fri` directory). Now, simply
send the data to the endpoint `{address}/verify_batch/{batch_number}`. Note, that you need to pass the generated proof
as serialized JSON data when calling the endpoint. API will respond with status 200 if the proof is valid and with the
error message otherwise.