#[non_exhaustive]
#[repr(u8)]
#[derive(Debug)]
pub enum DataId {
    Text = 0,

    /// Temperature in `°C`.
    Temperature = 10,

    /// Pressure in `Pa`
    Pressure = 11,
}

impl DataId {
    /// Length of the ID variant in bytes
    const fn length(&self) -> usize {
        match self {
            DataId::Text => 255,
            DataId::Temperature => 2,
            DataId::Pressure => 2,
        }
    }
}

#[derive(Debug)]
pub struct Entry<const L: usize> {
    id: DataId,
    length: u16,
    data: [u8; L],
}

impl<const L: usize> Entry<L> {
    /// Create a new entry from a [DataId] and data.
    pub fn from_array(id: DataId, data: [u8; L]) -> Entry<L> {
        assert!(L <= 65536, "Length of data must be <= 65536");
        if id.length() != L {
            panic!("Length of `L` and id do not match!");
        }

        Entry::<L> {
            id,
            length: L as u16,
            data,
        }
    }

    pub fn from_int<T: Into<u64>>(id: DataId, number: T) -> Entry<L> {
        let encoded: u64 = number.into();

        if id.length() != L {
            panic!("Length of `L` and id do not match!");
        }

        Entry::<L> {
            id,
            length: L as u16,
            data: [0u8; L],
        }
    }
}

pub fn get_bytes<const S: usize>(iterator: &mut IntoIter<u8>) -> [u8; S] {
    let mut bytes = [0; S];

    for i in 0..S {
        bytes[i] = iterator.next().unwrap();
    }

    return bytes
}
