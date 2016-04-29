use std::error;
use std::fmt;
use std::result::Result as StdResult;

pub type Result<T> = StdResult<T, error::Error>;

#[derive(Debug)]
pub struct AppendError;

impl error::Error for AppendError {
  fn description(&self) -> &str {
    unimplemented!()
  }

  fn cause(&self) -> Option<&error::Error> {
    unimplemented!()
  }
}

impl fmt::Display for AppendError {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "IO error: {}", "test")
  }
}

#[derive(Debug)]
pub struct SearchError;

#[derive(Debug)]
pub struct FetchError;

#[derive(Debug)]
pub struct StoreError;

#[derive(Debug)]
pub struct CopyError;

#[derive(Debug)]
pub struct UnknownCommandOrInvalidArgsError;

#[derive(Debug)]
pub struct NoSuchMailboxError;

#[derive(Debug)]
pub struct InvalidCredentialsError;

#[derive(Debug)]
pub struct UnableToCreateMailboxError;

#[derive(Debug)]
pub struct UnableToDeleteMailboxError;

#[derive(Debug)]
pub struct UnableToRenameMailboxError;

#[derive(Debug)]
pub struct UnableToSubscribeToMailboxError;