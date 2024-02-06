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

impl SmartSocket {
    pub const fn new(name: String) -> Self {
        let state = SmartSocketState::Off;
        Self { name, state }
    }

    pub fn description(&self) -> &str {
        &self.name
    }

    pub fn turn_on(&mut self) {
        self.state = SmartSocketState::On;
    }

    pub fn turn_off(&mut self) {
        self.state = SmartSocketState::Off;
    }

    pub const fn get_current_power_usage(&self) -> Watt {
        match self.state {
            // TODO
            SmartSocketState::On => 1242,
            SmartSocketState::Off => 0,
        }
    }
}
