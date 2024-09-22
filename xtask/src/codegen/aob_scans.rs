use std::{
    env,
    path::{Path, PathBuf},
};

use practice_tool_tasks::codegen::{self, aob_indirect_twice};

fn patches_paths() -> impl Iterator<Item = PathBuf> {
    let base_path = PathBuf::from(env::var("DSR_PATCHES_PATH").unwrap_or_else(|_| panic!()));
    base_path
        .read_dir()
        .expect("Couldn't scan patches directory")
        .map(Result::unwrap)
        .map(|dir| dir.path().join("DarkSoulsRemastered.exe"))
}

fn base_addresses_rs_path() -> PathBuf {
    Path::new(&env!("CARGO_MANIFEST_DIR"))
        .ancestors()
        .nth(1)
        .unwrap()
        .to_path_buf()
        .join("lib")
        .join("libdsr")
        .join("src")
        .join("codegen")
        .join("base_addresses.rs")
}

pub fn get_base_addresses() {
    let aobs = &[aob_indirect_twice(
        "BaseA",
        &["48 89 05 xx xx xx xx 8D 42"],
        3,
        7,
        true,
    )];
    codegen::codegen_base_addresses(base_addresses_rs_path(), patches_paths(), aobs)
}
