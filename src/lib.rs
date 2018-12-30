extern crate ez_io;

pub mod decompression;
pub mod direct;
pub mod error;
pub mod file;

use crate::decompression::{CompressInfo, CompressedData};
use crate::direct::files::Packing;
use crate::direct::DPac;
use crate::file::DataType;
use crate::file::File;

/// Result type that ties to general Error used in this crate
pub type Result<T> = ::std::result::Result<T, error::PacError>;

/// Main Pac type
#[derive(Clone)]
pub struct Pac {
    /// Contains general info about the file
    pub files: Vec<File>,
}

// Would be nice to implement some kind of iterator so that it does not take 2x the size of th entire pac file

impl Pac {
    pub fn from_direct(direct: DPac) -> Result<Pac> {
        let nb_files = direct.header.file_cnt as usize;
        let mut files = Vec::with_capacity(nb_files);
        for file_index in 0..nb_files {
            // Get path as a UTF-8 String
            let path = {
                let mut first_zero = None;
                for (i, character) in direct.files.files_info[file_index].path.iter().enumerate() {
                    if *character == 0 {
                        first_zero = Some(i);
                    }
                }
                let len = match first_zero {
                    None => 264,
                    Some(i) => i,
                };
                String::from_utf8(direct.files.files_info[file_index].path[0..len].to_vec())?
            };
            // Get correct flavour of data (compressed or not)
            let packed_data = match &direct.files.files_packing[file_index] {
                Packing::Direct => {
                    DataType::Uncompressed(direct.files.files_data[file_index].clone()) // That is going to be a lot of data
                }
                Packing::Compressed(c) => {
                    DataType::Compressed(CompressedData {
                        data: direct.files.files_data[file_index].clone(), // Here too
                        info: {
                            let mut info = Vec::with_capacity(c.info.len());
                            for direct_info in &c.info {
                                info.push(CompressInfo {
                                    offset: direct_info.offset as usize,                 // Lossy
                                    decompressed_size: direct_info.unpack_size as usize, // Lossy
                                });
                            }
                            info
                        },
                        total_decompressed_size: direct.files.files_info[file_index].unpack_size
                            as usize, // Lossy
                    })
                }
            };
            files.push(File { path, packed_data });
        }
        Ok(Pac { files })
    }
}
