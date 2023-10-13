use std::error::{Error};
use std::fmt;

#[derive(Debug, Copy, Clone)]
pub enum LibError {
    LoadDLLError,
    PathDLLError,
    DLLSymbolNotFoundError,
    GenericError
}

impl fmt::Display for LibError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            LibError::LoadDLLError => write!(f,"Error opening DLL file..."),
            LibError::PathDLLError =>  write!(f,"Error reading DLL Path as Environment Variable..."),
            LibError::DLLSymbolNotFoundError => write!(f,"Error loading DLL Symbol..."),
            LibError::GenericError => write!(f,"Generic Error..."),
        }
    }
}

impl Error for LibError {
    fn description(&self) -> &str {
        match self {
            LibError::LoadDLLError => "Error opening DLL file",
            LibError::PathDLLError =>  "Error reading DLL Path as Environment Variable",
            LibError::DLLSymbolNotFoundError => "Error loading DLL Symbol",
            LibError::GenericError => "Generic Error",
        }
    }
}