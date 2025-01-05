use anyhow::{anyhow, Result};
use display_interface_spi::SPIInterface;
use embedded_graphics::{draw_target::DrawTarget, pixelcolor::Rgb565, prelude::RgbColor};
use esp_idf_hal::{
    delay::Ets,
    gpio::{AnyIOPin, PinDriver},
    peripherals::Peripherals,
    spi::{
        config::{Config, MODE_3},
        SpiDeviceDriver, SpiDriverConfig,
    },
    units::FromValueType,
};
use mipidsi::{models::ST7789, options::ColorInversion, Builder};
use std::{thread, time::Duration};

fn main() -> Result<()> {
    // Required call, see https://github.com/esp-rs/esp-idf-template/issues/71
    esp_idf_svc::sys::link_patches();

    // Bind the log crate to the ESP Logging facilities.
    esp_idf_svc::log::EspLogger::initialize_default();

    log::info!("Hello, Lily!");

    let peripherals = Peripherals::take()?;

    let data_command = PinDriver::output(peripherals.pins.gpio16)?;
    let reset = PinDriver::output(peripherals.pins.gpio23)?;
    let mut backlight = PinDriver::output(peripherals.pins.gpio4)?;

    let mut delay = Ets;

    let serial_clock = peripherals.pins.gpio18;
    let serial_data_out = peripherals.pins.gpio19;
    let chip_select = peripherals.pins.gpio5;

    let spi_driver = SpiDeviceDriver::new_single(
        peripherals.spi2,
        serial_clock,
        serial_data_out,
        None::<AnyIOPin>,
        Some(chip_select),
        &SpiDriverConfig::new(),
        &Config::new().baudrate(26.MHz().into()).data_mode(MODE_3),
    )?;

    let display_interface = SPIInterface::new(spi_driver, data_command);

    let mut display = Builder::new(ST7789, display_interface)
        .display_size(135, 240)
        .display_offset(52, 40)
        .invert_colors(ColorInversion::Inverted)
        .reset_pin(reset)
        .init(&mut delay)
        .map_err(|e| anyhow!("failed to initialize display ({e:?})"))?;

    // Turn on the backlight.
    backlight.set_high()?;

    // Hello, world! The display state will show if it works, all pixels should be red.
    display
        .clear(Rgb565::RED)
        .map_err(|e| anyhow!("failed to clear display ({e:?})"))?;

    log::info!("Done!");

    loop {
        thread::sleep(Duration::from_millis(1000));
    }
}
