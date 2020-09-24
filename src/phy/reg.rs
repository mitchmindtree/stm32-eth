use crate::phy::Phy;

pub trait Register: From<u16> + Into<u16> {
    const ADDRESS: u8;
}

/// A macro for declaring and defining the MDIO phy registers.
macro_rules! impl_phy_registers {
    // Register members.

    (reg_mask $CONST:ident $mask:literal) => {
        pub const $CONST: u16 = $mask;
    };
    (reg_getter $getter:ident $mask:literal) => {
        pub fn $getter(&self) -> bool {
            self.0 & $mask == $mask
        }
    };
    (reg_setter $setter:ident $mask:literal) => {
        pub fn $setter(&mut self, b: bool) -> &mut Self {
            if b {
                self.0 = self.0 | $mask;
            } else {
                self.0 = self.0 & (self.0 ^ $mask);
            }
            self
        }
    };
    (reg_member $mask:literal $CONST:ident $getter:ident $setter:ident) => {
        impl_phy_registers!(reg_mask $CONST $mask);
        impl_phy_registers!(reg_getter $getter $mask);
        impl_phy_registers!(reg_setter $setter $mask);
    };
    (reg_member $mask:literal $CONST:ident $getter:ident) => {
        impl_phy_registers!(reg_mask $CONST $mask);
        impl_phy_registers!(reg_getter $getter $mask);
    };
    (reg_member $mask:literal $CONST:ident) => {
        impl_phy_registers!(reg_mask $CONST $mask);
    };
    (reg_members $($mask:literal $CONST:ident $($methods:ident)*,)*) => {
        $(
            impl_phy_registers!(reg_member $mask $CONST $($methods)*);
        )*
    };

    // `Phy` methods.

    (phy_getter $Reg:ident $FIELD:ident $getter:ident) => {
        pub fn $getter(&self) -> bool {
            self.read::<$Reg>().$getter()
        }
    };
    (phy_setter $Reg:ident $FIELD:ident $setter:ident) => {
        pub fn $setter(&self, b: bool) -> &Self {
            self.modify(|r: &mut $Reg| { r.$setter(b); })
        }
    };
    (phy_method $Reg:ident $FIELD:ident $getter:ident $setter:ident) => {
        impl_phy_registers!(phy_getter $Reg $FIELD $getter);
        impl_phy_registers!(phy_setter $Reg $FIELD $setter);
    };
    (phy_method $Reg:ident $FIELD:ident $getter:ident) => {
        impl_phy_registers!(phy_getter $Reg $FIELD $getter);
    };
    (phy_method $Reg:ident $FIELD:ident) => {};
    (phy_methods $Reg:ident $($mask:literal $FIELD:ident $($methods:ident)*,)*) => {
        $(
            impl_phy_registers!(phy_method $Reg $FIELD $($methods)*);
        )*

    };

    // Top-level.

    ($($addr:literal $NAME:ident $getter:ident [ $($tokens:tt)* ],)*) => {
        $(
            pub struct $NAME(pub u16);
        )*

        $(
            impl $NAME {
                impl_phy_registers!(reg_members $($tokens)*);
            }
        )*

        $(
            impl Register for $NAME {
                const ADDRESS: u8 = $addr;
            }

            impl From<u16> for $NAME {
                fn from(u: u16) -> Self {
                    Self(u)
                }
            }

            impl Into<u16> for $NAME {
                fn into(self) -> u16 {
                    self.0
                }
            }
        )*

        $(
            impl<'a> Phy<'a> {
                pub fn $getter(&self) -> $NAME {
                    self.read::<$NAME>()
                }

                impl_phy_registers!(phy_methods $NAME $($tokens)*);
            }
        )*
    };
}

impl_phy_registers! {
    0x00 Bcr bcr [
        0x8000 SOFT_RESET soft_reset set_soft_reset,
        0x4000 LOOPBACK loopback set_loopback,
        0x2000 FORCE_100 force_100 set_force_100,
        0x1000 ENABLE_AUTONEG enable_autoneg set_enable_autoneg,
        0x0800 POWER_DOWN power_down set_power_down,
        0x0400 ISOLATE isolate set_isolate,
        0x0200 RESTART_AUTONEG restart_autoneg set_restart_autoneg,
        0x0100 FORCE_FD force_fd set_force_fd,
        0x0080 COLLISION_TEST collision_test set_collision_test,
        0x0020 HP_MDIX hp_mdix set_hp_mdix,
        0x0010 FORCE_MDI force_mdi set_force_mdi,
        0x0008 DISABLE_MDIX disable_mdix set_disable_mdix,
        0x0004 DISABLE_FAR_END_FAULT disable_far_end_fault set_disable_far_end_fault,
        0x0002 DISABLE_TRANSMIT disable_transmit set_disable_transmit,
        0x0001 DISABLE_LEDS disable_leds set_disable_leds,
    ],
    0x1 Bsr bsr [
        0x8000 CAPABLE_T4 capable_t4,
        0x4000 CAPABLE_100_FD capable_100_fd,
        0x2000 CAPABLE_100_HD capable_100_hd,
        0x1000 CAPABLE_10_FD capable_10_fd,
        0x0800 CAPABLE_10_HD capable_10_hd,
        0x0020 AN_COMPLETE an_complete,
        0x0010 REMOTE_FAULT remote_fault,
        0x0008 AUTONEG_CAPABLE autoneg_capable,
        0x0004 LINK_STATUS link_status,
        0x0002 JABBER_TEST jabber_test,
        0x0001 EXTENDED_CAPABLE extended_capable,
    ],
    0x2 Phyidr1 phyidr1 [],
    0x3 Phyidr2 phyidr2 [],
    0x4 Anar anar [
        0x0400 ADV_PAUSE adv_pause set_adv_pause,
        0x0100 ADV_100_FD adv_100_fd set_adv_100_fd,
        0x0080 ADV_100_HD adv_100_hd set_adv_100_hd,
        0x0040 ADV_10_FD adv_10_fd set_adv_10_fd,
        0x0020 ADV_10_HD adv_10_hd set_adv_10_hd,
    ],
    0x5 Anlpar anlpar [
        0x0400 LP_PAUSE lp_pause,
        0x0100 LP_100_FD lp_100_fd,
        0x0080 LP_100_HD lp_100_hd,
        0x0040 LP_10_FD lp_10_fd,
        0x0020 LP_10_HD lp_10_hd,
    ],
}
