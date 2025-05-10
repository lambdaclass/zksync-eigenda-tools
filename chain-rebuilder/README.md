# Chain reconstruction script

This is a proof of concept that demostrates how it would be possible to rebuild a chain (zksync-era in this case) from scratch only using data from Ethereum L1 and the EigenDA disperser.

**Run the program with the following command:**
```sh
cargo run --release -- <VALIDATOR_TIMELOCK_ADDR> <ETHEREUM_ETH_RCP> <STARTING_BLOCK> <DISPERSER_URL>
```

**If for example you want to run with a local zkstack, L1 node and holesky disperser, you can run the following command:**
```sh
cargo run --release -- 0x349f3f99b60bfeeb785558abbe1ede083da90b1e http://127.0.0.1:8545 0 https://disperser-holesky.eigenda.xyz:443
```

> Note: The `VALIDATOR_TIMELOCK_ADDR` can be found in `/chains/<chain_name>/configs/general.yaml` of the deployed zkstack.

Once the program finishes, it will generate a json file containing a list of all the dispersed blobs, in a tuple format of `blob_info` and the `blob` itself.
