use clap::{App};

use crate::{get_argument, ArgumentType};
 
pub enum SubCommandType {
    Encode,
    Decode,
    Remove,
    Print
}

pub fn get_subcommand(subcommand_type: SubCommandType) -> App<'static> {
    match subcommand_type {
        SubCommandType::Encode => App::new("encode")
                        .about("Encoding message into png")
                        .arg(get_argument(ArgumentType::FilePath))
                        .arg(get_argument(ArgumentType::ChunkType))
                        .arg(get_argument(ArgumentType::Message))
                        .arg(get_argument(ArgumentType::OutputFile)),
        SubCommandType::Decode => App::new("decode")
                        .about("Decoding a message from png file")
                        .arg(get_argument(ArgumentType::FilePath))
                        .arg(get_argument(ArgumentType::ChunkType)),
        SubCommandType::Remove => App::new("remove")
                        .about("Decoding a message from png file")
                        .arg(get_argument(ArgumentType::FilePath))
                        .arg(get_argument(ArgumentType::ChunkType)),
        SubCommandType::Print => App::new("print")
                        .about("print the message")
                        .arg(get_argument(ArgumentType::FilePath)) 
    }
}

/*
const ENCODE_ARGUMENTS: &'static [Arg] = &[
    Arg::new("File Path")
        .value_name("FILE_PATH")
        .help("file path like ./my-cat.png")
        .required(true),
    Arg::new("Chunk Type")
        .value_name("CHUNK_TYPE")
        .help("chunk type for example ruSt")
        .required(true),
    Arg::new("Secret Message")
        .value_name("MESSAGE")
        .help("the message that you wanna encode")
        .required(true),
    Arg::new("Output file")
        .value_name("OUTPUT_FILE")
        .help("output file like ./new-cat.png")

]; */
