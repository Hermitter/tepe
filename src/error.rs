use std::{error::Error as StdError, fmt};

#[derive(Debug)]
pub enum Error {
    /// Some unspecified error.
    Any(Box<dyn StdError + Send + Sync + 'static>),
    UnknownError,
    UnreadableMessage,
    FileNotFound {
        path: String,
    },
    RequestError {
        description: String,
    },
    ParsingError {
        item: String,
    },
}

impl Error {
    /// Prints error to stderr and exits the program.
    pub fn exit(self) {
        eprintln!("{}", self);
        std::process::exit(1);
    }
}

impl<'a> fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::RequestError { ref description } => {
                write!(f, "Message failed to send due to:\n\t{}", description)
            }
            Error::FileNotFound { ref path } => write!(f, "Could not find file:\n\t{}", path),
            Error::ParsingError { ref item } => write!(f, "Error from parsing:\n\t{}", item),
            Error::UnreadableMessage => write!(f, "Issue parsing message"),
            _ => write!(f, "TODO: add error description"),
        }
    }
}

use teloxide::RequestError;
impl From<RequestError> for Error {
    fn from(error: RequestError) -> Self {
        Error::RequestError {
            description: format!("{}", error),
        }
    }
}

impl From<std::num::ParseIntError> for Error {
    fn from(error: std::num::ParseIntError) -> Self {
        Error::ParsingError {
            item: error.to_string(),
        }
    }
}

/// Trait to allow for an panic without Rust errors printing.
/// This is mainly meant for Option and Result.
pub trait CliExit<T> {
    /// Prints message to stderr and exits the program.
    fn cli_expect(self, message: &str) -> T;
}

impl<T, E> CliExit<T> for Result<T, E> {
    fn cli_expect(self, message: &str) -> T {
        match self {
            Ok(t) => t,
            Err(_e) => {
                eprintln!("{}", message);
                std::process::exit(1);
            }
        }
    }
}

impl<T> CliExit<T> for Option<T> {
    fn cli_expect(self, message: &str) -> T {
        match self {
            Some(t) => t,
            None => {
                eprintln!("{}", message);
                std::process::exit(1);
            }
        }
    }
}
