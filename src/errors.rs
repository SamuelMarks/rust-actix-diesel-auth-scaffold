use std::string::ToString;

#[derive(derive_more::Display, derive_more::Error, derive_more::From, Debug)]
#[repr(u16)]
pub enum AuthError {
    #[error(ignore)]
    #[from(skip)]
    #[display("Unauthorised({_0:#?})")]
    Unauthorised(&'static str) = 401,

    #[error(ignore)]
    #[from(skip)]
    #[display("NotFound({_0:#?})")]
    NotFound(&'static str) = 404,

    #[display("{body:?}")]
    BadRequest { mime: mime::Mime, body: String } = 500,

    #[error(ignore)]
    #[display("{_0:?}")]
    HttpError(u16) = 596,

    #[display("{_0:?}\r\n{_1:?}")]
    HttpErrorWithBody(u16, String) = 597,

    #[error(ignore)]
    #[from(skip)]
    #[display("{_0:?}")]
    NotInstalled(String) = 598,

    // ************************
    // * Library level errors *
    // ************************
    #[display("`std::io::Error` error. {error:?}")]
    StdIoError { error: std::io::Error } = 700,

    #[display("`diesel::result::Error` error. {error:?}")]
    DieselError { error: diesel::result::Error } = 704,

    #[display("`diesel::r2d2::Error` error. {error:?}")]
    DieselR2d2Error { error: diesel::r2d2::Error } = 705,

    #[display("`diesel_migrations::MigrationError` error. {error:?}")]
    DieselMigrationError {
        error: diesel_migrations::MigrationError,
    } = 706,

    #[display("`r2d2::Error` error. {error:?}")]
    R2d2Error { error: diesel::r2d2::PoolError } = 707,

    #[error(ignore)]
    #[display("{_0:?}")]
    ExitCode(std::process::ExitCode) = 710,

    #[display("`serde_json::Error` error. {error:?}")]
    SerdeJsonError { error: serde_json::Error } = 721,

    #[display("`std::str::Utf8Error` error. {error:?}")]
    Utf8Error { error: std::str::Utf8Error } = 739,

    #[display("`redis::RedisError` error. {error:?}")]
    RedisError { error: redis::RedisError } = 750,

    #[display("`diesel::result::ConnectionError` error. {error:?}")]
    DieselConnectionError {
        error: diesel::result::ConnectionError,
    } = 751,

    #[display("`argon2::password_hash::Error` error. {error:?}")]
    Argon2PasswordHashError { error: argon2::password_hash::Error } = 752,

    #[display("`argon2::Error` error. {error:?}")]
    Argon2Error { error: argon2::Error } = 753,
}

impl AuthError {
    fn discriminant(&self) -> u16 {
        unsafe { *<*const _>::from(self).cast::<u16>() }
    }
}

fn to_http_response(auth_error: AuthError) -> actix_web::HttpResponse {
    use actix_web::ResponseError;
    let status_code = auth_error.status_code();
    let (body, mime) = {
        if let AuthError::BadRequest { mime, body } = auth_error {
            (body, mime)
        } else {
            (auth_error.to_string(), mime::APPLICATION_JSON)
        }
    };
    actix_web::HttpResponseBuilder::new(status_code)
        .content_type(mime)
        .body(body)
}

impl Into<actix_web::HttpResponse> for AuthError {
    fn into(self) -> actix_web::HttpResponse {
        to_http_response(self)
    }
}

const VALID_HTTP_CODES: [u16; 57] /*std::collections::HashSet<u16>*/ = [
    100, 101, 102, 200, 201, 202, 203, 204, 205, 206, 300, 301, 302, 303, 304, 305, 307, 308, 400,
    401, 402, 403, 404, 405, 406, 407, 408, 409, 410, 411, 412, 413, 414, 415, 416, 417, 418, 421,
    422, 423, 424, 426, 428, 429, 431, 451, 500, 501, 502, 503, 504, 505, 506, 507, 508, 510, 511,
];

impl actix_web::ResponseError for AuthError {
    fn status_code(&self) -> http::StatusCode {
        let _status_code: u16 = {
            if let AuthError::ExitCode(exit_code) = self {
                if exit_code == &std::process::ExitCode::SUCCESS {
                    200u16
                } else {
                    500u16
                }
            } else {
                let code = self.discriminant();
                if VALID_HTTP_CODES.contains(&code) {
                    code
                } else {
                    500u16
                }
            }
        };

        http::StatusCode::from_u16(_status_code).unwrap()
    }

    fn error_response(&self) -> actix_web::HttpResponse {
        actix_web::HttpResponse::build(self.status_code()).json(serde_json::json!({
        "error": "AuthError",
        "error_message": if let AuthError::BadRequest { mime: _, body } = self {
                format!("{}", body)
            } else {
                format!("{}", self)
            }
        }))
    }
}

impl std::process::Termination for AuthError {
    fn report(self) -> std::process::ExitCode {
        if let AuthError::ExitCode(exit_code) = self {
            return exit_code;
        }
        let status_code = self.discriminant();
        if status_code > u8::MAX as u16 {
            eprintln!("exit code {}", status_code);
            std::process::ExitCode::FAILURE
        } else {
            std::process::ExitCode::from(status_code as u8)
        }
    }
}

pub enum SuccessOrAuthError<T> {
    Ok(T),
    Err(AuthError),
}

impl<T> From<Result<T, AuthError>> for SuccessOrAuthError<T> {
    fn from(value: Result<T, AuthError>) -> Self {
        match value {
            Ok(val) => SuccessOrAuthError::Ok(val),
            Err(error) => SuccessOrAuthError::Err(error),
        }
    }
}

// Can't use `Result` because
// [E0117] Only traits defined in the current crate can be implemented for arbitrary types
impl<T: std::any::Any> std::process::Termination for SuccessOrAuthError<T> {
    fn report(self) -> std::process::ExitCode {
        const PROCESS_EXIT_CODE: fn(i32) -> std::process::ExitCode = |e: i32| {
            if e > u8::MAX as i32 {
                eprintln!("exit code {}", e);
                std::process::ExitCode::FAILURE
            } else {
                std::process::ExitCode::from(e as u8)
            }
        };

        /* const REPORT: fn(impl Termination + ToString + Sized) -> ExitCode = |err: impl std::process::Termination + std::string::ToString| -> std::process::ExitCode {
            eprintln!("{}", err.to_string());
            err.report()
        }; */

        match self {
            SuccessOrAuthError::Ok(e)
                if std::any::TypeId::of::<T>()
                    == std::any::TypeId::of::<std::process::ExitCode>() =>
            {
                *(&e as &dyn std::any::Any)
                    .downcast_ref::<std::process::ExitCode>()
                    .unwrap()
            }
            SuccessOrAuthError::Ok(_) => std::process::ExitCode::SUCCESS,
            SuccessOrAuthError::Err(err) => match err {
                AuthError::StdIoError { ref error } if error.raw_os_error().is_some() => {
                    let e = unsafe { error.raw_os_error().unwrap_unchecked() };
                    eprintln!("{}", e.to_string());
                    PROCESS_EXIT_CODE(e)
                }
                AuthError::ExitCode(exit_code) => exit_code,
                _ => {
                    eprintln!("{}", err.to_string());
                    err.report()
                }
            },
        }
    }
}

// TODO: Get `Into<AuthError>` syntax working
impl std::ops::FromResidual<Result<std::convert::Infallible, AuthError>>
    for SuccessOrAuthError<std::process::ExitCode>
{
    fn from_residual(residual: Result<std::convert::Infallible, AuthError>) -> Self {
        SuccessOrAuthError::Err(residual./*into_*/err().unwrap())
    }
}

impl std::ops::FromResidual<Result<std::convert::Infallible, std::io::Error>>
    for SuccessOrAuthError<std::process::ExitCode>
{
    fn from_residual(residual: Result<std::convert::Infallible, std::io::Error>) -> Self {
        SuccessOrAuthError::Err(AuthError::from(residual./*into_*/err().unwrap()))
    }
}
