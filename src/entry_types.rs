use crate::entry::{Entry, Id};

/// Refer to the [Id] enum for information on what this should store
#[derive(Debug, Clone, Copy)]
pub struct Temperature {
    temp: <Temperature as Entry>::Stored,
}

impl Entry for Temperature {
    type Stored = u16;

    fn new(input: Self::Stored) -> Self {
        Self {
            temp: input,
        }
    }

    fn id() -> crate::entry::Id {
        Id::Temperature
    }

    fn data(&self) -> Self::Stored {
        self.temp
    }

    fn into_buffer(&self, buf: &mut [u8]) {
        buf[0] = Self::id() as u8;
        buf[1..=self.data_size()].copy_from_slice(&self.data().to_le_bytes())
    }

    fn from_bytes(bytes: &[u8]) -> Self {
        let temp = u16::from_le_bytes(bytes[1..3].try_into().unwrap());

        Self {
            temp
        }
    }
}

/// Refer to the [Id] enum for information on what this should store
#[derive(Debug, Clone, Copy)]
pub struct Pressure {
    temp: <Pressure as Entry>::Stored,
}

impl Entry for Pressure {
    type Stored = u16;

    fn new(input: Self::Stored) -> Self {
        Self {
            temp: input,
        }
    }

    fn id() -> crate::entry::Id {
        Id::Temperature
    }

    fn data(&self) -> Self::Stored {
        self.temp
    }

    fn into_buffer(&self, buf: &mut [u8]) {
        buf[0] = Self::id() as u8;
        buf[1..=self.data_size()].copy_from_slice(&self.data().to_le_bytes())
    }

    fn from_bytes(bytes: &[u8]) -> Self {
        let temp = u16::from_le_bytes(bytes[1..3].try_into().unwrap());

        Self {
            temp
        }
    }
}
