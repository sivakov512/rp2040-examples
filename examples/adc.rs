#![no_main]
#![no_std]

use rp_pico as bsp;

use bsp::entry;
use defmt_rtt as _;
use panic_probe as _;

use bsp::hal::{
    clocks::{init_clocks_and_plls, Clock},
    watchdog::Watchdog,
    adc::{Adc, AdcPin},
    pac,
    sio::Sio,
};
use defmt::info;
use embedded_hal::adc::OneShot;

const SAMPLES_COUNT: u8 = 100;

#[entry]
fn main() -> ! {
    info!("Running adc program");

    let mut pac = pac::Peripherals::take().unwrap();
    let core = pac::CorePeripherals::take().unwrap();
    let sio = Sio::new(pac.SIO);

    let mut watchdog = Watchdog::new(pac.WATCHDOG);
    let mut adc = Adc::new(pac.ADC, &mut pac.RESETS);

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

    let mut tilt_sensor = AdcPin::new(pins.gpio26.into_floating_input());

    let mut sample: u16 = 0;
    let mut samples_count = 0;
    loop {
        while samples_count < SAMPLES_COUNT {
            let value: u16 = adc.read(&mut tilt_sensor).unwrap();
            if value > sample {
                sample = value;
            }

            samples_count += 1;
            delay.delay_ms(10);
        }
        info!("Got {}", sample);

        sample = 0;
        samples_count = 0;
    }
}
