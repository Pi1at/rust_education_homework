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

#[test]
fn test_remove_device_positive() {
    let mut room = Room::new("Kitchen".into()).with_devices(
        vec![
            "device1".to_string(),
            "device2".to_string(),
            "device3".to_string(),
        ]
        .into_iter(),
    );
    assert_eq!(room.device_names().count(), 3);

    room.remove_device("device2".to_string()).unwrap();

    assert_eq!(room.device_names().count(), 2);
    assert!(!room.device_names().any(|d| d == "device2"));
}

#[test]
fn test_remove_device_negative() {
    let mut room = Room::new("Kitchen".into()).with_devices(
        vec![
            "device1".to_string(),
            "device2".to_string(),
            "device3".to_string(),
        ]
        .into_iter(),
    );
    assert_eq!(room.device_names().count(), 3);

    // Trying to remove a device that doesn't exist - still Ok
    assert!(room.remove_device("device4".to_string()).is_ok());

    // The devices vector should remain unchanged
    assert_eq!(room.device_names().count(), 3);
}
