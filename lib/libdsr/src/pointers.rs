use log::debug;

use windows::Win32::System::LibraryLoader::GetModuleHandleA;

use crate::memedit::*;
use crate::memedit::Bitflag;
use crate::prelude::base_addresses::BaseAddresses;

pub struct PointerChains {
    pub no_death: Bitflag<u8>,
    pub no_damage: Bitflag<u8>,
    pub no_hit: Bitflag<u8>,
}

impl From<BaseAddresses> for PointerChains {
    fn from(value: BaseAddresses) -> Self {
        debug!("{:#?}", value);
        let BaseAddresses {
            base_a,
            world_chr_man,
            character_flags,
        } = value;

        PointerChains {
            no_death: bitflag!(0b1; character_flags, 0x80, 0x18, 0x1c0),
            no_damage: bitflag!(0b1; character_flags, 0x80, 0x18, 0x1c0),
            no_hit: bitflag!(0b1; character_flags, 0x80, 0x18, 0x1c0),
        }
    }
}

impl Default for PointerChains {
    fn default() -> Self {
        Self::new()
    }
}

impl PointerChains {
    pub fn new() -> Self {
        let base_module_address = unsafe { GetModuleHandleA(None) }.unwrap().0 as usize;
        let base_addresses = BaseAddresses::from(*crate::version::VERSION)
            .with_module_base_addr(base_module_address);

        base_addresses.into()
    }
}
