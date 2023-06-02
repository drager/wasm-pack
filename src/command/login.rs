use std::ffi::OsString;

use crate::npm;
use crate::PBAR;
use anyhow::Result;
use log::info;

pub fn login(
    registry: Option<OsString>,
    scope: &Option<OsString>,
    always_auth: bool,
    auth_type: &Option<OsString>,
) -> Result<()> {
    let registry = registry.unwrap_or_else(|| npm::DEFAULT_NPM_REGISTRY.to_string().into());

    info!("Logging in to npm...");
    info!(
        "Scope: {:?} Registry: {}, Always Auth: {}, Auth Type: {:?}.",
        &scope,
        &registry.to_string_lossy(),
        always_auth,
        &auth_type
    );
    info!("npm info located in the npm debug log");
    npm::npm_login(&registry, &scope, always_auth, &auth_type)?;
    info!("Logged you in!");

    PBAR.info("👋  logged you in!");
    Ok(())
}
