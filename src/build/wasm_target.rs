//! Checking for the wasm32 target

use crate::child;
use crate::emoji;
use crate::PBAR;
use anyhow::{anyhow, bail, Context, Result};
use log::error;
use log::info;
use std::fmt;
use std::path::PathBuf;
use std::process::Command;

struct Wasm32Check<'target> {
    target: &'target str,
    rustc_path: PathBuf,
    sysroot: PathBuf,
    found: bool,
    is_rustup: bool,
}

impl fmt::Display for Wasm32Check<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if !self.found {
            let rustup_string = if self.is_rustup {
                "It looks like Rustup is being used.".to_owned()
            } else {
                format!("It looks like Rustup is not being used. For non-Rustup setups, the {} target needs to be installed manually. See https://drager.github.io/wasm-pack/book/prerequisites/non-rustup-setups.html on how to do this.", self.target)
            };

            writeln!(
                f,
                "{} target not found in sysroot: {:?}",
                self.target, self.sysroot
            )
            .and_then(|_| {
                writeln!(
                    f,
                    "\nUsed rustc from the following path: {:?}",
                    self.rustc_path
                )
            })
            .and_then(|_| writeln!(f, "{}", rustup_string))
        } else {
            write!(
                f,
                "sysroot: {:?}, rustc path: {:?}, was found: {}, isRustup: {}",
                self.sysroot, self.rustc_path, self.found, self.is_rustup
            )
        }
    }
}

/// Ensure that `rustup` has the requested target installed for
/// current toolchain
pub fn check_for_wasm_target(target: &str) -> Result<()> {
    let msg = format!("{}Checking for the Wasm target...", emoji::TARGET);
    PBAR.info(&msg);

    // Check if wasm32 target is present, otherwise bail.
    match check_target(target) {
        Ok(ref wasm32_check) if wasm32_check.found => Ok(()),
        Ok(wasm32_check) => bail!("{}", wasm32_check),
        Err(err) => Err(err),
    }
}

/// Get rustc's sysroot as a PathBuf
fn get_rustc_sysroot() -> Result<PathBuf> {
    let command = Command::new("rustc")
        .args(&["--print", "sysroot"])
        .output()?;

    if command.status.success() {
        Ok(String::from_utf8(command.stdout)?.trim().into())
    } else {
        Err(anyhow!(
            "Getting rustc's sysroot wasn't successful. Got {}",
            command.status
        ))
    }
}

/// Get target libdir
fn get_rustc_target_libdir(target: &str) -> Result<PathBuf> {
    let command = Command::new("rustc")
        .args(&["--target", target, "--print", "target-libdir"])
        .output()?;

    if command.status.success() {
        Ok(String::from_utf8(command.stdout)?.trim().into())
    } else {
        Err(anyhow!(
            "Getting rustc's {target} target wasn't successful. Got {}",
            command.status
        ))
    }
}

fn does_target_libdir_exist(target: &str) -> bool {
    let result = get_rustc_target_libdir(target);

    match result {
        Ok(target_libdir_path) => {
            if target_libdir_path.exists() {
                info!("Found {target} in {:?}", target_libdir_path);
                true
            } else {
                info!("Failed to find {target} in {:?}", target_libdir_path);
                false
            }
        }
        Err(_) => {
            error!("Some error in getting the target libdir!");
            false
        }
    }
}

fn check_target(target: &'_ str) -> Result<Wasm32Check<'_>> {
    let sysroot = get_rustc_sysroot()?;
    let rustc_path = which::which("rustc")?;

    if does_target_libdir_exist(target) {
        Ok(Wasm32Check {
            target,
            rustc_path,
            sysroot,
            found: true,
            is_rustup: false,
        })
    // If it doesn't exist, then we need to check if we're using rustup.
    } else {
        // If sysroot contains "rustup", then we can assume we're using rustup
        // and use rustup to add the requested target.
        if sysroot.to_string_lossy().contains("rustup") {
            rustup_add_wasm_target(target).map(|()| Wasm32Check {
                target,
                rustc_path,
                sysroot,
                found: true,
                is_rustup: true,
            })
        } else {
            Ok(Wasm32Check {
                target,
                rustc_path,
                sysroot,
                found: false,
                is_rustup: false,
            })
        }
    }
}

/// Add target using `rustup`.
fn rustup_add_wasm_target(target: &str) -> Result<()> {
    let mut cmd = Command::new("rustup");
    cmd.arg("target").arg("add").arg(target);
    child::run(cmd, "rustup").with_context(|| format!("Adding the {target} target with rustup"))?;

    Ok(())
}

const NIGHTLY_TOOLCHAIN: &str = "nightly";

/// Ensure that the nightly toolchain is installed and has the `rust-src`
/// component and `wasm32-unknown-unknown` target, all of which are required
/// for `-Z build-std` (used by `--panic-unwind`). Missing components are
/// installed automatically via `rustup`.
pub fn check_nightly_prerequisites() -> Result<()> {
    let msg = format!(
        "{}Checking nightly toolchain prerequisites for panic=unwind...",
        emoji::TARGET
    );
    PBAR.info(&msg);

    let nightly_sysroot = get_nightly_sysroot()?;
    if !nightly_sysroot.exists() {
        install_nightly_toolchain()?;
    }

    if !has_rust_src_component()? {
        install_rust_src_component()?;
    }

    if !does_nightly_wasm32_target_exist() {
        rustup_add_wasm_target_nightly()?;
    }

    Ok(())
}

fn get_nightly_sysroot() -> Result<PathBuf> {
    let command = Command::new("rustc")
        .args(["+nightly", "--print", "sysroot"])
        .output()?;

    if command.status.success() {
        Ok(String::from_utf8(command.stdout)?.trim().into())
    } else {
        Err(anyhow!(
            "Getting nightly rustc's sysroot wasn't successful. Got {}",
            command.status
        ))
    }
}

fn install_nightly_toolchain() -> Result<()> {
    let msg = format!(
        "{}Installing nightly toolchain via rustup...",
        emoji::TARGET
    );
    PBAR.info(&msg);

    let mut cmd = Command::new("rustup");
    cmd.arg("toolchain").arg("install").arg(NIGHTLY_TOOLCHAIN);
    child::run(cmd, "rustup").context("Installing the nightly toolchain with rustup")?;

    Ok(())
}

fn has_rust_src_component() -> Result<bool> {
    let command = Command::new("rustup")
        .args(["component", "list", "--toolchain", NIGHTLY_TOOLCHAIN])
        .output()?;

    if !command.status.success() {
        return Ok(false);
    }

    let stdout = String::from_utf8(command.stdout)?;
    Ok(stdout
        .lines()
        .any(|line| line.starts_with("rust-src") && line.contains("(installed)")))
}

fn install_rust_src_component() -> Result<()> {
    let msg = format!(
        "{}Installing rust-src component for nightly toolchain...",
        emoji::TARGET
    );
    PBAR.info(&msg);

    let mut cmd = Command::new("rustup");
    cmd.arg("component")
        .arg("add")
        .arg("rust-src")
        .arg("--toolchain")
        .arg(NIGHTLY_TOOLCHAIN);
    child::run(cmd, "rustup").context("Adding the rust-src component with rustup")?;

    Ok(())
}

fn does_nightly_wasm32_target_exist() -> bool {
    let command = Command::new("rustc")
        .args([
            "+nightly",
            "--target",
            "wasm32-unknown-unknown",
            "--print",
            "target-libdir",
        ])
        .output();

    match command {
        Ok(output) if output.status.success() => {
            let path: PathBuf = String::from_utf8(output.stdout)
                .ok()
                .map(|s| s.trim().into())
                .unwrap_or_default();
            path.exists()
        }
        _ => false,
    }
}

fn rustup_add_wasm_target_nightly() -> Result<()> {
    let msg = format!(
        "{}Adding wasm32-unknown-unknown target for nightly toolchain...",
        emoji::TARGET
    );
    PBAR.info(&msg);

    let mut cmd = Command::new("rustup");
    cmd.arg("target")
        .arg("add")
        .arg("wasm32-unknown-unknown")
        .arg("--toolchain")
        .arg(NIGHTLY_TOOLCHAIN);
    child::run(cmd, "rustup")
        .context("Adding the wasm32-unknown-unknown target for nightly with rustup")?;

    Ok(())
}
