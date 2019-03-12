// URLify: Write a method to replace all spaces in a string with '%20:
// You may assume that the string has sufficient space at the end to
// hold the additional characters, and that you are given the "true"
// length of the string. (Note: If implementing in Java, please use a
// character array so that you can perform this operation in place.)

// q's:
// must we modify the string in place?

fn urlify(s: &str, length: usize) -> String {
    // Ideally, we'd modify the characters in place instead of
    // creating a separate Vec of chars, but rust doesn't seem to
    // allow for that. Perhaps this is possible using `unsafe`?
    let mut chars: Vec<char> = s.chars().collect();
    let mut diff = s.len() - length;

    for i in (0..length).rev() {
        println!("iteratiing over i: {}", i);
        println!("diff: {}", diff);
        let j = i + diff;
        let c = chars[i];
        match c {
            ' ' => {
                chars[j] = '0';
                chars[j - 1] = '2';
                chars[j - 2] = '%';
                diff -= 2;
            }
            _ => chars[j] = c,
        }
    }
    chars.into_iter().collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn urlify_1() {
        assert_eq!(urlify("Mr John Smith    ", 13), "Mr%20John%20Smith");
    }

}

fn main() {}
