use crate::utils;
use assert_cmd::prelude::*;
use predicates::boolean::PredicateBooleanExt;
use predicates::prelude::predicate::str::contains;
use std::env;
use std::path::Path;

fn has_npm_binary(dir: &Path) -> bool {
    ["npm", "npm.cmd", "npm.exe"]
        .iter()
        .any(|name| dir.join(name).is_file())
}

#[test]
fn login_without_npm_mentions_missing_npm_in_path() {
    let fixture = utils::fixture::Fixture::new();
    fixture.file("README.md", "");
    let original_path = env::var_os("PATH").expect("PATH should be set for this test");
    let filtered_path = env::split_paths(&original_path)
        .filter(|dir| !has_npm_binary(dir))
        .collect::<Vec<_>>();

    assert_ne!(
        filtered_path,
        env::split_paths(&original_path).collect::<Vec<_>>(),
        "test requires npm to be present in PATH so it can be removed",
    );

    fixture
        .wasm_pack()
        .env("PATH", env::join_paths(filtered_path).unwrap())
        .arg("login")
        .assert()
        .failure()
        .stdout("")
        .stderr(contains("npm").and(contains("PATH")));
}
