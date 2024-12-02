use thiserror::Error;

#[derive(Error, Debug)]
pub enum SolutionError {
    #[error("Solution failed: {msg}")]
    ExecutionFailed {
        msg: String,
        #[source]
        source: Option<Box<dyn std::error::Error + Send + Sync>>,
    },

    #[error("Solution timed out after {seconds} seconds")]
    Timeout { seconds: u64 },

    #[error("Invalid solution result: {0}")]
    InvalidResult(String),

    #[error("Solution not implemented")]
    NotImplemented,
}

impl SolutionError {
    pub fn execution_error(msg: impl Into<String>) -> Self {
        Self::ExecutionFailed {
            msg: msg.into(),
            source: None,
        }
    }

    pub fn with_source(mut self, err: impl std::error::Error + Send + Sync + 'static) -> Self {
        if let Self::ExecutionFailed { ref mut source, .. } = self {
            *source = Some(Box::new(err));
        }
        self
    }
}
