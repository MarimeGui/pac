use crate::Result;

/// A File contained inside of the PAC file
#[derive(Clone)]
pub struct PacFile {
    /// Relative Path of this File
    pub path: String,
    /// Data for the file
    pub packed_data: PacDataType,
}

/// Data stored inside the PAC File for an included file. In most cases, the data will be compressed.
#[derive(Clone)]
pub enum PacDataType {
    Uncompressed(Vec<u8>),
    Compressed(Vec<u8>),
}

impl PacDataType {
    pub fn get_data(&self) -> Result<Vec<u8>> {
        match self {
            PacDataType::Uncompressed(u) => Ok(u.clone()),
            PacDataType::Compressed(c) => decompress_data(c),
        }
    }
}

pub fn decompress_data(_input: &[u8]) -> Result<Vec<u8>> {
    unimplemented!();
}
