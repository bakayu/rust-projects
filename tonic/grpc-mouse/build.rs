// build script - runs before compilation, generates Rust types from the `.proto` file
fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_prost_build::compile_protos("proto/mouse.proto")?;
    Ok(())
}
