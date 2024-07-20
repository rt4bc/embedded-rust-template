#![no_std]
#![no_main]

use panic_halt as _;
use cortex_m_rt::entry;
use stm32f4xx_hal as hal;

#[entry]
fn main() -> ! {
    // 初始化外设,编写您的嵌入式代码
    loop {
        // 主循环
    }
}