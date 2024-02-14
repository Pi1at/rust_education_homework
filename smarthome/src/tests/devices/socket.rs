use crate::devices::{
    socket::{SmartSocket, SmartSocketState},
    Construct,
};

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
