#![no_main]
#![no_std]

use cortex_m_rt::entry;
use embedded_hal::{
    digital::{InputPin, OutputPin},
    spi::MODE_0,
};
use nrf52833_hal::{
    gpio::{Disconnected, Level, Pin},
    pac::Peripherals,
    spi::{Frequency, Pins, Spi},
};
use panic_halt as _;
use sh1106::{mode::GraphicsMode, Builder};

#[entry]
fn main() -> ! {
    let peripherals = Peripherals::take().unwrap();
    // let mut timer = Timer::new(peripherals.TIMER0);

    // OLED
    let mut oled: GraphicsMode<_> = {
        let spi_pins = Pins {
            sck: unsafe {
                Some(
                    Pin::<Disconnected>::from_psel_bits(to_hal(14).unwrap())
                        .into_push_pull_output(Level::Low),
                )
            },
            mosi: unsafe {
                Some(
                    Pin::<Disconnected>::from_psel_bits(to_hal(15).unwrap())
                        .into_push_pull_output(Level::Low),
                )
            },
            miso: None,
        };

        Builder::new()
            .connect_spi(
                Spi::new(peripherals.SPI0, spi_pins, Frequency::M4, MODE_0),
                unsafe {
                    Pin::<Disconnected>::from_psel_bits(to_hal(2).unwrap())
                        .into_push_pull_output(Level::High)
                },
                unsafe {
                    Pin::<Disconnected>::from_psel_bits(to_hal(13).unwrap())
                        .into_push_pull_output(Level::High)
                },
            )
            .into()
    };
    let mut reset_pin: Pin<nrf52833_hal::gpio::Output<nrf52833_hal::gpio::PushPull>> =
        unsafe { Pin::<Disconnected>::from_psel_bits(to_hal(12).unwrap()) }
            .into_push_pull_output(Level::High);
    oled.init().unwrap();

    // let PAD(mut B, mut A, mut Y, mut X) = xyab();
    let Pad(mut right, mut down, mut up, mut left) = directions();

    let mut lil_human = (64, 32);

    #[allow(unused_must_use)]
    loop {
        reset_pin.set_high();

        oled.set_pixel(lil_human.0, lil_human.1, 0); // erase lil_human
        if right.is_low().unwrap() && lil_human.0 < 127 {
            lil_human.0 += 1;
        }
        if left.is_low().unwrap() && lil_human.0 > 0 {
            lil_human.0 -= 1;
        }
        if down.is_low().unwrap() && lil_human.1 < 63 {
            lil_human.1 += 1;
        }
        if up.is_low().unwrap() && lil_human.1 > 0 {
            lil_human.1 -= 1;
        }
        // oled.clear();
        oled.set_pixel(lil_human.0, lil_human.1, 1); // draw new lil_human

        oled.flush().unwrap();
    }
}
#[derive(Debug)]
struct InvalidMicrobitPinError;

fn to_hal(microbit_pin: u32) -> Result<u32, InvalidMicrobitPinError> {
    const HAL_ADDRESSESS: [i32; 21] = [
        2, 3, 4, 31, 28, 14, 37, 11, 10, 9, 30, 23, 12, 17, 1, 13, 34, -1, -1, 26, 32,
    ];

    HAL_ADDRESSESS
        .get(microbit_pin as usize)
        .copied()
        .ok_or(InvalidMicrobitPinError)?
        .try_into()
        .map_err(|_| InvalidMicrobitPinError)
}

struct Pad(
    Pin<nrf52833_hal::gpio::Input<nrf52833_hal::gpio::PullUp>>,
    Pin<nrf52833_hal::gpio::Input<nrf52833_hal::gpio::PullUp>>,
    Pin<nrf52833_hal::gpio::Input<nrf52833_hal::gpio::PullUp>>,
    Pin<nrf52833_hal::gpio::Input<nrf52833_hal::gpio::PullUp>>,
);

fn xyab() -> Pad {
    Pad(
        unsafe { Pin::<Disconnected>::from_psel_bits(to_hal(8).unwrap()) }.into_pullup_input(),
        unsafe { Pin::<Disconnected>::from_psel_bits(to_hal(1).unwrap()) }.into_pullup_input(),
        unsafe { Pin::<Disconnected>::from_psel_bits(to_hal(5).unwrap()) }.into_pullup_input(),
        unsafe { Pin::<Disconnected>::from_psel_bits(to_hal(4).unwrap()) }.into_pullup_input(),
    )
}

fn directions() -> Pad {
    Pad(
        unsafe { Pin::<Disconnected>::from_psel_bits(to_hal(7).unwrap()) }.into_pullup_input(),
        unsafe { Pin::<Disconnected>::from_psel_bits(to_hal(6).unwrap()) }.into_pullup_input(),
        unsafe { Pin::<Disconnected>::from_psel_bits(to_hal(0).unwrap()) }.into_pullup_input(),
        unsafe { Pin::<Disconnected>::from_psel_bits(to_hal(3).unwrap()) }.into_pullup_input(),
    )
}
