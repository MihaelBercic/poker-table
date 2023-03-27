#![no_std]
#![no_main]

use arduino_hal::{delay_ms};
use arduino_hal::port::{Pin, PinOps};
use arduino_hal::port::mode::{Input, PullUp};
use panic_halt as _;

use crate::motor::stepper::StepDirection::{Backward, Forward};

mod motor;

#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);

    let button = pins.d12.into_pull_up_input();
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
            for _ in 0..501 {
                stepper_motor.step(Forward);
                delay_ms(10);
            }
            delay_ms(1000);
            for _ in 0..501 {
                stepper_motor.step(Backward);
                delay_ms(10);
            }
            stepper_motor.rest();
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