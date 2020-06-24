use bittery::{BitOrder, BitReader, ReadOrder};

#[test]
fn direct_general_be() {
    let bytes = [0b11100111, 0b10100110, 0b11110100, 0b00100010, 0b00000111];

    let mut reader = BitReader::new(&bytes, BitOrder::BigEndian, ReadOrder::Direct);
    assert_eq!(reader.current_position(), 0);
    assert_eq!(reader.bits_len(), 40);

    let result = reader.read::<u16>(11);
    assert_eq!(result, Some(0b0000011100111101));
    assert_eq!(reader.current_position(), 11);

    let result = reader.read::<u16>(10);
    assert_eq!(result, Some(0b0000000011011110));
    assert_eq!(reader.current_position(), 21);

    let result = reader.read::<u16>(8);
    assert_eq!(result, Some(0b0000000010000100));
    assert_eq!(reader.current_position(), 29);

    let result = reader.read::<u16>(11);
    assert_eq!(result, Some(0b0000001000000111));
    assert_eq!(reader.current_position(), 40);

    reader.set_current_position(29);
    let result = reader.read::<u16>(11);
    assert_eq!(result, Some(0b0000001000000111));
    assert_eq!(reader.current_position(), 40);

    reader.set_current_position(35);
    let result = reader.read::<u16>(10);
    assert_eq!(result, None);
    assert_eq!(reader.current_position(), 35);
}

#[test]
fn direct_general_le() {
    let bytes = [0b11100111, 0b00110110, 0b01100010, 0b10100110, 0b00000110];

    let mut reader = BitReader::new(&bytes, BitOrder::LittleEndian, ReadOrder::Direct);
    assert_eq!(reader.current_position(), 0);
    assert_eq!(reader.bits_len(), 40);

    let result = reader.read::<u16>(11);
    assert_eq!(result, Some(0b0000011011100111));
    assert_eq!(reader.current_position(), 11);

    let result = reader.read::<u16>(10);
    assert_eq!(result, Some(0b0000000001000110));
    assert_eq!(reader.current_position(), 21);

    let result = reader.read::<u16>(8);
    assert_eq!(result, Some(0b0000000000110011));
    assert_eq!(reader.current_position(), 29);

    let result = reader.read::<u16>(11);
    assert_eq!(result, Some(0b0000000000110101));
    assert_eq!(reader.current_position(), 40);

    reader.set_current_position(29);
    let result = reader.read::<u16>(11);
    assert_eq!(result, Some(0b0000000000110101));
    assert_eq!(reader.current_position(), 40);

    reader.set_current_position(35);
    let result = reader.read::<u16>(10);
    assert_eq!(result, None);
    assert_eq!(reader.current_position(), 35);
}
