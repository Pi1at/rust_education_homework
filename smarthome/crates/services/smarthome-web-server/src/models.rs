use uuid::Uuid;

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct LocationData {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<Uuid>,
    pub name: String,
}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct RoomData {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<Uuid>,
    pub name: String,
}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct DeviceData {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<Uuid>,
    pub name: String,
}
