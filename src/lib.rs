use std::convert::TryInto;
use std::mem::size_of;

//TODO: Support for all integer types
//TODO: Support for endianness
//TODO: Better error handling
//TODO: Ability to read in reverse direction
//TODO: Ability to wrap around io::Read implementors

pub struct BitReader<'a> {
    current_position: u64,
    bits_len: u64,
    data: &'a [u8],
}

impl BitReader<'_> {
    pub fn new(data: &[u8]) -> BitReader {
        BitReader {
            current_position: 0,
            bits_len: (data.len() * 8) as u64,
            data,
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
        let result_len_bits = size_of::<u16>() as u16 * 8;
        assert!(nbits <= result_len_bits);

        if self.current_position + nbits as u64 > self.bits_len {
            return None;
        }

        let result = {
            let result_len = size_of::<u16>();
            let byte_idx = (self.current_position / 8) as usize;
            let enough_data = self.data.len() >= result_len;

            if enough_data && byte_idx < self.data.len() - (result_len - 1) {
                u16::from_le_bytes(
                    self.data[byte_idx..byte_idx + result_len]
                        .try_into()
                        .unwrap(),
                )
            } else {
                (self.data[byte_idx] as u16) << 8
            }
        };

        let relative_bit_idx = (self.current_position % 8) as u16;
        let zero_mask_head = 0b1111111111111111u16 >> relative_bit_idx;
        let right_boundary_shift = result_len_bits - (nbits + relative_bit_idx);

        self.current_position += nbits as u64;

        Some((result & zero_mask_head) >> right_boundary_shift as u16)
    }
}

#[cfg(test)]
mod tests {
    use crate::BitReader;

    #[test]
    fn test_bit_stream_general() {
        let bytes = [0b10100110, 0b10100110, 0b10100110, 0b10100110, 0b10100110];

        let mut reader = BitReader::new(&bytes);
        assert_eq!(reader.current_position(), 0);
        assert_eq!(reader.bits_len(), 40);

        let result = reader.read(11);
        assert_eq!(result, Some(0b0000010100110101));
        assert_eq!(reader.current_position, 11);

        let result = reader.read(10);
        assert_eq!(result, Some(0b0000000011010100));
        assert_eq!(reader.current_position, 21);

        let result = reader.read(8);
        assert_eq!(result, Some(0b0000000011010100));
        assert_eq!(reader.current_position, 29);

        let result = reader.read(11);
        assert_eq!(result, Some(0b0000011010100110));
        assert_eq!(reader.current_position, 40);

        reader.set_current_position(29);
        let result = reader.read(11);
        assert_eq!(result, Some(0b0000011010100110));
        assert_eq!(reader.current_position, 40);

        reader.set_current_position(35);
        let result = reader.read(10);
        assert_eq!(result, None);
        assert_eq!(reader.current_position, 35);
    }

    #[test]
    fn test_bit_stream_edge_cases() {
        let bytes = [0b10100110, 0b10100110, 0b10100110];

        let mut reader = BitReader::new(&bytes);
        assert_eq!(reader.current_position(), 0);
        assert_eq!(reader.bits_len(), 24);

        let result = reader.read(16);
        assert_eq!(result, Some(0b1010011010100110));
        assert_eq!(reader.current_position, 16);

        let result = reader.read(1);
        assert_eq!(result, Some(0b0000000000000001));
        assert_eq!(reader.current_position, 17);

        let result = reader.read(1);
        assert_eq!(result, Some(0b0000000000000000));
        assert_eq!(reader.current_position, 18);

        let result = reader.read(1);
        assert_eq!(result, Some(0b0000000000000001));
        assert_eq!(reader.current_position, 19);

        let result = reader.read(2);
        assert_eq!(result, Some(0b0000000000000000));
        assert_eq!(reader.current_position, 21);

        let result = reader.read(2);
        assert_eq!(result, Some(0b0000000000000011));
        assert_eq!(reader.current_position, 23);

        let result = reader.read(2);
        assert_eq!(result, None);
        assert_eq!(reader.current_position, 23);

        let result = reader.read(1);
        assert_eq!(result, Some(0b0000000000000000));
        assert_eq!(reader.current_position, 24);
    }
}
