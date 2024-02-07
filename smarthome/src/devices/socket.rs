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
            SmartSocketState::On => 1242,
            SmartSocketState::Off => 0,
        }
    }
}
// region:    --- Tests

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_socket() -> SmartSocket {
        SmartSocket::new("TestSocket".into())
    }

    #[test]
    fn test_description_contains_name() {
        let name = "TestSocket";
        let ts = SmartSocket::new(name.into());
        assert!(ts.description().contains(name));
    }

    #[test]
    fn test_after_creation_turned_off() {
        let ts = create_test_socket();
        assert_eq!(ts.state, SmartSocketState::Off);
    }

    #[test]
    fn test_no_power_when_turned_off() {
        let mut ts = create_test_socket();
        ts.turn_off();
        assert_eq!(ts.get_current_power_usage(), 0);
    }

    #[test]
    fn test_power_when_turned_on() {
        let mut ts = create_test_socket();
        ts.turn_on();
        assert!(ts.get_current_power_usage() > 0);
    }

    #[test]
    fn test_turning_on_off_twice_in_a_row() {
        let mut ts = create_test_socket();
        ts.turn_on();
        // we can turn it on
        assert_eq!(ts.state, SmartSocketState::On);
        ts.turn_on();
        // even twice in a row
        assert_eq!(ts.state, SmartSocketState::On);

        ts.turn_off();
        // we can turn it off
        assert_eq!(ts.state, SmartSocketState::Off);

        ts.turn_off();
        // even twice in a row
        assert_eq!(ts.state, SmartSocketState::Off);
    }
}

// endregion: --- Tests
