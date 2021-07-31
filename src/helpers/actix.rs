use actix_web::error;
use tracing::error;

pub fn wrap_err<T>(result: eyre::Result<T>) -> actix_web::Result<T> {
    result.map_err(|e| {
        error!("{:?}", e);
        error::ErrorInternalServerError(format!("{}", e))
    })
}
