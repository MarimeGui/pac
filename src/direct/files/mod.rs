pub mod c_packing;
pub mod file;

use self::c_packing::PacData;
use self::file::PacFile;
use crate::error::PacError;
use crate::Result;
use std::io::{Read, Seek, SeekFrom, Write};

/// Files in a PAC File. "files_info" and "files_packed" should have the same length and should be indexed together.
#[derive(Clone)]
pub struct PacFiles {
    pub files_info: Vec<PacFile>,
    pub files_packed: Vec<PackingType>,
}

#[derive(Clone)]
pub enum PackingType {
    Direct(Vec<u8>),
    Packed(PacData),
}

impl PacFiles {
    pub fn import<R: Read + Seek>(reader: &mut R, file_cnt: u32) -> Result<PacFiles> {
        let mut files_info = Vec::with_capacity(file_cnt as usize); // Lossy
        let mut files_packed = Vec::with_capacity(file_cnt as usize); // Lossy
        for _ in 0..file_cnt {
            files_info.push(PacFile::import(reader)?);
        }
        let base_offset = reader.seek(SeekFrom::Current(0))?;
        for file_info in &files_info {
            let data_offset = u64::from(file_info.file_offset) + base_offset;
            reader.seek(SeekFrom::Start(data_offset))?;
            match file_info.packing_flag {
                0 => {
                    let mut data = vec![0u8; file_info.pack_size as usize]; // Lossy
                    reader.read_exact(&mut data)?;
                    files_packed.push(PackingType::Direct(data));
                }
                1 => {
                    files_packed.push(PackingType::Packed(PacData::import(reader)?));
                }
                x => return Err(PacError::UnknownPackingType(x)),
            }
        }
        Ok(PacFiles {
            files_info,
            files_packed,
        })
    }
    pub fn export<W: Write>(&self, writer: &mut W) -> Result<()> {
        for file_info in &self.files_info {
            file_info.export(writer)?;
        }
        for file_data in &self.files_packed {
            match file_data {
                PackingType::Direct(d) => writer.write_all(&d)?,
                PackingType::Packed(p) => p.export(writer)?,
            }
        }
        Ok(())
    }
}
