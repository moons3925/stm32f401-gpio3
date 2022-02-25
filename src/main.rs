#![no_std]
#![no_main]

use panic_halt as _; // you can put a breakpoint on `rust_begin_unwind` to catch panics
use cortex_m_rt::entry;
use stm32f4xx_hal::{prelude::*, stm32, gpio::*};    // (1)
use stm32f4xx_hal::gpio::gpioa::PA5;    // (2)
use stm32f4xx_hal::gpio::gpioa::PA6;    // (3)

#[entry]
fn main() -> ! {
    let dp = stm32::Peripherals::take().unwrap();   // (4)
    let gpioa = dp.GPIOA.split();   // (5)
    let mut led = Led::new(gpioa.pa5);  // (6)
    let sw = Sw::new(gpioa.pa6);    // (7)

    loop {
        if sw.is_pressed() {    // (8)
            led.turn_on();  // (9)
        } else {
            led.turn_off(); // (10)
        }
    }
}

struct Sw { // (11)
    pin: PA6<Input<Floating>>   // (12)
}

impl Sw {   // (13)
    fn new(pin: PA6<Input<Floating>>) -> Sw {   // (14)
        Sw { pin: pin.into_floating_input() }   // (15) Sw { pin: pin }
    }
    fn is_pressed(&self) -> bool {  // (16)
        self.pin.is_low().unwrap()  // (17)
    }
    fn is_released(&self) -> bool { // (18)
        self.pin.is_high().unwrap() // (19)
    }
}

struct Led {    // (20)
    pin: PA5<Output<PushPull>>  // (21)
}

impl Led {  // (22)
    fn new(pin: PA5<Input<Floating>>) -> Led {  // (23)
        Led { pin: pin.into_push_pull_output() }    // (24)
    }
    fn turn_on(&mut self) { // (25)
        self.pin.set_high().unwrap();   // (26)
    }
    fn turn_off(&mut self) {    // (27)
        self.pin.set_low().unwrap();    // (28)
    }
}
