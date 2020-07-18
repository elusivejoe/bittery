//TODO: Better error handling
//TODO: Ability to wrap around io::Read implementors
//TODO: Better navigation like move(Forward(10))
//TODO: Support for Huffman codes

mod convert;

pub struct BitReader<'a> {
    current_position: u64,
    bits_len: u64,
    data: &'a [u8],
    bit_order: BitOrder,
}

pub enum BitOrder {
    BigEndian,
    LittleEndian,
}

impl BitReader<'_> {
    pub fn new(data: &[u8], bit_order: BitOrder) -> BitReader {
        BitReader {
            current_position: 0,
            bits_len: (data.len() * 8) as u64,
            data,
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

    pub fn read<T: convert::FromU128>(&mut self, nbits: u8) -> Option<T> {
        assert!(nbits > 0);

        let type_size = (std::mem::size_of::<T>() * 8) as u8;

        assert!(nbits <= type_size);

        let nbits = nbits as u64;

        if self.current_position + nbits > self.bits_len {
            return None;
        }

        let start_byte = (self.current_position / 8) as usize;
        let end_byte = ((self.current_position + (nbits - 1)) / 8) as usize;

        let mut portion = 0u128;

        for index in start_byte..=end_byte {
            let current = {
                if let BitOrder::BigEndian = self.bit_order {
                    self.data[index]
                } else {
                    self.data[end_byte - (index - start_byte)]
                }
            };

            portion <<= 8;
            portion |= current as u128;
        }

        let mask = u128::MAX >> (128 - nbits);

        let result = if let BitOrder::BigEndian = self.bit_order {
            let end_bit_relative = (self.current_position + nbits) % 8;

            let right_shift = if end_bit_relative > 0 {
                8 - end_bit_relative
            } else {
                0
            } as u128;

            (portion >> right_shift) & mask
        } else {
            let start_bit_relative = self.current_position % 8;
            (portion >> start_bit_relative as u128) & mask
        };

        self.current_position += nbits;

        Some(T::from_u128_lossy(result))
    }
}
