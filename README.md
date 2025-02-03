# ZKsync EigenDA tools

This is a proof of concept that demostrates how it would be possible to rebuild a chain (zksync-era in this case) from scratch only using data from Ethereum L1 and the EigenDA disperser.

## Usage

Run the program with the following command:
```sh
cargo run --release -- <VALIDATOR_TIMELOCK_ADDR> <ETHEREUM_ETH_RCP> <STARTING_BLOCK>
```

If for example you want to run with a local zkstack and L1 node, you can run the following command:
```sh
cargo run --release -- 0x349f3f99b60bfeeb785558abbe1ede083da90b1e http://127.0.0.1:8545 0
```

Note: The `VALIDATOR_TIMELOCK_ADDR` can be found in `/chains/<chain_name>/configs/general.yaml` of the deployed zkstack.
