//! Contains errors in this crate

use std::io::Error as IOError;
use std::string::FromUtf8Error;

/// The main error type used in this crate
pub enum PacError {
    /// IO Error
    IO(IOError),
    /// Expected a certain magic number, found something else
    MagicNumber,
    /// A value in the "packing_flag" field in DPacFile is not recognized.
    UnknownPackingType(u32),
    FromUtf8(FromUtf8Error),
}

impl From<IOError> for PacError {
    fn from(e: IOError) -> PacError {
        PacError::IO(e)
    }
}

impl From<FromUtf8Error> for PacError {
    fn from(e: FromUtf8Error) -> PacError {
        PacError::FromUtf8(e)
    }
}
