#![no_std]
#![no_main]

mod robot;

use defmt::*;
use defmt_rtt as _;
use embedded_graphics::Drawable;
use embedded_graphics::geometry::Point;
use embedded_graphics::mono_font::MonoTextStyleBuilder;
use embedded_graphics::mono_font::ascii::FONT_6X10;
use embedded_graphics::pixelcolor::BinaryColor;
use embedded_graphics::text::Text;
use embedded_hal::digital::OutputPin;
use hal::fugit::RateExtU32;
use hal::uart::DataBits;
use hal::uart::StopBits;
use heapless::format;
use panic_probe as _;
use rp235x_hal::binary_info::DataType::BinaryInfoListZeroTerminated;
use rp235x_hal::clocks::init_clocks_and_plls;
use rp235x_hal::fugit::HertzU32;
use rp235x_hal::gpio::FunctionI2c;
use rp235x_hal::gpio::Pin;
use rp235x_hal::gpio::PullUp;
use rp235x_hal::uart::{UartConfig, UartPeripheral};
use rp235x_hal::{self as hal, entry};
use rp235x_hal::{Clock, pac};
use ssd1306::I2CDisplayInterface;
use ssd1306::Ssd1306;
use ssd1306::mode::DisplayConfig;
use ssd1306::rotation::DisplayRotation;
use ssd1306::size::DisplaySize128x64;

use crate::robot::expressions::Expression;
use crate::robot::tox::Tox;

#[unsafe(link_section = ".start_block")]
#[used]
pub static IMAGE_DEF: hal::block::ImageDef = hal::block::ImageDef::secure_exe();

#[entry]
fn main() -> ! {
    let mut pac = pac::Peripherals::take().unwrap();
    let mut watchdog = hal::Watchdog::new(pac.WATCHDOG);
    let sio = hal::Sio::new(pac.SIO);

    let clocks = init_clocks_and_plls(
        12_000_000,
        pac.XOSC,
        pac.CLOCKS,
        pac.PLL_SYS,
        pac.PLL_USB,
        &mut pac.RESETS,
        &mut watchdog,
    )
    .ok()
    .unwrap();

    let pins = hal::gpio::Pins::new(
        pac.IO_BANK0,
        pac.PADS_BANK0,
        sio.gpio_bank0,
        &mut pac.RESETS,
    );

    let sda_pin = pins.gpio18.into_function::<FunctionI2c>().into_pull_type();
    let scl_pin = pins.gpio19.into_function::<FunctionI2c>().into_pull_type();

    let i2c = hal::I2C::i2c1(
        pac.I2C1,
        sda_pin,
        scl_pin,
        400.kHz(),
        &mut pac.RESETS,
        &clocks.system_clock,
    );

    let interface = I2CDisplayInterface::new(i2c);
    let mut display = Ssd1306::new(interface, DisplaySize128x64, DisplayRotation::Rotate0)
        .into_buffered_graphics_mode();

    display.init().unwrap();

    let mut tox = Tox::new(display);
    tox.set_expression(Expression::Idle);

    loop {
        tox.update_display();

        cortex_m::asm::wfi();
    }
}
