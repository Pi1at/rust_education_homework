use crate::devices::{socket::SmartSocket, thermometer::SmartThermometer};
use crate::location::{home::SmartHome, room::Room};
use crate::providers::DeviceInfoProvider;

#[test]
fn test_new_smart_home() {
    let home = SmartHome::new("My Home".into());
    assert_eq!(home.name, "My Home");
    assert_eq!(home.rooms.len(), 0);
}

#[test]
fn test_add_room() {
    let mut home = SmartHome::new("My Home".into());
    let room1 = Room::new("Living Room".into());
    let room2 = Room::new("Bedroom".into());

    home.add_room(room1.clone());
    assert_eq!(home.rooms.len(), 1);
    assert!(home.rooms.contains(&room1));

    home.add_room(room2.clone());
    assert_eq!(home.rooms.len(), 2);
    assert!(home.rooms.contains(&room2));

    // Add room with same name, it should replace the existing one
    let room3 = Room::new("Living Room".into());
    home.add_room(room3.clone());
    assert_eq!(home.rooms.len(), 2);
    assert!(home.rooms.contains(&room3));
}

#[test]
fn test_with_room() {
    let room1 = Room::new("Living Room".into());
    let room2 = Room::new("Bedroom".into());

    let home = SmartHome::new("My Home".into())
        .with_room(room1.clone())
        .with_room(room2.clone());

    assert_eq!(home.rooms.len(), 2);
    assert!(home.rooms.contains(&room1));
    assert!(home.rooms.contains(&room2));

    // Add room with same name, it should replace the existing one
    let room3 = Room::new("Living Room".into());
    let home = home.with_room(room3.clone());
    assert_eq!(home.rooms.len(), 2);
    assert!(home.rooms.contains(&room3));
}

#[test]
fn test_with_rooms() {
    let room1 = Room::new("Living Room".into());
    let room2 = Room::new("Bedroom".into());
    let room3 = Room::new("Kitchen".into());

    let new_rooms = vec![room2.clone(), room3.clone()];

    let home = SmartHome::new("My Home".into())
        .with_rooms(new_rooms.into_iter())
        .with_room(room1.clone());

    assert_eq!(home.rooms.len(), 3);
    assert!(home.rooms.get(&room1).is_some());
    assert!(home.rooms.get(&room2).is_some());
    assert!(home.rooms.get(&room3).is_some());

    // Add room with same name, it should replace the existing one
    let room4 = Room::new("Living Room".into());
    let home = home.with_room(room4.clone());
    assert_eq!(home.rooms.len(), 3);
    assert!(home.rooms.contains(&room4));
}

#[test]
fn test_get_rooms() {
    let room1 = Room::new("Living Room".into());
    let room2 = Room::new("Bedroom".into());

    let home = SmartHome::new("My Home".into())
        .with_room(room1.clone())
        .with_room(room2.clone());

    let returned_rooms = home.get_rooms();
    assert_eq!(returned_rooms.len(), 2);
    assert!(returned_rooms.contains(&room1.name));
    assert!(returned_rooms.contains(&room2.name));
}

#[test]
fn test_get_devices_in_room() {
    let device1 = SmartSocket::new("Device 1".into());
    let device2 = SmartThermometer::new("Device 2".into());

    let room = Room::new("Living Room".into())
        .with_device(device1.name.clone())
        .with_device(device2.name.clone());

    let home = SmartHome::new("My Home".into()).with_room(room);

    let returned_devices = home.get_devices_in_room("Living Room");
    assert_eq!(returned_devices.len(), 2);
    assert!(returned_devices.contains(&device1.name));
    assert!(returned_devices.contains(&device2.name));

    let should_be_empty = home.get_devices_in_room("Bedroom");
    assert_eq!(should_be_empty.len(), 0);
}

#[test]
fn test_create_report() {
    struct MockDeviceInfoProvider {
        device1: SmartSocket,
        device2: SmartThermometer,
    }

    impl DeviceInfoProvider for MockDeviceInfoProvider {
        fn get_device_state(&self, room: &str, device: &str) -> String {
            if room == "Living Room" && device == self.device1.description() {
                "Device 1 is on".to_string()
            } else if room == "Bedroom" && device == self.device2.name {
                "Device 2 is off".to_string()
            } else {
                "ERROR: Device state not available".to_string()
            }
        }
    }

    let device1 = SmartSocket::new("Device 1".into());
    let device2 = SmartThermometer::new("Device 2".into());
    let room1 = Room::new("Living Room".into()).with_device(device1.name.clone());

    let room2 = Room::new("Bedroom".into()).with_device(device2.name.clone());

    let home = SmartHome::new("My Home".into())
        .with_room(room1)
        .with_room(room2);

    let info_provider = MockDeviceInfoProvider { device1, device2 };
    let report = home.create_report(&info_provider);

    assert!(report.contains("Room: Living Room"));
    assert!(report.contains("Device: Device 1"));
    assert!(report.contains("Device 1 is on"));

    assert!(report.contains("Room: Bedroom"));
    assert!(report.contains("Device: Device 2"));
    assert!(report.contains("Device 2 is off"));

    assert!(!report.contains("ERROR"));
}
