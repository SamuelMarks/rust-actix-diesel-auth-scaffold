#[derive(serde::Deserialize, serde::Serialize, strum_macros::Display, utoipa::ToSchema, Debug)]
#[serde(rename_all = "snake_case")]
pub enum GrantType {
    Password,
    AuthorizationCode,
    ClientCredentials,
    RefreshToken,
    #[serde(skip)]
    Invalid,
}

#[derive(serde::Deserialize, serde::Serialize, utoipa::ToSchema, Debug)]
pub struct TokenRequest {
    /// RFC6749 grant type
    #[schema(example = "password")]
    pub grant_type: GrantType,

    /// optional username (as used, for example, in RFC6749's password grant flow)
    #[schema(example = "user0")]
    pub username: Option<String>,

    /// optional password (as used, for example, in RFC6749's password grant flow)
    #[schema(example = "pass0")]
    pub password: Option<String>,

    /// optional refresh token (as used, for example, in RFC6749's refresh grant flow)
    #[schema(example = crate::option_default::<String>)]
    pub refresh_token: Option<String>,

    /// optional client ID (required, for example, in RFC6749's non password non refresh grant flow)
    #[schema(example = crate::option_default::<String>)]
    pub client_id: Option<String>,

    /// optional client secret (as used, e.g., in RFC6749's non (password|refresh) grant flow)
    #[schema(example = crate::option_default::<String>)]
    pub client_secret: Option<String>,

    /// optional redirect_uri (used, e.g., in datatracker.ietf.org/doc/html/rfc6749#section-4.1.3)
    #[schema(example = crate::option_default::<String>)]
    pub redirect_uri: Option<String>,

    /// optional code (required, e.g., in datatracker.ietf.org/doc/html/rfc6749#section-4.1.3)
    #[schema(example = crate::option_default::<String>)]
    pub code: Option<String>,
}

impl Default for TokenRequest {
    fn default() -> Self {
        Self {
            grant_type: GrantType::Password,
            username: None,
            password: None,
            refresh_token: None,
            client_id: None,
            client_secret: None,
            redirect_uri: None,
            code: None,
        }
    }
}

pub const NO_PUBLIC_REGISTRATION: bool = match option_env!("NO_PUBLIC_REGISTRATION") {
    Some(_) => true, // s == "" || s == "true" || s == "True"|| s == "t" || s == "T" || s == "1",
    None => false,
};
