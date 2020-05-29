//TODO: Support for all integer types
//TODO: Better error handling
//TODO: Ability to read in reverse direction
//TODO: Ability to wrap around io::Read implementors

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

    pub fn read(&mut self, nbits: u8) -> Option<u16> {
        let nbits = nbits as u16;
        assert!(nbits <= 16);

        if self.current_position + nbits as u64 > self.bits_len {
            return None;
        }

        let start_byte = (self.current_position / 8) as usize;
        let start_bit_relative = (self.current_position % 8) as u16;

        let portion = if start_bit_relative + nbits > 8 {
            [self.data[start_byte], self.data[start_byte + 1]]
        } else {
            [self.data[start_byte], 0u8]
        };

        let portion = if let BitOrder::BigEndian = self.bit_order {
            u16::from_be_bytes(portion)
        } else {
            u16::from_le_bytes(portion)
        };

        self.current_position += nbits as u64;

        if let BitOrder::LittleEndian = self.bit_order {
            let mask = 0b1111111111111111u16 >> 16 - nbits;
            Some((portion >> start_bit_relative) & mask)
        } else {
            let mask = 0b1111111111111111u16 >> start_bit_relative;
            let right_boundary_shift = 16 - (nbits + start_bit_relative);
            Some((portion & mask) >> right_boundary_shift)
        }
    }
}
