use std::collections::HashMap;
use std::ops::{Deref, DerefMut};
use bytes::{Buf, BytesMut};
use crate::{RespDecode, RespDecodeError, RespEncode, RespFrame, SimpleString};
use crate::resp::{calc_total_length, parse_length, CRLF_LEN};

#[derive(Debug, PartialEq)]
pub struct RespMap(HashMap<String,RespFrame>);

impl Deref for RespMap {
    type Target=HashMap<String,RespFrame>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for RespMap {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl RespMap {
    fn new() -> Self {
        RespMap(HashMap::new())
    }
}

impl RespEncode for RespMap {

    // - map: "%<number-of-entries>\r\n<key-1><value-1>...<key-n><value-n>"
    // we only support string key which encode to SimpleString
    fn encode(self) -> Vec<u8> {
        let mut buf = Vec::with_capacity(4096);
        buf.extend_from_slice(&format!("%{}\r\n", self.len()).into_bytes());
        for (k, v) in self.0 {
            buf.extend_from_slice(&SimpleString::new(k).encode());
            buf.extend_from_slice(&v.encode())
        }
        buf
    }
}

impl RespDecode for RespMap {
    const PREFIX: &'static str = "%";

    fn decode(buf: &mut BytesMut) -> Result<Self, RespDecodeError> {
        let (end,len)=parse_length(buf,Self::PREFIX)?;
        let total_len = calc_total_length(buf,end,len,Self::PREFIX)?;
        if buf.len()<total_len{
            return Err(RespDecodeError::NotComplete)
        }
        buf.advance(end+CRLF_LEN);
        let mut frames = RespMap::new();
        for _ in 0..len {
            let key = SimpleString::decode(buf)?;
            let value = RespFrame::decode(buf)?;
            frames.insert(key.0, value);
        }
        Ok(frames)
    }
}