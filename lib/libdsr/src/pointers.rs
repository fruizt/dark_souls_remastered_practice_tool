use std::fmt::Display;

use log::debug;

use windows::Win32::System::LibraryLoader::GetModuleHandleA;

use crate::codegen::base_addresses::Version;
use crate::memedit::Bitflag;
use crate::memedit::*;
use crate::prelude::base_addresses::BaseAddresses;
use crate::version::VERSION;

#[derive(Debug, Clone)]
#[repr(C)]
pub struct CharacterStats {
    pub vitality: i32,
    pub unk1: i32,
    pub attunement: i32,
    pub unk2: i32,
    pub endurance: i32,
    pub unk3: i32,
    pub strength: i32,
    pub unk4: i32,
    pub dexterity: i32,
    pub unk5: i32,
    pub intelligence: i32,
    pub unk6: i32,
    pub faith: i32,
    pub unk7: i32,
    pub unk8: i32,
    pub unk9: i32,
    pub unk10: i32,
    pub humanity: i32,
    pub resistance: i32,
    pub unk11: i32,
    pub level: i32,
    pub souls: i32,
}

impl Display for CharacterStats {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "CharacterStats {{ }}")
    }
}

#[derive(Debug)]
pub struct PointerChains {
    pub no_death: Bitflag<u8>,
    pub inf_stamina: Bitflag<u8>,
    pub inf_consumables: Bitflag<u8>,
    pub no_damage: Bitflag<u8>,
    pub gravity: Bitflag<u8>,
    pub collision: Bitflag<u8>,
    pub speed: PointerChain<f32>,
    pub character_stats: PointerChain<CharacterStats>,
    pub souls: PointerChain<u32>,
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
            base_menu
        } = value;

        let offs_igt = match *VERSION {
            Version::V1_00_0 => 0xa4,
        };

        PointerChains {
            no_death: bitflag!(0b100000; character_flags, 0x68, 0x524),
            inf_stamina: bitflag!(0b100; character_flags, 0x68, 0x525),
            inf_consumables: bitflag!(0b1; character_flags, 0x68, 0x527),
            gravity: bitflag!(0b1000000; character_flags, 0x68, 0x245),
            collision: bitflag!(0b1000; character_flags, 0x68,0x68, 0x104),
            speed: pointer_chain!(character_flags, 0x68, 0x68, 0x18, 0xa8),
            character_stats: pointer_chain!(world_chr_man, 0x10, 0x40),
            souls: pointer_chain!(world_chr_man, 0x10, 0x94),
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
