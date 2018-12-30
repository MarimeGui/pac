use crate::error::PacError;
use crate::Result;
use ez_io::{ReadE, WriteE};
use std::io::{Read, Write};

#[derive(Clone)]
pub struct PacData {
    pub magic_number: [u8; 2],
    /// Number of PacInfo sections
    pub pack_cnt: u32,
    pub file_type: u32,
    /// Where the compressed data begins, relative to beginning of PacData (@ 0x1234)
    pub hdr_offset: u32,
    pub info: Vec<PacInfo>,
}

#[derive(Clone)]
pub struct PacInfo {
    pub unpack_size: u32,
    pub pack_size: u32,
    pub offset: u32,
}

impl PacData {
    pub fn import<R: Read>(reader: &mut R) -> Result<PacData> {
        let pack_cnt = reader.read_le_to_u32()?;
        Ok(PacData {
            magic_number: {
                let mut magic_number = [0u8; 2];
                reader.read_exact(&mut magic_number)?;
                if magic_number != [0x34, 0x12] {
                    return Err(PacError::MagicNumber);
                }
                magic_number
            },
            pack_cnt,
            file_type: reader.read_le_to_u32()?,
            hdr_offset: reader.read_le_to_u32()?,
            info: {
                let mut info = Vec::with_capacity(pack_cnt as usize); // Lossy
                for _ in 0..pack_cnt {
                    info.push(PacInfo::import(reader)?);
                }
                info
            },
        })
    }
    pub fn export<W: Write>(&self, writer: &mut W) -> Result<()> {
        writer.write_all(&self.magic_number)?;
        writer.write_le_to_u32(self.pack_cnt)?;
        writer.write_le_to_u32(self.file_type)?;
        writer.write_le_to_u32(self.hdr_offset)?;
        for i in &self.info {
            i.export(writer)?;
        }
        Ok(())
    }
}

impl PacInfo {
    pub fn import<R: Read>(reader: &mut R) -> Result<PacInfo> {
        Ok(PacInfo {
            unpack_size: reader.read_le_to_u32()?,
            pack_size: reader.read_le_to_u32()?,
            offset: reader.read_le_to_u32()?,
        })
    }
    pub fn export<W: Write>(&self, writer: &mut W) -> Result<()> {
        writer.write_le_to_u32(self.unpack_size)?;
        writer.write_le_to_u32(self.pack_size)?;
        writer.write_le_to_u32(self.offset)?;
        Ok(())
    }
}
