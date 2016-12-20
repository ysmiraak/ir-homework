use std::error;
use std::fmt;
use std::num;
use std::io;

#[derive(Debug)]
pub struct FormatError(String);

impl FormatError {
    pub fn new(trouble: &str) -> Self {
        FormatError(trouble.to_owned())
    }
}

impl fmt::Display for FormatError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "illformed: {}", self.0)
    }
}

impl error::Error for FormatError {
    fn description(&self) -> &str {
        "unknown format."
    }

    fn cause(&self) -> Option<&error::Error> { None }
}

#[derive(Debug)]
pub enum LoadError {
    Io(io::Error),
    Parse(num::ParseIntError),
    Format(FormatError)
}

impl fmt::Display for LoadError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            LoadError::Io(ref err) => write!(f, "IO error: {}", err),
            LoadError::Parse(ref err) => write!(f, "Parse error: {}", err),
            LoadError::Format(ref err) => write!(f, "Format error: {}", err)
        }
    }
}

impl error::Error for LoadError {
    fn description(&self) -> &str {
        match *self {
            LoadError::Io(ref err) => err.description(),
            LoadError::Parse(ref err) => err.description(),
            LoadError::Format(ref err) => err.description()
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        match *self {
            LoadError::Io(ref err) => Some(err),
            LoadError::Parse(ref err) => Some(err),
            LoadError::Format(ref err) => Some(err)
        }
    }
}

impl From<io::Error> for LoadError {
    fn from(err: io::Error) -> LoadError {
        LoadError::Io(err)
    }
}

impl From<num::ParseIntError> for LoadError {
    fn from(err: num::ParseIntError) -> LoadError {
        LoadError::Parse(err)
    }
}

impl From<FormatError> for LoadError {
    fn from(err: FormatError) -> LoadError {
        LoadError::Format(err)
    }
}
