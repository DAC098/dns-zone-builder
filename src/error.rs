use std::{fmt};
use std::convert::{From};
use std::ffi::{OsString};

pub type Result<T> = std::result::Result<T, RuntimeError>;

pub enum RuntimeError {
    Error(String),
    InvalidFile(OsString),
    FileNotFound(String),

    UnknownFileExtension,
    InvalidFileExtension(OsString),

    JsonError(serde_json::Error),
    YamlError(serde_yaml::Error),

    IoError(std::io::Error)
}

impl RuntimeError {

    pub fn get_msg(&self) -> String {
        match &*self {
            RuntimeError::Error(msg) => msg.clone(),
            RuntimeError::InvalidFile(arg) => format!("configuration file is not a file: {:?}", arg),
            RuntimeError::FileNotFound(file) => format!("file not found: {}", file),
            RuntimeError::UnknownFileExtension => format!("unknown file extension given"),
            RuntimeError::InvalidFileExtension(ext) => format!("invalid file extension given. {:?}", ext),
            RuntimeError::JsonError(err) => {
                match err.classify() {
                    serde_json::error::Category::Io => format!(
                        "json io error"
                    ),
                    serde_json::error::Category::Syntax => format!(
                        "json syntax error {}:{}", err.line(), err.column()
                    ),
                    serde_json::error::Category::Data => format!(
                        "json data error"
                    ),
                    serde_json::error::Category::Eof => format!(
                        "json eof error"
                    )
                }
            },
            RuntimeError::YamlError(err) => {
                if let Some(location) = err.location() {
                    format!("yaml error {}:{} {:?}", location.line(), location.column(), err)
                } else {
                    format!("yaml error {:?}", err)
                }
            },
            RuntimeError::IoError(err) => format!("{:?}", err)
        }
    }
    
    pub fn get_code(&self) -> i32 {
        match &*self {
            RuntimeError::Error(_) => 1,
            RuntimeError::InvalidFile(_) => 1,
            RuntimeError::FileNotFound(_) => 1,
            RuntimeError::UnknownFileExtension => 1,
            RuntimeError::InvalidFileExtension(_) => 1,
            RuntimeError::JsonError(_) => 1,
            RuntimeError::YamlError(_) => 1,
            RuntimeError::IoError(_) => 1
        }
    }
}

impl fmt::Display for RuntimeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.get_msg())
    }
}

impl From<serde_json::Error> for RuntimeError {
    fn from(error: serde_json::Error) -> Self {
        RuntimeError::JsonError(error)
    }
}

impl From<serde_yaml::Error> for RuntimeError {
    fn from(error: serde_yaml::Error) -> Self {
        RuntimeError::YamlError(error)
    }
}

impl From<std::io::Error> for RuntimeError {
    fn from(error: std::io::Error) -> Self {
        RuntimeError::IoError(error)
    }
}

impl From<std::fmt::Error> for RuntimeError {
    fn from(error: std::fmt::Error) -> Self {
        RuntimeError::Error(format!("{:?}", error))
    }
}