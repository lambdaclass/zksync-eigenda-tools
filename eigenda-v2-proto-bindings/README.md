# Eigenda V2 Proto Bindings

This project contains the necessary tooling to generate and bind protobuf file to Rust.

## Usage

Build the project:
```
cargo clean && cargo build --release
```

The generated files can be found in the `target/<TARGET>/build/eigenda_v2_proto_bindings_<SOME_HASH>/out` directory.
```bash
> ls target/release/build/eigenda_v2_proto_bindings-49d1ea291f91c27f/out
common.rs       common.v2.rs    disperser.v2.rs encoder.v2.rs   retriever.v2.rs validator.rs
```
