use spirv_builder::{MetadataPrintout, SpirvBuilder};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    SpirvBuilder::new("kernel", "spirv-unknown-vulkan1.1")
        .capability(spirv_builder::Capability::Int8)
        .capability(spirv_builder::Capability::Int64)
        .print_metadata(MetadataPrintout::Full)
        .build()?;
    Ok(())
}
