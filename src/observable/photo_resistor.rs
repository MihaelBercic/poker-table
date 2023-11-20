use arduino_hal::delay_ms;
use arduino_hal::hal::port::{Dynamic, PD7};
use arduino_hal::port::mode::{Input, PullUp};
use arduino_hal::port::Pin;
use embedded_hal::digital::v2::PinState;
use embedded_hal::digital::v2::PinState::{High, Low};


pub type PullUpInput = Pin<Input<PullUp>, Dynamic>;

pub fn new<F: FnMut() -> ()>(pin: PullUpInput, on_change: F) -> Resistor<F> {
    return Resistor { pin, on_change, previous_state: High };
}

pub struct Resistor<F: FnMut() -> ()> {
    pin: PullUpInput,
    on_change: F,
    previous_state: PinState,
}

impl<F: FnMut() -> ()> Resistor<F> {
    pub fn listen(&mut self) {
        let current_state = if self.pin.is_high() { High } else { Low };
        let did_change = self.previous_state != current_state;
        if did_change {
            self.previous_state = current_state.clone(); // Why does this not work without .clone(); ?
            (self.on_change)();
        }
    }
}