use super::{BulkString, RespArray, RespEncode, RespFrame, RespNull, RespNullBulkString, SimpleError, SimpleString};


impl RespEncode for RespFrame {
    fn encode(self) -> Vec<u8> {
        todo!()
    }
}

impl RespEncode for i64 {
    fn encode(self) -> Vec<u8> {
        let sign = if self < 0 { "" } else { "+" };
        format!(":{}{}\r\n", sign, self).into_bytes()
    }
}


impl RespEncode for SimpleString {
    fn encode(self) -> Vec<u8> {
        format!("+{}\r\n",self.0).into_bytes()
    }
}

impl RespEncode for SimpleError {
    fn encode(self) -> Vec<u8> {
        format!("+{}\r\n",self.0).into_bytes()
    }
}

impl RespEncode for BulkString{
    fn encode(self) -> Vec<u8> {
        let mut buf =Vec::with_capacity(self.len() + 16);
        buf.extend_from_slice(&format!("${}\r\n",self.len()).into_bytes());
        buf.extend_from_slice(&self);
        buf.extend_from_slice(b"\r\n");
        buf
    }
}


impl RespEncode for RespNullBulkString {
    fn encode(self) -> Vec<u8> {
        b"$-1\r\n".to_vec()
    }
}

impl RespEncode for RespNull {
    fn encode(self) -> Vec<u8> {
        b"_\r\n".to_vec()
    }
}


impl RespEncode for RespArray {
    fn encode(self) -> Vec<u8> {
        let mut buf = Vec::with_capacity(4096);
        buf.extend_from_slice(&format!("*{}\r\n",self.len()).into_bytes());
        for item in self.0 {
            buf.extend_from_slice(&item.encode());
        }
        buf
    }
}