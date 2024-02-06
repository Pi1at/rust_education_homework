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
// region:    --- Tests

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_description_matches_name() {
        let name = "TestSocket";
        let ts = SmartSocket::new(name.into());
        assert_eq!(ts.description(), name);
    }

    #[test]
    fn test_switching_on_off() {
        let mut ss1 = SmartSocket {
            name: "Socket1".into(),
            state: SmartSocketState::On,
        };
        // let mut ss2 = SmartSocket {
        //     name: "Socket2".into(),
        //     state: SmartSocketState::Off,
        // };
        let s3 = ss1.clone();
        let mut ts = SmartSocket::new("TestSocket".into());

        println!("Working with socket {ts:?}");

        // after creation we have State::Off
        assert_eq!(ts.state, SmartSocketState::Off);

        ts.turn_on();
        // we can turnng on
        assert_eq!(ts.state, SmartSocketState::On);

        ts.turn_on();
        // even twice in a row
        assert_eq!(ts.state, SmartSocketState::On);

        // get some power when turned on
        assert_ne!(ts.get_current_power_usage(), 0);

        ts.turn_off();
        // we can turn it off
        assert_eq!(ts.state, SmartSocketState::Off);

        ts.turn_off();
        // even twice in a row
        assert_eq!(ts.state, SmartSocketState::Off);

        // No power when turned off
        assert_eq!(ts.get_current_power_usage(), 0);
    }
}

// endregion: --- Tests
