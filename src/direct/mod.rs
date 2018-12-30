//! Contains basic interpretation of a Pac file, then re-used for higher interpretations

pub mod files;
pub mod header;

use self::files::PacFiles;
use self::header::PacHeader;

/// The Low-Level interpretation of PAC data
#[derive(Clone)]
pub struct DPac {
    pub header: PacHeader,
    pub files: PacFiles,
}
