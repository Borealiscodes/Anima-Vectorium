#[derive(Debug)]
pub enum VectoriumError {
    InvalidPacket(String),
    OperatorFailure(String),
    RuntimeFailure(String),
}

impl std::fmt::Display for VectoriumError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            VectoriumError::InvalidPacket(msg) => write!(f, "InvalidPacket: {}", msg),
            VectoriumError::OperatorFailure(msg) => write!(f, "OperatorFailure: {}", msg),
            VectoriumError::RuntimeFailure(msg) => write!(f, "RuntimeFailure: {}", msg),
        }
    }
}

impl std::error::Error for VectoriumError {}
