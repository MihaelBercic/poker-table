use arduino_hal::hal::port::Dynamic;
use arduino_hal::port::mode::Output;
use arduino_hal::port::Pin;

use crate::observable::stepper::StepDirection::Forward;

pub fn new(pins: [Pin<Output, Dynamic>; 4]) -> StepperMotor {
    StepperMotor { pins, current_step: 0 }
}


/**
Written for bipolar stepper motors.
 */
pub struct StepperMotor {
    pins: [Pin<Output, Dynamic>; 4],
    current_step: usize,
}

impl StepperMotor {
    pub fn step(&mut self, direction: StepDirection) {
        let number_of_pins = self.pins.len();
        let high_pin = self.current_step as usize % number_of_pins;
        let indices = 0..number_of_pins;

        for index in indices {
            let pin = &mut self.pins[index];
            if index == high_pin { pin.set_high(); } else { pin.set_low(); }
        }

        let is_forward = direction == Forward;
        let current_step = &mut self.current_step;

        let reset_condition = if is_forward { *current_step >= number_of_pins - 1 } else { *current_step <= 0 };
        let reset_value = if is_forward { 0 } else { 3 };

        if reset_condition {
            self.current_step = reset_value
        } else {
            if is_forward { *current_step = *current_step + 1 } else { *current_step = *current_step - 1 };
        };
    }

    pub fn rest(&mut self) {
        let pins = &mut self.pins.iter_mut();
        for x in pins {
            x.set_low();
        }
    }
}

#[derive(PartialEq)]
pub enum StepDirection {
    Forward,
    Backward,
}


