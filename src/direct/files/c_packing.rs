use crate::error::PacError;
use crate::Result;
use ez_io::{ReadE, WriteE};
use std::io::{Read, Seek, SeekFrom, Write};

#[derive(Clone)]
pub struct PacCompressedPacking {
    pub data: PacData,
    pub info: Vec<PacInfo>,
}

#[derive(Clone)]
pub struct PacData {
    pub magic_number: [u8; 4],
    /// Number of PacInfo sections
    pub pack_cnt: u32,
    pub file_type: u32,
    /// Where the compressed data begins, relative to beginning of PacData (@ 0x1234)
    pub hdr_offset: u32,
}

#[derive(Clone)]
pub struct PacInfo {
    pub unpack_size: u32,
    pub pack_size: u32,
    pub offset: u32,
}

impl PacCompressedPacking {
    pub fn import<R: Read>(reader: &mut R) -> Result<PacCompressedPacking> {
        let data = PacData::import(reader)?;
        let mut info = Vec::with_capacity(data.pack_cnt as usize);
        for _ in 0..data.pack_cnt {
            info.push(PacInfo::import(reader)?);
        }
        Ok(PacCompressedPacking { data, info })
    }
    pub fn export<W: Write + Seek>(&self, writer: &mut W) -> Result<()> {
        self.data.export(writer)?;
        for info in &self.info {
            info.export(writer)?;
        }
        Ok(())
    }
}

impl PacData {
    pub fn import<R: Read>(reader: &mut R) -> Result<PacData> {
        let mut magic_number = [0u8; 4];
        reader.read_exact(&mut magic_number)?;
        if magic_number != [0x34, 0x12, 0, 0] {
            return Err(PacError::MagicNumber);
        }
        let pack_cnt = reader.read_le_to_u32()?;
        let file_type = reader.read_le_to_u32()?;
        let hdr_offset = reader.read_le_to_u32()?;
        Ok(PacData {
            magic_number,
            pack_cnt,
            file_type,
            hdr_offset,
        })
    }
    pub fn export<W: Write + Seek>(&self, writer: &mut W) -> Result<()> {
        writer.write_all(&self.magic_number)?;
        writer.write_le_to_u32(self.pack_cnt)?;
        writer.write_le_to_u32(self.file_type)?;
        writer.write_le_to_u32(self.hdr_offset)?;
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
