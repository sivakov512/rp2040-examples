#![no_main]
#![no_std]

use rp_pico as bsp;

use bsp::entry;
use defmt_rtt as _;
use panic_probe as _;

use bsp::hal::{
    clocks::{init_clocks_and_plls, Clock},
    pac,
    sio::Sio,
    watchdog::Watchdog,
};
use defmt::info;
use embedded_hal::digital::v2::OutputPin;

const DELAY_MS: u32 = 2_000;

#[entry]
fn main() -> ! {
    info!("Running program");

    let mut pac = pac::Peripherals::take().unwrap();
    let core = pac::CorePeripherals::take().unwrap();
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

    let mut delay = cortex_m::delay::Delay::new(core.SYST, clocks.system_clock.freq().to_Hz());

    let pins = bsp::Pins::new(
        pac.IO_BANK0,
        pac.PADS_BANK0,
        sio.gpio_bank0,
        &mut pac.RESETS,
    );

    let mut pin = pins.gpio15.into_push_pull_output();

    let mut state = false;
    loop {
        state = !state;

        info!("Turn pin power on: {:?}", state);
        pin.set_state(state.into()).unwrap();

        info!("Sleep for {:?} ms", DELAY_MS);
        delay.delay_ms(DELAY_MS);
    }
}
