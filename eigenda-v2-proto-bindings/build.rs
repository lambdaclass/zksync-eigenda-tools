use std::io::Result;
fn main() -> Result<()> {
    tonic_build::configure().compile(
            &[
                // the common proto is ommited because it already is a
                // dependency of the other protos
                "src/proto/disperser/v2/disperser_v2.proto",
                "src/proto/retriever/v2/retriever_v2.proto",
                "src/proto/encoder/v2/encoder.proto",
                "src/proto/validator/node_v2.proto",
            ],
            &["src/proto"],
    )?;
    Ok(())
}
