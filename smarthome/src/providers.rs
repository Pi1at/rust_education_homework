use crate::location::{DeviceName, RoomName};

pub trait DeviceInfoProvider {
    fn get_device_state(&self, room: &RoomName, device: &DeviceName) -> String;
}
