//! Contains basic interpretation of a Pac file, then re-used for higher interpretations

pub mod files;
pub mod header;

use self::files::DPacFiles;
use self::header::DPacHeader;

/// The Low-Level interpretation of PAC data
#[derive(Clone)]
pub struct DPac {
    pub header: DPacHeader,
    pub files: DPacFiles,
}
