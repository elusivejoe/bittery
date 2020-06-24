use bittery::{BitOrder, BitReader, ReadOrder};

#[test]
fn reverse_boundary_conditions_be() {
    let bytes = [0b10100110, 0b00110110, 0b00100011];
    //same in reverse: [0b11000100, 0b01101100, 0b01100101]

    let mut reader = BitReader::new(&bytes, BitOrder::BigEndian, ReadOrder::Reverse);
    assert_eq!(reader.current_position(), 0);
    assert_eq!(reader.bits_len(), 24);

    let result = reader.read::<u16>(16);
    assert_eq!(result, Some(0b1100010001101100));
    assert_eq!(reader.current_position(), 16);

    let result = reader.read::<u16>(1);
    assert_eq!(result, Some(0b0000000000000000));
    assert_eq!(reader.current_position(), 17);

    let result = reader.read::<u16>(1);
    assert_eq!(result, Some(0b0000000000000001));
    assert_eq!(reader.current_position(), 18);

    let result = reader.read::<u16>(1);
    assert_eq!(result, Some(0b0000000000000001));
    assert_eq!(reader.current_position(), 19);

    let result = reader.read::<u16>(2);
    assert_eq!(result, Some(0b0000000000000000));
    assert_eq!(reader.current_position(), 21);

    let result = reader.read::<u16>(2);
    assert_eq!(result, Some(0b0000000000000010));
    assert_eq!(reader.current_position(), 23);

    let result = reader.read::<u16>(2);
    assert_eq!(result, None);
    assert_eq!(reader.current_position(), 23);

    let result = reader.read::<u16>(1);
    assert_eq!(result, Some(0b0000000000000001));
    assert_eq!(reader.current_position(), 24);
}

#[test]
fn reverse_boundary_conditions_le() {
    let bytes = [0b10100110, 0b00110110, 0b10100011];
    //same in reverse: [0b11000101, 0b01101100, 0b01100101]

    let mut reader = BitReader::new(&bytes, BitOrder::LittleEndian, ReadOrder::Reverse);
    assert_eq!(reader.current_position(), 0);
    assert_eq!(reader.bits_len(), 24);

    let result = reader.read::<u16>(16);
    assert_eq!(result, Some(0b0110110011000101));
    assert_eq!(reader.current_position(), 16);

    let result = reader.read::<u16>(1);
    assert_eq!(result, Some(0b0000000000000001));
    assert_eq!(reader.current_position(), 17);

    let result = reader.read::<u16>(1);
    assert_eq!(result, Some(0b0000000000000000));
    assert_eq!(reader.current_position(), 18);

    let result = reader.read::<u16>(1);
    assert_eq!(result, Some(0b0000000000000001));
    assert_eq!(reader.current_position(), 19);

    let result = reader.read::<u16>(2);
    assert_eq!(result, Some(0b0000000000000000));
    assert_eq!(reader.current_position(), 21);

    let result = reader.read::<u16>(2);
    assert_eq!(result, Some(0b0000000000000011));
    assert_eq!(reader.current_position(), 23);

    let result = reader.read::<u16>(2);
    assert_eq!(result, None);
    assert_eq!(reader.current_position(), 23);

    let result = reader.read::<u16>(1);
    assert_eq!(result, Some(0b0000000000000000));
    assert_eq!(reader.current_position(), 24);
}
