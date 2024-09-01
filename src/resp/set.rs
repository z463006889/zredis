use std::ops::Deref;
use bytes::{Buf, BytesMut};
use crate::{RespDecode, RespDecodeError, RespEncode, RespFrame};
use crate::resp::{calc_total_length, parse_length, BUF_CAP, CRLF_LEN};

#[derive(Debug,PartialEq)]
pub struct RespSet(Vec<RespFrame>);



impl Deref for RespSet {
    type Target=Vec<RespFrame>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}



impl RespSet {
    fn new(s:impl Into<Vec<RespFrame>>) -> Self {
        RespSet(s.into())
    }
}


impl RespEncode for RespSet {
    fn encode(self) -> Vec<u8> {
        let mut buf = Vec::with_capacity(BUF_CAP);
        buf.extend_from_slice(&format!("~{}\r\n", self.len()).into_bytes());
        for frame in self.0 {
            buf.extend_from_slice(&frame.encode());
        }
        buf
    }
}

impl RespDecode for RespSet {
    const PREFIX: &'static str = "";

    fn decode(buf: &mut BytesMut) -> Result<Self, RespDecodeError> {
        let (end, len) = parse_length(buf, Self::PREFIX)?;

        let total_len = calc_total_length(buf, end, len, Self::PREFIX)?;

        if buf.len() < total_len {
            return Err(RespDecodeError::NotComplete);
        }

        buf.advance(end + CRLF_LEN);

        let mut frames = Vec::new();
        for _ in 0..len {
            frames.push(RespFrame::decode(buf)?);
        }
        Ok(RespSet::new(frames))
    }
}