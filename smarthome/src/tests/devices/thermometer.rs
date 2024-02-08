use std::fmt::Debug;

use crate::devices::thermometer::SmartThermometer;

fn create_test_thermometer() -> SmartThermometer {
    SmartThermometer::new("TestThermo".into())
}

#[test]
fn test_name_is_saved_after_creation() {
    let tt = create_test_thermometer();
    assert!(tt.name.contains("TestThermo"));
}

#[test]
fn test_can_get_some_temp() {
    let tt = create_test_thermometer();
    assert!(tt.get_temperature() != 0.);
}

#[test]
const fn test_clone() {
    const fn assert_clone<T: Clone>() {}
    assert_clone::<SmartThermometer>();
}

#[test]
const fn test_debug() {
    const fn assert_debug<T: Debug>() {}
    assert_debug::<SmartThermometer>();
}
