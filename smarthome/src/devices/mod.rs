// region:    --- SmartThermometer
type Tempreature = f32;

#[derive(Clone, Debug)]
pub struct SmartThermometer {
    pub name: String,
}

impl SmartThermometer {
    pub const fn new(name: String) -> Self {
        Self { name }
    }

    pub const fn get_temperature(&self) -> Tempreature {
        // TODO
        10.0
    }
}
// endregion: --- SmartThermometer

// region:    --- SmartSocket
type Watt = usize;

#[derive(Clone, Debug)]
enum SmartSocketState {
    On,
    Off,
}

#[derive(Clone, Debug)]
pub struct SmartSocket {
    pub name: String,
    state: SmartSocketState,
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
// endregion: --- SmartSocket
