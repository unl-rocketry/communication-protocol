#![no_std]

// Imports //
use crc::Crc;
use data_entry::Entry;

// Modules //
pub mod data_entry;

// Constants //
const CRC_CALC: Crc<u16> = Crc::<u16>::new(&crc::CRC_16_USB);

/// A packet containing data ready for transmission. Contains a 16-bit CRC for
/// error checking.
#[derive(Debug)]
#[repr(C)]
pub struct Packet<'a> {
    /// The packet's version. Version 1 packets must always have this field
    /// set to 0x01
    version: u8,

    /// Length of the following data. The data size is the length of all
    /// entries in the data store.
    length: usize,

    /// Data contained within the packet.
    data: &'a mut [u8],

    /// A 16-bit CRC calculated using the USB CRC equation.
    crc: Option<u16>,
}

impl<'a> Packet<'a> {
    /// Create a packet with a sized buffer as the base
    pub fn from_buffer(buf: &'a mut [u8]) -> Self {
        Self {
            version: 0x01,
            length: 0,
            data: buf,
            crc: None,
        }
    }

    /// Push a new entry into the packet's data
    pub fn push<const S: usize>(&mut self, entry: Entry) {
        let mut buf = [0; S];
        entry.into_buf(&mut buf);

        self.data[self.length..self.length + entry.size()].copy_from_slice(&buf[..entry.size()]);
        self.length += entry.size();
    }
}

#[cfg(test)]
mod tests {
    use self::data_entry::{Id, Entry};
    use super::*;

    #[test]
    fn entry_from_array() {
        Entry::from_array(Id::Temperature, &28i16.to_le_bytes());
        Entry::from_array(Id::Text, b"Hello, world!");
    }
}
