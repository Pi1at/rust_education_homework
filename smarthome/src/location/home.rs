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
