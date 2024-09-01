use bytes::BytesMut;

use super::{extract_fixed_data, RespDecode, RespDecodeError, RespEncode};




//#<t|f>\r\n
impl RespEncode for bool {
    fn encode(self) -> Vec<u8> {
        format!("#{}\r\n", if self { "t" } else { "f" }).into_bytes()
    }
}

//#<t|f>\r\n
impl RespDecode for bool {
        const PREFIX: &'static str="#";
    fn decode(buf:&mut BytesMut) -> Result<Self,RespDecodeError> {
        match extract_fixed_data(buf, "#t\r\n", "Bool") {
            Ok(_) => Ok(true),
            Err(RespDecodeError::NotComplete) => Err(RespDecodeError::NotComplete),
            Err(_) => match extract_fixed_data(buf, "#f\r\n", "Bool") {
                Ok(_) => Ok(false),
                Err(e) => Err(e),
            },
        }
    }

}

#[cfg(test)]

mod tests {
    use super::*;

    #[test]
    fn test_bool_encode() {
        let frame = true.encode();
        assert_eq!(frame, b"#t\r\n".to_vec());

        let frame = false.encode();
        assert_eq!(frame, b"#f\r\n".to_vec());
    }

    #[test]
    fn test_bool_decode() {
        let mut buf = BytesMut::from("#t\r\n");
        let frame = bool::decode(&mut buf).unwrap();
        assert_eq!(frame, true);

        let mut buf = BytesMut::from("#f\r\n");
        let frame = bool::decode(&mut buf).unwrap();
        assert_eq!(frame, false);

        let mut buf = BytesMut::from("invalid\r\n");
        let frame = bool::decode(&mut buf).unwrap_err();
    }
}





