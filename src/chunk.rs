use std::{convert::TryFrom, fmt::Display};
use crate::chunk_type::ChunkType;
use crate::{Error, Result};

#[derive(Debug)]
pub struct Chunk {
    length: u32,
    chunk_type: ChunkType,
    data: Vec<u8>,
    crc: u32
}

impl TryFrom<&[u8]> for Chunk {
    type Error = Error;

    fn try_from(value: &[u8]) -> Result<Self> {
        let length = u32::from_be_bytes(<[u8; 4]>::try_from(&value[0..4])?);
        println!("{:?}", length);
        let chunk_type = ChunkType::try_from(<[u8; 4]>::try_from(&value[4..8])?)?;
        println!("{:?}", chunk_type);
        let data = value[8..length as usize + 8].to_vec();
        println!("{:?}", data);
        let correct_crc = crc::crc32::checksum_ieee(&value[4..length as usize + 8]);
        let provided_crc = u32::from_be_bytes(<[u8; 4]>::try_from(&value[value.len()-4..value.len()])?);
        
        if correct_crc != provided_crc {
            println!("{:?}", correct_crc);
            println!("{:?}", provided_crc);
            return Err(Box::new(ChunkError::InvalidCrc));
        }

        if length != data.len() as u32 {
            return Err(Box::new(ChunkError::InvalidDataLength));
        }

        Ok (Chunk {
            length,
            chunk_type,
            data,
            crc: correct_crc
        })
    }
}

impl Display for Chunk {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "length: {}\n type: {}\n data: {:?}\n crc: {}\n",
        self.length, self.chunk_type, 
        self.data_as_string(), self.crc)
    }
}

impl Chunk {
    pub fn new(chunk_type: ChunkType, data: Vec<u8>) -> Self {
        let crc_data: Vec<u8> = chunk_type.bytes().iter().chain(&data).cloned().collect();
        Chunk {
            length: data.len() as u32,
            chunk_type: chunk_type,
            data: data,
            crc: crc::crc32::checksum_ieee(&crc_data)
        }
    }

    pub fn length(&self) -> u32 {
        self.length
    }

    pub fn chunk_type(&self) -> &ChunkType {
        &self.chunk_type
    }

    pub fn data(&self) -> &[u8] {
        &self.data
    }

    pub fn crc(&self) -> u32 {
        self.crc
    }

    pub fn data_as_string(&self) -> Result<String> {
        match std::str::from_utf8(&self.data) {
            Ok(str_data) => Ok(String::from(str_data)),
            Err(error) => Err(Box::new(error))
        }
    }

    pub fn as_bytes(&self) -> Vec<u8> {
        self.length.to_be_bytes()
        .iter()
        .cloned()
        .chain(self.chunk_type.bytes().iter().cloned())
        .chain(self.data.iter().cloned())
        .chain(self.crc.to_be_bytes().iter().cloned())
        .collect()
    }
}

#[derive(Debug)]
pub enum ChunkError {
    InvalidDataLength,
    InvalidCrc
}

impl std::error::Error for ChunkError {}

impl Display for ChunkError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            ChunkError::InvalidDataLength => write!(f, "Invalid data length"),
            &ChunkError::InvalidCrc => write!(f, "Incorrect crc")
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    fn testing_chunk() -> Chunk {
        let data_length: u32 = 42;
        let chunk_type = "RuSt".as_bytes();
        let message_bytes = "This is where your secret message will be!".as_bytes();
        let crc: u32 = 2882656334;

        let chunk_data: Vec<u8> = data_length
            .to_be_bytes()
            .iter()
            .chain(chunk_type.iter())
            .chain(message_bytes.iter())
            .chain(crc.to_be_bytes().iter())
            .copied()
            .collect();
        
        Chunk::try_from(chunk_data.as_ref()).unwrap()
    }

    #[test]
    fn test_chunk_length() {
        let chunk = testing_chunk();
        assert_eq!(chunk.length(), 42);
    }
     
    #[test]
    fn test_chunk_type() {
        let chunk = testing_chunk();
        assert_eq!(chunk.chunk_type().to_string(), String::from("RuSt"));
    }
    
    #[test]
    fn test_chunk_string() {
        let chunk = testing_chunk();
        let chunk_string = chunk.data_as_string().unwrap();
        let expected_chunk_string = String::from("This is where your secret message will be!");
        assert_eq!(chunk_string, expected_chunk_string);
    }
    
    #[test]
    fn test_chunk_crc() {
        let chunk = testing_chunk();
        assert_eq!(chunk.crc(), 2882656334);
    }
    
    #[test]
    fn test_valid_chunk_from_bytes() {
        let data_length: u32 = 42;
        let chunk_type = "RuSt".as_bytes();
        let message_bytes = "This is where your secret message will be!".as_bytes();
        let crc: u32 = 2882656334;

        let chunk_data: Vec<u8> = data_length
            .to_be_bytes()
            .iter()
            .chain(chunk_type.iter())
            .chain(message_bytes.iter())
            .chain(crc.to_be_bytes().iter())
            .copied()
            .collect();

        let chunk = Chunk::try_from(chunk_data.as_ref()).unwrap();

        let chunk_string = chunk.data_as_string().unwrap();
        let expected_chunk_string = String::from("This is where your secret message will be!");

        assert_eq!(chunk.length(), 42);
        assert_eq!(chunk.chunk_type().to_string(), String::from("RuSt"));
        assert_eq!(chunk_string, expected_chunk_string);
        assert_eq!(chunk.crc(), 2882656334);
    }
    
    #[test]
    fn test_invalid_chunk_from_bytes() {
        let data_length: u32 = 42;
        let chunk_type = "RuSt".as_bytes();
        let message_bytes = "This is where your secret message will be!".as_bytes();
        let crc: u32 = 2882656333;

        let chunk_data: Vec<u8> = data_length
            .to_be_bytes()
            .iter()
            .chain(chunk_type.iter())
            .chain(message_bytes.iter())
            .chain(crc.to_be_bytes().iter())
            .copied()
            .collect();

        let chunk = Chunk::try_from(chunk_data.as_ref());

        assert!(chunk.is_err());
    }
     
    #[test]
    pub fn test_chunk_trait_impls() {
        let data_length: u32 = 42;
        let chunk_type = "RuSt".as_bytes();
        let message_bytes = "This is where your secret message will be!".as_bytes();
        let crc: u32 = 2882656334;

        let chunk_data: Vec<u8> = data_length
            .to_be_bytes()
            .iter()
            .chain(chunk_type.iter())
            .chain(message_bytes.iter())
            .chain(crc.to_be_bytes().iter())
            .copied()
            .collect();
        
        let chunk: Chunk = TryFrom::try_from(chunk_data.as_ref()).unwrap();
        
        let _chunk_string = format!("{}", chunk);
    }
    
}
