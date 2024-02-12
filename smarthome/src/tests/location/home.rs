use crate::devices::Construct;
use crate::devices::{socket::SmartSocket, thermometer::SmartThermometer};
use crate::location::home::LocationSchema;
use crate::location::room::DeviceLocation;
use crate::location::{self, DeviceName, RoomName};
use crate::location::{home::SmartHome, room::Room};
use crate::providers::DeviceInfoProvider;

#[test]
fn test_new_smart_home() {
    let home = SmartHome::new("My Home".into());
    assert_eq!(home.get_name(), "My Home");
    assert_eq!(home.get_rooms_names().len(), 0);
}

#[test]
fn test_add_room() {
    let mut home = SmartHome::new("My Home".into());
    let room1 = Room::new("Living Room".into());
    let room2 = Room::new("Bedroom".into());

    home.add_room(room1.clone());
    let rooms = home.get_rooms_names();
    assert_eq!(rooms.len(), 1);
    assert!(rooms.contains(room1.get_location_name()));

    home.add_room(room2.clone());
    let rooms = home.get_rooms_names();
    assert_eq!(rooms.len(), 2);
    assert!(rooms.contains(room2.get_location_name()));

    // Add room with same name, it should replace the existing one
    let room3 = Room::new("Living Room".into());
    home.add_room(room3.clone());
    let rooms = home.get_rooms_names();
    assert_eq!(rooms.len(), 2);
    assert!(rooms.contains(room3.get_location_name()));
}

#[test]
fn test_with_room() {
    let room1 = Room::new("Living Room".into());
    let room2 = Room::new("Bedroom".into());

    let home = SmartHome::new("My Home".into())
        .with_room(room1.clone())
        .with_room(room2.clone());

    let rooms = home.get_rooms_names();
    assert_eq!(rooms.len(), 2);
    assert!(rooms.contains(room1.get_location_name()));
    assert!(rooms.contains(room2.get_location_name()));

    // Add room with same name, it should replace the existing one
    let room3 = Room::new("Living Room".into());
    let home = home.with_room(room3.clone());
    let rooms = home.get_rooms_names();
    assert_eq!(rooms.len(), 2);
    assert!(rooms.contains(room3.get_location_name()));
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

    let rooms = home.get_rooms_names();
    assert_eq!(rooms.len(), 3);
    assert!(rooms.contains(room1.get_location_name()));
    assert!(rooms.contains(room2.get_location_name()));
    assert!(rooms.contains(room3.get_location_name()));

    // Add room with same name, it should replace the existing one
    let room4 = Room::new("Living Room".into());
    let home = home.with_room(room4.clone());
    let rooms = home.get_rooms_names();
    assert_eq!(rooms.len(), 3);
    assert!(rooms.contains(room4.get_location_name()));
}

#[test]
fn test_get_rooms() {
    let room1 = Room::new("Living Room".into());
    let room2 = Room::new("Bedroom".into());

    let home = SmartHome::new("My Home".into())
        .with_room(room1.clone())
        .with_room(room2.clone());

    let returned_rooms = home.get_rooms_names();
    assert_eq!(returned_rooms.len(), 2);
    assert!(returned_rooms.contains(room1.get_location_name()));
    assert!(returned_rooms.contains(room2.get_location_name()));
}

#[test]
fn test_get_devices_in_room() {
    let device1 = SmartSocket::new("Device 1".into());
    let device2 = SmartThermometer::new("Device 2".into());

    let room = Room::new("Living Room".into())
        .with_device(device1.name.clone())
        .with_device(device2.name.clone());

    let home = SmartHome::new("My Home".into()).with_room(room.clone());

    let returned_devices = home.get_devices_in_room(&room).unwrap();
    assert_eq!(returned_devices.len(), 2);
    assert!(returned_devices.contains(&device1.name));
    assert!(returned_devices.contains(&device2.name));

    let should_be_none = home.get_devices_in_room(&Room::new("Bedroom".into()));
    assert_eq!(should_be_none, None);
}

#[test]
fn test_create_report() {
    struct MockDeviceInfoProvider {
        device1: SmartSocket,
        device2: SmartThermometer,
    }

    impl DeviceInfoProvider for MockDeviceInfoProvider {
        type DeviceName = location::DeviceName;
        type RoomName = location::RoomName;
        fn get_device_state(
            &self,
            room: &RoomName,
            device: &DeviceName,
        ) -> Result<String, &'static str> {
            if room == "Living Room" && device == self.device1.description() {
                Ok("Device 1 is on".to_string())
            } else if room == "Bedroom" && device == &self.device2.name {
                Ok("Device 2 is off".to_string())
            } else {
                Err("Device state not available")
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

#[test]
fn test_delete_room_positive() {
    let mut hotel = SmartHome::new("Hotel".into());
    let room1 = Room::new("Room 1".into());
    let room2 = Room::new("Room 2".into());
    hotel.add_room(room1.clone());
    hotel.add_room(room2);

    assert_eq!(hotel.get_rooms_names().len(), 2);

    hotel.delete_room(room1);

    assert_eq!(hotel.get_rooms_names().len(), 1);
    assert_eq!(hotel.get_rooms_names()[0], "Room 2");
}

#[test]
fn test_delete_room_negative() {
    let mut hotel = SmartHome::new("Hotel".into());
    let room1 = Room::new("Room 1".into());
    let room2 = Room::new("Room 2".into());
    hotel.add_room(room1.clone());
    hotel.add_room(room2);

    assert_eq!(hotel.get_rooms_names().len(), 2);

    hotel.delete_room(room1.clone());

    assert_eq!(hotel.get_rooms_names().len(), 1);
    assert_eq!(hotel.get_rooms_names()[0], "Room 2");

    hotel.delete_room(room1);

    assert_eq!(hotel.get_rooms_names().len(), 1);
    assert_eq!(hotel.get_rooms_names()[0], "Room 2");
}
