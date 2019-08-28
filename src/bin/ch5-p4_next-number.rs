// Next Number: Given a positive integer, print the next smallest and
// the next largest number that have the same number of 1 bits in
// their binary representation.

// what if there is no next number that matches the bits?

fn next_numbers(n: i32) -> (i32, i32) {
    (get_next_smallest(n), get_next_largest(n))
}

fn get_next_smallest(n: i32) -> i32 {
    // find p = the index of the right-most non-trailing 1
    let mut temp = n;
    let mut p = 0;
    while (temp & 1) == 1 {
        temp = temp >> 1;
        p += 1;
    }
    let mut zeroes = 0;
    while (temp & 1) == 0 {
        temp = temp >> 1;
        p += 1;
        zeroes += 1;
    }

    // flip bit at position p to a 0
    let mut next_smallest = n & !(1 << p);

    // c = count the number of 0's to the right of p, and subtract 1 from it
    // (to account for the bit just flipped)
    let c = zeroes - 1;

    // now we want to INCREASE as much as possible the bits following our bit flipped at p
    // set the right-most (p-1) bits to 1
    let p_minus_one_zeroes_mask = (1 << p) - 1;
    next_smallest = next_smallest | p_minus_one_zeroes_mask;

    // set the left-most c bits to 0
    let c_zeroes_mask = !((1 << c) - 1);
    next_smallest = next_smallest & c_zeroes_mask;

    next_smallest
}

fn get_next_largest(n: i32) -> i32 {
    // find the next largest:
    // p = find index of the right-most non-trailing 0
    let mut temp = n;
    let mut p = 0;
    while (temp & 1) == 0 {
        temp = temp >> 1;
        p += 1;
    }
    let mut ones = 0;
    while (temp & 1) == 1 {
        temp = temp >> 1;
        p += 1;
        ones += 1;
    }
    // flip bit at position p to 1
    let mut next_largest = n | (1 << p);

    // c = count the number of 1's to the right of p, and add 1 (to account for the bit we just flipped)
    let c = ones - 1;

    // set the right-most (p - 1) bits to 0
    let p_minus_one_mask = !((1 << p) - 1);
    next_largest = next_largest & p_minus_one_mask;

    // set the right most c bits to 1
    let c_ones_mask = (1 << c) - 1;
    next_largest = next_largest | c_ones_mask;

    next_largest
}

#[test]
fn test_next_number() {
    assert_eq!(
        next_numbers(0b11_0110_0111_1100),
        (0b11_0110_0111_1010, 0b11_0110_1000_1111)
    )
}
