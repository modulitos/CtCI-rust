// Draw Line: A monochrome screen is stored as a single array of
// bytes, allowing eight consecutive pixels to be stored in one byte.
// The screen has width w, where w is divisible by 8 (that is, no byte
// will be split across rows). The height of the screen, of course,
// can be derived from the length of the array and t he w idth.
// Implement a function that draws a horizontal line from (x1, y) to
// (x2, y).

// The method signature should look something like:
// drawLine(byte[] screen, int width, int x1, int x2, int y)

// use std::u8::MAX;
const MAX: u8 = std::u8::MAX;
use std::convert::TryFrom;

// draws a line from (x1, x2) (inclusive), at row y.
fn draw_line(screen: &mut [u8], width: usize, x1: u8, x2: u8, y: usize) {
    let row_start = y * (width / 8);
    for (i, byte) in screen[row_start..(row_start + (width / 8))]
        .iter_mut()
        .enumerate()
    {
        let byte_start_index = u8::try_from(i * 8).unwrap();
        let x1_i = usize::from(x1 / 8);
        let x2_i = usize::from(x2 / 8);
        let in_same_byte = x1_i == x2_i && x1_i == i;
        let is_starting = x1_i < x2_i && x1_i == i;
        let is_ending = x1_i < x2_i && x2_i == i;
        let mask = if in_same_byte {
            let x1_mask = _get_x1_mask(x1 - byte_start_index);
            let x2_mask = _get_x2_mask(x2 - byte_start_index);
            x1_mask & x2_mask
        } else if is_starting {
            _get_x1_mask(x1 - byte_start_index)
        } else if is_ending {
            _get_x2_mask(x2 - byte_start_index)
        } else {
            // drawn line is outsite of x1/x2 range
            0b0000_0000_u8
        };
        // println!("mask: {:#010b}", mask);
        // println!("byte: {:#010b}", byte);
        *byte = *byte | mask;
    }
}

fn _get_x1_mask(x1: u8) -> u8 {
    if x1 == 0 {
        MAX
    } else {
        let x1mask = (1 << 8 - x1) - 1;
        // println!("x1mask: {:b}", x1mask);
        x1mask
    }
}
fn _get_x2_mask(x2: u8) -> u8 {
    let bits_to_shift = 8 - (x2 + 1);
    let x2mask = (MAX >> bits_to_shift) << bits_to_shift;
    // println!("x2mask: {:b}", x2mask);
    x2mask
}

fn _get_mask(x1: u8, x2: u8) -> u8 {
    _get_x1_mask(x1) & _get_x2_mask(x2)
}

#[test]
fn test_masks() {
    assert_eq!(_get_x2_mask(7), 0b1111_1111_u8);
    assert_eq!(_get_x2_mask(6), 0b1111_1110_u8);
    assert_eq!(_get_x2_mask(2), 0b1110_0000_u8);
    assert_eq!(_get_x2_mask(0), 0b1000_0000_u8);

    assert_eq!(_get_x1_mask(0), 0b1111_1111_u8);
    assert_eq!(_get_x1_mask(2), 0b0011_1111_u8);
    assert_eq!(_get_x1_mask(3), 0b0001_1111_u8);
    assert_eq!(_get_x1_mask(6), 0b0000_0011_u8);
    assert_eq!(_get_x1_mask(7), 0b0000_0001_u8);

    assert_eq!(_get_mask(2, 6), 0b0011_1110_u8);
    assert_eq!(_get_mask(3, 6), 0b0001_1110_u8);
    assert_eq!(_get_mask(0, 7), 0b1111_1111_u8);
    assert_eq!(_get_mask(0, 6), 0b1111_1110_u8);
    assert_eq!(_get_mask(1, 7), 0b0111_1111_u8);
}

// #[test]
fn test_conversion_width_1() {
    let mut screen = [
        0b0000_0000_u8,
        0b0000_0000_u8,
        0b0000_0000_u8,
        0b0000_0000_u8,
    ];
    let mut screen_result = [
        0b0000_0000_u8,
        0b0000_0000_u8,
        0b0011_1110_u8,
        0b0000_0000_u8,
    ];
    assert_eq!(screen[0..2], screen_result[0..2]);
    assert_ne!(screen[0..3], screen_result[0..3]);

    draw_line(&mut screen, 8, 2, 6, 2);
    assert_eq!(screen, screen_result);

    screen_result = [
        0b0000_0000_u8,
        0b0000_0000_u8,
        0b0011_1110_u8,
        0b0000_1111_u8,
    ];
    draw_line(&mut screen, 8, 4, 7, 3);
    assert_eq!(screen, screen_result);

    screen_result = [
        0b0000_0000_u8,
        0b0000_0000_u8,
        0b0011_1110_u8,
        0b1100_1111_u8,
    ];
    draw_line(&mut screen, 8, 0, 1, 3);
    assert_eq!(screen, screen_result);
}

#[test]
fn test_conversion() {
    let mut screen = [
        0b0000_0000_u8,
        0b0000_0000_u8,
        0b0000_0000_u8,
        0b0000_0000_u8,
        0b0000_0000_u8,
        0b0000_0000_u8,
        0b0000_0000_u8,
        0b0000_0000_u8,
    ];
    let mut screen_result = [
        0b0000_0000_u8,
        0b0000_0000_u8,
        0b0000_0000_u8,
        0b0000_0000_u8,
        0b0011_1110_u8,
        0b0000_0000_u8,
        0b0000_0000_u8,
        0b0000_0000_u8,
    ];
    draw_line(&mut screen, 16, 2, 6, 2);
    assert_eq!(screen, screen_result);

    screen = [
        0b0000_0000_u8,
        0b0000_0000_u8,
        0b0000_0000_u8,
        0b0000_0000_u8,
        0b0000_0000_u8,
        0b0000_0000_u8,
        0b0000_0000_u8,
        0b0000_0000_u8,
    ];
    screen_result = [
        0b0000_0000_u8,
        0b0000_0000_u8,
        0b0000_0000_u8,
        0b0000_0000_u8,
        0b0011_1111_u8,
        0b1100_0000_u8,
        0b0000_0000_u8,
        0b0000_0000_u8,
    ];
    draw_line(&mut screen, 16, 2, 9, 2);
    assert_eq!(screen, screen_result);

    screen = [
        0b0000_0000_u8,
        0b0000_0000_u8,
        0b0000_0000_u8,
        0b0000_0000_u8,
        0b0000_0000_u8,
        0b0000_0000_u8,
        0b0000_0000_u8,
        0b0000_0000_u8,
    ];
    screen_result = [
        0b0000_0000_u8,
        0b0000_0000_u8,
        0b0000_0000_u8,
        0b0000_0000_u8,
        0b0000_0000_u8,
        0b0000_1110_u8,
        0b0000_0000_u8,
        0b0000_0000_u8,
    ];
    draw_line(&mut screen, 16, 12, 14, 2);
    assert_eq!(screen, screen_result);
}
