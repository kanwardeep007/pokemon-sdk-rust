#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Unable to generate URL for the requested resource. Error: {0}")]
    UrlParseError(#[from] url::ParseError),
    #[error("Http Error: {0}")]
    HttpError(#[from] reqwest::Error),
    #[error("Error occurred when running a middleware")]
    MiddlewareError(#[from] reqwest_middleware::Error),
    #[error("Unexpected error happened: {0}")]
    Other(#[from] anyhow::Error),
}
