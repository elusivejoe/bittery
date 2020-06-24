//TODO: Better error handling
//TODO: Ability to wrap around io::Read implementors
//TODO: Better navigation like move(Forward(10))
//TODO: Support for Huffman codes

mod data_accessor;
mod integer;

use data_accessor::DataAccessor;
pub use data_accessor::ReadOrder;

pub struct BitReader<'a> {
    current_position: u64,
    bits_len: u64,
    data: DataAccessor<'a>,
    bit_order: BitOrder,
}

pub enum BitOrder {
    BigEndian,
    LittleEndian,
}

impl BitReader<'_> {
    pub fn new(data: &[u8], bit_order: BitOrder, read_order: ReadOrder) -> BitReader {
        BitReader {
            current_position: 0,
            bits_len: (data.len() * 8) as u64,
            data: DataAccessor::new(data, read_order),
            bit_order,
        }
    }

    pub fn bits_len(&self) -> u64 {
        self.bits_len
    }

    pub fn current_position(&self) -> u64 {
        self.current_position
    }

    pub fn set_current_position(&mut self, pos: u64) {
        assert!(pos < self.bits_len);
        self.current_position = pos;
    }

    pub fn read<T>(&mut self, nbits: u8) -> Option<T>
    where
        T: integer::Integer
            + Copy
            + From<u8>
            + std::ops::Shr<Output = T>
            + std::ops::BitAnd<Output = T>,
    {
        assert!(nbits > 0);

        let type_size = (std::mem::size_of::<T>() * 8) as u8;

        assert!(nbits <= type_size);

        if self.current_position + nbits as u64 > self.bits_len {
            return None;
        }

        let mut buffer = [0u8; 16];
        let mut buf_idx = 0;

        let start_byte = (self.current_position / 8) as usize;
        let end_byte = ((self.current_position + (nbits - 1) as u64) / 8) as usize;

        for index in start_byte..=end_byte {
            buffer[buf_idx] = self.data.get(index);
            buf_idx += 1;
        }

        let portion = if let BitOrder::BigEndian = self.bit_order {
            T::from_be(&buffer[0..std::mem::size_of::<T>()])
        } else {
            T::from_le(&buffer[0..std::mem::size_of::<T>()])
        };

        let start_bit_relative = (self.current_position % 8) as u8;

        self.current_position += nbits as u64;

        if let BitOrder::LittleEndian = self.bit_order {
            let mask = T::max_value() >> (type_size - nbits).into();
            Some((portion >> start_bit_relative.into()) & mask)
        } else {
            let mask = T::max_value() >> start_bit_relative.into();
            let right_boundary_shift = type_size - (nbits + start_bit_relative);
            Some((portion & mask) >> right_boundary_shift.into())
        }
    }
}
