//! Contains basic interpretation of a Pac file, then re-used for higher interpretations

pub mod files;
pub mod header;

use self::files::PacFiles;
use self::header::PacHeader;
use std::io::{Read, Seek};
use crate::Result;

/// The Low-Level interpretation of PAC data
#[derive(Clone)]
pub struct DPac {
    pub header: PacHeader,
    pub files: PacFiles,
}

impl DPac {
    pub fn import<R: Read + Seek>(reader: &mut R) -> Result<DPac> {
        let header = PacHeader::import(reader)?;
        let files = PacFiles::import(reader, header.file_cnt)?;
        Ok(DPac {
            header,
            files,
        })
    }
}