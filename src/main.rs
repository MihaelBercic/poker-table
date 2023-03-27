#![no_std]
#![no_main]

mod motor;

use core::any::Any;
use arduino_hal::hal::port::{DynamicPort, PB5};
use arduino_hal::pac::portb::PORTB;
use arduino_hal::pac::portc::PORTC;
use arduino_hal::pac::portd::PORTD;
use arduino_hal::{delay_ms, Pins};
use arduino_hal::port::mode::{AnyInput, Floating, Input, Output, PullUp};
use arduino_hal::port::{mode, Pin, PinMode, PinOps};
use embedded_hal::digital::v2::{InputPin, OutputPin, PinState, ToggleableOutputPin};
use panic_halt as _;

#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);

    let mut button = pins.d12.into_pull_up_input();
    let mut motor_pins = [
        pins.d2.into_output().downgrade(),
        pins.d3.into_output().downgrade(),
        pins.d4.into_output().downgrade(),
        pins.d5.into_output().downgrade(),
    ];

    let mut stepper_motor = motor::stepper::new(&mut motor_pins);
    let mut led = pins.d13.into_output();

    let mut my_button = ObservableButton {
        pin: button,
        on_change: || {
            led.toggle();
            for _ in 0..2048 {
                stepper_motor.step();
                delay_ms(3);
            }
            delay_ms(50);

        },
    };

    loop {
        if my_button.pin.is_low() {
            (my_button.on_change)();
        }
    }
}

struct ObservableButton<F: FnMut() -> (), P: PinOps> {
    pin: Pin<Input<PullUp>, P>,
    on_change: F,
}