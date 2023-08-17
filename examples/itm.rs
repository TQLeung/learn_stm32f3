#![no_main]
#![no_std]
pub use panic_itm; // panic handler
pub use cortex_m_rt::entry;
#[allow(unused_imports)]
use stm32f3_discovery::stm32f3xx_hal::prelude::*;
use cortex_m::{asm::bkpt, iprint, iprintln, peripheral::ITM};
#[entry]
fn main() -> ! {
    let mut p = cortex_m::Peripherals::take().unwrap();
    let mut sitm0 = &mut p.ITM.stim[0];
    iprintln!(&mut sitm0, "Hello world!");
    bkpt();
    loop {}
}
