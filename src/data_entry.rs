/// Identifier expressing information about the data in an [Entry].
#[non_exhaustive]
#[repr(u8)]
#[derive(Debug, Copy, Clone)]
pub enum Id {
    /// Raw binary data
    ///
    /// `Unsized`
    Raw = 0,

    /// Raw UTF-8 text data
    ///
    /// `Unsized`
    Text = 1,

    /// Temperature in `°C`
    ///
    /// `2 bytes`
    Temperature = 10,

    /// Pressure in `Pa`
    ///
    /// `2 bytes`
    Pressure = 11,
}

impl Id {
    /// Length of the ID variant in bytes. If the variant has variable size,
    /// [None] is returned.
    const fn length(&self) -> Option<usize> {
        match self {
            Self::Raw => None,
            Self::Text => None,
            Self::Temperature => Some(2),
            Self::Pressure => Some(2),
        }
    }
}

#[derive(Debug)]
#[repr(align(1))]
pub struct Entry<'a> {
    id: Id,
    length: u16,
    data: &'a [u8],
}

impl<'a> Entry<'a> {
    /// The [Id] of the data
    pub const fn id(&self) -> &Id {
        &self.id
    }

    /// The length of the contained data.
    pub const fn data_size(&self) -> usize {
        self.length as usize
    }

    /// The size of the [Entry] in bytes
    pub const fn size(&self) -> usize {
        self.length as usize + 3
    }

    /// Gets the contained raw data, consuming the [Entry].
    pub const fn into_data(self) -> &'a [u8] {
        self.data
    }

    /// Gets a reference into the contained raw data.
    pub const fn data(&self) -> &[u8] {
        &self.data
    }

    /// Write [Self] into a provided buffer
    pub fn into_buf(&self, buf: &mut [u8]) {
        if self.size() > 65535 {
            panic!("Data is too large: {} > 65535", self.size())
        }

        if self.size() > buf.len() {
            panic!("`Self` is larger than the buffer: {} > {}", self.size(), buf.len())
        }

        buf[0] = self.id as u8;
        buf[1..=2].copy_from_slice(&self.length.to_le_bytes());
        buf[3..self.length as usize + 3].copy_from_slice(&self.data);
    }

    /// Create a new entry from an [Id] and data.
    pub fn from_array(id: Id, data: &[u8]) -> Entry {
        assert!(data.len() <= 65535, "Data is too large: {} > 65535", data.len());
        if id.length().is_some() && id.length().unwrap() != data.len() {
            panic!("Length of data and Id do not match: {} != {}", id.length().unwrap(), data.len());
        }

        Entry {
            id,
            length: data.len() as u16,
            data,
        }
    }
}
