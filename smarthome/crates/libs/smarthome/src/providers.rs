pub trait DeviceInfoProvider {
    type DeviceName;
    type RoomName;
    type Error;
    /// .
    ///
    /// # Errors
    ///
    /// This function will return an error if device not found in room.
    fn get_device_state(
        &self,
        room: &Self::RoomName,
        device: &Self::DeviceName,
    ) -> Result<String, Self::Error>;
}
