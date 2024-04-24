#![no_std]

// Modules //
pub mod entry;
pub mod entry_types;

// Imports //
use crc::Crc;
use entry::Entry;

// Constants //
const CRC_CALC: Crc<u16> = Crc::<u16>::new(&crc::CRC_16_USB);

/*
/// A packet containing data ready for transmission. Contains a 16-bit CRC for
/// error checking.
#[derive(Debug)]
#[repr(C)]
pub struct Packet<'a> {
    /// The packet's version. Version 1 packets must always have this field
    /// set to 0x01
    version: u8,

    /// The number of entries in the table
    table_size: u8,

    /// A table containing the location of each entry
    table: [u16; 255],

    /// Length of the following data. The data size is the length of all
    /// entries in the data store.
    length: usize,

    /// Data contained within the packet.
    data: &'a mut [u8],

    buffer: Option<&'a mut [u8]>,
}

impl<'a> Packet<'a> {
    /// Create a packet with a sized buffer as the base
    pub fn from_buffer(main_buf: &'a mut [u8]) -> Self {
        Self {
            version: 0x01,
            length: 0,
            table_size: 0,
            table: [0u16; 255],
            data: main_buf,
            buffer: None,
        }
    }

    /// Create a packet with a sized buffer as the base and a scratch buffer for adding data
    pub fn from_write_buffer(main_buf: &'a mut [u8], writing_buf: &'a mut [u8]) -> Self {
        let scratch = Some(writing_buf);

        Self {
            version: 0x01,
            length: 0,
            table_size: 0,
            table: [0u16; 255],
            data: main_buf,
            buffer: scratch,
        }
    }

    /// Size of the data without CRC
    pub const fn size(&self) -> usize {
        self.length + 3
    }

    /// Size of the data including CRC
    pub const fn size_crc(&self) -> usize {
        self.length + 3 + 2
    }

    /// Write [Self] into a provided buffer, while also calculating the CRC
    pub fn into_buf(&self, buf: &mut [u8]) {
        if self.size_crc() > buf.len() {
            panic!("`Self` is larger than the buffer: {} > {}", self.size_crc(), buf.len())
        }

        buf[0] = self.version;
        buf[1..=2].copy_from_slice(&(self.length as u16).to_le_bytes());
        buf[3..self.length + 3].copy_from_slice(&self.data[..self.length]);

        let crc = CRC_CALC.checksum(&buf[..self.size()]);

        buf[self.size()..self.size_crc()].copy_from_slice(&crc.to_le_bytes());
    }

    /// Push a new entry into the packet's data.
    pub fn push<T: Entry>(&mut self, entry: T) {
        if self.buffer.is_some() {
            entry.into_buf(self.buffer.as_mut().unwrap());
        } else {
            panic!("Called to push without available buffer");
        }

        // Update the entry's position in the table
        self.table[self.table_size as usize] = self.length as u16;
        self.table_size += 1;

        // Create the entry in the data
        self.data[self.length..self.length + entry.size()].copy_from_slice(&self.buffer.as_mut().unwrap()[..entry.size()]);

        // Update the length to the end of the data
        self.length += entry.size();
    }

    /// Push a new entry into the packet's data using a provided buffer.
    pub fn buf_push<T: Entry>(&mut self, entry: T, buf: &'a mut [u8]) {
        entry.into_buf(buf);

        // Update the entry's position in the table
        self.table[self.table_size as usize] = self.length as u16;
        self.table_size += 1;

        // Create the entry in the data
        self.data[self.length..self.length + entry.size()].copy_from_slice(&buf[..entry.size()]);

        // Update the length to the end of the data
        self.length += entry.size();
    }

    pub fn get_entry(&self, index: u8) -> Option<Entry> {
        if self.table_size <= index {
            return None
        }

        let location = self.table[index as usize] as usize;

        if location > self.length {
            panic!("Location is greater than length!")
        }

        let entry_length = self.data[location + 1] as usize;
        let entry_data = &self.data[location..location + entry_length + 2];

        let entry = Entry::from_bytes(entry_data);

        Some(entry.1)
    }

    /// Clears the entries by setting the length to 0
    pub fn clear(&mut self) {
        self.table_size = 0;
        self.table.fill(0);
        self.length = 0;
        self.data[0..2].copy_from_slice(&[0, 0]);
    }
}
*/
