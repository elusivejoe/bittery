use bittery::{BitOrder, BitReader};

//TODO: More clear tests logic
/*
1. read exact [amount of bits for each type] +
2. shift by 1 and read exact [..]
3. shift in the middle of a byte and read exact [..]
4. shift to the pre-last bit and read exact [..]
5. do the same as above but read exactly half amount of bits for each type
6. read exactly 1 bit for each type
7. read 5 bits for each type so they always cover two bytes
*/

fn bytes() -> Vec<u8> {
    vec![
        0b10110101, 0b01010001, 0b00000111, 0b01011010, 0b10010100, 0b11101110, 0b01000110,
        0b00110011, 0b00100101, 0b10110001, 0b01111101, 0b00110110, 0b01000011, 0b11000010,
        0b10100111, 0b00110110, 0b00011000, 0b00111011, 0b10010001, 0b10101010, 0b11101110,
        0b10101010, 0b01101111, 0b01010011, 0b00101111, 0b11100101, 0b10110111, 0b10111100,
        0b10101100, 0b01100111, 0b01000010,
    ]
}

#[test]
fn mixed_types_be() {
    let bytes = bytes();

    let mut reader = BitReader::new(&bytes, BitOrder::BigEndian);
    assert_eq!(reader.current_position(), 0);
    assert_eq!(reader.bits_len(), 248);

    let result = reader.read::<u8>(8);
    assert_eq!(result, Some(0b10110101));
    assert_eq!(reader.current_position(), 8);

    let result = reader.read::<u16>(16);
    assert_eq!(result, Some(0b0101000100000111));
    assert_eq!(reader.current_position(), 24);

    let result = reader.read::<u32>(32);
    assert_eq!(result, Some(0b01011010100101001110111001000110));
    assert_eq!(reader.current_position(), 56);

    let result = reader.read::<u64>(64);
    assert_eq!(
        result,
        Some(0b0011001100100101101100010111110100110110010000111100001010100111)
    );
    assert_eq!(reader.current_position(), 120);

    let result = reader.read::<u128>(128);
    assert_eq!(
        result,
        Some(0b00110110000110000011101110010001101010101110111010101010011011110101001100101111111001011011011110111100101011000110011101000010)
    );
    assert_eq!(reader.current_position(), 248);
}
