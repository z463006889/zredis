use std::{ops::Deref, str::FromStr};

use bytes::{Buf, BytesMut};

use super::{extract_fixed_data, extract_simple_frame_data, parse_length, RespDecode, RespDecodeError, RespEncode};



#[derive(Debug, PartialEq, Eq)]
pub struct BulkString(pub Vec<u8>);

#[derive(Debug, PartialEq, Eq)]
pub struct RespNullBulkString;


impl BulkString {
    pub fn new(buf: impl Into<Vec<u8>>) -> Self {
        BulkString(buf.into())
    }
}

impl  Deref for BulkString {
    type Target = Vec<u8>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl AsRef<[u8]> for BulkString {
    fn as_ref(&self) -> &[u8] {
        self.0.as_ref()
    }
}

impl From<&str> for BulkString {
    fn from(s: &str) -> Self {
        BulkString(s.as_bytes().to_vec())
    }
}

impl<const N:usize> From<[u8; N]> for BulkString{
    fn from(value: [u8; N]) -> Self {
        BulkString(value.to_vec())
    }
}

impl From<String> for BulkString {
    fn from(s: String) -> Self {
        BulkString(s.into_bytes())
    }
}

impl From<&[u8]> for BulkString {
    fn from(s: &[u8]) -> Self {
        BulkString(s.to_vec())
    }
}

//A bulk string represents a single binary string

//$<length>\r\n<data>\r\n
impl RespEncode for BulkString{
    fn encode(self) -> Vec<u8> {
        let mut buf =Vec::with_capacity(self.len() + 16);
        buf.extend_from_slice(&format!("${}\r\n",self.len()).into_bytes());
        buf.extend_from_slice(&self);
        buf.extend_from_slice(b"\r\n");
        buf
    }
}

// - null bulk string: "$-1\r\n"
impl RespEncode for RespNullBulkString{
    fn encode(self) -> Vec<u8> {
        b"$-1\r\n".to_vec()
    }
}

//$<length>\r\n<data>\r\n
impl RespDecode for BulkString {
    
    const PREFIX: &'static str="$";
    fn decode(buf:&mut BytesMut) -> Result<Self,RespDecodeError> {
        let (end,len)=parse_length(buf, Self::PREFIX)?;
        let remained = &buf[end + 2..];
        if remained.len() < len + 2 {
            return Err(RespDecodeError::NotComplete);
        }
        buf.advance(end + 2);
        let data = buf.split_to(len + 2);
        Ok(BulkString::new(data[..len].to_vec()))
    }
}

// - null bulk string: "$-1\r\n"
impl RespDecode for RespNullBulkString {
    const PREFIX: &'static str="$";
    fn decode(buf:&mut BytesMut) -> Result<Self,RespDecodeError> {
        extract_fixed_data(buf, "$-1\r\n", "NullBulkString")?;
        Ok(RespNullBulkString)
    }
}

#[cfg(test)]

mod tests {
    use crate::RespFrame;

    use super::*;
    use bytes::BytesMut;

    #[test]
    fn test_bulk_string_encode() {
        
        let frame:RespFrame = BulkString::new(b"Hello, World!".to_vec()).into();
        assert_eq!(frame.encode(), b"$13\r\nHello, World!\r\n".to_vec());
    }

    #[test]
    fn test_bulk_string_decode() {
        let mut buf = BytesMut::new();
        buf.extend_from_slice(b"$13\r\nHello, World!\r\n");
        let frame = BulkString::decode(&mut buf).unwrap();
        assert_eq!(frame.as_ref(), b"Hello, World!");
    }
}