#![no_main]
#![no_std]

use cortex_m_rt::entry;
use panic_halt as _;

use stm32f4xx_hal::{pac, prelude::*};

#[entry]
fn main() -> ! {

    // 获取对外设的访问权限
    //将 PAC（Peripheral Access Crate）的变量命名为 dp 是一种常见的惯例
    //dp 代表 "Device Peripherals"（设备外设）。
    let dp = pac::Peripherals::take().unwrap();

    // Cortex-M 处理器中，通常还有 cp（Core Peripherals），用于访问核心特定的外设。使用 dp 可以清楚地与 cp 区分开。
    // 配置系统时钟
    let rcc = dp.RCC.constrain();
    let clocks = rcc.cfgr.use_hse(8.MHz()).sysclk(180.MHz()).freeze();

    // 配置 GPIO
    let gpiok = dp.GPIOK.split();
    let mut green_led = gpiok.pk5.into_push_pull_output();

    loop {
        green_led.toggle();
    }
}
