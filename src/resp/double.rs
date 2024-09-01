use bytes::BytesMut;
use crate::{RespDecode, RespDecodeError, RespEncode};
use crate::resp::{extract_simple_frame_data, CRLF_LEN};

// - double: ",[<+|->]<integral>[.<fractional>][<E|e>[sign]<exponent>]\r\n"
impl RespEncode for f64{
    fn encode(self) -> Vec<u8> {
        let mut buf = Vec::with_capacity(2048);
        let ret = if self.abs()>1e+8 || self.abs()<1e-8{
            format!(",{:+e}\r\n",self)
        }else {
            let sign = if self < 0.0 { "" } else { "+" };
            format!(",{}{}\r\n", sign, self)
        };
        buf.extend_from_slice(&ret.into_bytes());
        buf
    }
}


// - double: ",[<+|->]<integral>[.<fractional>][<E|e>[sign]<exponent>]\r\n"
impl RespDecode for f64 {
    const PREFIX: &'static str = ",";

    fn decode(buf: &mut BytesMut) -> Result<Self, RespDecodeError> {
        let end = extract_simple_frame_data(buf, Self::PREFIX)?;
        let data = buf.split_to(end+CRLF_LEN);
        let s = String::from_utf8_lossy(&data[Self::PREFIX.len()..end]);
        Ok(s.parse()?)
    }
}


#[cfg(test)]
mod tests {
    use bytes::BytesMut;
    use crate::{RespDecode, RespEncode, RespFrame};

    #[test]
    fn test_double_encode() {
        let frame:RespFrame = 3.14.into();
        assert_eq!(frame.encode(),b",+3.14\r\n" );

        let frame: RespFrame = (-1.23456e-9).into();
        assert_eq!(&frame.encode(), b",-1.23456e-9\r\n");
    }


    #[test]
    fn test_double_decode() {
        let mut buf = BytesMut::new();
        buf.extend_from_slice(b",3.14\r\n");
        assert_eq!(f64::decode(&mut buf).unwrap(), 3.14);
    }
}



