use std::fs::File;
use std::io::Write;
use std::str::FromStr;

use crate::args::{DecodeArgs, EncodeArgs, PrintArgs, RemoveArgs};
use crate::chunk::Chunk;
use crate::chunk_type::ChunkType;
use crate::png::Png;
use anyhow::Result;

/// Encodes a message into a PNG file and saves the result
pub fn encode(args: EncodeArgs) -> Result<()> {
    let mut the_png = Png::from_file(args.file_path)?;

    let data = args.message.into_bytes();
    let chunk_type = match ChunkType::from_str(args.chunk_type.as_str()) {
        Ok(chunk) => chunk,
        Err(err) => return Err(anyhow::anyhow!(err)),
    };

    let chunk = Chunk::new(chunk_type, data);

    the_png.append_chunk(chunk);

    match args.output_file {
        Some(file) => {
            let mut f = File::create(file)?;
            f.write_all(&the_png.as_bytes())?;
            Ok(())
        }
        None => Ok(()),
    }
}

/// Searches for a message hidden in a PNG file and prints the message if one is found
pub fn decode(args: DecodeArgs) -> Result<()> {
    let the_png = Png::from_file(args.file_path)?;

    match the_png.chunk_by_type(args.chunk_type.as_str()) {
        Some(chunk) => {
            let data_str = chunk.data_as_string()?;
            print!("{}", data_str);
        }
        None => eprint!("Your Chunk Type does not exist"),
    }
    Ok(())
}

/// Removes a chunk from a PNG file and saves the result
pub fn remove(args: RemoveArgs) -> Result<()> {
    let file_path = args.file_path;
    let mut the_png = Png::from_file(file_path.clone())?;

    match the_png.chunk_by_type(args.chunk_type.as_str()) {
        Some(chunk) => {
            let chunk_type = chunk.chunk_type();
            let chunk_type = chunk_type.to_string();
            the_png.remove_chunk(chunk_type.as_str())?;
            let mut f = File::create(file_path)?;
            f.write_all(&the_png.as_bytes())?;
        }
        None => eprint!("Your Chunk Type does not exist"),
    }
    Ok(())
}

/// Prints all of the chunks in a PNG file
pub fn print_chunks(args: PrintArgs) -> Result<()> {
    let the_png = Png::from_file(args.file_path)?;
    println!("{the_png}");
    Ok(())
}
