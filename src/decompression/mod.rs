pub mod hint_table;
pub mod dictionary;
pub mod utils;

use std::io::{Read, Seek, SeekFrom, Write};
use crate::Result;
use ez_io::WriteE;
use self::dictionary::make_dict;
use self::hint_table::make_hint_table;
use self::utils::{load_new_data, load_new_data_drop};
use crate::direct::files::c_packing::{PacData, PacInfo};
use crate::error::PacError;

const DICT_LEN: usize = 256;
const HINT_BITS: usize = 10;

pub fn decompress<R: Read + Seek, W: Write>(reader: &mut R, writer: &mut W) -> Result<()> {
    // Get pos of data start
    let data_start_offset = reader.seek(SeekFrom::Current(0))?;

    // Read PacData
    let pac_data = PacData::import(reader)?;

    // Read all PacInfos
    let mut pac_info = Vec::with_capacity(pac_data.pack_cnt as usize);
    for _ in 0..pac_data.pack_cnt {
        pac_info.push(PacInfo::import(reader)?);
    }

    // Calculate absolute offset for compressed data
    let compressed_binary_offset = data_start_offset + u64::from(pac_data.hdr_offset);

    // Check if hdr_info corresponds
    if reader.seek(SeekFrom::Current(0))? != compressed_binary_offset {
        return Err(PacError::WrongHdrOffset)
    }

    // Init Dict and Hints
    let mut dict = [0u16; DICT_LEN * 2];
    let mut hints = [[0u16; 2]; 1 << HINT_BITS];

    // Process the data
    for info in pac_info {
        // Count how many bytes we wrote
        let mut written_bytes = 0u32;

        // Go to location specified by PacInfo
        reader.seek(SeekFrom::Start(
            compressed_binary_offset + u64::from(info.offset),
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
                        let index = 2*read_value - 512 + bit_test;
                        read_value = u32::from(dict[index as usize]);
                        if read_value <= 255 {
                            break;
                        }
                    }
                }
                // put_ch
                writer.write_to_u8(read_value as u8)?;
                written_bytes += 1;
                if written_bytes >= info.unpack_size {
                    break;
                }
            }
        } else {
            // This part of the data is the same byte repeated, write the output
            writer.write_all(&vec![dict_result as u8; info.unpack_size as usize])?;
        }
    }
    Ok(())
}