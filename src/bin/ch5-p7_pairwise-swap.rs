// Pairwise Swap: Write a program to swap odd and even bits in an
// integer with as few instructions as possible (e.g., bit 0 and bit
// 1 are swapped, bit 2 and bit 3 are swapped, and so on).

fn pairwise_swap(n: u8) -> u8 {
    let even_mask = 0b10101010_u8;
    let odd_mask = even_mask >> 1;

    ((n & even_mask) >> 1) | ((n & odd_mask) << 1)
}

#[test]
fn test_swap() {
    assert_eq!(pairwise_swap(0b1_u8), 0b10_u8);
    assert_eq!(pairwise_swap(0b01010101_u8), 0b10101010_u8);
    assert_eq!(pairwise_swap(0b01010111_u8), 0b10101011_u8);
}
