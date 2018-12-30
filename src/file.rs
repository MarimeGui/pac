use crate::decompression::CompressedData;
use crate::Result;

/// A File contained inside of the PAC file
#[derive(Clone)]
pub struct File {
    /// Relative Path of this File
    pub path: String,
    /// Data for the file
    pub packed_data: DataType,
}

/// Data stored inside the PAC File for an included file.
#[derive(Clone)]
pub enum DataType {
    /// The data is not compressed, it is directly readable
    Uncompressed(Vec<u8>),
    /// The data is compressed, it must be decompressed
    Compressed(CompressedData),
}

impl DataType {
    pub fn get_data(&self) -> Result<Vec<u8>> {
        match self {
            DataType::Uncompressed(u) => Ok(u.clone()),
            DataType::Compressed(c) => c.decompress(),
        }
    }
}
