use super::{DeviceName, RoomName};
use std::{borrow::Borrow, collections::HashSet};

// Помещение содержит названия нескольких устройств.
// Устройство имеет уникальное в рамках помещения имя.
#[derive(Clone, Eq, PartialEq)]
pub struct Room {
    pub(crate) name: RoomName,
    pub(crate) devices: HashSet<DeviceName>,
}

impl Room {
    #[must_use]
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
