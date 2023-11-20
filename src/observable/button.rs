use core::time;
use core::time::Duration;
use arduino_hal::delay_ms;
use arduino_hal::hal::port::Dynamic;
use arduino_hal::port::{Pin, PinOps};
use arduino_hal::port::mode::{Input, InputMode, PullUp};
use arduino_hal::simple_pwm::Prescaler::Prescale64;

const CLOCK_SPEED: i32 = 16_000_000; // Hz

pub fn new<F: FnMut() -> ()>(pin: Pin<Input<PullUp>, Dynamic>, on_change: F) -> ObservableButton<F> {
    ObservableButton { pin, on_change, is_pressed: false, elapsed_cycles: 0 }
}

pub struct ObservableButton<F: FnMut() -> ()> {
    pub pin: Pin<Input<PullUp>, Dynamic>,
    pub on_change: F,
    is_pressed: bool,
    elapsed_cycles: u32,
}

impl<F: FnMut() -> ()> ObservableButton<F> {
    pub fn listen(&mut self) {
        let next_value = self.elapsed_cycles.checked_add(1);
        match next_value {
            None => self.elapsed_cycles = 0,
            Some(x) => self.elapsed_cycles = x
        }

        if self.elapsed_cycles <= 5 * 16_000 { return; }

        let pin = &self.pin;
        let is_pin_pressed = pin.is_low();

        if !self.is_pressed {
            if is_pin_pressed {
                self.is_pressed = true;
                (self.on_change)();
            }
        } else {
            if !is_pin_pressed {
                self.is_pressed = false;
                self.elapsed_cycles = 0;
            }
        }
    }
}