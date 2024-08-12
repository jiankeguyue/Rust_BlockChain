use thiserror::Error;

#[derive(Debug, Error)]
pub enum BlockChainError {
    #[error("序列化或反序列化错误")]
    SerializeError(#[from] Box<bincode::ErrorKind>),

    #[error("Failed to access sled db")]
    SledError(#[from] sled::Error),

    #[error("UTF-8 conversion error")]
    FromUtf8Error(#[from] std::string::FromUtf8Error),

}

