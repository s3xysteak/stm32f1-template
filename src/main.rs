#![no_main]
#![no_std]

use cortex_m_rt::entry;
use panic_rtt_target as _;
use rtt_target::rtt_init_print;

mod examples;

#[entry]
fn main() -> ! {
    rtt_init_print!();
    examples::digital_tube::digital_tube();
}
