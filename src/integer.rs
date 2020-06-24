mod signed;
mod unsigned;

pub trait Integer {
    fn new() -> Self;
    fn from_be(bytes: &[u8]) -> Self;
    fn from_le(bytes: &[u8]) -> Self;
    fn max_value() -> Self;
}
