#![no_main]
#![no_std]

#[allow(unused_imports)]
use aux7::{entry, iprint, iprintln};

#[entry]
fn main() -> ! {
    x();
    loop {}
}

fn x(){

    let (mut itm, gpioe) = aux7::init();
    let mut i = &mut itm.stim[0]; 
    const GPIOE_BSRR: u32 = 0x48001018;
    const GPIOE_ODR: u32 = 0x48001014;
    const GPIOE_RCC: u32 = 0x40021014;
    unsafe {
        // *(GPIOE_BSRR as *mut u32) = 1 << 8;
        *(GPIOE_RCC as *mut u32) = 0x00c0_0000;
        *(GPIOE_ODR as *mut u16) = 0xffff;
        *(GPIOE_ODR as *mut u16) = 0xffff;
    }
    // Turn on the North LED+
    gpioe.bsrr.write(|w| w.bs9().set_bit());
    iprintln!(&mut i, "North");
    // Turn on the East LED
    gpioe.bsrr.write(|w| w.bs11().set_bit());
    iprintln!(&mut i, "East");
    
    // Turn off the North LED
    gpioe.bsrr.write(|w| w.br9().set_bit());

    // Turn off the East LED
    gpioe.bsrr.write(|w| w.br11().set_bit());
}

#[allow(dead_code)]
fn m(){
    let _e = aux7::init().1;
    unsafe {
        // A magic address!
        const GPIOE_BSRR: u32 = 0x48001018;

        // Turn on the "North" LED (red)
        *(GPIOE_BSRR as *mut u32) = 1 << 9;

        // Turn on the "East" LED (green)
        *(GPIOE_BSRR as *mut u32) = 1 << 11;

        // Turn off the "North" LED
        *(GPIOE_BSRR as *mut u32) = 1 << (9 + 16);

        // Turn off the "East" LED
        *(GPIOE_BSRR as *mut u32) = 1 << (11 + 16);
    }
}

