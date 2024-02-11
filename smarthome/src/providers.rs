pub trait DeviceInfoProvider {
    type DeviceName;
    type RoomName;
    fn get_device_state(&self, room: &Self::RoomName, device: &Self::DeviceName) -> String;
}
