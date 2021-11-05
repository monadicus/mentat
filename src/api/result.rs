use crate::misc::Error;


impl From<anyhow::Error> for Error {
    fn from(error: anyhow::Error) -> Self {
        Self {
            code: 1,
            message: "Internal Failure".to_string(),
            description: Some(error.to_string()),
            retriable: true,
            details: Default::default(),
        }
    }
}

pub type Result<T, E = Error> = std::result::Result<T, E>;