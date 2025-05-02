#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("http: {0}")]
    Http(String),

    #[error("deserializing reply: {err}")]
    Deserializing { err: serde_json::Error },

    #[error("non OK reply: {status:03}")]
    NonOkReply { status: u16 },
}

impl From<reqwest::Error> for Error {
    fn from(e: reqwest::Error) -> Self {
        Self::Http(e.to_string())
    }
}
