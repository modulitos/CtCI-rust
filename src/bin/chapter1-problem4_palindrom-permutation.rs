// Palindrome Permutation: Given a string, write a function to check
// if it is a permutation of a palindrome. A palindrome is a word or
// phrase that is the same forwards and backwards. A permutation is a
// rearrangement of letters. The palindrome does not need to be
// limited to just dictionary words. EXAMPLE Input: Tact Coa Output:
// True (permutations: "taco cat". "atco cta". etc.)

// spaces don't count? case sensitive?
// limited to ascii???

// checking for palindrome:
// eliminate spaces, lowercase
// add all keys to a map, counting the occurances. The values should all be even.
// if odd chars, only one value can be odd.
use std::collections::HashMap;

fn is_palindrome_perm(s: &str) -> bool {
    let clean: Vec<char> = s
        .to_ascii_lowercase()
        .chars()
        .filter(|c| *c != ' ')
        .collect();

    let mut map: HashMap<char, i32> = HashMap::new();

    // use `-- --nocapture` to reveal the printlns:
    println!("length of clean: {}", clean.len());
    let is_even = clean.len() % 2 == 0;
    for c in clean.into_iter() {
        if let Some(value) = map.get(&c) {
            map.insert(c, value + 1);
        } else {
            map.insert(c, 1);
        }
    }

    let mut odd_counts = 0;
    for value in map.values() {
        if value % 2 != 0 {
            odd_counts += 1;
            if is_even || odd_counts > 1 {
                return false;
            };
        }
    }

    true
}

// a more clever solution, without using external datastructures:
// (assumes ASCII chars)
fn is_palindrome_perm_bitv(s: &str) -> bool {
    let mut bitv: i32 = 0; // assuming ascii chars

    for mut c in s.chars() {
        c.make_ascii_lowercase();
        if c == ' ' {
            continue;
        }

        let int_char = c as i32 - 'a' as i32;

        if int_char < 0 || 26 < int_char {
            panic!("int_char is invalid: {}", int_char);
        }

        // toggle the bit (using exclusive or):
        bitv ^= 1 << int_char;
        println!(
            "c: {}, int_char: {}, bitv: {}",
            c,
            int_char,
            format!("{:b}", bitv)
        );
    }
    bitv & (bitv - 1) == 0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_1() {
        assert_eq!(is_palindrome_perm("Tact Coa"), true);
        assert_eq!(is_palindrome_perm_bitv("Tact Coa"), true);
    }
    #[test]
    fn check_2() {
        assert_eq!(is_palindrome_perm("Mr waldo jackson"), false);
        assert_eq!(is_palindrome_perm_bitv("Mr waldo jackson"), false);
    }
    #[test]
    fn check_3() {
        assert_eq!(is_palindrome_perm("Tact Ca"), true);
        assert_eq!(is_palindrome_perm_bitv("Tact Ca"), true);
    }
    #[test]
    fn check_4() {
        assert_eq!(is_palindrome_perm("Mr waldos jackson"), false);
        assert_eq!(is_palindrome_perm_bitv("Mr waldos jackson"), false);
    }

}

fn main() {}
