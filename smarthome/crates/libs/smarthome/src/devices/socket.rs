use std::convert::Infallible;

use crate::location::DeviceName;

use super::{Construct, Gauge, SendCommand};

type Watt = usize;

#[derive(Clone, Debug, PartialEq)]
pub(crate) enum SmartSocketState {
    On,
    Off,
}

#[derive(Clone, Debug)]
pub struct SmartSocket {
    pub name: String,
    pub(crate) state: SmartSocketState,
}

impl Construct for SmartSocket {
    fn new(name: DeviceName) -> Self {
        let state = SmartSocketState::Off;
        Self { name, state }
    }
}
impl Gauge<Watt> for SmartSocket {
    fn get_measure(&self) -> Watt {
        match self.state {
            SmartSocketState::On => 1242,
            SmartSocketState::Off => 0,
        }
    }
}

struct TurnOn;
struct TurnOff;

impl SendCommand<TurnOn> for SmartSocket {
    type R = Result<(), Infallible>;
    fn send_command(&mut self, _: TurnOn) -> Self::R {
        self.state = SmartSocketState::On;
        Ok(())
    }
}

impl SendCommand<TurnOff> for SmartSocket {
    type R = Result<(), Infallible>;
    fn send_command(&mut self, _: TurnOff) -> Self::R {
        self.state = SmartSocketState::Off;
        Ok(())
    }
}

impl SmartSocket {
    #[must_use]
    pub fn description(&self) -> &str {
        &self.name
    }

    pub fn turn_on(&mut self) {
        let _ = self.send_command(TurnOn);
    }

    pub fn turn_off(&mut self) {
        let _ = self.send_command(TurnOff);
    }

    #[must_use]
    pub fn get_current_power_usage(&self) -> Watt {
        self.get_measure()
    }
}
