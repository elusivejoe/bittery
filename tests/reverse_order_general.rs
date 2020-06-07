use bittery::{BitOrder, BitReader, ReadOrder};

#[test]
fn reverse_general_be() {
    let bytes = [0b11100111, 0b10100110, 0b11110100, 0b00100010, 0b00000111];
    //same in reverse: [0b11100000, 0b01000100, 0b00101111, 0b01100101, 0b11100111]

    let mut reader = BitReader::new(&bytes, BitOrder::BigEndian, ReadOrder::Reverse);
    assert_eq!(reader.current_position(), 0);
    assert_eq!(reader.bits_len(), 40);

    let result = reader.read(11);
    assert_eq!(result, Some(0b0000011100000010));
    assert_eq!(reader.current_position(), 11);

    let result = reader.read(10);
    assert_eq!(result, Some(0b0000000010000101));
    assert_eq!(reader.current_position(), 21);

    let result = reader.read(8);
    assert_eq!(result, Some(0b0000000011101100));
    assert_eq!(reader.current_position(), 29);

    let result = reader.read(11);
    assert_eq!(result, Some(0b0000010111100111));
    assert_eq!(reader.current_position(), 40);

    reader.set_current_position(29);
    let result = reader.read(11);
    assert_eq!(result, Some(0b0000010111100111));
    assert_eq!(reader.current_position(), 40);

    reader.set_current_position(35);
    let result = reader.read(10);
    assert_eq!(result, None);
    assert_eq!(reader.current_position(), 35);
}

#[test]
fn reverse_general_le() {
    let bytes = [0b11100111, 0b00110110, 0b01100010, 0b10100110, 0b00000110];
    //same in reverse: [0b01100000, 0b01100101, 0b01000110, 0b01101100, 0b11100111];

    let mut reader = BitReader::new(&bytes, BitOrder::LittleEndian, ReadOrder::Reverse);
    assert_eq!(reader.current_position(), 0);
    assert_eq!(reader.bits_len(), 40);

    let result = reader.read(11);
    assert_eq!(result, Some(0b0000010101100000));
    assert_eq!(reader.current_position(), 11);

    let result = reader.read(10);
    assert_eq!(result, Some(0b0000000011001100));
    assert_eq!(reader.current_position(), 21);

    let result = reader.read(8);
    assert_eq!(result, Some(0b0000000001100010));
    assert_eq!(reader.current_position(), 29);

    let result = reader.read(11);
    assert_eq!(result, Some(0b0000011100111011));
    assert_eq!(reader.current_position(), 40);

    reader.set_current_position(29);
    let result = reader.read(11);
    assert_eq!(result, Some(0b0000011100111011));
    assert_eq!(reader.current_position(), 40);

    reader.set_current_position(35);
    let result = reader.read(10);
    assert_eq!(result, None);
    assert_eq!(reader.current_position(), 35);
}
