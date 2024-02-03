// разрешим пока прототип
#![allow(dead_code)]

use std::{borrow::Borrow, collections::HashSet};

// region:    --- SmartThermometer
type Tempreature = f32;

#[derive(Clone, Debug)]
pub struct SmartThermometer {
    pub name: String,
}

impl SmartThermometer {
    pub const fn new(name: String) -> Self {
        Self { name }
    }

    pub const fn get_temperature(&self) -> Tempreature {
        // TODO
        10.0
    }
}
// endregion: --- SmartThermometer

// region:    --- SmartSocket
type Watt = usize;

#[derive(Clone, Debug)]
enum SmartSocketState {
    On,
    Off,
}

#[derive(Clone, Debug)]
pub struct SmartSocket {
    pub name: String,
    state: SmartSocketState,
}

impl SmartSocket {
    pub const fn new(name: String) -> Self {
        let state = SmartSocketState::Off;
        Self { name, state }
    }

    pub fn description(&self) -> &str {
        &self.name
    }

    pub fn turn_on(&mut self) {
        self.state = SmartSocketState::On;
    }

    pub fn turn_off(&mut self) {
        self.state = SmartSocketState::Off;
    }

    pub const fn get_current_power_usage(&self) -> Watt {
        match self.state {
            // TODO
            SmartSocketState::On => 1242,
            SmartSocketState::Off => 0,
        }
    }
}
// endregion: --- SmartSocket

// region:    --- Room
type DeviceName = String;
type RoomName = String;

// Помещение содержит названия нескольких устройств.
// Устройство имеет уникальное в рамках помещения имя.
#[derive(Clone, Eq, PartialEq)]
pub struct Room {
    name: RoomName,
    devices: HashSet<DeviceName>,
}

impl Room {
    pub fn new(name: RoomName) -> Self {
        Self {
            name,
            devices: HashSet::new(),
        }
    }
    pub fn add_device(&mut self, device: DeviceName) {
        self.devices.insert(device);
    }
    #[must_use]
    pub fn with_device(mut self, device: DeviceName) -> Self {
        self.devices.insert(device);
        self
    }
    #[must_use]
    pub fn with_devices(mut self, r: impl Iterator<Item = DeviceName>) -> Self {
        self.devices.extend(r);
        self
    }
}

// TODO: a bit hacky, just fallback to vec implementation? - it's unlikely to be 100500 Rooms

impl std::hash::Hash for Room {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.name.hash(state);
    }
}

impl Borrow<str> for Room {
    fn borrow(&self) -> &str {
        self.name.as_str()
    }
}
impl Borrow<RoomName> for Room {
    fn borrow(&self) -> &RoomName {
        &self.name
    }
}

// endregion: --- Room

// region:    --- SmartHouse

// Дом имеет название и содержит несколько помещений
// Помещение имеет уникальное название
pub struct SmartHome {
    name: String,
    rooms: HashSet<Room>,
}
impl SmartHome {
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
pub trait DeviceInfoProvider {
    fn get_device_state(&self, room: &str, device: &str) -> String;
}
// endregion: --- SmartHouse

#[cfg(test)]
mod tests {
    //use super::*;

    // #[test]
    // fn it_works() {
    //     //TODO: реализовать в дальнейшем тесты
    // }
}
