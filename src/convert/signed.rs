use crate::convert::FromU128;

impl FromU128 for i8 {
    fn from_u128_lossy(value: u128) -> Self {
        value as i8
    }
}

impl FromU128 for i16 {
    fn from_u128_lossy(value: u128) -> Self {
        value as i16
    }
}

impl FromU128 for i32 {
    fn from_u128_lossy(value: u128) -> Self {
        value as i32
    }
}

impl FromU128 for i64 {
    fn from_u128_lossy(value: u128) -> Self {
        value as i64
    }
}

impl FromU128 for i128 {
    fn from_u128_lossy(value: u128) -> Self {
        value as i128
    }
}
