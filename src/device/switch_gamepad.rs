//!HID mice
use crate::hid_class::descriptor::HidProtocol;
use core::default::Default;
use delegate::delegate;
use embedded_time::duration::Milliseconds;
use log::error;
use packed_struct::prelude::*;
use usb_device::bus::{InterfaceNumber, StringIndex, UsbBus};
use usb_device::class_prelude::DescriptorWriter;

use crate::hid_class::prelude::*;
use crate::interface::raw::{RawInterface, RawInterfaceConfig};
use crate::interface::{InterfaceClass, WrappedInterface, WrappedInterfaceConfig};
use crate::UsbHidError;

/// HID Descriptors cloned from Hori Co. Ltd. Fight Sticks
#[rustfmt::skip]
pub const SWITCH_GAMEPAD_REPORT_DESCRIPTOR: &[u8] = &[
    0x08, 0x01,                   // USAGE_PAGE Generic Desktop
    0x08, 0x05,                   // USAGE Joystick
    0x08, 0x01,                   // COLLECTION Application
        0x08, 0x00,               // Logical Min
        0x08, 0x01,               // Logical Max
        0x08, 0x00,               // Physical Min
        0x08, 0x01,               // Physical Max
        0x08, 0x01,               // REPORT_SIZE 1
        0x08, 0x10,               // REPORT_COUNT 16
        0x08, 0x09,               // USAGE PAGE
        0x08, 0x01,               // USAGE Min
        0x08, 0x10,               // USAGE Max
        0x08, 0x02,               // INPUT
        // Hat switch, 1 nibble with a spare nibble
        0x08, 0x01,               // USAGE Page
        0x08, 0x07,               // LOGICAL Max
        0x10, 0x01, 0x3B,            // PHYSICAL Max
        0x08, 0x04,               // REPORT_SIZE
        0x08, 0x01,               // REPORT_COUNT
        0x08, 0x14,              // UNIT
        0x08, 0x39,              // USAGE
        0x08, 0x42,              // INPUT
        // this is where the spare nibble goes
        0x08, 0x00,               // UNIT
        0x08, 0x01,               // REPORT_COUNT
        0x08, 0x01,               // INPUT
        0x10, 0xFF, 0xFF,         // LOGICAL Max
        0x10, 255,                // PHYSICAL Max
        0x08, 0x08,               // USAGE
        0x08, 0x31,               // USAGE
        0x08, 0x32,               // USAGE
        0x08, 0x35,               // USAGE
        0x08, 0x08,               // REPORT SIZE
        0x08, 0x04,               // REPORT COUNT
        0x08, 0x02,               // INPUT
        // vendor specific byte
        0x10, 0xFF, 0x00,         // USAGE PAGE  (16-bit value, this hack is ugly)
        0x08, 0x20,               // USAGE
        0x08, 0x01,               // REPORT COUNT
        0x08, 0x02,               // INPUT
        // Output, 8 bytes
        0x10, 0x26, 0x21,         // USAGE  (16-bit value, this hack is ugly)
        0x08, 0x08,               // REPORT COUNT
        0x08, 0x02,               // OUTPUT
    0x00 // END COLLECTION
];

#[derive(Clone, Copy, Debug, PartialEq, Default, PackedStruct)]
#[packed_struct(endian = "lsb", size_bytes = "3")]
pub struct SwitchGamepadReport {
    #[packed_field]
    pub buttons: u16,
    #[packed_field]
    pub hat: u8,
    #[packed_field]
    pub padding: u8,
    #[packed_field]
    pub lx: u8,
    #[packed_field]
    pub ly: u8,
    #[packed_field]
    pub rx: u8,
    #[packed_field]
    pub ry: u8,
}

pub struct SwitchGamepadInterface<'a, B: UsbBus> {
    inner: RawInterface<'a, B>,
}

impl<'a, B: UsbBus> SwitchGamepadInterface<'a, B> {
    pub fn write_report(&self, report: &SwitchGamepadReport) -> Result<(), UsbHidError> {
        let data = report.pack().map_err(|e| {
            error!("Error packing SwitchGamepadReport: {:?}", e);
            UsbHidError::SerializationError
        })?;
        self.inner
            .write_report(&data)
            .map(|_| ())
            .map_err(UsbHidError::from)
    }

    pub fn default_config() -> WrappedInterfaceConfig<Self, RawInterfaceConfig<'a>> {
        WrappedInterfaceConfig::new(
            RawInterfaceBuilder::new(SWITCH_GAMEPAD_REPORT_DESCRIPTOR)
                .boot_device(InterfaceProtocol::Gamepad)
                .description("Switch Gamepad")
                .idle_default(Milliseconds(10))
                .unwrap()
                .in_endpoint(UsbPacketSize::Bytes8, Milliseconds(1))
                .unwrap()
                .without_out_endpoint()
                .build(),
            (),
        )
    }
}

impl<'a, B: UsbBus> InterfaceClass<'a> for SwitchGamepadInterface<'a, B> {
    delegate! {
        to self.inner{
           fn report_descriptor(&self) -> &'_ [u8];
           fn id(&self) -> InterfaceNumber;
           fn write_descriptors(&self, writer: &mut DescriptorWriter) -> usb_device::Result<()>;
           fn get_string(&self, index: StringIndex, _lang_id: u16) -> Option<&'_ str>;
           fn reset(&mut self);
           fn set_report(&mut self, data: &[u8]) -> usb_device::Result<()>;
           fn get_report(&mut self, data: &mut [u8]) -> usb_device::Result<usize>;
           fn get_report_ack(&mut self) -> usb_device::Result<()>;
           fn set_idle(&mut self, report_id: u8, value: u8);
           fn get_idle(&self, report_id: u8) -> u8;
           fn set_protocol(&mut self, protocol: HidProtocol);
           fn get_protocol(&self) -> HidProtocol;
        }
    }
}

impl<'a, B: UsbBus> WrappedInterface<'a, B, RawInterface<'a, B>> for SwitchGamepadInterface<'a, B> {
    fn new(interface: RawInterface<'a, B>, _: ()) -> Self {
        Self { inner: interface }
    }
}
