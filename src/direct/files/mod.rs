pub mod data;
pub mod file;

use self::data::DPacData;
use self::file::DPacFile;
use crate::error::PacError;
use crate::Result;
use std::io::{Read, Seek, SeekFrom, Write};

/// Files in a PAC File. "files_info" and "files_data" should have the same length and should be indexed together.
#[derive(Clone)]
pub struct DPacFiles {
    pub files_info: Vec<DPacFile>,
    pub files_data: Vec<PackingType>,
}

#[derive(Clone)]
pub enum PackingType {
    Direct(Vec<u8>),
    Packed(DPacData),
}

impl DPacFiles {
    pub fn import<R: Read + Seek>(reader: &mut R, file_cnt: u32) -> Result<DPacFiles> {
        let mut files_info = Vec::with_capacity(file_cnt as usize); // Lossy
        let mut files_data = Vec::with_capacity(file_cnt as usize); // Lossy
        for _ in 0..file_cnt {
            files_info.push(DPacFile::import(reader)?);
        }
        let base_offset = reader.seek(SeekFrom::Current(0))?;
        for file_info in &files_info {
            let data_offset = u64::from(file_info.file_offset) + base_offset;
            reader.seek(SeekFrom::Start(data_offset))?;
            match file_info.packing_flag {
                0 => {
                    let mut data = vec![0u8; file_info.pack_size as usize]; // Lossy
                    reader.read_exact(&mut data)?;
                    files_data.push(PackingType::Direct(data));
                }
                1 => {
                    files_data.push(PackingType::Packed(DPacData::import(reader)?));
                }
                x => return Err(PacError::UnknownPackingType(x)),
            }
        }
        Ok(DPacFiles {
            files_info,
            files_data,
        })
    }
    pub fn export<W: Write>(&self, writer: &mut W) -> Result<()> {
        for file_info in &self.files_info {
            file_info.export(writer)?;
        }
        for file_data in &self.files_data {
            match file_data {
                PackingType::Direct(d) => writer.write_all(&d)?,
                PackingType::Packed(p) => p.export(writer)?,
            }
        }
        Ok(())
    }
}
