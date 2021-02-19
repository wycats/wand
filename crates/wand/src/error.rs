use core::fmt::Display;
use swc_ecma_parser::error::Error as SwcError;

pub enum WandError {
    ParseError(SwcError),
}

impl From<SwcError> for WandError {
    fn from(error: SwcError) -> Self {
        WandError::ParseError(error)
    }
}

pub type WandResult<T> = Result<T, WandError>;

impl Display for WandError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            WandError::ParseError(e) => write!(f, "{:?}", e),
        }
    }
}
