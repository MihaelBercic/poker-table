use arduino_hal::port::{Pin, PinOps};
use arduino_hal::port::mode::{Input, PullUp};

pub struct ObservableButton<F: FnMut() -> (), P: PinOps> {
    pub(crate) pin: Pin<Input<PullUp>, P>,
    pub(crate) on_change: F,
}