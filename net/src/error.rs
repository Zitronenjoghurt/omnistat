use thiserror::Error;

pub type NetResult<T> = Result<T, NetError>;

#[derive(Debug, Error)]
pub enum NetError {
    #[error("Decode error: {0}")]
    Decode(#[from] bincode::error::DecodeError),
    #[error("Encode error: {0}")]
    Encode(#[from] bincode::error::EncodeError),
    #[error("IO error: {0}")]
    IO(#[from] std::io::Error),
}
