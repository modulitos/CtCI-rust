// One Away: There are three types of edits that can be performed on
// strings: insert a character, remove a character, or replace a
// character. Given two strings, write a function to check if they are
// one edit (or zero edits) away. EXAMPLE pale, ple -> true pales.
// pale -> true pale. bale -> true pale. bake -> false

fn is_one_away(s1: &str, s2: &str) -> bool {
    // if same length, iterate over the chars, and if one or less
    // chars is different, return true
    let length1 = s1.len();
    let length2 = s2.len();
    let vec1: Vec<char> = s1.chars().collect();
    let vec2: Vec<char> = s2.chars().collect();
    if length1 == length2 {
        let mut diffs = 0;
        for (c1, c2) in s1.chars().zip(s2.chars()) {
            if c1 != c2 {
                diffs += 1;
            }
        }
        return diffs <= 1;
    }

    // if different length, ensure length doesn't differ by more than
    // one. Then iterate over chars, and if chars differ, increment
    // the pointer on the longer string. The chars can only differ
    // once.
    if (length1 as i32 - length2 as i32).abs() <= 1 {
        let shorter;
        let longer;
        if length1 < length2 {
            shorter = vec1;
            longer = vec2;
        } else {
            shorter = vec2;
            longer = vec1;
        }
        let mut diffs = 0;
        let mut i_short = 0;
        let mut i_long = 0;
        while i_short < shorter.len() && i_long < longer.len() {
            // println!(
            //     "shorter[i_short]: {}, longer[i_long]: {}",
            //     shorter[i_short], longer[i_long]
            // );
            if shorter[i_short] != longer[i_long] {
                // represents a character that should be deleted from
                // the longer string (or added to the shorter string
                diffs += 1;
                i_long += 1;
                if diffs > 1 || shorter[i_short] != longer[i_long] {
                    return false;
                }
            }
            i_short += 1;
            i_long += 1;
        }
        return true;
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(is_one_away("pale", "ple"), true);
    }
    #[test]
    fn test_2() {
        assert_eq!(is_one_away("pales", "pale"), true);
    }
    #[test]
    fn test_2_b() {
        assert_eq!(is_one_away("palas", "pale"), false);
    }
    #[test]
    fn test_3() {
        assert_eq!(is_one_away("pale", "bale"), true);
    }
    #[test]
    fn test_4() {
        assert_eq!(is_one_away("pale", "bake"), false);
    }
}

fn main() {
}
