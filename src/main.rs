#![no_main]
#![no_std]


use cortex_m;
use cortex_m_rt::entry;
use stm32f1xx_hal::{pac, prelude};
use panic_halt as _;


#[entry]
fn main() -> ! {
    // Get access to the core peripherals from the cortex-m crate
    let cp = cortex_m::Peripherals::take().unwrap();
    // Get access to the device specific peripherals from the peripheral access crate
    let dp = stm32f1xx_hal::pac::Peripherals::take().unwrap();
    loop {}
}
