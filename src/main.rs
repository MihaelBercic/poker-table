#![no_std]
#![no_main]

mod observable;

use arduino_hal::{delay_ms, pac};
use arduino_hal::hal::port::PD7;
use arduino_hal::port::mode::Output;
use arduino_hal::port::Pin;
use arduino_hal::simple_pwm::{IntoPwmPin, Prescaler, Timer2Pwm};
use embedded_hal::digital::v2::{OutputPin, PinState};
use embedded_hal::digital::v2::PinState::{High, Low};
use panic_halt as _;


fn setup() {}

#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);

    let mut led_pin = pins.d13.into_output();
    let button_pin = pins.d12.into_pull_up_input().downgrade();

    let photo_resistor_pin = pins.d7.into_pull_up_input().downgrade();



    let mut my_button = observable::button::new(button_pin, || {
    });

    let mut photo_resistor = observable::photo_resistor::new(photo_resistor_pin, || {
        led_pin.toggle();
        delay_ms(100);
    });


    loop {
        my_button.listen();
        photo_resistor.listen();
    }
}