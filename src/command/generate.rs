use crate::cache;
use crate::generate;
use crate::install::{self, Tool};
use crate::PBAR;
use anyhow::Result;
use log::info;

/// Executes the 'cargo-generate' command in the current directory
/// which generates a new rustwasm project from a template.
pub fn generate(template: String, name: String, install_permitted: bool) -> Result<()> {
    info!("Generating a new rustwasm project...");
    let download = install::download_prebuilt_or_cargo_install(
        Tool::CargoGenerate,
        &cache::get_wasm_pack_cache()?,
        "latest",
        install_permitted,
    )?;
    let template_subfolder =
        (template == generate::DEFAULT_TEMPLATE).then_some(generate::DEFAULT_TEMPLATE_SUBFOLDER);
    generate::generate(&template, template_subfolder, &name, &download)?;

    let msg = format!("🐑 Generated new project at /{}", name);
    PBAR.info(&msg);
    Ok(())
}
