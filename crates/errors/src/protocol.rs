pub mod user {
    pub const USER_INVALID_PASSWORD: &str = "user:invalid_password";
    pub const USER_PASSWORD_NOT_SET: &str = "user:password_not_set";
    pub const USER_INVALID_SESSION: &str = "user:invalid_session";
    pub const USER_NOT_VERIFIED: &str = "user:not_verified";
    pub const USER_NOT_FOUND: &str = "user:not_found";
    pub const USER_UNAUTHORIZED: &str = "user:unauthorized";
    pub const USER_BANNED: &str = "user:banned";
    pub const USER_PERMISSION_INSUFFICIENT: &str = "user:permission_insufficient";
    pub const USER_HANDLE_ALREADY_EXISTS: &str = "user:handle_already_exists";
    pub const USER_EMAIL_ALREADY_EXISTS: &str = "user:email_already_exists";
    pub const USER_TOKEN_EXPIRED: &str = "user:token_expired";
    pub const USER_NO_REFRESH_TOKEN: &str = "user:no_refresh_token";
    pub const USER_INVALID_TOKEN: &str = "user:invalid_token";
}

pub mod system {
    pub const SYS_INTERNAL_ERROR: &str = "system:internal_error";
    pub const SYS_HASHING_ERROR: &str = "system:hashing_error";
    pub const SYS_NOT_FOUND: &str = "system:not_found";
    pub const SYS_TRANSACTION_ERROR: &str = "system:transaction_error";
    pub const SYS_DATABASE_ERROR: &str = "system:database_error";
    pub const SYS_TOKEN_CREATION_ERROR: &str = "system:token_creation_error";
}
