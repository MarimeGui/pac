//! Contains errors in this crate

use std::io::Error as IOError;

/// The main error type used in this crate
pub enum PacError {
    /// IO Error
    IO(IOError),
    /// Expected a certain magic number, found something else
    MagicNumber,
    /// A value in the "packing_flag" field in DPacFile is not recognized.
    UnknownPackingType(u32),
}

impl From<IOError> for PacError {
    fn from(e: IOError) -> PacError {
        PacError::IO(e)
    }
}
