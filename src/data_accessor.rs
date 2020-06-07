pub enum ReadOrder {
    Direct,
    Reverse,
}

pub struct DataAccessor<'a> {
    data: &'a [u8],
    read_order: ReadOrder,
}

impl DataAccessor<'_> {
    pub fn new(data: &[u8], read_order: ReadOrder) -> DataAccessor {
        DataAccessor { data, read_order }
    }

    pub fn get(&self, index: usize) -> u8 {
        match self.read_order {
            ReadOrder::Direct => self.data[index],
            ReadOrder::Reverse => Self::reverse_bits(self.data[self.data.len() - index - 1]),
        }
    }

    fn reverse_bits(mut byte: u8) -> u8 {
        let mut result = 0;

        for _ in 0u8..8 {
            result = (result << 1) | byte & 0b00000001;
            byte >>= 1;
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use crate::data_accessor::DataAccessor;
    use crate::ReadOrder;

    #[test]
    fn read_direct() {
        let data = [1, 2, 3, 100, 255];
        let accessor = DataAccessor::new(&data, ReadOrder::Direct);

        assert_eq!(data[0], accessor.get(0));
        assert_eq!(data[1], accessor.get(1));
        assert_eq!(data[2], accessor.get(2));
        assert_eq!(data[3], accessor.get(3));
        assert_eq!(data[4], accessor.get(4));
    }

    #[test]
    fn read_reverse() {
        let data = [
            0b00001111, 0b11111111, 0b01001101, 0b00000000, 0b11110000, 0b00000001, 0b10010011,
        ];

        let accessor = DataAccessor::new(&data, ReadOrder::Reverse);

        assert_eq!(0b11001001, accessor.get(0));
        assert_eq!(0b10000000, accessor.get(1));
        assert_eq!(0b00001111, accessor.get(2));
        assert_eq!(0b00000000, accessor.get(3));
        assert_eq!(0b10110010, accessor.get(4));
        assert_eq!(0b11111111, accessor.get(5));
        assert_eq!(0b11110000, accessor.get(6));
    }
}
