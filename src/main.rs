mod args;
mod chunk;
mod chunk_type;
mod commands;
mod png;

use anyhow::Result;
use args::PngMeArgs;
use clap::Parser;
use commands::{decode, encode, print_chunks, remove};

fn main() -> Result<()> {
    let args = PngMeArgs::parse();

    match args {
        PngMeArgs::Encode(args) => encode(args)?,
        PngMeArgs::Decode(args) => decode(args)?,
        PngMeArgs::Remove(args) => remove(args)?,
        PngMeArgs::Print(args) => print_chunks(args)?,
    }

    Ok(())
}
