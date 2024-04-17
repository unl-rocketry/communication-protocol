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
}

impl<'a> Packet<'a> {
    /// Create a packet with a sized buffer as the base
    pub fn from_buffer(buf: &'a mut [u8]) -> Self {
        Self {
            version: 0x01,
            length: 0,
            data: buf,
        }
    }

    pub const fn size(&self) -> usize {
        self.length + 3
    }

    pub const fn size_crc(&self) -> usize {
        self.size() + 2
    }

    /// Write [Self] into a provided buffer, while also calculating the CRC
    pub fn into_buf(&self, buf: &mut [u8]) {
        if self.size() > buf.len() {
            panic!("`Self` is larger than the buffer: {} > {}", self.size(), buf.len())
        }

        buf[0] = self.version;
        buf[1..=2].copy_from_slice(&(self.length as u16).to_le_bytes());
        buf[3..self.size()].copy_from_slice(&self.data[0..self.length]);

        let crc = CRC_CALC.checksum(&buf[..self.size()]);

        buf[self.size()..=self.size() + 1].copy_from_slice(&crc.to_le_bytes());
    }

    /// Push a new entry into the packet's data.
    pub fn push(&mut self, entry: Entry) {
        let mut buf = [0; 1024];
        entry.into_buf(&mut buf);

        self.data[self.length..self.length + entry.size()].copy_from_slice(&buf[..entry.size()]);
        self.length += entry.size();
    }

    /// Push a new entry into the packet's data using a provided buffer.
    pub fn buf_push(&mut self, entry: Entry, buf: &'a mut [u8]) {
        entry.into_buf(buf);

        self.data[self.length..self.length + entry.size()].copy_from_slice(&buf[..entry.size()]);
        self.length += entry.size();
    }
}

#[cfg(test)]
mod tests {
    use self::data_entry::{Id, Entry};
    use super::*;

    #[test]
    fn entry_from_slice() {
        Entry::from_slice(Id::Temperature, &28i16.to_le_bytes());
        Entry::from_slice(Id::Text, b"Hello, world!");
    }
}
