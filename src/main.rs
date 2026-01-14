#![no_std]
#![no_main]

use panic_halt as _;

#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);
    arduino_console::init_console(dp.USART0, pins.d0, pins.d1.into_output());
    let mut led = pins.d13.into_output_high();
    loop {
        arduino_console::println!("hello world");
        led.toggle().then_delay(500);
    }
}

pub trait Delay {
    fn then_delay(&self, ms: u32) {
        arduino_hal::delay_ms(ms);
    } 
}

impl Delay for () {}