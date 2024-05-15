use thiserror::Error;

#[derive(Error, Debug)]
pub enum ClientError {
    #[error("missing parameter {param}")]
    MissingParameter { param: String },
    #[error("missing value for {param}")]
    MissingValue { param: String },
    #[error("invalid value {value} for {param}")]
    InvalidValue { param: String, value: String },
    #[error("sever plugin error - {msg}")]
    Severe { msg: String },
    #[error("unknown plugin error")]
    Unknown,
}
