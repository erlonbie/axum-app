use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use validator::Validate;

#[derive(Deserialize, Serialize, Validate, ToSchema, Debug, Clone)]
pub struct UpdateUserRequest {
    #[validate(length(min = 1, max = 50))]
    pub display_name: Option<String>,

    #[validate(length(min = 3, max = 30))]
    pub handle: Option<String>,

    #[validate(length(max = 500))]
    pub bio: Option<String>,

    pub profile_image: Option<String>,

    pub banner_image: Option<String>,
}
