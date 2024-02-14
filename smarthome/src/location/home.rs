use crate::providers::DeviceInfoProvider;

use super::room::DeviceLocation;
use super::room::Room;

pub trait LocationSchema {
    /// Represents a location for devices, including the location name and devices names.
    type R: DeviceLocation<LocationName = Self::L, DeviceName = Self::D>;
    /// Represents the location name.
    type L;
    /// Represents the device name.
    type D;
    //type DeviceName = DeviceLocation::DeviceName;
    #[must_use]
    fn new(name: String) -> Self;

    fn get_name(&self) -> &str;

    /// room with equal name will be replaced by new one
    fn add_room(&mut self, room: Self::R);

    // Библиотека позволяет запросить список помещений в доме.
    #[must_use]
    fn get_rooms_names(&self) -> Vec<Self::L>;

    // Библиотека позволяет получать список устройств в помещении.
    fn get_devices_in_room(&self, room: &Self::R) -> Option<Vec<Self::D>>;

    // Библиотека имеет функцию, возвращающую текстовый отчёт о состоянии дома
    fn create_report<T>(&self, info_provider: &T) -> String
    where
        T: DeviceInfoProvider<RoomName = Self::L, DeviceName = Self::D>;
}

// Дом имеет название и содержит несколько помещений
// Помещение имеет уникальное название
pub struct SmartHome {
    name: String,
    rooms: Vec<Room>,
}

impl SmartHome {
    /// finds room by location name
    fn try_find_room_mut(&mut self, r: &Room) -> Option<&mut Room> {
        self.rooms
            .iter_mut()
            .find(|room| room.get_location_name() == r.get_location_name())
    }
    #[must_use]
    pub fn with_rooms(mut self, rooms: impl Iterator<Item = Room>) -> Self {
        for room in rooms {
            self.add_room(room);
        }
        self
    }

    #[must_use]
    pub fn with_room(mut self, room: Room) -> Self {
        self.add_room(room);
        self
    }
}

impl LocationSchema for SmartHome {
    type R = Room;
    type D = <Self::R as DeviceLocation>::DeviceName;
    type L = <Self::R as DeviceLocation>::LocationName;
    #[must_use]
    fn new(name: String) -> Self {
        Self {
            name,
            rooms: Vec::new(),
        }
    }
    /// room with equal name will be replaced by new one
    fn add_room(&mut self, room: Room) {
        match self.try_find_room_mut(&room) {
            Some(entry) => *entry = room,
            None => self.rooms.push(room),
        }
    }

    // Библиотека позволяет запросить список помещений в доме.
    #[must_use]
    fn get_rooms_names(&self) -> Vec<Self::L> {
        self.rooms
            .iter()
            .map(|r| r.get_location_name().clone())
            .collect()
    }
    // Библиотека позволяет получать список устройств в помещении.
    fn get_devices_in_room(&self, room: &Self::R) -> Option<Vec<Self::D>> {
        let room_name = room.get_location_name();
        self.rooms
            .iter()
            .find(|r| r.get_location_name() == room_name)
            .map(|r| r.device_names().cloned().collect())
    }
    // Библиотека имеет функцию, возвращающую текстовый отчёт о состоянии дома
    fn create_report<T>(&self, info_provider: &T) -> String
    where
        T: DeviceInfoProvider<RoomName = Self::L, DeviceName = Self::D>,
    {
        let mut report = String::new();
        report.push_str(&format!("Home: {}\n", self.name));
        for room in &self.rooms {
            report.push_str(&format!("Room: {}\n", room.get_location_name()));
            for device in room.device_names() {
                if let Ok(state) = info_provider.get_device_state(room.get_location_name(), device)
                {
                    let device_line = format!("Device: {device}\n{state}\n");
                    report.push_str(&device_line);
                };
            }
        }
        report
    }

    fn get_name(&self) -> &str {
        self.name.as_str()
    }
}
