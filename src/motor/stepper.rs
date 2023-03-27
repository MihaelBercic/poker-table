use arduino_hal::hal::port::Dynamic;
use arduino_hal::port::mode::Output;
use arduino_hal::port::Pin;

pub fn new(pins: &mut [Pin<Output, Dynamic>]) -> StepperMotor {
    StepperMotor { pins, current_step: 0 }
}

pub struct StepperMotor<'a> {
    pub pins: &'a mut [Pin<Output, Dynamic>],
    current_step: i8,
}

impl StepperMotor<'_> {
    pub fn step(&mut self) {
        let number_of_pins = self.pins.len() as i8;
        let high_pin: usize = (self.current_step % number_of_pins) as usize;
        for index in 0..number_of_pins as usize {
            let mut pin = &mut self.pins[index];
            if index == high_pin { pin.set_high(); } else { pin.set_low(); }
        }
        self.current_step = if self.current_step >= number_of_pins { 0 } else { self.current_step + 1 };
    }
}