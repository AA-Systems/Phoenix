use serde::Deserialize;
use validator::Validate;
use validator::ValidationError;

pub fn validate_password(password: &str) -> Result<(), ValidationError> {
    let has_uppercase = password.chars().any(char::is_uppercase);
    let has_lowercase = password.chars().any(char::is_lowercase);
    let has_number = password.chars().any(char::is_numeric);
    let has_special = password
        .chars()
        .any(|character| !character.is_alphanumeric() && !character.is_whitespace());
    let has_control_character = password.chars().any(char::is_control);

    if has_uppercase && has_lowercase && has_number && has_special && !has_control_character {
        return Ok(());
    }

    let mut error = ValidationError::new("password_complexity");
    error.message =
        Some("password must contain uppercase, lowercase, number, and special characters".into());
    Err(error)
}

#[derive(Deserialize, Validate)]
pub struct RegisterUserRequest {
    #[validate(length(min = 1, max = 64))]
    pub name: String,
    #[validate(email)]
    pub email: String,
    #[validate(length(min = 12, max = 128), custom(function = "validate_password"))]
    pub password: String,
}
