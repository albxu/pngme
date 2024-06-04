use std::path::PathBuf;
use clap::{Parser, Args, Subcommand, ValueEnum};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
#[group(GROUP ATTRIBUTE)]
pub enum PngMeArgs {
    /// Encode a message in a PNG file
    #[command(subcommand)]
    Encode(EncodeArgs),
    /// Decode a message from a PNG file
    #[command(subcommand)]
    Decode(DecodeArgs),
    /// Remove a chunk from a PNG file
    #[command(subcommand)]
    Remove(RemoveArgs),
    /// Print the chunks of a PNG file
    #[command(subcommand)]
    Print(PrintArgs),
}

#[derive(Parser, Debug)]
pub struct EncodeArgs{
    /// The path to the input PNG file
    #[arg(short, long)]
    file_path: String,
    /// The type of chunk to encode the message in
    #[arg(short, long)]
    chunk_type: String,
    /// The message to encode
    #[arg(short, long, default_value_t = String::from("Hello"))]
    message: String,
    /// The path to the output PNG file
    #[arg(short, long)]
    output_file: Option<String>,
}

#[derive(Parser, Debug)]
pub struct DecodeArgs {
    /// The path to the input PNG file
    #[arg(short, long)]
    file_path: String,
    /// The path to the output file
    #[arg(short, long)]
    chunk_type: String,
}

#[derive(Parser, Debug)]
pub struct RemoveArgs {
    /// The path to the input PNG file
    #[arg(short, long)]
    file_path: String,
    /// The type of chunk to encode the message in
    #[arg(short, long)]
    chunk_type: String,
}

#[derive(Parser, Debug)]
pub struct PrintArgs {
    /// The path to the input PNG file
    #[arg(short, long)]
    file_path: String,
}
