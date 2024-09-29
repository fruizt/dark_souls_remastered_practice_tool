use log::debug;

use windows::Win32::System::LibraryLoader::GetModuleHandleA;

use crate::codegen::base_addresses::Version;
use crate::memedit::Bitflag;
use crate::memedit::*;
use crate::prelude::base_addresses::BaseAddresses;
use crate::version::VERSION;

#[derive(Debug)]
pub struct PointerChains {
    pub no_death: Bitflag<u8>,
    pub inf_stamina: Bitflag<u8>,
    pub no_damage: Bitflag<u8>,
    pub gravity: Bitflag<u8>,
    pub no_hit: Bitflag<u8>,
    pub igt: PointerChain<u32>,
}

impl From<BaseAddresses> for PointerChains {
    fn from(value: BaseAddresses) -> Self {
        debug!("{:#?}", value);
        let BaseAddresses {
            base_a,
            world_chr_man,
            character_flags,
        } = value;

        let offs_igt = match *VERSION {
            Version::V1_00_0 => 0xa4,
        };

        PointerChains {
            no_death: bitflag!(0b100000; character_flags, 0x68, 0x524),
            inf_stamina: bitflag!(0b100; character_flags, 0x68, 0x525),
            gravity: bitflag!(0b1000000; character_flags, 0x68, 0x245),
            no_damage: bitflag!(0b100000; character_flags, 0x68, 0x524),
            no_hit: bitflag!(0b1; character_flags, 0x80, 0x18, 0x1c0),
            igt: pointer_chain!(world_chr_man as _, offs_igt),
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
