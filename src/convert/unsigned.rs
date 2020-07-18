use crate::convert::FromU128;

impl FromU128 for u8 {
    fn from_u128_lossy(value: u128) -> Self {
        value as u8
    }
}

impl FromU128 for u16 {
    fn from_u128_lossy(value: u128) -> Self {
        value as u16
    }
}

impl FromU128 for u32 {
    fn from_u128_lossy(value: u128) -> Self {
        value as u32
    }
}

impl FromU128 for u64 {
    fn from_u128_lossy(value: u128) -> Self {
        value as u64
    }
}

impl FromU128 for u128 {
    fn from_u128_lossy(value: u128) -> Self {
        value
    }
}
