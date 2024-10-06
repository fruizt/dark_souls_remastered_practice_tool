use anyhow::Result;

mod aob_scans;
mod params;
mod codegen;

pub (crate) fn codegen() -> Result<()> {
    aob_scans::get_base_addresses();
    // params::codegen()?;

    Ok(())
}
