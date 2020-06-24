use std::convert::TryInto;

use crate::integer::Integer;

impl Integer for i8 {
    fn new() -> Self {
        0
    }

    fn from_be(bytes: &[u8]) -> Self {
        bytes[0] as i8
    }

    fn from_le(bytes: &[u8]) -> Self {
        bytes[0] as i8
    }

    fn max_value() -> Self {
        u8::MAX as i8
    }
}

impl Integer for i16 {
    fn new() -> Self {
        0
    }

    fn from_be(bytes: &[u8]) -> Self {
        i16::from_be_bytes(bytes.try_into().unwrap())
    }

    fn from_le(bytes: &[u8]) -> Self {
        i16::from_le_bytes(bytes.try_into().unwrap())
    }

    fn max_value() -> Self {
        u16::MAX as i16
    }
}

impl Integer for i32 {
    fn new() -> Self {
        0
    }

    fn from_be(bytes: &[u8]) -> Self {
        i32::from_be_bytes(bytes.try_into().unwrap())
    }

    fn from_le(bytes: &[u8]) -> Self {
        i32::from_le_bytes(bytes.try_into().unwrap())
    }

    fn max_value() -> Self {
        u32::MAX as i32
    }
}

impl Integer for i64 {
    fn new() -> Self {
        0
    }

    fn from_be(bytes: &[u8]) -> Self {
        i64::from_be_bytes(bytes.try_into().unwrap())
    }

    fn from_le(bytes: &[u8]) -> Self {
        i64::from_le_bytes(bytes.try_into().unwrap())
    }

    fn max_value() -> Self {
        u64::MAX as i64
    }
}

impl Integer for i128 {
    fn new() -> Self {
        0
    }

    fn from_be(bytes: &[u8]) -> Self {
        i128::from_be_bytes(bytes.try_into().unwrap())
    }

    fn from_le(bytes: &[u8]) -> Self {
        i128::from_le_bytes(bytes.try_into().unwrap())
    }

    fn max_value() -> Self {
        u128::MAX as i128
    }
}
