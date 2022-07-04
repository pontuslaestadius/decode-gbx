extern crate xml;

use crate::read::*;
use std::io::Read;

use xml::reader::{EventReader, XmlEvent};

#[derive(Debug)]
pub enum ByteFormat {
    BINARY,
    TEXT,
}
#[derive(Debug)]
pub enum ByteCompression {
    UNCOMPRESSED,
    COMPRESSED,
}

#[derive(Debug)]
pub struct EntryHeader {
    pub id: u32,
    pub size: u32,
    pub heavy: bool,
    pub chunk_fmt: ChunkFmt,
}

#[derive(Debug)]
pub struct Header {
    pub entries: Vec<EntryHeader>,
    pub body_compression: ByteCompression,
    pub binary_or_text: ByteFormat,
}

#[derive(Debug)]
pub enum ChunkFmt {
    XML,
    AT_SV, // values seperated with '@'
    UNKNOWN,
}

#[derive(Debug)]
enum XMLDocType {
    REPLAY,
}

struct XMLHeader {
    r#type: String,
    exever: String,
    exebuild: String,
    title: String
}
pub fn parse_header<T: Read>(file: &mut T) -> Result<Header, Box<dyn std::error::Error + 'static>> {
    read_gbx_or_panic(file)?;

    let mut header = Header {
        entries: Vec::new(),
        body_compression: ByteCompression::UNCOMPRESSED,
        binary_or_text: ByteFormat::BINARY,
    };

    let version: u16 = read_u16(file)?;
    println!("Version: {}", version);
    if version >= 3 {
        let mut buffer: [u8; 3] = [0; 3];
        file.read(&mut buffer)?;
        if version < 6 {
            // byte format: 'B' or 'T': Binary or Text (always B for version 6 gbx files)
        }
        // byte compression: 'U' or 'C': Uncompressed or Compressed reference table (unused, always U)

        header.body_compression = match buffer[2] {
            67 => ByteCompression::COMPRESSED,
            85 => ByteCompression::UNCOMPRESSED,
            _ => panic!("Unsupported compression type"),
        };
        // byte compression: 'U' or 'C': Uncompressed or Compressed body (typically C for binary files)
        if version >= 4 {
            let mut _buffer: [u8; 1] = [0; 1];
            file.read(&mut _buffer)?;
            //byte unknown: 'R' or 'E': unknown purpose (typically R for binary files)
            // uint32 classID (class ID of main class instance)
            let class_id: u32 = read_u32(file)?;
            println!("class ID: {}", class_id);

            if version >= 6 {
                let user_data_size: u32 = read_u32(file)?;
                let num_header_chunks: u32 = read_u32(file)?;
                println!(
                    "userDataSize: {}, Header Chunks: {}",
                    user_data_size, num_header_chunks
                );

                header.entries = Vec::with_capacity(num_header_chunks as usize);

                for _ in 0..num_header_chunks {
                    let chunk_id = read_u32(file)?;
                    let chunk_size = read_u32(file)?;
                    // Magic bitwise operations, credits goes to:
                    // https://github.com/ThaumicTom/gbx.js/blob/master/src/gbx.js#L80
                    header.entries.push(EntryHeader {
                        heavy: (chunk_size & (1 << 31)) != 0,
                        size: chunk_size & !0x80000000,
                        id: chunk_id,
                        chunk_fmt: ChunkFmt::UNKNOWN,
                    })
                }
                // concatenated data of header chunks
                for (i, entry) in header.entries.iter().enumerate() {
                    println!("Entry: {:?}", entry);
                    let bytes = read_fixed_vec(file, entry.size as usize)?;
                    // Remove control sequence characters from the string.
                    let filtered_bytes = bytes
                        .iter()
                        .map(|x| *x)
                        .filter(|x| *x > 31 && *x < 127)
                        .collect::<Vec<u8>>();

                    if filtered_bytes.len() == 0 {
                        continue;
                    }

                    match &filtered_bytes[0] {
                        &60u8 => {
                            // header.entries[i].chunk_fmt = ChunkFmt::XML;
                            // XML, "<" checks for opening XML tag.
                            println!("XX Entry Type: XML");
                            print_xml(filtered_bytes.as_slice());
                            byte_print(filtered_bytes);
                        }
                        &64u8 => {
                            // "@" seperated values.
                            println!("## Entry Type: @SV");
                            let split = filtered_bytes.split(|x| x == &64u8);
                            for item in split {
                                byte_print(item.to_vec())
                            }
                        }
                        _ => {
                            byte_print(filtered_bytes);
                        }
                    };
                }
            }
        }
    }

    Ok(header)
}

fn print_xml<T: Read>(item: T) {
    let parser = EventReader::new(item);
    let mut depth = 0;
    for e in parser {
        match e {
            Ok(XmlEvent::StartElement {
                name, attributes, ..
            }) => {
                println!("{}+{}", indent(depth), name);

                match name.local_name.as_str() {
                    "header" => {
                        let fields = XMLHeader::field_names();
                        println!("Fields: {:?}", fields);
                        for attr in attributes {
                            if fields.contains(&attr.name.local_name.as_str()) {
                                println!(
                                    "atts: {:?}, value: {:?}",
                                    attr.name.local_name, attr.value
                                );
                            } else {
                                println!("Unknown Property {}", attr.name.local_name);
                            }
                        }
                    }
                    _ => {}
                }
                depth += 1;
            }
            Ok(XmlEvent::EndElement { name }) => {
                depth -= 1;
                println!("{}-{}", indent(depth), name);
            }
            Err(e) => {
                println!("Error: {}", e);
                break;
            }
            _ => {}
        }
    }
}

fn indent(size: usize) -> String {
    const INDENT: &'static str = "    ";
    (0..size)
        .map(|_| INDENT)
        .fold(String::with_capacity(size * INDENT.len()), |r, s| r + s)
}

fn byte_print(bytes: Vec<u8>) {
    let s = String::from_utf8(bytes).expect("Found invalid UTF-8");
    println!("{}", s);
}
