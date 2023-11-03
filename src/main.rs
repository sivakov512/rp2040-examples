#![no_main]
#![no_std]

use rp_pico as bsp;

use bsp::entry;
use defmt_rtt as _;
use panic_probe as _;

#[entry]
fn main() -> ! {
    loop {}
}
