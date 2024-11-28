#![no_main]
#![no_std]

use core::f32::consts::TAU;

use cortex_m_rt::entry;
use embedded_graphics::{
    pixelcolor::BinaryColor,
    prelude::Point,
    primitives::{Circle, Line, PrimitiveStyle, StyledDrawable},
};
use embedded_hal::spi::MODE_0;
use libm::sincosf;
use micromath::F32;
use nrf52833_hal::{
    gpio::{Disconnected, Level, Pin},
    pac::Peripherals,
    spi::{Frequency, Pins, Spi},
    Timer,
};
use panic_halt as _;
use sh1106::{mode::GraphicsMode, Builder};

#[entry]
fn main() -> ! {
    let peripherals = Peripherals::take().unwrap();
    let mut timer = Timer::new(peripherals.TIMER0);

    // OLED
    let mut target: GraphicsMode<_> = {
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
    let _reset_pin: Pin<nrf52833_hal::gpio::Output<nrf52833_hal::gpio::PushPull>> =
        unsafe { Pin::<Disconnected>::from_psel_bits(to_hal(12).unwrap()) }
            .into_push_pull_output(Level::High);
    target.init().unwrap();

    // let PAD(mut B, mut A, mut Y, mut X) = xyab();
    // let Pad(mut right, mut down, mut up, mut left) = directions();

    // let mut lil_human = (64, 32);

    // let rect = RoundedRectangle::new(
    //     Rectangle::new(Point::new(10, 10), Size::new(60, 40)),
    //     CornerRadii::new(Size::new(6, 12)),
    // );
    // rect.translate(Point::new(10, 12)).draw_styled(&stroke_1, &mut oled).unwrap();
    // Line::new(Point::new(16, 24), Point::new(51, 34))
    //     .into_styled(stroke_1)
    //     .draw(&mut oled)
    //     .unwrap();
    // let x = BinaryColor::On;

    let thin = PrimitiveStyle::with_stroke(BinaryColor::On, 1);
    let normal = PrimitiveStyle::with_stroke(BinaryColor::On, 2);
    let thick = PrimitiveStyle::with_stroke(BinaryColor::On, 3);

    const SECS_PER_12_HOURS: u64 = 43200;
    let mut epoch: u64 = 1732816980;

    let center = Point::new(63, 31);
    let circle = Circle::with_center(center, 64);
    let sec_hand_step = 60.0; // 60 seconds
    let sec_hand_length = 29.0f32;

    let min_hand_length = 25.0f32;
    let min_hand_step = 60.0 * 60.0; // 3600 seconds

    let hour_hand_length = 15.0f32;
    let hour_hand_step = 12.0 * 60.0 * 60.0; // 3600 seconds

    loop {
        let time = (epoch % SECS_PER_12_HOURS) as u32;

        let sec_hand_angle = time as f32 * (TAU / sec_hand_step);
        let (sec_sin, sec_cos) = sincosf(sec_hand_angle - TAU / 4.0);
        let end_x = sec_cos * sec_hand_length;
        let end_y = sec_sin * sec_hand_length;
        let sec_end = Point::new(end_x as i32, end_y as i32);
        let sec_begining = Point::new((0.85 * end_x) as i32, (0.85 * end_y) as i32);

        let min_hand_angle = time as f32 * (TAU / min_hand_step);
        let (min_sin, min_cos) = sincosf(min_hand_angle - TAU / 4.0);
        let min_hand_delta = Point::new(
            (min_cos * min_hand_length) as i32,
            (min_sin * min_hand_length) as i32,
        );

        let hour_hand_angle = time as f32 * (TAU / hour_hand_step);
        let (hour_sin, hour_cos) = sincosf(hour_hand_angle - TAU / 4.0);
        let hour_hand_delta = Point::new(
            (hour_cos * hour_hand_length) as i32,
            (hour_sin * hour_hand_length) as i32,
        );

        target.clear();
        circle.draw_styled(&normal, &mut target).unwrap();

        Line::new(center + sec_begining, center + sec_end)
            .draw_styled(&thin, &mut target)
            .unwrap();

        Line::with_delta(center, min_hand_delta)
            .draw_styled(&normal, &mut target)
            .unwrap();

        Line::with_delta(center, hour_hand_delta)
            .draw_styled(&thick, &mut target)
            .unwrap();

        timer.delay(1_000_000);
        epoch += 1;

        target.flush().unwrap();

        // oled.set_pixel(lil_human.0, lil_human.1, 0); // erase lil_human
        // if right.is_low().unwrap() && lil_human.0 < 127 {
        //     lil_human.0 += 1;
        // }
        // if left.is_low().unwrap() && lil_human.0 > 0 {
        //     lil_human.0 -= 1;
        // }
        // if down.is_low().unwrap() && lil_human.1 < 63 {
        //     lil_human.1 += 1;
        // }
        // if up.is_low().unwrap() && lil_human.1 > 0 {
        //     lil_human.1 -= 1;
        // }
        // // oled.clear();
        // oled.set_pixel(lil_human.0, lil_human.1, 1); // draw new lil_human
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

// struct Pad(
//     Pin<nrf52833_hal::gpio::Input<nrf52833_hal::gpio::PullUp>>,
//     Pin<nrf52833_hal::gpio::Input<nrf52833_hal::gpio::PullUp>>,
//     Pin<nrf52833_hal::gpio::Input<nrf52833_hal::gpio::PullUp>>,
//     Pin<nrf52833_hal::gpio::Input<nrf52833_hal::gpio::PullUp>>,
// );

// fn xyab() -> Pad {
//     Pad(
//         unsafe { Pin::<Disconnected>::from_psel_bits(to_hal(8).unwrap()) }.into_pullup_input(),
//         unsafe { Pin::<Disconnected>::from_psel_bits(to_hal(1).unwrap()) }.into_pullup_input(),
//         unsafe { Pin::<Disconnected>::from_psel_bits(to_hal(5).unwrap()) }.into_pullup_input(),
//         unsafe { Pin::<Disconnected>::from_psel_bits(to_hal(4).unwrap()) }.into_pullup_input(),
//     )
// }

// fn directions() -> Pad {
//     Pad(
//         unsafe { Pin::<Disconnected>::from_psel_bits(to_hal(7).unwrap()) }.into_pullup_input(),
//         unsafe { Pin::<Disconnected>::from_psel_bits(to_hal(6).unwrap()) }.into_pullup_input(),
//         unsafe { Pin::<Disconnected>::from_psel_bits(to_hal(0).unwrap()) }.into_pullup_input(),
//         unsafe { Pin::<Disconnected>::from_psel_bits(to_hal(3).unwrap()) }.into_pullup_input(),
//     )
// }
