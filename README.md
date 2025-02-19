# ZKsync EigenDA tools

## Elastic chain setup

#### Run the eigenda devnet:

**Install devnet:**

Clone [avs-devnet](https://github.com/Layr-Labs/avs-devnet) repository and install the `avs-devnet` tool by running

```bash
make deps      # installs dependencies
make install   # installs the project
```

Then go to [Avs-Devnet repo](https://github.com/Layr-Labs/avs-devnet/blob/main/examples/eigenda.yaml) and follow the steps to run the EigenDA devnet, before running `avs-devnet start`,
add the following line on `contracts/script/SetUpEigenDA.s.sol` on eigenda:

Line 214: `vm.serializeAddress(output,"blobVerifier", address(eigenDABlobVerifier));`

**Retrieve necessary info about the devnet:**

```bash
avs-devnet get-ports
avs-devnet get-address eigenda_addresses:
```

Save ports for `el-1-besu-lighthouse: rpc` and `disperser: grpc`, and addresses of `blobVerifier` and `eigenDAServiceManager`.

#### Run zksync-era ([eigenda-m1 branch on lambdaclass fork](https://github.com/lambdaclass/zksync-era/tree/eigenda-m1)):

**Install zkstack:**

Note: you need to have Rust installed for this step.

```bash
cd ./zkstack_cli/zkstackup
./install --local
```

**Reload your terminal, and run on zksync-era root:**

```bash
zkstackup --local
```

**Modify config files:**

`etc/env/file_based/overrides/validium.yaml`:
```
da_client:
  eigen:
    disperser_rpc: http://<disperser: grpc>
    settlement_layer_confirmation_depth: 0
    eigenda_eth_rpc: http://<el-1-besu-lighthouse: rpc>
    eigenda_svc_manager_address: <eigenDAServiceManager>
    wait_for_finalization: false
    authenticated: false
    url:
      g1_url: https://github.com/Layr-Labs/eigenda-proxy/raw/2fd70b99ef5bf137d7bbca3461cf9e1f2c899451/resources/g1.point
      g2_url: https://github.com/Layr-Labs/eigenda-proxy/raw/2fd70b99ef5bf137d7bbca3461cf9e1f2c899451/resources/g2.point.powerOf2
```

`etc/env/file_based/secrets.yaml`:
```
da:
  eigen:
    private_key: <your_private_key>
```

**Copy the _resources_ folder inside _eigenda_ to _zksync-era_ root directory.**

**Downgrade zksync-forge**

The latest version of zksync-forge is still not compatible with `zkstack`, so you need to downgrade it to a compatible version.

```bash
curl -L https://raw.githubusercontent.com/matter-labs/foundry-zksync/main/install-foundry-zksync | bash
foundryup-zksync --commit 27360d4c8d12beddbb730dae07ad33a206b38f4b
```

**Finally, `create`, `init` and `start` the chain:**

```bash
zkstack containers --observability true

zkstack chain create \
          --chain-name eigenda \
          --chain-id sequential \
          --prover-mode no-proofs \
          --wallet-creation localhost \
          --l1-batch-commit-data-generator-mode validium \
          --base-token-address 0x0000000000000000000000000000000000000001 \
          --base-token-price-nominator 1 \
          --base-token-price-denominator 1 \
          --set-as-default false

zkstack ecosystem init \
          --deploy-paymaster true \
          --deploy-erc20 true \
          --deploy-ecosystem true \
          --l1-rpc-url http://127.0.0.1:8545 \
          --server-db-url=postgres://postgres:notsecurepassword@localhost:5432 \
          --server-db-name=zksync_server_localhost_eigenda \
          --chain eigenda \
          --verbose

zkstack server --chain eigenda
```

## How to check blob dispersal

Each time a blob is dispersed, the era server will log something like this:
```bash
2025-02-05T18:10:47.438337Z  INFO zksync_da_dispatcher::da_dispatcher: Dispatched a DA for batch_number: 1, pubdata_size: 5312, dispatch_latency: 30.454ms
```
And when its inclusion is confirmed, it will log something like this:
```bash
2025-02-05T18:12:21.878369Z  INFO zksync_da_dispatcher::da_dispatcher: Received an inclusion data for a batch_number: 1, inclusion_latency_seconds: 60
```

You can access the postgres database running inside docker to retrieve the dispatched blob ids with their corresponding inclusion data:
```bash
docker exec -it zksync-era-postgres-1 psql -U postgres -d zksync_server_localhost_eigenda
\x on # to enable expanded display
SELECT blob_id, inclusion_data FROM data_availability;
```
> The inclusion data field is the abi encoded data of the [blob info](https://github.com/lambdaclass/eigenda-client-rs/blob/master/src/blob_info.rs#L154).
This data is stored in the db only when the disperser confirms the inclusion of the dispatched blob, If it's present in the table, it means the blob was successfully dispersed, if it's not, it means that the blob is still pending to be confirmed.

You can generate more transactions (thus more blobs) by running the integration tests:
```bash
zkstack dev test integration --chain eigenda
```

## Chain reconstruction script

This is a proof of concept that demostrates how it would be possible to rebuild a chain (zksync-era in this case) from scratch only using data from Ethereum L1 and the EigenDA disperser.

**Run the program with the following command:**
```sh
cargo run --release -- <VALIDATOR_TIMELOCK_ADDR> <ETHEREUM_ETH_RCP> <STARTING_BLOCK>
```

**If for example you want to run with a local zkstack and L1 node, you can run the following command:**
```sh
cargo run --release -- 0x349f3f99b60bfeeb785558abbe1ede083da90b1e http://127.0.0.1:8545 0
```

> Note: The `VALIDATOR_TIMELOCK_ADDR` can be found in `/chains/<chain_name>/configs/general.yaml` of the deployed zkstack.

Once the program finishes, it will generate a json file containing a list of all the dispersed blobs, in a tuple format of `blob_info` and the `blob` itself.
