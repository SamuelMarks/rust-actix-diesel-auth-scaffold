#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct Token {
    pub access_token: String,
    pub token_type: String,
    pub expires_in: u64,
}
