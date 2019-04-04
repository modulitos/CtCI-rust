// String Compression: Implement a method to perform basic string
// compression using the counts of repeated characters. For example,
// the string aabcccccaaa would become a2b1c5a3. If the "compressed"
// string would not become smaller than the original string, your
// method should return the original string. You can assume the string
// has only uppercase and lowercase letters (a -z).

// number always required after string?
// if only one letter, always add a 1 after it?

// How big might these strings be?
// How many times in a row might a char occur?

// are we receiving lots of strings that don't need to be
// "compressed"? If so, then we should optimize for not having to
// create the compressed string up front.

// calculates the string length beforehand
fn get_compressed_length(s: &str) -> usize {
    let mut prev = None;
    let mut count = 0;
    let mut length = 0;
    for c in s.chars() {
        if let Some(temp) = prev {
            if temp == c {
                count += 1;
            } else {
                length += 1; // for the char
                length += (count / 10) + 1; // for the base 10 count following the char
                count = 0;
                prev = Some(c);
            }
        } else {
            prev = Some(c);
        }
    }
    if prev.is_some() {
        length += 1;
        length += (count / 10) + 1;
    }
    length
}

fn compress(s: &str) -> String {
    let length = get_compressed_length(s);

    println!("length: {}", length);
    if length > s.len() {
        return String::from(s);
    }
    let mut compressed = String::with_capacity(length);

    let mut count = 0;
    let mut prev: Option<char> = None;
    for c in s.chars() {
        if let Some(inner_prev) = prev {
            if c == inner_prev {
                count += 1;
            } else {
                compressed.push(inner_prev);
                count.to_string().chars().for_each(|ic| compressed.push(ic));
                // reset our counts to the current c:
                count = 1;
                prev = Some(c);
            }
        } else {
            prev = Some(c);
            count = 1;
        }
    }
    if let Some(unwrapped_c) = prev {
        compressed.push(unwrapped_c);
        count.to_string().chars().for_each(|ic| compressed.push(ic));
    }

    if compressed.len() > s.len() {
        s.to_string()
    } else {
        compressed
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(compress("aabcccccaaa"), "a2b1c5a3");
        assert_eq!(compress("abcaa"), "abcaa");
        assert_eq!(compress("doooonnkeee"), "d1o4n2k1e3");
    }
}

fn main() {
    compress("doooonnkeee");
}
