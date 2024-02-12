use super::{DeviceName, RoomName};

// Помещение содержит названия нескольких устройств.
// Устройство имеет уникальное в рамках помещения имя.
// TODO: Better naming
pub trait DeviceLocation {
    type LocationName;
    type DeviceName;
    type Error;
    #[must_use]
    fn new(name: Self::LocationName) -> Self;
    fn get_location_name(&self) -> &Self::LocationName;
    fn get_location_name_mut(&mut self) -> &mut Self::LocationName;
    /// .
    ///
    /// # Errors
    ///
    /// This function will return an error if device name is not unique.
    fn add_device(&mut self, device: Self::DeviceName) -> Result<(), Self::Error>;
    fn device_names(&self) -> impl Iterator<Item = &Self::DeviceName>;
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Room {
    name: RoomName,
    devices: Vec<DeviceName>,
}
impl Room {
    /// ignored, if device already in list
    #[must_use]
    pub fn with_device(mut self, device: RoomName) -> Self {
        let _ = self.add_device(device);
        self
    }

    /// All device already in list ignored
    #[must_use]
    pub fn with_devices(mut self, devices: impl Iterator<Item = DeviceName>) -> Self {
        for device in devices {
            let _ = self.add_device(device);
        }
        self
    }
}

impl DeviceLocation for Room {
    type DeviceName = DeviceName;
    type LocationName = RoomName;
    type Error = &'static str;
    #[must_use]
    fn new(name: RoomName) -> Self {
        Self {
            name,
            devices: Vec::new(),
        }
    }
    fn add_device(&mut self, device: DeviceName) -> Result<(), Self::Error> {
        if self.devices.contains(&device) {
            Err("device already in list")
        } else {
            self.devices.push(device);
            Ok(())
        }
    }

    fn get_location_name(&self) -> &Self::LocationName {
        &self.name
    }

    fn get_location_name_mut(&mut self) -> &mut Self::LocationName {
        &mut self.name
    }

    fn device_names(&self) -> impl Iterator<Item = &Self::DeviceName> {
        self.devices.iter()
    }
}
