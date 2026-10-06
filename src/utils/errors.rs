#[derive(Debug, Clone)]
pub enum VectoriumError {
    InvariantViolation(String),
    InvalidState(String),
    FFIError(String),
}

impl std::fmt::Display for VectoriumError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            VectoriumError::InvariantViolation(msg) => write!(f, "Invariant violation: {}", msg),
            VectoriumError::InvalidState(msg) => write!(f, "Invalid state: {}", msg),
            VectoriumError::FFIError(msg) => write!(f, "FFI error: {}", msg),
        }
    }
}

impl std::error::Error for VectoriumError {}
