pub trait DeviceInfoProvider {
    type DeviceName;
    type RoomName;
    /// .
    ///
    /// # Errors
    ///
    /// This function will return an error if device not found in room.
    fn get_device_state(
        &self,
        room: &Self::RoomName,
        device: &Self::DeviceName,
    ) -> Result<String, &'static str>;
}
