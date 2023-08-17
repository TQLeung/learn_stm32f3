#![no_main]
#![no_std]
pub use panic_itm; // panic handler
pub use cortex_m_rt::entry;
#[allow(unused_imports)]
use stm32f3_discovery::stm32f3xx_hal::prelude::*;
#[entry]
fn main() -> ! {
    let (mut itm, _) = aux7::init();

    


    loop {}
}