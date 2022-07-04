use crate::header::ByteCompression;
use crate::read::*;
use lzokay::decompress::decompress;
use std::io::Read;

pub fn parse_body<T: Read>(
    file: &mut T,
    body_compression: ByteCompression,
) -> Result<(), Box<dyn std::error::Error + 'static>> {
    println!("Body is {:?}", body_compression);
    match body_compression {
        ByteCompression::UNCOMPRESSED => {
            // TODO
        }
        ByteCompression::COMPRESSED => {
            loop {
                match read_compressed_chunk(file)? {
                    Some(chunk) => {
                        // loop {
                        let slice = &chunk[0..4].to_vec();
                        let body_chunk_id = sum_bytes_u32(&slice);

                        if body_chunk_id == 4207599105 {
                            // 0xFACADE01
                            break;
                        }
                        println!("Slice: {:?}", body_chunk_id);
                        // }

                        // dest_vec.drain(0..4);
                    }
                    None => break,
                }
            }
        }
    }
    Ok(())
}

pub fn read_compressed_chunk<T: Read>(
    file: &mut T,
) -> Result<Option<Vec<u8>>, Box<dyn std::error::Error + 'static>> {
    let uncompressed_size: u32 = read_u32(file)?;
    let compressed_size: u32 = read_u32(file)?;
    // If there is nothing to read, just end.
    if uncompressed_size + compressed_size == 0 {
        return Ok(None);
    }
    println!(
        "Body Chunk [compressed: {}, uncompressed: {}]",
        compressed_size, uncompressed_size
    );

    let source_vec = read_fixed_vec(file, compressed_size as usize)?;
    let mut dest_vec: Vec<u8> = fixed_vec_len(uncompressed_size as usize);

    match decompress(&source_vec, &mut dest_vec) {
        Ok(size) => {
            if uncompressed_size != size as u32 {
                panic!("Body chunk decompressed size did not match uncompressed size hint.");
            }
        }
        Err(e) => {
            panic!("Failed to decompress: {:?}", e);
        }
    }
    Ok(Some(dest_vec))
}
