// glorified notepad

const PUBLIC_SERVICE_ANNOUNCEMENT: &str =
    "This is another public service announcement brought to you, in part, by Slim Shady.";

fn stuff() {
    let text = Text::with_baseline(
        PUBLIC_SERVICE_ANNOUNCEMENT,
        Point::zero(),
        MonoTextStyleBuilder::new()
            .font(&FONT_10X20)
            .text_color(BinaryColor::On)
            .build(),
        Baseline::Middle,
    );

    let mut text_translation = Point::new(128, 32);

    // loop is missing, we were doing some text moving so it scrolls
    // check "ssh1106 oled display is operational" commit for details
}

// this stinks a little bit
fn wrapping_decr(num: &mut i32, decr_value: i32, messg_len: usize, font_width: i32) {
    *num -= decr_value;
    if *num < -(messg_len as i32 * font_width) {
        *num = 128;
    }
}

/*
    nrf -> microbit
    0 -> SPEAKER
    1 -> 14
    2 -> 0
    3 -> 1
    4 -> 2
    9 -> 9
    10 -> 8
    11 -> 7 / COL_1
    12 -> 12
    13 -> 15
    14 -> 5
    15 -> ROW_2
    16 -> ?
    17 -> 13
    18 -> ?
    19 -> ROW_4
    20 -> ?
    21 -> ROW_0
    22 -> ROW_1
    23 -> 11
    24 -> ROW_3
    25 -> ?
    26 -> 19 / I2C_SCL
    27 -> ?
    28 -> 4 / COL_0
    29 -> ?
    30 -> 10 / COL_4
    31 -> 3 / COL_2
    32 -> 20 / I2C_SDA
    33 -> ?
    34 -> 16
    35 -> ?
    36 -> ?
    37 -> 6 / COL_3
*/

// const FIRST_PIN: u32 = 0;
// const LAST_PIN: u32 = 48;
// const SPEAKER: [usize; 1] = [0];
// const I2C: [usize; 2] = [26, 32];
// const I2C_SCL = 26;
// const I2C_SDA = 32;
// const MB_I2C_SCL: usize = 19;
// const MB_I2C_SDA: usize = 20;
// const GPIO: [usize; 19] = [
//     2, 3, 4, 31, 28, 14, 37, 11, 10, 9, 30, 23, 12, 17, 1, 13, 34, 26, 32,
// ]; // pins(0..=20).skip(17, 18)
// const LED_ROWS: [usize; 5] = [21, 22, 15, 24, 19];
// const LED_COLS: [usize; 5] = [28, 11, 31, 37, 30];
