use std::fmt;

#[derive(Debug)]
pub enum Error {
    InvalidColorFile,
    UnparseableColor,
    InvalidImageFile,
}

impl Error {
    pub const fn message(&self) -> &str {
        match self {
            Error::InvalidColorFile => "Failed to read color file passed to --color-file",
            Error::UnparseableColor => "Failed to parse RGB string on line",
            Error::InvalidImageFile => "Failed to read image file passed to --image-file",
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.message())
    }
}

pub type Result<T> = std::result::Result<T, Error>;

pub const INVALID_COLOR_FILE: &str = Error::InvalidColorFile.message();
pub const UNPARSEBALE_COLOR: &str = Error::UnparseableColor.message();
pub const INVALID_IMAGE_FILE: &str = Error::InvalidImageFile.message();
