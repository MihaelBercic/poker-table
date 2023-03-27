#![no_std]
#![no_main]

use arduino_hal::{delay_ms};
use panic_halt as _;

use crate::motor::stepper::StepDirection::{Backward, Forward};
use crate::motor::stepper::StepperMotor;

mod motor;
mod button;

#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);
    let mut stepper_pins = [
        pins.d2.into_output().downgrade(),
        pins.d3.into_output().downgrade(),
        pins.d4.into_output().downgrade(),
        pins.d5.into_output().downgrade(),
    ];

    let button = pins.d12.into_pull_up_input();
    let mut stepper_motor: StepperMotor = motor::stepper::new(&mut stepper_pins);
    let mut led = pins.d13.into_output();

    let mut my_button = button::ObservableButton {
        pin: button,
        on_change: || {
            led.toggle();
            let delay = 3;
            let steps = 2048;

            for _ in 0..steps {
                stepper_motor.step(Forward);
                delay_ms(delay);
            }
            delay_ms(1000);
            for _ in 0..steps {
                stepper_motor.step(Backward);
                delay_ms(delay);
            }
            delay_ms(1000);
            stepper_motor.rest();
        },
    };

    loop {
        if my_button.pin.is_low() {
            (my_button.on_change)();
        }
    }
}


