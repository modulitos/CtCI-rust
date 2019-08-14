// Insertion: You are given two 32-bit numbers, N and M, and two bit
// positions, i and j. Write a method to insert Minto N such that M
// starts at bit j and ends at bit i. You can assume that the bits j
// through i have enough space to fit all of M. That is, if M = 10011,
// you can assume that there are at least 5 bits between j and i. You
// would not, for example, have j = 3 and i = 2, because M could not
// fully fit between bit 3 and bit 2.

// EXAMPLE
// Input:  N = 10000000000, M = 10011, i = 2, j = 6
// Output: N = 10001001100

// clarifying questions:
// if j - i > M's bit length, should we replace the remaining values with 0's?

fn update_bits(n: u32, m: u32, i: u8, j: u8) -> u32 {
    // First, clear the values between bits i and j.
    // Do this by creating 2 masks, then combining them.
    // Make the left mask (left = 0b111_1000_0000):
    let left_mask = !0 << (j + 1);
    // Then, make the right mask (right = 0b000_0000_0011):
    let right_mask = (1 << i) - 1;
    let mask = left_mask | right_mask;
    let cleared_n = n & mask;

    // set the bits for m:
    let m_shifted = m << i;
    cleared_n | m_shifted
}

#[test]
fn test_bits() {
    let mut updated = update_bits(0b100_0000_0000, 0b10_011, 2, 6);
    assert_eq!(updated, 0b100_0100_1100);

    updated = update_bits(0b100_1101_1000, 0b1011, 1, 5);
    assert_eq!(updated, 0b100_1101_0110);
}
