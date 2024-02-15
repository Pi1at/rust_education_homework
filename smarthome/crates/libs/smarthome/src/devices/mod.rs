#![allow(clippy::module_name_repetitions)]
pub mod socket;
pub mod thermometer;
use crate::location::DeviceName;

//TODO: Need better naming
pub trait Construct {
    #[must_use]
    fn new(name: DeviceName) -> Self;
}
pub trait Gauge<T> {
    #[must_use]
    fn get_measure(&self) -> T;
}
pub trait SendCommand<Command> {
    type R;
    fn send_command(&mut self, command: Command) -> Self::R;
}
