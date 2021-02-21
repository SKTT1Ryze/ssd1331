//! Draw a square, circle and triangle on the screen using the embedded_graphics library over a 4
//! wire SPI interface.
//!
//! This example is for the STM32F103 "Blue Pill" board using a 4 wire interface to the display on
//! SPI1.
//!
//! Wiring connections are as follows
//!
//! ```
//! GND -> GND
//! 3V3 -> VCC
//! PA5 -> SCL
//! PA7 -> SDA
//! PB0 -> RST
//! PB1 -> D/C
//! ```
//!
//! Run on a Blue Pill with `cargo run --example graphics`.

#![no_std]
#![no_main]

use cortex_m_rt::{entry, exception, ExceptionFrame};
use embedded_graphics::{
    geometry::Point,
    pixelcolor::Rgb565,
    prelude::*,
    primitives::{Circle, Rectangle, Triangle, PrimitiveStyleBuilder},
};
use panic_semihosting as _;
use ssd1331::{DisplayRotation::Rotate0, Ssd1331};
use stm32f1xx_hal::{
    delay::Delay,
    prelude::*,
    spi::{Mode, Phase, Polarity, Spi},
    stm32,
};

#[entry]
fn main() -> ! {
    let cp = cortex_m::Peripherals::take().unwrap();
    let dp = stm32::Peripherals::take().unwrap();

    let mut flash = dp.FLASH.constrain();
    let mut rcc = dp.RCC.constrain();

    let clocks = rcc.cfgr.freeze(&mut flash.acr);

    let mut afio = dp.AFIO.constrain(&mut rcc.apb2);

    let mut gpioa = dp.GPIOA.split(&mut rcc.apb2);
    let mut gpiob = dp.GPIOB.split(&mut rcc.apb2);

    // SPI1
    let sck = gpioa.pa5.into_alternate_push_pull(&mut gpioa.crl);
    let miso = gpioa.pa6;
    let mosi = gpioa.pa7.into_alternate_push_pull(&mut gpioa.crl);

    let mut delay = Delay::new(cp.SYST, clocks);

    let mut rst = gpiob.pb0.into_push_pull_output(&mut gpiob.crl);
    let dc = gpiob.pb1.into_push_pull_output(&mut gpiob.crl);

    let spi = Spi::spi1(
        dp.SPI1,
        (sck, miso, mosi),
        &mut afio.mapr,
        Mode {
            polarity: Polarity::IdleLow,
            phase: Phase::CaptureOnFirstTransition,
        },
        8.mhz(),
        clocks,
        &mut rcc.apb2,
    );

    let mut disp = Ssd1331::new(spi, dc, Rotate0);

    disp.reset(&mut rst, &mut delay).unwrap();
    disp.init().unwrap();
    disp.flush().unwrap();

    Triangle::new(
        Point::new(8, 16 + 16),
        Point::new(8 + 16, 16 + 16),
        Point::new(8 + 8, 16),
    )
    .into_styled(
        PrimitiveStyleBuilder::new()
            .stroke_color(Rgb565::RED)
            .stroke_width(1)
            .build(),
    )
    .draw(&mut disp)
    .unwrap();

    Rectangle::new(Point::new(36, 16), Size::new(36 + 16, 16 + 16))
        .into_styled(
            PrimitiveStyleBuilder::new()
                .stroke_color(Rgb565::GREEN)
                .stroke_width(1)
                .build(),
        )
        .draw(&mut disp)
        .unwrap();

    Circle::new(Point::new(72, 16 + 8), 8)
        .into_styled(
            PrimitiveStyleBuilder::new()
                .stroke_color(Rgb565::BLUE)
                .stroke_width(1)
                .build(),
        )
        .draw(&mut disp)
        .unwrap();

    disp.flush().unwrap();

    loop {}
}

#[exception]
fn HardFault(ef: &ExceptionFrame) -> ! {
    panic!("{:#?}", ef);
}
