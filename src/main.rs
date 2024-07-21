#![no_main]
#![no_std]

use cortex_m::peripheral::{self, syst, Peripherals};
use cortex_m_rt::entry;
use panic_halt as _;

use stm32f4xx_hal::{pac, prelude::*};

#[entry]
fn main() -> ! {

    let peripheral = Peripherals::take().unwrap();
    let mut systick = peripheral.SYST;

    systick.set_clock_source(syst::SystClkSource::Core);
    systick.set_reload(1_000);
    systick.clear_current();
    systick.enable_counter();

    while !systick.has_wrapped(){
        
    }
    loop {
    }
}
