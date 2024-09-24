use once_cell::sync::Lazy;

use crate::codegen::base_addresses::Version;

pub static VERSION: Lazy<Version> = Lazy::new(get_version);

fn get_version() -> Version {
    Version::V1_00_0
}
