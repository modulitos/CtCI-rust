// Binary to String: Given a real number between 0 and 1 (e.g., 0.72)
// that is passed in as a double, print the binary representation. If
// the number cannot be represented accurately in binary with at most
// 32 characters, print "ERROR"

use std::fmt::Write;

fn get_binary_float(mut n: f32) -> String {
    if n < 0.0 || n >= 1.0 {
        return String::from("ERROR");
    }

    let mut current = 0.5;
    let mut binary_rep: Vec<char> = Vec::new();
    while n > 0.0 {
        if binary_rep.len() > 32 {
            return String::from("ERROR");
        }

        if n >= current {
            binary_rep.push('1');
            n -= current;
        } else {
            binary_rep.push('0');
        }

        current = current / 2.0;
    }

    let mut res = String::from("0.");
    for c in binary_rep.iter() {
        write!(&mut res, "{}", c).unwrap();
    }
    res
}

#[test]
fn test_error() {
    assert_eq!(get_binary_float(1.1), "ERROR");
}

#[test]
fn test_basic() {
    assert_eq!(get_binary_float(0.625), "0.101");
    assert_eq!(get_binary_float(0.6875), "0.1011");
    assert_eq!(get_binary_float(0.0625), "0.0001");
    assert_eq!(get_binary_float(0.9999999999999), "ERROR");
}

