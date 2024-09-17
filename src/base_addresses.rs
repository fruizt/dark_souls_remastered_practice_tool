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
