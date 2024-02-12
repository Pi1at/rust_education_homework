use crate::location::room::{DeviceLocation, Room};

#[test]
fn test_new_room() {
    let room = Room::new("Living Room".into());
    assert_eq!(room.get_location_name(), "Living Room");
    assert!(room.device_names().count() == 0);
}

#[test]
fn test_add_device() {
    let mut room = Room::new("Bedroom".into());
    let r = room.add_device("Lamp".into());
    assert!(r.is_ok());
    assert!(room.device_names().any(|n| n == "Lamp"));
}

#[test]
fn test_with_device() {
    let room = Room::new("Bathroom".into()).with_device("Shower".into());
    assert!(room.device_names().any(|n| n == "Shower"));
}

#[test]
fn test_with_devices() {
    let room = Room::new("Kitchen".into())
        .with_devices(vec!["Oven".into(), "Microwave".into()].into_iter());
    assert!(room.device_names().any(|n| n == "Oven"));
    assert!(room.device_names().any(|n| n == "Microwave"));
}
