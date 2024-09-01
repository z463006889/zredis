use std::ops::{Deref, DerefMut};

use bytes::{Buf, BytesMut};

use crate::resp::extract_simple_frame_data;

use super::{calc_total_length, extract_fixed_data, parse_length, RespDecode, RespDecodeError, RespEncode, RespFrame, BUF_CAP, CRLF_LEN};


#[derive(Debug, PartialEq)]
pub struct RespArray(Vec<RespFrame>);

#[derive(Debug, PartialEq, Eq)]
pub struct RespNullArray;

impl RespArray {
    fn new(value:impl Into<Vec<RespFrame>> ) -> Self {
        RespArray(value.into())
    }
}

impl Deref for RespArray {
    type Target=Vec<RespFrame>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for RespArray {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl From<Vec<RespFrame>> for RespArray {
    fn from(s: Vec<RespFrame>) -> Self {
        RespArray(s)
    }
}


// *<number-of-elements>\r\n<element-1>...<element-n>
impl RespEncode for RespArray {
    fn encode(self) -> Vec<u8> {
        let mut buf = Vec::with_capacity(BUF_CAP);
        buf.extend_from_slice(&format!("*{}\r\n",self.len()).into_bytes());
        for item in self.0 {
            buf.extend_from_slice(&item.encode());
        }
        buf
    }
}

// *<number-of-elements>\r\n<element-1>...<element-n>
impl RespDecode for RespArray {
    const PREFIX: &'static str="*";
    fn decode(buf:&mut BytesMut) -> Result<Self,RespDecodeError> {
        let (end,len)= parse_length(buf, Self::PREFIX)?;
        let total_len = calc_total_length(buf, end, len, Self::PREFIX)?;

        if buf.len()<total_len {
            return  Err(RespDecodeError::NotComplete);
        }
        buf.advance(end+CRLF_LEN);

        let mut frame = Vec::with_capacity(len);
        for _ in 0..len {
            frame.push(RespFrame::decode(buf)?);
        }
        Ok(RespArray::new(frame))
    }
}

impl RespEncode for RespNullArray {
    fn encode(self) -> Vec<u8> {
        b"*-1\r\n".to_vec()
    }
}

impl RespDecode for RespNullArray {
    const PREFIX: &'static str="*";
    fn decode(buf:&mut BytesMut) -> Result<Self,RespDecodeError> {
        extract_fixed_data(buf, "*-1\r\n", "NullArray")?;
        Ok(RespNullArray)
    }    
}



#[cfg(test)]
mod tese{
    use bytes::BytesMut;
    use crate::{resp::BulkString, RespArray, RespDecode, RespDecodeError, RespEncode};

    #[test]
    fn test_resp_array_encode(){
        let frame:RespArray = RespArray::new(vec![
            BulkString::new("set").into(),
            BulkString::new("hello").into(),
            BulkString::new("world").into(),
        ]);
        assert_eq!(frame.encode(),b"*3\r\n$3\r\nset\r\n$5\r\nhello\r\n$5\r\nworld\r\n".to_vec());
    }


    #[test]
    fn test_resp_array_decode(){
        let mut buf = BytesMut::new();
        buf.extend_from_slice(b"*2\r\n$3\r\nset\r\n$5\r\nhello\r\n");

        let frame = RespArray::decode(&mut buf).unwrap();
        assert_eq!(frame, RespArray::new([b"set".into(), b"hello".into()]));

        buf.extend_from_slice(b"*2\r\n$3\r\nset\r\n");
        let ret = RespArray::decode(&mut buf);
        assert_eq!(ret.unwrap_err(), RespDecodeError::NotComplete);

        buf.extend_from_slice(b"$5\r\nhello\r\n");

        let frame = RespArray::decode(&mut buf).unwrap();
        assert_eq!(frame, RespArray::new([b"set".into(), b"hello".into()]));
    }








}






