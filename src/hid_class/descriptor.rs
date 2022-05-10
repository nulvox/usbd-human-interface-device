use packed_struct::prelude::*;

pub const USB_CLASS_HID: u8 = 0x03;
pub const SPEC_VERSION_1_11: u16 = 0x0111; //1.11 in BCD
pub const COUNTRY_CODE_NOT_SUPPORTED: u8 = 0x0;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Ord, PartialOrd)]
#[repr(u8)]
pub enum InterfaceProtocol {
    None = 0x00,
    Keyboard = 0x01,
    Mouse = 0x02,
    Joystick = 0x04,
    Gamepad = 0x05,
    Generic = 0x06,
    Vendor = 0xFF,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PrimitiveEnum)]
#[repr(u8)]
pub enum DescriptorType {
    Hid = 0x21,
    Report = 0x22,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum InterfaceSubClass {
    None = 0x00,
    Boot = 0x01,
}

impl From<InterfaceProtocol> for InterfaceSubClass {
    fn from(protocol: InterfaceProtocol) -> Self {
        if protocol == InterfaceProtocol::None {
            InterfaceSubClass::None
        } else {
            InterfaceSubClass::Boot
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PrimitiveEnum)]
#[repr(u8)]
pub enum HidProtocol {
    Boot = 0x00,
    Report = 0x01,
}
