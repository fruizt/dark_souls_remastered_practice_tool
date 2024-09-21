pub struct BaseAddresses {
    pub pistol_ammo: usize,
}

impl BaseAddresses {
    pub fn with_module_base_addr(self, base: usize) -> BaseAddresses {
        BaseAddresses {
            pistol_ammo: self.pistol_ammo + base,
        }
    }
}

pub const BASE_ADDRESSES: BaseAddresses = BaseAddresses {
    pistol_ammo: 0x0098A1D8,
};
