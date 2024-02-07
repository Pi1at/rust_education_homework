use crate::providers::DeviceInfoProvider;

use super::room::Room;
use super::DeviceName;
use super::RoomName;

use std::collections::HashSet;

// Дом имеет название и содержит несколько помещений
// Помещение имеет уникальное название
pub struct SmartHome {
    pub(crate) name: String,
    pub(crate) rooms: HashSet<Room>,
}

impl SmartHome {
    #[must_use]
    pub fn new(name: &str) -> Self {
        Self {
            name: name.into(),
            rooms: HashSet::new(),
        }
    }
    /// room with equal name will be replaced by new one
    pub fn add_room(&mut self, room: Room) {
        self.rooms.insert(room);
    }
    /// room with equal name will be replaced by new one
    #[must_use]
    pub fn with_room(mut self, room: Room) -> Self {
        self.rooms.insert(room);
        self
    }
    /// room with equal name will be replaced by new one
    #[must_use]
    pub fn with_rooms(mut self, new_rooms: impl Iterator<Item = Room>) -> Self {
        self.rooms.extend(new_rooms);
        self
    }

    // Библиотека позволяет запросить список помещений в доме.
    #[must_use]
    pub fn get_rooms(&self) -> Vec<RoomName> {
        self.rooms.iter().map(|r| r.name.clone()).collect()
    }
    // Библиотека позволяет получать список устройств в помещении.
    pub fn get_devices_in_room(&self, room: &str) -> Vec<DeviceName> {
        self.rooms
            .get(room)
            .map_or_else(Vec::new, |r| r.devices.iter().cloned().collect())
    }
    // Библиотека имеет функцию, возвращающую текстовый отчёт о состоянии дома
    pub fn create_report<T: DeviceInfoProvider>(&self, info_provider: &T) -> String {
        let mut report = String::new();
        for room in &self.rooms {
            report.push_str(&format!("Room: {}\n", room.name));
            for device in &room.devices {
                let state = info_provider.get_device_state(&room.name, device);
                if !state.to_uppercase().contains("ERROR") {
                    let device_line = format!("Device: {device}\n{state}\n");
                    report.push_str(&device_line);
                }
            }
        }
        report
    }
}

#[cfg(test)]
mod tests {
    use crate::devices::{socket::SmartSocket, thermometer::SmartThermometer};

    use super::*;

    #[test]
    fn test_new_smart_home() {
        let home = SmartHome::new("My Home");
        assert_eq!(home.name, "My Home");
        assert_eq!(home.rooms.len(), 0);
    }

    #[test]
    fn test_add_room() {
        let mut home = SmartHome::new("My Home");
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

        let home = SmartHome::new("My Home")
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

        let home = SmartHome::new("My Home")
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

        let home = SmartHome::new("My Home")
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

        let home = SmartHome::new("My Home").with_room(room);

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

        let home = SmartHome::new("My Home").with_room(room1).with_room(room2);

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
}
