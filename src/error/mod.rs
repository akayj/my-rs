use thiserror::Error;

#[derive(Error, Debug)]
pub enum DataStoreError {
    // #[error("data store disconnected")]
    // Disconnect(#[from] io::Error),
    #[error("the data for key `{0}` is not available")]
    Redaction(String),

    #[error("invalid header (expected {expected:?}, found {found:?})")]
    InvalidHeader { expected: String, found: String },

    #[error("unknown data store error")]
    Unknown,
}

#[derive(Error, Debug)]
pub enum HTTPError {
    #[error("invalid http dns lookup error {0} (expected < {})", i32::MAX)]
    InvalidLookup(u32),
}
