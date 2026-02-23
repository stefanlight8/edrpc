use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

#[derive(Serialize_repr, Deserialize_repr, Debug, Default)]
#[repr(u8)]
pub enum ActivityType {
    #[default]
    Playing = 0,
    Streaming = 1,
    Listening = 2,
    Watching = 3,
    Custom = 4,
    Competing = 5,
}

#[derive(Serialize_repr, Deserialize_repr, Debug, Default)]
#[repr(u8)]
pub enum StatusDisplayType {
    #[default]
    Name = 0,
    State = 1,
    Details = 2,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ActivityTimestamps {
    pub end: Option<u64>,
    pub start: Option<u64>,
}

#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(default)]
pub struct ActivityAssets {
    pub large_image: Option<String>,
    pub large_text: Option<String>,
    pub large_url: Option<String>,
    pub small_image: Option<String>,
    pub small_text: Option<String>,
    pub small_url: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ActivityButton {
    pub label: Option<String>,
    pub url: Option<String>
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Activity {
    pub name: String,
    pub activity_type: ActivityType,
    pub created_at: i64,
    pub status_display_type: StatusDisplayType,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_id: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub details: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub details_url: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub state_url: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub timestamps: Option<ActivityTimestamps>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub assets: Option<ActivityAssets>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    
    // дискорд позволяет добавить лишь 2 кнопки
    #[serde(skip_serializing_if = "Option::is_none")]
    pub buttons: Option<Vec<ActivityButton>>,
}
