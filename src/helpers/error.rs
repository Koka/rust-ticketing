use actix_web::ResponseError;
use core::fmt::Debug;
use core::fmt::Display;
use tracing::error;

pub struct ActixError {
    err: eyre::Error,
}

pub type ActixResult<T> = Result<T, ActixError>;

impl Debug for ActixError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        f.write_fmt(format_args!("{}", self.err))
    }
}

impl Display for ActixError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str("Internal server error")
    }
}

impl From<eyre::Report> for ActixError {
    fn from(err: eyre::Report) -> Self {
        error!("{:?}", err);
        ActixError { err }
    }
}

impl ResponseError for ActixError {}
