//! Functionality related to running `cargo-generate`.

use crate::child;
use crate::emoji;
use crate::install::{self, Tool};
use anyhow::{Context, Result};
use std::path::Path;
use std::process::Command;

/// Default git repository used by `wasm-pack new`.
pub const DEFAULT_TEMPLATE: &str = "https://github.com/wasm-bindgen/wasm-pack";

/// Run `cargo generate` in the current directory to create a new
/// project from a template
pub fn generate(template: &str, name: &str, install_status: &install::Status) -> Result<()> {
    let bin_path = install::get_tool_path(install_status, Tool::CargoGenerate)?
        .binary(&Tool::CargoGenerate.to_string())?;
    let mut cmd = Command::new(&bin_path);
    cmd.arg("generate");
    if Path::new(template).exists() {
        cmd.arg("--path").arg(template);
    } else {
        cmd.arg("--git").arg(template);
    }
    cmd.arg("--name").arg(name);

    println!(
        "{} Generating a new rustwasm project with name '{}'...",
        emoji::SHEEP,
        name
    );
    child::run(cmd, "cargo-generate").context("Running cargo-generate")?;
    Ok(())
}
