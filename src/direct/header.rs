//! Low-level reading of the Header foudn in PAC files.

use crate::error::PacError;
use crate::Result;
use ez_io::{ReadE, WriteE};
use std::io::{Read, Write};

/// Direct representation of the data found in the header
#[derive(Clone)]
pub struct PacHeader {
    pub magic_number: [u8; 8], // 'DW_PACK\0'
    pub file_pos: u32,         // Always 0
    pub file_cnt: u32,
    pub status: u32, // Always 0
}

impl PacHeader {
    /// Reads the data from a Read object
    pub fn import<R: Read>(reader: &mut R) -> Result<PacHeader> {
        Ok(PacHeader {
            magic_number: {
                let mut magic_number = [0u8; 8];
                reader.read_exact(&mut magic_number)?;
                if magic_number != [b'D', b'W', b'_', b'P', b'A', b'C', b'K', 0] {
                    return Err(PacError::MagicNumber);
                }
                magic_number
            },
            file_pos: reader.read_le_to_u32()?,
            file_cnt: reader.read_le_to_u32()?,
            status: reader.read_le_to_u32()?,
        })
    }
    /// Writes the data to a Write object
    pub fn export<W: Write>(&self, writer: &mut W) -> Result<()> {
        writer.write_all(&self.magic_number)?;
        writer.write_le_to_u32(self.file_pos)?;
        writer.write_le_to_u32(self.file_cnt)?;
        writer.write_le_to_u32(self.status)?;
        Ok(())
    }
}
