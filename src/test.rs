pub mod asm_test {
    use core::arch::asm;
    use core::option;
    use core::sync::atomic::{compiler_fence, Ordering};

    pub fn asm_test_1() -> u32 {
        let a: u32;
        unsafe {
            asm!("mov {}, #10",
                out(reg) a)
        }
        a
    }
}

use cortex_m::iprint;
use cortex_m::iprintln;
use cortex_m::peripheral::Peripherals;

pub fn itm_test() {
    let mut peripherals = Peripherals::take().unwrap();
    let stim0 = &mut peripherals.ITM.stim[0];
    iprintln!(stim0, "Hello, ITM!");
    for i in 1..10{
        iprintln!(stim0, "i = {}",i);
    }
}
