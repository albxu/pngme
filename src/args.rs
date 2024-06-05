use clap::Parser;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(version = "1.0", about = "PNG metadata manipulation tool", long_about = None)]
pub enum PngMeArgs {
    /// Encode a message in a PNG file
    Encode(EncodeArgs),
    /// Decode a message from a PNG file
    Decode(DecodeArgs),
    /// Remove a chunk from a PNG file
    Remove(RemoveArgs),
    /// Print the chunks of a PNG file
    Print(PrintArgs),
}

#[derive(Parser, Debug)]
pub struct EncodeArgs {
    /// The path to the input PNG file
    #[arg(short, long)]
    pub(crate) file_path: PathBuf,
    /// The type of chunk to encode the message in
    #[arg(short, long)]
    pub(crate) chunk_type: String,
    /// The message to encode
    #[arg(short, long, default_value_t = String::from("Hello"))]
    pub(crate) message: String,
    /// The path to the output PNG file
    #[arg(short, long)]
    pub(crate) output_file: Option<PathBuf>,
}

#[derive(Parser, Debug)]
pub struct DecodeArgs {
    /// The path to the input PNG file
    #[arg(short, long)]
    pub(crate) file_path: PathBuf,
    /// The type of chunk to decode the message from
    #[arg(short, long)]
    pub(crate) chunk_type: String,
}

#[derive(Parser, Debug)]
pub struct RemoveArgs {
    /// The path to the input PNG file
    #[arg(short, long)]
    pub(crate) file_path: PathBuf,
    /// The type of chunk to encode the message in
    #[arg(short, long)]
    pub(crate) chunk_type: String,
}

#[derive(Parser, Debug)]
pub struct PrintArgs {
    /// The path to the input PNG file
    #[arg(short, long)]
    pub(crate) file_path: PathBuf,
}
