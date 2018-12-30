pub mod dictionary;
pub mod hint_table;
pub mod utils;

use self::dictionary::make_dict;
use self::hint_table::make_hint_table;
use self::utils::{load_new_data, load_new_data_drop};
use crate::Result;
use ez_io::WriteE;
use std::io::{Cursor, Seek, SeekFrom, Write};

const DICT_LEN: usize = 256;
const HINT_BITS: usize = 10;

/// In case the data is compressed, we need some extra info to decompress it.
#[derive(Clone)]
pub struct CompressedData {
    /// Starts where hdr_offset in PacData starts
    pub data: Vec<u8>,
    pub info: Vec<CompressInfo>,
    pub total_decompressed_size: usize,
}

#[derive(Clone)]
pub struct CompressInfo {
    pub offset: usize,
    pub decompressed_size: usize,
}

impl CompressedData {
    /// Decompresses data in .pac files.
    pub fn decompress(&self) -> Result<Vec<u8>> {
        // Create Cursor for Input
        let reader = &mut Cursor::new(&self.data);

        // Create vector for output
        let mut out = Vec::with_capacity(self.total_decompressed_size);

        // Create Cursor for output
        let writer = &mut Cursor::new(&mut out);

        // Init Dict and Hints
        let mut dict = [0u16; DICT_LEN * 2];
        let mut hints = [[0u16; 2]; 1 << HINT_BITS];

        // Process the data
        for info in &self.info {
            // Count how many bytes we wrote
            let mut written_bytes = 0usize;

            // Go to location specified by PacInfo
            reader.seek(SeekFrom::Start(
                info.offset as u64, // Lossy
            ))?;

            // Make the dict and values
            let mut pak_k = 0;
            let mut pak_m = 0;
            let dict_result = make_dict(&mut dict, &mut 256, &mut pak_m, &mut pak_k, reader);

            // Check if data is always the same value
            if dict_result > 255 {
                // Make the hints
                make_hint_table(&dict, &mut hints);

                loop {
                    // decode_rep
                    if pak_m < HINT_BITS as u32 {
                        load_new_data(reader, &mut pak_k, &mut pak_m)?;
                    }
                    // test_hint_bits
                    pak_m -= HINT_BITS as u32;
                    let hints_index = (pak_k >> (pak_m & 255)) & ((1 << HINT_BITS) - 1);
                    let mut read_value = u32::from(hints[hints_index as usize][0]);
                    pak_m += u32::from(hints[hints_index as usize][1]);
                    if read_value > 255 {
                        loop {
                            // search_ch_rep
                            if pak_m != 0 {
                                pak_m -= 1;
                            } else {
                                load_new_data_drop(reader, &mut pak_k, &mut pak_m)?;
                            }
                            // test_hbit
                            let bit_test = (pak_k >> (pak_m & 255)) & 1;
                            let index = 2 * read_value - 512 + bit_test;
                            read_value = u32::from(dict[index as usize]);
                            if read_value <= 255 {
                                break;
                            }
                        }
                    }
                    // put_ch
                    writer.write_to_u8(read_value as u8)?;
                    written_bytes += 1;
                    if written_bytes >= info.decompressed_size {
                        break;
                    }
                }
            } else {
                // This part of the data is the same byte repeated, write the output
                writer.write_all(&vec![dict_result as u8; info.decompressed_size])?;
            }
        }
        Ok(out)
    }
}
