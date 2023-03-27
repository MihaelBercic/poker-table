use arduino_hal::hal::port::Dynamic;
use arduino_hal::port::mode::Output;
use arduino_hal::port::Pin;

use crate::motor::stepper::StepDirection::Forward;

pub fn new(pins: &mut [Pin<Output, Dynamic>]) -> StepperMotor {
    StepperMotor { pins, current_step: 0 }
}

pub struct StepperMotor<'a> {
    pub pins: &'a mut [Pin<Output, Dynamic>],
    current_step: usize,
}

impl StepperMotor<'_> {
    pub fn step(&mut self, direction: StepDirection) {
        let number_of_pins = self.pins.len();
        let high_pin: usize = self.current_step % number_of_pins;
        let indices = 0..number_of_pins;

        for index in indices {
            let pin = &mut self.pins[index];
            if index == high_pin { pin.set_high(); } else { pin.set_low(); }
        }

        // I don't like this...
        self.current_step = if direction == Forward {
            if self.current_step >= number_of_pins { 0 } else { self.current_step + 1 }
        } else {
            if self.current_step <= 0 { 3 } else { self.current_step - 1 }
        }
    }

    pub fn rest(&mut self){
        for index in 0..self.pins.len() {
            let pin = &mut self.pins[index];
            pin.set_low();
        }
    }
}

#[derive(PartialEq)]
pub enum StepDirection {
    Forward,
    Backward,
}