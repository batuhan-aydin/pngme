use std::{convert::TryFrom, fmt::Display, str::FromStr};
use crate::Error;

#[derive(Debug)]
pub struct ChunkType {
    data: [u8; 4]
}

impl TryFrom<[u8; 4]> for ChunkType {
    type Error = ();
    fn try_from(value: [u8; 4]) -> Result<Self, Self::Error> {
        Ok(ChunkType {data: value})
    }
}

impl FromStr for ChunkType {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() != 4 {
            return Err(Box::new(ChunkTypeError::InvalidLength(s.len())));
        }

        let characters = s.as_bytes();
        for character in characters {
            if !Self::is_valid_byte(*character) {
                return Err(Box::new(ChunkTypeError::InvalidCharacter(*character)));
            } 
        }

        match <[u8; 4]>::try_from(characters) {
            Ok(element) => Ok (ChunkType {data: element}),
            Err(error) => Err(Box::new(error))
        }
    }  
}

impl Display for ChunkType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", String::from_utf8_lossy(&self.data))
    }
}

impl PartialEq for ChunkType {
    fn eq(&self, other: &Self) -> bool {
        self.data == other.data
    }
}

impl Eq for ChunkType {}

impl ChunkType {
    fn bytes(&self) -> [u8; 4] {
        self.data
    }

    // the values needs to be in range A-Z and a-z, or 65-90 and 97-122 decimal
    fn is_valid_byte(b: u8) -> bool {
        (b >= 65 && b <= 90) || (b >= 97 && b <= 122)
    }

    // it depends on ancillary bit which is 5th bit of first byte
    fn is_critical(&self) -> bool {
        match self.data[0] >> 5 & 0x1 {
            0 => true,
            _ => false
        }
    }

    // bit 5 of the second byte
    fn is_public(&self) -> bool {
        match self.data[1] >> 5 & 0x1 {
            0 => true,
            _ => false
        }
    }

    // bit 5 of the third byte
    // Must be 0 (uppercase)
    // At the present time all chunk names must have uppercase third letters
    fn is_reserved_bit_valid(&self) -> bool {
        match self.data[2] >> 5 & 0x1 {
            0 => true,
            _ => false
        }
    }

    // bit 5 of the fourth byte
    fn is_safe_to_copy(&self) -> bool {
        match self.data[3] >> 5 & 0x1 {
            1 => true,
            _ => false
        }
    }

    // validation check
    fn is_valid(&self) -> bool {
        let values = self.bytes();
        for value in values {
            if !Self::is_valid_byte(value) {
                return false;
            }
        }
        self.is_reserved_bit_valid()
    }
}

#[derive(Debug)]
pub enum ChunkTypeError {
    InvalidCharacter(u8),
    InvalidLength(usize)
}

impl std::error::Error for ChunkTypeError {}

impl Display for ChunkTypeError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::InvalidCharacter(byte) => write!(f, "invalid character: {}", byte),
            Self::InvalidLength(length) => write!(f, "length must be 4, right now: {}", length)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::convert::TryFrom;
    use std::str::FromStr;

    #[test]
    pub fn test_chunk_type_from_bytes() {
        let expected = [82, 117, 83, 116];
        let actual = ChunkType::try_from([82, 117, 83, 116]).unwrap();

        assert_eq!(expected, actual.bytes());
    }

    #[test]
    pub fn test_chunk_type_from_str() {
        let expected = ChunkType::try_from([82, 117, 83, 116]).unwrap();
        let actual = ChunkType::from_str("RuSt").unwrap();
        assert_eq!(expected, actual);
    }
    
    #[test]
    pub fn test_chunk_type_is_critical() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(chunk.is_critical());
    }

    #[test]
    pub fn test_chunk_type_is_not_critical() {
        let chunk = ChunkType::from_str("ruSt").unwrap();
        assert!(!chunk.is_critical());
    }
    
    #[test]
    pub fn test_chunk_type_is_public() {
        let chunk = ChunkType::from_str("RUSt").unwrap();
        assert!(chunk.is_public());
    }

    #[test]
    pub fn test_chunk_type_is_not_public() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(!chunk.is_public());
    }
     
    #[test]
    pub fn test_chunk_type_is_reserved_bit_valid() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(chunk.is_reserved_bit_valid());
    }

    #[test]
    pub fn test_chunk_type_is_reserved_bit_invalid() {
        let chunk = ChunkType::from_str("Rust").unwrap();
        assert!(!chunk.is_reserved_bit_valid());
    }

    #[test]
    pub fn test_chunk_type_is_safe_to_copy() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(chunk.is_safe_to_copy());
    }

    #[test]
    pub fn test_chunk_type_is_unsafe_to_copy() {
        let chunk = ChunkType::from_str("RuST").unwrap();
        assert!(!chunk.is_safe_to_copy());
    }
    
    #[test]
    pub fn test_valid_chunk_is_valid() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(chunk.is_valid());
    }

    #[test]
    pub fn test_invalid_chunk_is_valid() {
        let chunk = ChunkType::from_str("Rust").unwrap();
        assert!(!chunk.is_valid());

        let chunk = ChunkType::from_str("Ru1t");
        assert!(chunk.is_err());
    }

    #[test]
    pub fn test_chunk_type_string() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert_eq!(&chunk.to_string(), "RuSt");
    }
    
    #[test]
    pub fn test_chunk_type_trait_impls() {
        let chunk_type_1: ChunkType = TryFrom::try_from([82, 117, 83, 116]).unwrap();
        let chunk_type_2: ChunkType = FromStr::from_str("RuSt").unwrap();
        let _chunk_string = format!("{}", chunk_type_1);
        let _are_chunks_equal = chunk_type_1 == chunk_type_2;
    }
    
}