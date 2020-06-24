use std::convert::TryInto;

use crate::integer::Integer;

impl Integer for u8 {
    fn new() -> Self {
        0
    }

    fn from_be(bytes: &[u8]) -> Self {
        bytes[0]
    }

    fn from_le(bytes: &[u8]) -> Self {
        bytes[0]
    }

    fn max_value() -> Self {
        u8::MAX
    }
}

impl Integer for u16 {
    fn new() -> Self {
        0
    }

    fn from_be(bytes: &[u8]) -> Self {
        u16::from_be_bytes(bytes.try_into().unwrap())
    }

    fn from_le(bytes: &[u8]) -> Self {
        u16::from_le_bytes(bytes.try_into().unwrap())
    }

    fn max_value() -> Self {
        u16::MAX
    }
}

impl Integer for u32 {
    fn new() -> Self {
        0
    }

    fn from_be(bytes: &[u8]) -> Self {
        u32::from_be_bytes(bytes.try_into().unwrap())
    }

    fn from_le(bytes: &[u8]) -> Self {
        u32::from_le_bytes(bytes.try_into().unwrap())
    }

    fn max_value() -> Self {
        u32::MAX
    }
}

impl Integer for u64 {
    fn new() -> Self {
        0
    }

    fn from_be(bytes: &[u8]) -> Self {
        u64::from_be_bytes(bytes.try_into().unwrap())
    }

    fn from_le(bytes: &[u8]) -> Self {
        u64::from_le_bytes(bytes.try_into().unwrap())
    }

    fn max_value() -> Self {
        u64::MAX
    }
}

impl Integer for u128 {
    fn new() -> Self {
        0
    }

    fn from_be(bytes: &[u8]) -> Self {
        u128::from_be_bytes(bytes.try_into().unwrap())
    }

    fn from_le(bytes: &[u8]) -> Self {
        u128::from_le_bytes(bytes.try_into().unwrap())
    }

    fn max_value() -> Self {
        u128::MAX
    }
}
