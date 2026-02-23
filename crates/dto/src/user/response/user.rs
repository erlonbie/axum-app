use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;
use chrono::{DateTime, FixedOffset};

#[derive(Serialize, Deserialize, ToSchema, Debug)]
pub struct UserResponse {
    pub id: Uuid,
    pub handle: String,
    pub display_name: String,
    pub bio: Option<String>,
    pub email: String,
    pub verified_at: Option<DateTime<FixedOffset>>,
    pub profile_image: Option<String>,
    pub banner_image: Option<String>,
    pub totp_enabled: bool,
    pub created_at: DateTime<FixedOffset>,
}
