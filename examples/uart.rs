#![no_main]
#![no_std]

use rp_pico as bsp;

use bsp::entry;
use defmt_rtt as _;
use fugit::RateExtU32;
use panic_probe as _;

use bsp::hal::{
    self,
    clocks::{init_clocks_and_plls, Clock},
    gpio::bank0::{Gpio0, Gpio1},
    pac,
    sio::Sio,
    uart::{DataBits, StopBits, UartConfig, UartPeripheral},
    watchdog::Watchdog,
};
use core::fmt::Write;
use core::str::from_utf8;
use defmt::info;
use embedded_hal::serial::Read;

type UartPins = (
    hal::gpio::Pin<Gpio0, hal::gpio::FunctionUart, hal::gpio::PullNone>,
    hal::gpio::Pin<Gpio1, hal::gpio::FunctionUart, hal::gpio::PullNone>,
);

const BUFFER_SIZE: usize = 32;

#[entry]
fn main() -> ! {
    info!("Running uart program");

    let mut pac = pac::Peripherals::take().unwrap();
    let mut watchdog = Watchdog::new(pac.WATCHDOG);
    let sio = Sio::new(pac.SIO);

    let clocks = init_clocks_and_plls(
        bsp::XOSC_CRYSTAL_FREQ,
        pac.XOSC,
        pac.CLOCKS,
        pac.PLL_SYS,
        pac.PLL_USB,
        &mut pac.RESETS,
        &mut watchdog,
    )
    .ok()
    .unwrap();

    let pins = bsp::Pins::new(
        pac.IO_BANK0,
        pac.PADS_BANK0,
        sio.gpio_bank0,
        &mut pac.RESETS,
    );

    let uart_pins: UartPins = (pins.gpio0.reconfigure(), pins.gpio1.reconfigure());
    let mut uart = UartPeripheral::new(pac.UART0, uart_pins, &mut pac.RESETS)
        .enable(
            UartConfig::new(115_200.Hz(), DataBits::Eight, None, StopBits::One),
            clocks.peripheral_clock.freq(),
        )
        .unwrap();

    let mut buffer: [u8; BUFFER_SIZE] = [0; 32];
    let mut buffer_position = 0;

    loop {
        while let Ok(byte) = uart.read() {
            if buffer_position == BUFFER_SIZE {
                info!("Buffer is full, flushing");
                write!(uart, "{}", from_utf8(&buffer).unwrap()).unwrap();
                buffer_position = 0
            }

            buffer[buffer_position] = byte;
            buffer_position += 1;
        }
    }
}
