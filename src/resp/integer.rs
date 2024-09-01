use bytes::BytesMut;
use crate::{RespDecode, RespDecodeError, RespEncode};
use crate::resp::{extract_simple_frame_data, CRLF_LEN};

// - integer: ":[<+|->]<value>\r\n"
impl RespEncode for i64{
    fn encode(self) -> Vec<u8> {
        let sign = if self<0{
            ""
        }else {
            "+"
        };
        format!(":{}{}\r\n",sign,self).into_bytes()
    }
}


// - integer: ":[<+|->]<value>\r\n"
impl RespDecode for i64{
    const PREFIX: &'static str = ":";

    fn decode(buf: &mut BytesMut) -> Result<Self, RespDecodeError> {
        let end = extract_simple_frame_data(buf,Self::PREFIX)?;

        // :2\r\n
        let data = buf.split_to(end+CRLF_LEN);
        let s = String::from_utf8_lossy(&data[Self::PREFIX.len()..end]);
        Ok(s.parse()?)
    }
}


#[cfg(test)]
mod test{
    use bytes::BytesMut;
    use crate::{RespDecode, RespEncode, RespFrame};
    use anyhow::Result;

    #[test]
    fn test_integer_encode(){
        // - integer: ":[<+|->]<value>\r\n"
        let frame:RespFrame = 20.into();
        assert_eq!(frame.encode(),b":+20\r\n");

        let frame:RespFrame = (-20).into();
        assert_eq!(frame.encode(),b":-20\r\n");
    }


    #[test]
    fn test_integer_decode()->Result<()>{
        let mut buf = BytesMut::new();
        buf.extend_from_slice(b":+123\r\n");

        let frame = i64::decode(&mut buf)?;
        assert_eq!(frame, 123);

        buf.extend_from_slice(b":-123\r\n");

        let frame = i64::decode(&mut buf)?;
        assert_eq!(frame, -123);

        Ok(())
    }
}


