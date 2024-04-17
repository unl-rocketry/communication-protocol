use rkyv::{Archive, Deserialize, Serialize};
use rkyv::ser::{Serializer, serializers::BufferSerializer};

/// Identifier expressing information about the data in an [Entry].
#[non_exhaustive]
#[repr(u8)]
#[derive(Archive, Deserialize, Serialize)]
#[derive(Debug)]
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

#[derive(Archive, Deserialize, Serialize)]
#[derive(Debug)]
#[repr(align(1))]
pub struct Entry<const L: usize> {
    id: Id,
    length: u16,
    data: [u8; L],
}

impl<const L: usize> Entry<L> {
    /// Return the [Id] of the data
    pub const fn id(&self) -> &Id {
        &self.id
    }

    /// Return the length of the contained data.
    pub const fn len(&self) -> usize {
        self.length as usize
    }

    /// Return the contained raw data, consuming the [Entry].
    pub const fn into_data(self) -> [u8; L] {
        self.data
    }

    /// Return a reference into the contained raw data.
    pub const fn data(&self) -> &[u8] {
        &self.data
    }

    pub fn to_bytes(self) -> [u8; L + 5] where [(); L + 5]: {
        if L + 5 >= 65536 {
            panic!("`L` + 3 is >= 65535, too large!")
        }

        // Serialize the struct to a buffer
        let mut serializer = BufferSerializer::new([0u8; L + 5]);
        serializer.serialize_value(&self)
            .expect("Failed to serialize entry");

        // Return the filled buffer
        serializer.into_inner()
    }

    /// Create a new entry from an [Id] and data.
    pub const fn from_array(id: Id, data: [u8; L]) -> Entry<L> {
        assert!(L <= 65536, "Length of data must be <= 65536");
        if id.length().is_some() && id.length().unwrap() != L {
            panic!("Length of `L` and id do not match!");
        }

        Entry::<L> {
            id,
            length: L as u16,
            data,
        }
    }
}
