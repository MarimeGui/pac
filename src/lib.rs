extern crate ez_io;

pub mod direct;
pub mod decompression;
pub mod error;
pub mod file;

// use crate::direct::DPac;
use crate::file::File;

/// Result type that ties to general Error used in this crate
pub type Result<T> = ::std::result::Result<T, error::PacError>;

/// Main Pac type
#[derive(Clone)]
pub struct Pac {
    /// Contains general info about the file
    pub files: Vec<File>,
}

// impl From<DPac> for Pac {
//     fn from(direct: DPac) -> Pac {
//         Pac {
            
//         }
//     }
// }

// impl Into<DPac> for Pac {
//     fn into(self) -> DPac {
//         DPac {
//             header: self.header.into(),
//         }
//     }
// }
