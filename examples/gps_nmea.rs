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
    gpio::bank0::{Gpio0, Gpio1, Gpio4, Gpio5},
    pac,
    sio::Sio,
    uart::{DataBits, StopBits, UartConfig, UartPeripheral},
    watchdog::Watchdog,
};
use core::fmt::Write;
use defmt::info;
use embedded_hal::serial::Read;

type GPSUartPins = (
    hal::gpio::Pin<Gpio4, hal::gpio::FunctionUart, hal::gpio::PullNone>,
    hal::gpio::Pin<Gpio5, hal::gpio::FunctionUart, hal::gpio::PullNone>,
);

type DebugUartPins = (
    hal::gpio::Pin<Gpio0, hal::gpio::FunctionUart, hal::gpio::PullNone>,
    hal::gpio::Pin<Gpio1, hal::gpio::FunctionUart, hal::gpio::PullNone>,
);

const NMEA_LENGTH: usize = 164;
const START_BYTE: u8 = 36;

#[entry]
fn main() -> ! {
    info!("Running gps_nmea program");

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

    let mut gps_uart = UartPeripheral::new(
        pac.UART1,
        (pins.gpio4.reconfigure(), pins.gpio5.reconfigure()) as GPSUartPins,
        &mut pac.RESETS,
    )
    .enable(
        UartConfig::new(9_600.Hz(), DataBits::Eight, None, StopBits::One),
        clocks.peripheral_clock.freq(),
    )
    .unwrap();

    let mut debug_uart = UartPeripheral::new(
        pac.UART0,
        (pins.gpio0.reconfigure(), pins.gpio1.reconfigure()) as DebugUartPins,
        &mut pac.RESETS,
    )
    .enable(
        UartConfig::new(9_600.Hz(), DataBits::Eight, None, StopBits::One),
        clocks.peripheral_clock.freq(),
    )
    .unwrap();

    let mut buffer: [u8; NMEA_LENGTH] = [0; NMEA_LENGTH];
    let mut pointer = 0;

    let mut parser = nmea::Nmea::default();

    loop {
        while let Ok(byte) = gps_uart.read() {
            if pointer == 0 && byte != START_BYTE {
                continue;
            }

            if pointer >= NMEA_LENGTH {
                pointer = 0;
                continue;
            }

            if byte == START_BYTE {
                if pointer != 0 {
                    if let Ok(ascii) = core::str::from_utf8(&buffer[..pointer]) {
                        write!(debug_uart, "{}", ascii).unwrap();

                        if let Ok(_) = parser.parse(ascii) {
                            write!(debug_uart, "{:?}", parser).unwrap();
                        }

                        write!(debug_uart, "\n").unwrap();
                    }
                }

                pointer = 0
            }

            buffer[pointer] = byte;
            pointer += 1;
        }
    }
}
