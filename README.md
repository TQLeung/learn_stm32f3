# README.md

## RUN THE APPLICATION
1. run sh/itmdump.bat
2. run sh/connect board.bat
3. cargo run (--example led)?
    1. .cargo/config.toml assign the runner && target
    2. gdb -e openocd.gdb assign the GDB instructions.
    3. target:thumbv7em-none-eabihf
4. giving the GDB's directions...


## EXAMPLES
### board_pre_setting
#### minimal crates (Cargo.toml)
+ cortex-m-rt = "0.6.14"
+ stm32f3-discovery = "0.7.0"
+ panic-itm = "0.4.2"

``` 
// ==== main.rs codes ====
#![no_main]
#![no_std]
pub use panic_itm; // panic handler
pub use cortex_m_rt::entry;
#[allow(unused_imports)]
use stm32f3_discovery::stm32f3xx_hal::prelude::*;

#[entry]
fn main() -> ! {
    loop {}
}
```
cargo build --example board_pre_setting