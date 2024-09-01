use std::ops::Deref;

use bytes::BytesMut;

use super::{extract_simple_frame_data, RespDecode, RespDecodeError, RespEncode};



#[derive(Debug, PartialEq, Eq)]
pub struct SimpleError(String);

impl SimpleError {
    fn new(s: impl Into<String>) -> Self {
        SimpleError(s.into())
    }
}

impl Deref for SimpleError {
    type Target = String;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

//-Error message\r\n
impl RespEncode for SimpleError {
    fn encode(self) -> Vec<u8> {
        format!("-{}\r\n",self.0).into_bytes()
    }
}

// /-Error message\r\n
impl RespDecode for SimpleError {
    const PREFIX: &'static str="-";
    
    fn decode(buf:&mut BytesMut) -> Result<Self,RespDecodeError> {
        let end = extract_simple_frame_data(buf, Self::PREFIX)?;
        let data =buf.split_to(end+2);
        let ret = String::from_utf8_lossy(&data[1..end]);
        return Ok(SimpleError::new(ret.to_string()));
    }
    

}


#[cfg(test)]

mod tests {
    use crate::RespFrame;

    use super::*;

    #[test]
    fn test_error_encode() {
        let frame:RespFrame = SimpleError::new("error message".to_string()).into();
        assert_eq!(frame.encode(), b"-error message\r\n".to_vec());
    }

    #[test]
    fn test_error_decode() {
        let mut buf = BytesMut::new();
        buf.extend_from_slice(b"-error message\r\n");
        let frame = SimpleError::decode(&mut buf).unwrap();
        assert_eq!(frame, SimpleError::new("error message".to_string()));
    }
}

