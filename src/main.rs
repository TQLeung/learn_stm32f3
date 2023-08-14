#![no_main]
#![no_std]
pub use panic_itm; // panic handler
pub use cortex_m_rt::entry;
#[allow(unused_imports)]
use stm32f3_discovery::stm32f3xx_hal::prelude::*;
#[entry]
fn main() -> ! {
    const GPIOE_BSRR: u32 = 0x48001018;
    const GPIOE_ODR: u32 = 0x48001014;
    const GPIOE_RCC: u32 = 0x40021014;
    const GPIOE_MODER: u32 = 0x48001000;
    unsafe{
        *(GPIOE_RCC as *mut u32) = 0x0020_0000;
        *(GPIOE_MODER as *mut u32) = 0x5555_0000;
        // *(GPIOE_ODR as *mut u16) = 0xffff;
        *(GPIOE_BSRR as *mut u32) = 0xffff_ffff;
    }
    loop {}
}