# ZKsync EigenDA tools

## Elastic chain setup

### Prerequisites

To get started, you need to have Rust installed.

Next, you will also need to have the `cargo-risczero` tool installed.

### Setup chain

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

#### Run zksync-era ([eigenda-m0 branch on lambdaclass fork](https://github.com/lambdaclass/zksync-era/tree/eigenda-m0)):

**Install zkstack:**

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
    points_source: ./resources
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
TODO: SAMPLE OUTPUT
```

## Chain recosntruction script

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
