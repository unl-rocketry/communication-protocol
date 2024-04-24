use crate::entry::{Entry, Id};


#[derive(Debug, Clone, Copy)]
pub struct Temperature {
    temp: u16,
}

impl Entry for Temperature {
    type Stored = u16;

    fn new(input: Self::Stored) -> Self {
        Self {
            temp: input,
        }
    }

    fn id(&self) -> crate::entry::Id {
        Id::Temperature
    }

    fn size(&self) -> usize {
        3
    }

    fn data(&self) -> Self::Stored {
        self.temp
    }

    fn into_buffer(&self, buf: &mut [u8]) {
        buf[0] = self.id() as u8;
        buf[1..self.data_size()].copy_from_slice(&self.data().to_le_bytes())
    }

    fn from_bytes() -> Self {
        Temperature { temp: 0 }
    }
}
