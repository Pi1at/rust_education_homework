pub trait DeviceInfoProvider {
    fn get_device_state(&self, room: &str, device: &str) -> String;
}
