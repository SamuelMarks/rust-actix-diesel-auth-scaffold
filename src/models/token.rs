#[derive(serde::Serialize, serde::Deserialize, utoipa::ToSchema, Debug)]
pub struct Token {
    /// access token used for entry into protected endpoints
    #[schema(example = "username0::role1::access_token::1faf9af0-eac5-4066-b00d-e89e4a6b0b2e")]
    pub access_token: String,

    /// token type, e.g., Bearer is provided in the Authorization HTTP header
    #[schema(example = "Bearer")]
    pub token_type: String,

    /// how long until this token expires (in seconds)
    #[schema(example = 3600u64)]
    pub expires_in: u64,
}
