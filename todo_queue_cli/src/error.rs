use std::result;
use failure::{Backtrace, Context, Fail};
use std::fmt::{self, Display, Formatter};
pub use failure::ResultExt;

#[derive(Debug)]
pub struct Error {
    inner: Context<ErrorKind>,
}

#[derive(Debug, Fail)]
#[fail(display = "a list named {} already exists", _0)]
pub struct ListAlreadyExists(pub String);

#[derive(Debug, Fail)]
#[fail(display = "No list was selected")]
pub struct NoListSelected;

#[derive(Debug, Fail)]
#[fail(display = "No list named {} exists", _0)]
pub struct NoSuchListExists(pub String);

#[derive(Copy, Clone, Eq, PartialEq, Debug, Fail)]
pub enum ErrorKind {
    #[fail(display = "failed to load config")]
    LoadConfig,
    #[fail(display = "failed to save config")]
    SaveConfig,
    #[fail(display = "failed to launch app")]
    Launch,
    #[fail(display = "problem in the command line interface")]
    Cli,
    #[fail(display = "failed to load list")]
    LoadList,
    #[fail(display = "failed to save list")]
    SaveList,
    #[fail(display = "failed to save app")]
    SaveApp,
    #[fail(display = "failed to add list")]
    AddList,
    #[fail(display = "failed to remove list")]
    RmList,
    #[fail(display = "failed to get list")]
    GetList,
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

impl From<Context<ErrorKind>> for Error {
    fn from(inner: Context<ErrorKind>) -> Self {
        Self { inner: inner }
    }
}
