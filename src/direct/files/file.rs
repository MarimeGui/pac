use crate::Result;
use ez_io::{ReadE, WriteE};
use std::io::{Read, Write};

/// Represents the information stored about a file contained in a PAC file
#[derive(Clone)]
pub struct PacFile {
    pub unk1: u32,
    pub file_index: u16,
    pub unk2: u16,
    pub path: [u8; 264],
    pub pack_size: u32,
    pub unpack_size: u32,
    pub packing_flag: u32,
    pub file_offset: u32,
}

impl PacFile {
    pub fn import<R: Read>(reader: &mut R) -> Result<PacFile> {
        Ok(PacFile {
            unk1: reader.read_le_to_u32()?,
            file_index: reader.read_le_to_u16()?,
            unk2: reader.read_le_to_u16()?,
            path: {
                let mut path = [0u8; 264];
                reader.read_exact(&mut path)?;
                path
            },
            pack_size: reader.read_le_to_u32()?,
            unpack_size: reader.read_le_to_u32()?,
            packing_flag: reader.read_le_to_u32()?,
            file_offset: reader.read_le_to_u32()?,
        })
    }
    pub fn export<W: Write>(&self, writer: &mut W) -> Result<()> {
        writer.write_le_to_u32(self.unk1)?;
        writer.write_le_to_u16(self.file_index)?;
        writer.write_le_to_u16(self.unk2)?;
        writer.write_all(&self.path)?;
        writer.write_le_to_u32(self.pack_size)?;
        writer.write_le_to_u32(self.unpack_size)?;
        writer.write_le_to_u32(self.packing_flag)?;
        writer.write_le_to_u32(self.file_offset)?;
        Ok(())
    }
}
