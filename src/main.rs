#![no_std]
#![no_main]

use panic_halt as _;

#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);
    let mut led = pins.d9.into_output_high();
    loop {
        led.toggle();
        arduino_hal::delay_ms(500);
    }
}

pub trait Delay {
    fn then_delay(&self, ms: u32) {
        arduino_hal::delay_ms(ms);
    } 
}

impl Delay for () {}

#[allow(unused)]
mod console {
    use arduino_hal::{hal::port::{PD0, PD1}, pac::usart0::RegisterBlock, port::{Pin, mode::{Floating, Input, Output}}};
    use avr_device::{generic::Periph, interrupt};
    use core::cell::RefCell;

    pub fn init_console(p: Periph<RegisterBlock, 192>, 
        rx: Pin<Input<Floating>, PD0>, 
        tx: Pin<Output, PD1>
    ) {
        let console = arduino_hal::Usart::new(
            p,
            rx,
            tx,
            // See src/usart.rs for why some boards use the BaudrateArduinoExt trait
            // instead of BaudrateExt.
            arduino_hal::hal::usart::BaudrateArduinoExt::into_baudrate(57600),
        );
        interrupt::free(|cs| {
            *CONSOLE.borrow(cs).borrow_mut() = Some(console);
    })
}


    type Console = arduino_hal::hal::usart::Usart0<arduino_hal::DefaultClock>;
    pub static CONSOLE: interrupt::Mutex<RefCell<Option<Console>>> =
        interrupt::Mutex::new(RefCell::new(None));

    #[macro_export]
    macro_rules! print {
        ($($t:tt)*) => {
            avr_device::interrupt::free(
                |cs| {
                    if let Some(console) = crate::console::CONSOLE.borrow(cs).borrow_mut().as_mut() {
                        let _ = ufmt::uwrite!(console, $($t)*);
                    }
                },
            )
        };
    }
    
    #[macro_export]
    macro_rules! println {
        ($($t:tt)*) => {
            avr_device::interrupt::free(
                |cs| {
                    if let Some(console) = crate::console::CONSOLE.borrow(cs).borrow_mut().as_mut() {
                        let _ = ufmt::uwriteln!(console, $($t)*);
                    }
                },
            )
        };
    }
}