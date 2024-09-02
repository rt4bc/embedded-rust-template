#![no_main]
#![no_std]

mod scratch;

use core::iter::Scan;

use cortex_m::{iprintln};
use cortex_m_rt::entry;

use panic_halt as _;

use stm32f4xx_hal::{pac, prelude::*};

#[entry]
fn main() -> ! {
    {
        scratch::asm_test();

        scratch::fugit_test();
    }

    // 获取对外设的访问权限
    //将 PAC（Peripheral Access Crate）的变量命名为 dp 是一种常见的惯例
    //dp 代表 "Device Peripherals"（设备外设）。
    // Cortex-M 处理器中，通常还有 cp（Core Peripherals），用于访问核心特定的外设。
    //使用 dp 可以清楚地与 cp 区分开。
    // 配置系统时钟
    let dp = pac::Peripherals::take().unwrap();
    let rcc = dp.RCC.constrain();
    let clocks = rcc.cfgr.use_hse(8.MHz()).sysclk(180.MHz()).freeze();

    // cp 是cortex_m peripherals
    // 在launch里配置了
    // "swoConfig": {
    //     "enabled": true,
    //     "cpuFrequency": 180000000,
    //     "swoFrequency": 2000000,
    //     "source": "probe",
    //     "decoders": [
    //       {
    //         "port": 0,
    //         "type": "console",
    //         "label": "ITM"
    //       }
    //     ]
    //   }
    let mut cp = cortex_m::Peripherals::take().unwrap();
    let stim0 = &mut cp.ITM.stim[0];
    iprintln!(stim0, "Hello, ITM!");

    // 配置 GPIO
    let gpioa = dp.GPIOA.split();
    let mut green_led = gpioa.pa5.into_push_pull_output();
    
    let mut _i = 0;

    let mut delay = cp.SYST.delay(&clocks);
    loop {
        delay.delay_ms(1000);
        green_led.toggle();
        _i +=1;
        iprintln!(stim0, "i = {}.", _i);
    }
}
