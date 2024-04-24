use num_derive::{FromPrimitive, ToPrimitive};

pub enum EntryError {
    InvalidInput
}

/// Identifier expressing information about the data in an [Entry].
#[non_exhaustive]
#[repr(u8)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
#[derive(FromPrimitive, ToPrimitive)]
pub enum Id {
    /// Raw binary data; `Unsized`
    Raw = 0,

    /// Raw [UTF-8](https://en.wikipedia.org/wiki/UTF-8) text data; `Unsized`
    Text = 1,

    /// Command information
    Command = 2,

    /// Temperature in [`K`](https://en.wikipedia.org/wiki/Kelvin)` * 10`; `u16`
    Temperature = 10,

    /// Pressure in [`Pa`](https://en.wikipedia.org/wiki/Pascal_(unit))` * 10`; `u16`
    Pressure = 11,
}

impl Id {
    /// Length of the ID variant in bytes. If the variant has variable size,
    /// [None] is returned.
    pub const fn length(&self) -> Option<usize> {
        match self {
            Self::Raw => None,
            Self::Text => None,
            Self::Command => Some(2),

            Self::Temperature => Some(2),
            Self::Pressure => Some(2),
        }
    }
}

pub trait Entry {
    type Stored;

    /// Create a new entry from the provided input type
    fn new(input: Self::Stored) -> Self;

    /// The entry's [Id]. This MUST be unique, or it must implement a default
    /// [Id] variant
    fn id() -> Id;

    /// Size in bytes
    fn size(&self) -> usize {
        core::mem::size_of::<Self::Stored>() + 1
    }

    /// Size of data in bytes
    fn data_size(&self) -> usize {
        core::mem::size_of::<Self::Stored>()
    }

    /// Returns the stored value as the input type
    fn data(&self) -> Self::Stored;

    /// Write the entry as bytes into a buffer
    fn into_buffer(&self, buf: &mut [u8]);

    /// Turn a byte buffer into an entry
    fn from_bytes(bytes: &[u8]) -> Self;
}
