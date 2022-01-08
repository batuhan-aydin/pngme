#![allow(dead_code)]

use clap::{App, Arg};
use commands::{SubCommandType, get_subcommand, encode_operation, 
    decode_operation, remove_operation, print_operation};

mod args;
mod chunk;
mod chunk_type;
mod commands;
mod png;

pub type Error = Box<dyn std::error::Error>;
pub type Result<T> = std::result::Result<T, Error>;

fn main() -> Result<()> {
    
    let matches = App::new("pngme")
                    .about("An app to encode and decode message into png files")
                    .subcommand(get_subcommand(SubCommandType::Encode))
                    .subcommand(get_subcommand(SubCommandType::Decode))
                    .subcommand(get_subcommand(SubCommandType::Remove))
                    .subcommand(get_subcommand(SubCommandType::Print))
                    .get_matches();
    
    match matches.subcommand() {
        Some(("encode", sub_matches)) => encode_operation(sub_matches),
        Some(("decode", sub_matches)) => decode_operation(sub_matches),
        Some(("remove", sub_matches)) => remove_operation(sub_matches),
        Some(("print", sub_matches)) => print_operation(sub_matches),
        _ => eprintln!("not the droid you're looking for, use --help")
    } 
    Ok(())
}

// these two must be in args.rs but for some reason 
// it does not able to resolve when i put there

pub enum ArgumentType {
    FilePath,
    ChunkType,
    Message,
    OutputFile
}

pub fn get_argument(argument_type: ArgumentType) -> Arg<'static> {
    match argument_type {
        ArgumentType::FilePath => Arg::new("file_path")
        .required(true)
        .takes_value(true)
        .help("file path"),
        ArgumentType::ChunkType => Arg::new("chunk_type")
        .required(true)
        .takes_value(true)
        .help("Chunk type example: ruSt"),
        ArgumentType::Message => Arg::new("message")
        .required(true)
        .takes_value(true)
        .help("the message that you wanna encode"),
        ArgumentType::OutputFile => Arg::new("output_file")
        .takes_value(true)
        .help("output file")
    }
}