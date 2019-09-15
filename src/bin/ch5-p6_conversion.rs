// Conversion: Write a function to determine the number of bits you
// would need to flip to convert integer A to integer B.
// EXAMPLE
// Input: 29 (or: 11101), 15 (or: 01111)
// Output: 2

fn determine_bits_basic(mut n1: u16, mut n2: u16) -> u8 {
    let mut bits = 0;
    while n1 > 0 || n2 > 0 {
        // println!("n1: {:b}", n1);
        // println!("n2: {:b}", n2);
        if n1 & 1 != n2 & 1 {
            bits += 1;
        }
        n1 = n1 >> 1;
        n2 = n2 >> 1;
    }
    bits
}

// `c = c & (c - 1)` clears the LSB. eg:
// 1100 & (1011)-> 1111
// 1101 & (1100)-> 1100
// 1000 & (0111)-> 0000
fn determine_bits(n1: u16, n2: u16) -> u8 {
    let mut bits = 0;
    let mut c = n1 ^ n2;
    while c != 0 {
        bits += 1;
        c = c & (c - 1);
    }
    bits
}

#[test]
fn test_conversion() {
    assert_eq!(determine_bits(0b0000_u16, 0b0000_u16), 0);
    assert_eq!(determine_bits(0b0001_u16, 0b0000_u16), 1);
    assert_eq!(determine_bits(0b0101_u16, 0b0000_u16), 2);
    assert_eq!(determine_bits(0b0101_u16, 0b0010_u16), 3);
    assert_eq!(determine_bits(0b11101_u16, 0b01111_u16), 2);
}
