use crate::Result;
use crate::decompression::decompress;
use std::io::{Cursor};

/// A File contained inside of the PAC file
#[derive(Clone)]
pub struct File {
    /// Relative Path of this File
    pub path: String,
    /// Data for the file
    pub packed_data: DataType,
}

/// Data stored inside the PAC File for an included file. In most cases, the data will be compressed.
#[derive(Clone)]
pub enum DataType {
    Uncompressed(Vec<u8>),
    Compressed(Vec<u8>),
}

impl DataType {
    pub fn get_data(&self) -> Result<Vec<u8>> {
        match self {
            DataType::Uncompressed(u) => Ok(u.clone()),
            DataType::Compressed(c) => decompress_data(c),
        }
    }
}

pub fn decompress_data(input: &[u8]) -> Result<Vec<u8>> {
    let mut reader = Cursor::new(input);
    let mut out = Vec::new();
    let mut writer = Cursor::new(&mut out); // Could be more efficient if used with_capacity instead
    decompress(&mut reader, &mut writer)?;
    Ok(out)
}
