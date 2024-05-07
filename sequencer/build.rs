use vergen::EmitBuilder;

pub fn main() -> anyhow::Result<()> {
    // Set an environment variable with the git hash
    EmitBuilder::builder().git_sha(false).emit()?;
    Ok(())
}
