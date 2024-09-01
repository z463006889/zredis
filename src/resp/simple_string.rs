use std::ops::Deref;

use bytes::BytesMut;

use super::{extract_simple_frame_data, RespDecode, RespDecodeError, RespEncode};


#[derive(Debug, PartialEq, Eq)]
pub struct SimpleString(pub String);


impl SimpleString {
    pub fn new(s: impl Into<String>) -> Self {
        SimpleString(s.into())
    }
}

impl  Deref for SimpleString {
    type Target = String;
    fn deref(&self) -> &Self::Target {
        &self.0
    } 
}

impl From<&str> for SimpleString { 
    fn from(s: &str) -> Self {
        SimpleString::new(s.to_string())
    }
}

//+OK\r\n
impl RespEncode for SimpleString {
    fn encode(self) -> Vec<u8> {
        format!("+{}\r\n",self.0).into_bytes()
    }
}

//+OK\r\n
impl RespDecode for SimpleString {
    const PREFIX: &'static str="+";
    fn decode(buf:&mut BytesMut) -> Result<Self,RespDecodeError> {
        let end = extract_simple_frame_data(buf, Self::PREFIX)?;
        let data =buf.split_to(end+2);
        let ret = String::from_utf8_lossy(&data[1..end]);
        return Ok(SimpleString::new(ret.to_string()));
    }
}

#[cfg(test)]
mod tests {

    use bytes::BufMut;

    use crate::RespFrame;

    use super::*;
    
    #[test]
    fn test_simple_string_encode() {
        let ss:RespFrame = SimpleString::new("hello".to_string()).into();
        let encoded = ss.encode();
        assert_eq!(encoded, b"+hello\r\n".to_vec());
    }

    #[test]
    fn test_simple_string_decode() {
        let mut buf = BytesMut::from("+hello\r\n");
        let decoded = SimpleString::decode(&mut buf).unwrap();
        assert_eq!(decoded, SimpleString::new("hello"));

        buf.extend_from_slice(b"+world\r");
        let decoded = SimpleString::decode(&mut buf).unwrap_err();
        assert_eq!(decoded, RespDecodeError::NotComplete);

        buf.put_u8(b'\n');
        let decoded = SimpleString::decode(&mut buf).unwrap();
        assert_eq!(decoded, SimpleString::new("world"));
    }
    
}