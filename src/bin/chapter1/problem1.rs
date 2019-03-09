// Is Unique: Implement an algorithm to determine if a string has all
// unique characters. What if you cannot use additional data
// structures?

fn is_unique(string: &str) -> bool {
    // We can also immediately return false if the string length exceeds
    // the number of unique characters in the alphabet. After all, you
    // can't form a string of 280 unique characters out of a 128-character
    // alphabet.
    if string.len() > 128 {
        return false;
    }

    let mut matches: [bool; 128] = [false; 128]; // assume the 128 ASCII alphabet
    for character in string.chars() {
        // println!("checking char: {}", character);
        // println!("checking char value: {}", character as usize);
        if matches[character as usize] {
            return false;
        }
        matches[character as usize] = true;
    }
    true
}

// challenge: do this without data structures!
// clarifying questions - what kind of characters can I use???
// eg: are emojis allowed???
fn is_unique_raw(string: &str) -> bool {
    // ideally, we'd do an in-place sort, then run through the chars
    // linearly to check for adjacent duplicates, which is O(n*logn)

    // alternatively, we can do an O(n^2) solution:
    for (i, needle) in string.chars().enumerate() {
        for (j, character) in string.chars().enumerate() {
            // println!("comparing chars: {}, {}", needle, character);
            if j == i {
                break;
            }
            if needle == character {
                return false;
            }
        }
    }
    true
}

fn is_unique_part_b(s: &str) -> bool {
    let mut bitfield: i64 = 0; // assumes 8 bytes of possible char values
    let a_int_char: i16 = 'a' as i16; // single quotes means it's a char

    for c in s.chars() {
        let mut int_char: i16 = c as i16;
        int_char -= a_int_char;

        // check if bit is set in our bitfield:
        if (1 << int_char) & bitfield != 0 {
            return false;
        }

        // set bit in our bitfield:
        // "|=" is a "bitwise or", then take number 1, and rotate it by int_char bits
        bitfield |= 1 << int_char;
    }

    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn unique_1() {
        assert_eq!(is_unique("asdf"), true);
    }

    #[test]
    fn unique_2() {
        assert_eq!(is_unique("asdfa"), false);
    }

    #[test]
    fn unique_raw_1() {
        assert_eq!(is_unique_raw("asdf"), true);
    }

    #[test]
    fn unique_raw_2() {
        assert_eq!(is_unique_raw("asdfa"), false);
    }

    #[test]
    fn unique_part_b_1() {
        assert_eq!(is_unique_part_b("asdf"), true);
    }

    #[test]
    fn unique_part_b_2() {
        assert_eq!(is_unique_part_b("asdfa"), false);
    }

}
