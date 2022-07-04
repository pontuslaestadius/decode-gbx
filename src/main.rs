use std::env;
use std::fs::File;
use std::io::Read;

mod body;
mod header;
mod read;

use body::parse_body;
use header::{parse_header};
use read::*;

/// Implementation docs:
/// https://wiki.xaseco.org/wiki/GBX

fn parse_ref_table<T: Read>(file: &mut T) -> Result<(), Box<dyn std::error::Error + 'static>> {
    let num_external_nodes: u32 = read_u32(file)?;
    println!("numExternalNodes: {:?}", num_external_nodes);
    if num_external_nodes > 0 {
        panic!("Not implemented external Nodes");
    }
    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error + 'static>> {
    let args: Vec<String> = env::args().collect();
    let mut file = File::open(args[1].clone())?;

    println!("##### HEADER #####");

    let header = parse_header(&mut file)?;

    {
        // uint32 numNodes
        let num_nodes: u32 = read_u32(&mut file)?;
        println!("numNodes: {}", num_nodes);
    }

    println!("### REF. TABLE ###");

    // ### Refence Table
    parse_ref_table(&mut file)?;

    println!("#####  BODY  #####");

    parse_body(&mut file, header.body_compression)?;

    Ok(())
}
