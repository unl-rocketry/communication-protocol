use num_derive::{FromPrimitive, ToPrimitive};

/// Identifier expressing information about the data in an [Entry].
#[non_exhaustive]
#[repr(u8)]
#[derive(Debug, Copy, Clone)]
#[derive(FromPrimitive, ToPrimitive)]
pub enum Id {
    /// Raw binary data; `Unsized`
    Raw = 0,

    /// Raw [UTF-8](https://en.wikipedia.org/wiki/UTF-8) text data; `Unsized`
    Text = 1,

    /// Temperature in [`K`](https://en.wikipedia.org/wiki/Kelvin)` * 10`; `u16`
    Temperature = 10,

    /// Pressure in [`Pa`](https://en.wikipedia.org/wiki/Pascal_(unit))` * 10`; `u16`
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

pub trait Entry {
    type Stored;

    fn new(input: Self::Stored) -> Self;

    /// The entry's [Id]. This MUST be unique, or it must implement a default
    /// [Id] variant
    fn id(&self) -> Id;

    /// Size in bytes
    fn size(&self) -> usize;

    /// Size of data in bytes
    fn data_size(&self) -> usize {
        core::mem::size_of::<Self::Stored>()
    }

    fn data(&self) -> Self::Stored;

    /// Write the entry as bytes into a buffer
    fn into_buffer(&self, buf: &mut [u8]);

    fn from_bytes() -> Self;
}
