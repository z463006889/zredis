use bytes::BytesMut;
use crate::{RespDecode, RespDecodeError, RespEncode};
use crate::resp::extract_fixed_data;

#[derive(Debug, PartialEq, Eq)]
pub struct RespNull;


impl RespEncode for RespNull {
    fn encode(self) -> Vec<u8> {
        b"_\r\n".to_vec()
    }
}

impl RespDecode for RespNull {
    const PREFIX: &'static str = "";
    fn decode(buf: &mut BytesMut) -> Result<Self, RespDecodeError> {
        extract_fixed_data(buf,"_\r\n","Null")?;
        Ok(RespNull)
    }
}

