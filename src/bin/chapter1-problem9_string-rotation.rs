// String Rotation: Assume you have a method isSubstring which checks
// if one word is a substring of another. Given two strings, 51 and
// 52, write code to check if 52 is a rotation of 51 using only one
// call to isSubstring (e.g., "waterbottle" is a rotation
// of"erbottlewat").

// clarifying q's:
// are there any space requirements?

fn is_rotation(s1: &str, s2: &str) -> bool {
    if s1.len() != s2.len() {
        return false;
    }
    let mut doubled = s2.to_owned();
    doubled.push_str(s2);
    doubled.contains(s1)
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(is_rotation("waterbottle", "erbottlewat"), true);
        assert_eq!(is_rotation("waterbottle", "bottlewataa"), false);
        assert_eq!(is_rotation("", ""), true);
    }
}
fn main() {}
