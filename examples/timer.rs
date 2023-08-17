#![no_main]
#![no_std]
pub use panic_itm; // panic handler
pub use cortex_m_rt::entry;
#[allow(unused_imports)]
use stm32f3_discovery::stm32f3xx_hal::prelude::*;
use cortex_m::{asm::bkpt, iprint, iprintln, peripheral, peripheral::ITM};



#[entry]
fn main() -> ! {
    let (mut itm, _) = init();
    let mut sitm0 = &mut itm.stim[0];

    const TIM6_CR1:u32 = 0x4000_1000;
    const TIM6_CNT:u32 = 0x4000_1024;
    const TIM6_PSC:u32 = 0x4000_1028;
    const TIM6_ARR:u32 = 0x4000_102C;
    
    unsafe {
        iprintln!(&mut sitm0, "\n\nCR1:{:08b}", *(TIM6_CR1 as *const u32));
        iprintln!(&mut sitm0, "CNT:{}", *(TIM6_CNT as *const u32));
        iprintln!(&mut sitm0, "PSC:{}", *(TIM6_PSC as *const u32));
        iprintln!(&mut sitm0, "ARR:{}", *(TIM6_ARR as *const u32));
        iprintln!(&mut sitm0, "Hello world!");
        iprintln!(&mut sitm0, "Hello world!");
        *(TIM6_CR1 as *mut u16) = 0x0001;      //开启TIM6 CEN
        *(TIM6_PSC as *mut u16) = 72u16-1;     //设置分频器  STM32F303VCT6  72HMz
        *(TIM6_ARR as *mut u16) = 1000u16-1;   //自动重装载寄存器
        iprintln!(&mut sitm0, "CR1:{}", *(TIM6_CR1 as *const u32));
        iprintln!(&mut sitm0, "CNT:{}", *(TIM6_CNT as *const u32));
        iprintln!(&mut sitm0, "PSC:{}", *(TIM6_PSC as *const u32));
        iprintln!(&mut sitm0, "ARR:{}", *(TIM6_ARR as *const u32));
    }
    bkpt();
    iprintln!(&mut sitm0, "xxxx");
    // bkpt();
    iprintln!(&mut sitm0, "Hello world!");
    loop {
    }
}

fn init() -> (ITM, i32) {
    let p = cortex_m::Peripherals::take().unwrap();
    (p.ITM, 0)
}


#[inline(never)]
fn delay(ms: u16) {
}