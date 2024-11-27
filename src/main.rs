#![no_main]
#![no_std]

use cortex_m_rt::entry;
use embedded_hal::digital::OutputPin;
use nrf52833_hal::{
    gpio::{Disconnected, Level, Output, Pin, PushPull},
    pac::Peripherals,
    Timer,
};
use panic_halt as _;

#[entry]
fn main() -> ! {
    let peripherals = Peripherals::take().unwrap();
    let mut timer = Timer::new(peripherals.TIMER0);

    // let pins: [Pin<Disconnected>; 49] = core::array::from_fn(|i| unsafe { Pin::from_psel_bits(i as u32) });
    let mut pins: [Pin<Output<PushPull>>; 49] = core::array::from_fn(|i| {
        unsafe { Pin::<Disconnected>::from_psel_bits(i as u32) }.into_push_pull_output(Level::Low)
    });

    #[allow(unused_must_use)]
    loop {
        for pin in (0..=21).filter_map(|pin| to_hal(pin).ok()) {
            pins[pin].set_high();
            timer.delay(125_000);
            pins[pin].set_low();
        }
    }
}

#[derive(Debug)]
struct InvalidMicrobitPinError;

fn to_hal(microbit_pin: usize) -> Result<usize, InvalidMicrobitPinError> {
    const HAL_ADDRESSESS: [i32; 21] = [
        2, 3, 4, 31, 28, 14, 37, 11, 10, 9, 30, 23, 12, 17, 1, 13, 34, -1, -1, 26, 32,
    ];

    HAL_ADDRESSESS
        .get(microbit_pin)
        .copied()
        .ok_or(InvalidMicrobitPinError)?
        .try_into()
        .map_err(|_| InvalidMicrobitPinError)
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
    26 -> 19
    27 -> ?
    28 -> 4 / COL_0
    29 -> ?
    30 -> 10 / COL_4
    31 -> 3 / COL_2
    32 -> 20
    33 -> ?
    34 -> 16
    35 -> ?
    36 -> ?
    37 -> 6 / COL_3
*/

// const FIRST_PIN: u32 = 0;
// const LAST_PIN: u32 = 48;
// const SPEAKER: [usize; 1] = [0];
// const GPIO: [usize; 19] = [
//     2, 3, 4, 31, 28, 14, 37, 11, 10, 9, 30, 23, 12, 17, 1, 13, 34, 26, 32,
// ]; // pins(0..=20).skip(17, 18)
// const LED_ROWS: [usize; 5] = [21, 22, 15, 24, 19];
// const LED_COLS: [usize; 5] = [28, 11, 31, 37, 30];
