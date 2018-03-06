use std::result;
use failure::{Context, Fail};
use std::fmt::{self, Display, Formatter};

pub struct Error {
    inner: Context<ErrorKind>,
}

#[derive(Copy, Clone, Eq, PartialEq, Debug, Fail)]
pub enum ErrorKind {
    #[fail(display = "failed to load config")]
    LoadConfigError,
}

pub type Result<T> = result::Result<T, Error>;

impl Fail for Error {
    fn cause(&self) -> Option<&Fail> {
        self.inner.cause()
    }

    fn backtrace(&self) -> Option<&Backtrace> {
        self.inner.backtrace()
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        self.inner.fmt(f)
    }
}

impl Error {
    pub fn kind(&self) -> ErrorKind {
        *self.inner.get_context()
    }
}

impl From<ErrorKind> for Error {
    fn from(kind: ErrorKind) -> Self {
        Self {
            inner: Context::new(kind),
        }
    }
}

impl From<Context<ErrorKind>> for MError {
    fn from(inner: Context<ErrorKind>) -> Self {
        Self { inner: inner }
    }
}
