use anyhow::Result;

mod codegen;

fn main() -> Result<()> {
    let _ = codegen::codegen();
    Ok(())
}
