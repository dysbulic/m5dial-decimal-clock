use esp_idf_hal::delay::{Delay, FreeRtos};
use esp_idf_hal::gpio::{self, OutputPin, PinDriver};
use esp_idf_hal::spi::{
    self,
    config::{Config, Mode, Phase, Polarity},
    SpiDeviceDriver,
};
use esp_idf_hal::prelude::*;
use esp_idf_svc::{
    eventloop::EspSystemEventLoop, hal::peripherals::Peripherals, timer::EspTaskTimerService,
};

use embedded_graphics::{
    pixelcolor::Rgb565,
    prelude::{Point, Size},
    primitives::{Circle, Primitive, PrimitiveStyleBuilder, Rectangle},
    Drawable,
};
use gc9a01::{mode::BufferedGraphics, prelude::*, Gc9a01, SPIDisplayInterface};

use std::cmp;

type BoxedDisplayDriver<'a> = Box<
    Gc9a01<
        SPIInterface<
            SpiDeviceDriver<'a, spi::SpiDriver<'a>>,
            PinDriver<'a, gpio::AnyOutputPin, gpio::Output>,
        >,
        DisplayResolution240x240,
        BufferedGraphics<DisplayResolution240x240>,
    >,
>;

#[allow(unused_parens)]
fn draw<I: WriteOnlyDataCommand, D: DisplayDefinition>(
    display: &mut Gc9a01<I, D, BufferedGraphics<D>>,
    tick: u32,
) {
    let (w, h) = display.dimensions();
    let w = w as u32;
    let h = h as u32;
    let x = tick % w;
    let y = tick % h;

    // log::info!("w: {w}, h: {h}");

    let style = PrimitiveStyleBuilder::new()
        .stroke_width(4)
        .stroke_color(Rgb565::new(tick as u8, x as u8, y as u8))
        .fill_color(Rgb565::new(
            rand::random::<u8>(),
            rand::random::<u8>(),
            rand::random::<u8>(),
        ))
        .build();

    let cdiameter: i32 = (
        <u32 as TryInto<i32>>::try_into(cmp::min(w, h)).unwrap() / 10
    );

    Circle::new(
        Point::new(119 - cdiameter / 2 + 40, 119 - cdiameter / 2 + 40),
        cdiameter as u32,
    )
    .into_styled(style)
    .draw(display)
    .unwrap();

    Circle::new(
        Point::new(119 - cdiameter / 2 - 40, 119 - cdiameter / 2 + 40),
        cdiameter as u32,
    )
    .into_styled(style)
    .draw(display)
    .unwrap();

    let rw = 80;
    let rh = 20;
    Rectangle::new(
        Point::new(119 - rw / 2, 119 - rh / 2 - 40),
        Size::new(rw as u32, rh as u32),
    )
    .into_styled(style)
    .draw(display)
    .unwrap();
}

#[allow(unused_parens)]
fn main() {
    // It is necessary to call this function once. Otherwise some patches to the runtime
    // implemented by esp-idf-sys might not link properly.
    // See https://github.com/esp-rs/esp-idf-template/issues/71
    esp_idf_svc::sys::link_patches();

    // Bind the log crate to the ESP Logging facilities
    esp_idf_svc::log::EspLogger::initialize_default();

    let peripherals = Peripherals::take().unwrap();
    let pins = peripherals.pins;
    let _sysloop = EspSystemEventLoop::take().unwrap();
    let _timer_service = EspTaskTimerService::new().unwrap();
    let mut delay = Delay::new_default();

    let sck = pins.gpio6;
    let mosi = pins.gpio5;
    let cs = pins.gpio7;
    let dc = pins.gpio4;
    let reset = pins.gpio8;
    let backlight = pins.gpio9;

    let cs_output = cs;
    let dc_output = (
        PinDriver::output(dc.downgrade_output()).unwrap()
    );
    let mut backlight_output = (
        PinDriver::output(backlight.downgrade_output()).unwrap()
    );
    let mut reset_output = (
        PinDriver::output(reset.downgrade_output()).unwrap()
    );

    backlight_output.set_high().unwrap();

    let driver = spi::SpiDriver::new(
        peripherals.spi2,
        sck,
        mosi,
        None::<gpio::AnyIOPin>,
        &spi::SpiDriverConfig::new(),
    )
    .unwrap();

    let config = Config::new().baudrate(2.MHz().into()).data_mode(Mode {
        polarity: Polarity::IdleLow,
        phase: Phase::CaptureOnFirstTransition,
    });

    let spi_device = (
        SpiDeviceDriver::new(driver, Some(cs_output), &config).unwrap()
    );

    let interface = (
        SPIDisplayInterface::new(spi_device, dc_output)
    );

    let mut display_driver: BoxedDisplayDriver = Box::new(
        Gc9a01::new(
            interface,
            DisplayResolution240x240,
            DisplayRotation::Rotate0,
        )
        .into_buffered_graphics(),
    );

    display_driver.reset(&mut reset_output, &mut delay).ok();
    display_driver.init(&mut delay).ok();

    let mut tick: u32 = 0;
    loop {
        display_driver.clear();
        draw(&mut display_driver, tick);
        display_driver.flush().ok();
        tick += 1;
        FreeRtos::delay_ms(2000);
    }
}
