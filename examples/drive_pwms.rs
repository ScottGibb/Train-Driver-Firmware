#![no_std]
#![no_main]


use cortex_m_rt::entry;
use panic_probe as _;


#[entry]
fn main() -> ! {
    loop {
        panic!("This is a panic message!");
    }
}
