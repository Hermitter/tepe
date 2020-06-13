use std::{error::Error as StdError, fmt, str::Utf8Error};

#[derive(Debug)]
pub enum Error {
    /// Some unspecified error.
    Any(Box<dyn StdError + Send + Sync + 'static>),
    UnknownError,
    UnreadableMessage,
}

impl Error {
    /// Exit and prints Error to the standard error
    pub fn log_if_err(&self) {
        eprintln!("{}", self);
        std::process::exit(1);
    }
}

impl<'a> fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::UnreadableMessage => write!(f, "Issue parsing message"),
            _ => write!(f, "TODO: add error description"),
        }
    }
}

// TODO: review
// impl<'a> From<()> for Error {
//     fn from(_err: ()) -> Self {
//         Error::UnknownError
//     }
// }
