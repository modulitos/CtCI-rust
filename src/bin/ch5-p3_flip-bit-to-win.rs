// Flip Bit to Win: You have an integer and you can flip exactly one
// bit from a 0 to a 1. Write code to find the length of the longest
// sequence of 1s you could create.
// EXAMPLE
// Input: 1775 (or: 1113111131111)
// Output: 8

use std::cmp;

// This is O(b) time and space complexity, where b is the length of
// the binary representation.

// TODO: implement this function with O(1) space complexity.
fn flip_to_win(mut n: u32) -> u8 {
    if n == 0 {
        return 1;
    }
    // convert n into array of running sums of contiguous 0's and 1's
    // eg: 1101 -> [0, 2, 1, 1]
    // eg: 11101 -> [0, 3, 1, 1]
    // eg: 111001 -> [0, 3, 2, 1]
    // eg: 1110011 -> [0, 3, 2, 2]
    let mut arr: Vec<u8> = Vec::new();
    let mut count = 0;
    let mut searching_for = n & 1;
    arr.push(0);

    count += 1;
    n = n >> 1;

    while n > 0 {
        if (n & 1) != searching_for {
            arr.push(count);
            searching_for = n & 1;
            count = 0;
        }
        count += 1;
        n = n >> 1;
    }
    arr.push(count);

    // then traverse the array to find 1-2 1's groups to find the
    // maximum number of contiguous 1's
    let mut max_bits = 0;
    for i in (0..arr.len()).step_by(2) {
        let prev_bits = if i == 0 { 0 } else { arr[i - 1] };
        let next_bits = if i == arr.len() - 1 { 0 } else { arr[i + 1] };
        max_bits = cmp::max(
            max_bits,
            if arr[i] == 1 {
                prev_bits + next_bits + 1
            } else {
                cmp::max(prev_bits + 1, next_bits + 1)
            },
        );
    }
    max_bits
}

#[test]
fn test_flip_bit() {
    assert_eq!(flip_to_win(1775), 8);
    assert_eq!(flip_to_win(0b1101), 4);
    assert_eq!(flip_to_win(0b0000), 1);
    assert_eq!(flip_to_win(0b0001), 2);
    assert_eq!(flip_to_win(0b0101), 3);
    assert_eq!(flip_to_win(0b1100), 3);
    assert_eq!(flip_to_win(0b01010), 3);
    assert_eq!(flip_to_win(0b01011), 4);
    assert_eq!(flip_to_win(0b010011), 3);
    assert_eq!(flip_to_win(0b01110011), 4);
    assert_eq!(flip_to_win(0b101110011), 5);
}
