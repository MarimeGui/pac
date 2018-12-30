pub mod c_packing;
pub mod file;

use self::c_packing::PacCompressedPacking;
use self::file::PacFile;
use crate::error::PacError;
use crate::Result;
use std::io::{Read, Seek, SeekFrom, Write};

/// Files in a PAC File. These Vecs should be indexed together
#[derive(Clone)]
pub struct PacFiles {
    pub files_info: Vec<PacFile>,
    pub files_packing: Vec<Packing>,
    pub files_data: Vec<Vec<u8>>,
}

#[derive(Clone)]
pub enum Packing {
    Direct,
    Compressed(PacCompressedPacking),
}

impl PacFiles {
    pub fn import<R: Read + Seek>(reader: &mut R, file_cnt: u32) -> Result<PacFiles> {
        // Create Vecs for storing all info
        let mut files_info = Vec::with_capacity(file_cnt as usize);
        let mut files_packing = Vec::with_capacity(file_cnt as usize);
        let mut files_data = Vec::with_capacity(file_cnt as usize);

        // Read PacFiles at the beginning of the file
        for _ in 0..file_cnt {
            files_info.push(PacFile::import(reader)?);
        }

        let second_start = reader.seek(SeekFrom::Current(0))?;

        // For every file contained in the Pac File
        for info in &files_info {
            // Seek to the start of data in second part of pac file
            let current_start = second_start + u64::from(info.file_offset);
            reader.seek(SeekFrom::Start(current_start))?;

            // Read the Packing if any
            let packing = match info.packing_flag {
                0 => Packing::Direct,
                1 => Packing::Compressed(PacCompressedPacking::import(reader)?),
                x => return Err(PacError::UnknownPackingType(x)),
            };

            // If it uses Compressed Packing, seek to compressed data
            match &packing {
                Packing::Direct => {}
                Packing::Compressed(c) => {
                    reader.seek(SeekFrom::Start(
                        current_start + u64::from(c.data.hdr_offset),
                    ))?;
                }
            }

            // Read all compressed data to buffer
            let mut data = vec![0u8; info.pack_size as usize]; // Lossy
            reader.read_exact(&mut data)?;
            files_packing.push(packing);
            files_data.push(data);
        }
        Ok(PacFiles {
            files_info,
            files_packing,
            files_data,
        })
    }
    pub fn export<W: Write + Seek>(&self, writer: &mut W) -> Result<()> {
        unimplemented!();
    }
}
