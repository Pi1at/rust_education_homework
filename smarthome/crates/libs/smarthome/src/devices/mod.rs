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
    /// return type
    type R;
    #[must_use]
    fn get_measure(&self) -> Self::R;
}
pub trait SendCommand<Command> {
    type R;
    fn send_command(&mut self, command: Command) -> Self::R;
}

pub trait SendCommandAsync<Command> {
    type R;
    fn send_command(
        &mut self,
        command: Command,
    ) -> impl std::future::Future<Output = Self::R> + Send;
}
