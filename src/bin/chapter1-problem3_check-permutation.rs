// Check Permutation: Given two strings, write a method to decide if
// one is a permutation of the other.
use std::collections::HashMap;

// clarifying questions: memory and ds limitations?
// What does permutation mean in this context?

// We can sort the strings, but that'll be nlogn. Although some
// algorithms will sort faster if the string are mostly sorted.

fn count_chars(s: &str) -> HashMap<char, i32> {
    let mut map = HashMap::new();
    for c in s.chars() {
        if let Some(count) = map.get(&c) {
            map.insert(c, count + 1);
        } else {
            map.insert(c, 1);
        }
    }
    map
}

fn check_perm(s1: &str, s2: &str) -> bool {
    if s1.len() != s2.len() {
        return false;
    }
    let map1 = count_chars(s1);
    let map2 = count_chars(s2);

    // compare the hashmaps:
    for (key, value) in map1.into_iter() {
        if let Some(count) = map2.get(&key) {
            if *count != value {
                return false;
            }
        } else {
            return false;
        }
    }
    true
}

// doing with with only 1 hashmap
fn check_perm_alt(s1: &str, s2: &str) -> bool {
    if s1.len() != s2.len() {
        return false;
    }
    let mut map = count_chars(s1);

    // compare the hashmaps:
    for c in s2.chars() {
        if let Some(count) = map.get(&c) {
            let new_count = count - 1;
            if new_count < 0 {
                return false;
            }
            map.insert(c, new_count);
        } else {
            return false;
        }
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_perm_1() {
        assert_eq!(check_perm("asdf", "safd"), true);
    }

    #[test]
    fn check_perm_2() {
        assert_eq!(check_perm("asdf", "zsdf"), false);
    }

    #[test]
    fn check_perm_3() {
        assert_eq!(check_perm("adf", "adfsss"), false);
    }

    #[test]
    fn check_perm_4() {
        assert_eq!(
            check_perm(
                "a man a plan a canal: panama",
                "panama: a plan a man a lanac"
            ),
            true
        );
    }

    #[test]
    fn check_perm_alt_1() {
        assert_eq!(check_perm_alt("asdf", "safd"), true);
    }

    #[test]
    fn check_perm_alt_2() {
        assert_eq!(check_perm_alt("asdf", "zsdf"), false);
    }

    #[test]
    fn check_perm_alt_3() {
        assert_eq!(check_perm_alt("adf", "adfsss"), false);
    }

    #[test]
    fn check_perm_alt_4() {
        assert_eq!(
            check_perm_alt(
                "a man a plan a canal: panama",
                "panama: a plan a man a lanac"
            ),
            true
        );
    }

}

fn main() {}
