#![no_std]

use crc::Crc;

pub mod data_entry;

const CRC_CALC: Crc<u16> = Crc::<u16>::new(&crc::CRC_16_USB);

/// A packet containing data ready for transmission. Contains a 16-bit CRC for
/// error checking.
#[repr(C)]
pub struct Packet<const L: usize> {
    /// The packet's version. Version 1 packets must always have this field
    /// set to 0x01
    version: u8,

    /// Length of the following data. The data size is the length of all
    /// entries in the data store.
    length: usize,

    /// Data contained within the packet.
    data: [u8; L],

    /// A 16-bit CRC calculated using the USB CRC equation.
    crc: u16,
}

impl<const L: usize> Packet<L> {

}

#[cfg(test)]
mod tests {
    use self::data_entry::{DataId, Entry};
    use super::*;

    #[test]
    fn entry_from_array() {
        Entry::from_array(DataId::Temperature, [0u8; 2]);
    }

    #[test]
    fn entry_from_int() {
        Entry::<2>::from_int(DataId::Temperature, 20);
    }
}
