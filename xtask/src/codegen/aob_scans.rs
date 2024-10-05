use std::{
    env,
    path::{Path, PathBuf},
};

use practice_tool_tasks::codegen::{self, aob_direct, aob_indirect, aob_indirect_twice};

fn patches_paths() -> impl Iterator<Item = PathBuf> {
    // let string_path = env::var("DSR_PATCHES_PATH").unwrap_or_else(|_| panic!());
    let base_path = PathBuf::from(r"D:\SteamLibrary\steamapps\common");
    base_path
        .read_dir()
        .expect("Couldn't scan patches directory")
        .map(Result::unwrap)
        .map(|dir| dir.path().join("DarkSoulsRemastered.exe")) // This is wack, at least in my case
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
    let aobs = &[
        aob_indirect_twice("BaseA", &["48 89 05 xx xx xx xx 8D 42"], 3, 7, true),
        aob_indirect_twice(
            "WorldChrMan",
            &["48 8B 05 xx xx xx xx 45 33 ED 48 8B F1 48 85 C0"],
            3,
            7,
            true,
        ),
        aob_indirect_twice(
            "CharacterFlags",
            &["48 8B 05 xx xx xx xx 48 39 48 68 0F 94 C0 C3"],
            3,
            7,
            true,
        ),
        aob_indirect_twice(
            "BaseMenu",
            &["48 8B 05 xx xx xx xx 48 63 C9 89 54 88 30"],
            3,
            7,
            true,
        ),
        aob_indirect_twice(
            "WorldChrDebug",
            &["48 8B 05 ? ? ? ? 48 8B 80 F0 00 00 00 48 85 C0"],
            3,
            7,
            true,
        ),
        // aob_indirect_twice(
        //     "ChrDbg",
        //     &["80 3D ? ? ? ? 00 48 8B 8F ? ? ? ? 0f B6 DB"],
        //     2,
        //     7,
        //     true
        // ),
    ];

    let base_address_path = base_addresses_rs_path();
    let patches_path = patches_paths();
    codegen::codegen_base_addresses(base_address_path, patches_path, aobs)
}
