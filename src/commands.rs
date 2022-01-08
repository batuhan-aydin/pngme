use clap::{App, ArgMatches};

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

// TODO
pub fn encode_operation(args: &ArgMatches) {
    println!("{:?}", args);
}

pub fn decode_operation(args: &ArgMatches) {
    println!("{:?}", args);
}

pub fn remove_operation(args: &ArgMatches) {
    println!("{:?}", args);
}

pub fn print_operation(args: &ArgMatches) {
    println!("{:?}", args);
}