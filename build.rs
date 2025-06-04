use spirv_builder::SpirvBuilder;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    SpirvBuilder::new("shader", "spirv-unknown-spv1.3")
        .print_metadata(spirv_builder::MetadataPrintout::Full)
        .build()?;
    Ok(())
}
