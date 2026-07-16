use serde::Deserialize;
use validator::Validate;

#[derive(Deserialize, Validate)]
pub struct LoginUserRequest {
    #[validate(email)]
    pub email: String,
    #[validate(length(min = 12, max = 128))]
    pub password: String,
}
