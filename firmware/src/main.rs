#![no_std]
#![no_main]

use defmt::*;
use defmt_rtt as _;
use embedded_hal::digital::OutputPin;
use hal::fugit::RateExtU32;
use hal::uart::DataBits;
use hal::uart::StopBits;
use heapless::format;
use panic_probe as _;
use rp235x_hal::clocks::init_clocks_and_plls;
use rp235x_hal::fugit::HertzU32;
use rp235x_hal::uart::{UartConfig, UartPeripheral};
use rp235x_hal::{self as hal, entry};
use rp235x_hal::{Clock, pac};

#[unsafe(link_section = ".start_block")]
#[used]
pub static IMAGE_DEF: hal::block::ImageDef = hal::block::ImageDef::secure_exe();

#[entry]
fn main() -> ! {
    info!("Program start");
    let mut pac = pac::Peripherals::take().unwrap();
    let core = cortex_m::Peripherals::take().unwrap();
    let mut watchdog = hal::Watchdog::new(pac.WATCHDOG);
    let sio = hal::Sio::new(pac.SIO);

    let external_xtal_freq_hz = 12_000_000u32;
    let clocks = init_clocks_and_plls(
        external_xtal_freq_hz,
        pac.XOSC,
        pac.CLOCKS,
        pac.PLL_SYS,
        pac.PLL_USB,
        &mut pac.RESETS,
        &mut watchdog,
    )
    .ok()
    .unwrap();

    let mut delay = cortex_m::delay::Delay::new(core.SYST, clocks.system_clock.freq().to_Hz());

    let pins = hal::gpio::Pins::new(
        pac.IO_BANK0,
        pac.PADS_BANK0,
        sio.gpio_bank0,
        &mut pac.RESETS,
    );

    let tx = pins.gpio16.into_function();
    let rx = pins.gpio17.into_function();

    let uart = UartPeripheral::new(pac.UART0, (tx, rx), &mut pac.RESETS)
        .enable(
            UartConfig::new(9600.Hz(), DataBits::Eight, None, StopBits::One),
            clocks.peripheral_clock.freq(),
        )
        .unwrap();

    loop {
        info!("Sending servo 1 angle 0");
        uart.write_full_blocking(servo_cmd(1, 0, 500).as_bytes());
        delay.delay_ms(800);
        info!("Sending servo 1 angle 90");
        uart.write_full_blocking(servo_cmd(1, 180, 500).as_bytes());
        delay.delay_ms(800);
    }
}

fn angle_to_pulse(angle: u32) -> u32 {
    let min = 500;
    let max = 2500;

    min + (angle * (max - min) / 180)
}

fn servo_cmd(servo: u8, angle: u32, time: u16) -> heapless::String<20> {
    let pulse = angle_to_pulse(angle);

    format!("#{}P{}T{}\r\n", servo, pulse, time).unwrap()
}
