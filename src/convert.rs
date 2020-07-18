mod signed;
mod unsigned;

pub trait FromU128 {
    fn from_u128_lossy(value: u128) -> Self;
}
