// Debugger: Explain what the following code does: ( n & (n - 1)) == 0

// Answer:
// checks whether n is a 1 followed by 0 or more 0's
// IOW, checks whether n is a power of 2!
fn debugger(n: u8) -> bool {
    (n & (n - 1)) == 0
}

#[test]
fn test_debugger() {
    assert_eq!(debugger(0b10000000_u8), true);
    assert_eq!(debugger(0b00001000_u8), true);
    assert_eq!(debugger(0b00001001_u8), false);
    assert_eq!(debugger(0b00000001_u8), true);
    assert_eq!(debugger(0b00100100_u8), false);
    assert_eq!(debugger(0b11010000_u8), false);
    assert_eq!(debugger(0b11011111_u8), false);
    assert_eq!(debugger(0b01011111_u8), false);
    assert_eq!(debugger(0b01000000_u8), true);
}
