use crate::location::room::Room;

#[test]
fn test_new_room() {
    let room = Room::new("Living Room".into());
    assert_eq!(room.name, "Living Room");
    assert!(room.devices.is_empty());
}

#[test]
fn test_add_device() {
    let mut room = Room::new("Bedroom".into());
    room.add_device("Lamp".into());
    assert!(room.devices.contains("Lamp"));
}

#[test]
fn test_with_device() {
    let room = Room::new("Bathroom".into()).with_device("Shower".into());
    assert!(room.devices.contains("Shower"));
}

#[test]
fn test_with_devices() {
    let room = Room::new("Kitchen".into())
        .with_devices(vec!["Oven".into(), "Microwave".into()].into_iter());
    assert!(room.devices.contains("Oven"));
    assert!(room.devices.contains("Microwave"));
}
