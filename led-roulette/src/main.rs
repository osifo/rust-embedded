#![deny(unsafe_code)]
#![no_std]
#![no_main]

use cortex_m_rt::entry;
use panic_halt as _;
use microbit as _;

#[entry]
fn main() -> ! { // the exclamation return type indicates that this method does not return (it  runs infinitely)
    let _y;
    let x = 42;
    _y = x;

    loop {} //infinite loop to keep the function running
}
