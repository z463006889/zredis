use std::{collections:: HashMap, ops::DerefMut};
use bytes::{Buf, BytesMut};
use thiserror::Error;
use enum_dispatch::enum_dispatch;
use std::ops::Deref;
mod array;
mod simple_string;
mod simple_error;
mod bool;
mod bulk_string;
mod frame;
mod double;
mod integer;
mod map;
mod null;
mod set;

pub use array::*;
pub use simple_string::*;
pub use simple_error::*;
pub use bool::*;
pub use bulk_string::*;
pub use frame::*;
pub use null::*;
pub use set::*;
pub use map::*;

const BUF_CAP: usize = 4096;
const CRLF: &[u8] = b"\r\n";
const CRLF_LEN: usize = CRLF.len();


#[derive(Error, Debug,PartialEq, Eq)]
pub enum RespDecodeError {
    #[error("Invalid frame: {0}")]
    InvalidFrame(String),
    #[error("Invalid frame type: {0}")]
    InvalidFrameType(String),
    #[error("Invalid frame length： {0}")]
    InvalidFrameLength(isize),
    #[error("Frame is not complete")]
    NotComplete,
    #[error("Parse error: {0}")]
    ParseIntError(#[from] std::num::ParseIntError),
    #[error("Utf8 error: {0}")]
    Utf8Error(#[from] std::string::FromUtf8Error),
    #[error("Parse float error: {0}")]
    ParseFloatError(#[from] std::num::ParseFloatError),
}


#[enum_dispatch]
pub trait RespEncode {
    fn encode(self) -> Vec<u8>;
}

pub trait RespDecode:Sized {
    const PREFIX: &'static str;
    fn decode(buf:&mut BytesMut) -> Result<Self,RespDecodeError>;
}
fn extract_simple_frame_data(buf: &[u8], prefix: &str) -> Result<usize, RespDecodeError> {
    if buf.len() < 3 {
        return Err(RespDecodeError::NotComplete);
    }

    if !buf.starts_with(prefix.as_bytes()) {
        return Err(RespDecodeError::InvalidFrameType(format!(
            "expect: SimpleString({}), got: {:?}",
            prefix, buf
        )));
    }
    let end = find_crlf(buf, 1).ok_or(RespDecodeError::NotComplete)?;
    Ok(end)
}

fn find_crlf(buf: &[u8], nth: usize) -> Option<usize> {
    let mut count = 0;
    for i in 1..buf.len() - 1 {
        if buf[i] == b'\r' && buf[i + 1] == b'\n' {
            count += 1;
            if count == nth {
                return Some(i);
            }
        }
    }
    None
}

fn extract_fixed_data(
    buf: &mut BytesMut,
    expect: &str,
    expect_type: &str,
) -> Result<(), RespDecodeError> {
    if buf.len() < expect.len() {
        return Err(RespDecodeError::NotComplete);
    }
    if !buf.starts_with(expect.as_bytes()) {
        return Err(RespDecodeError::InvalidFrameType(format!(
            "expect: {}, got: {:?}",
            expect_type, buf
        )));
    }
    buf.advance(expect.len());
    Ok(())
}

fn parse_length(buf: &[u8], prefix: &str) -> Result<(usize, usize), RespDecodeError> {
    let end = extract_simple_frame_data(buf, prefix)?;
    let s = String::from_utf8_lossy(&buf[prefix.len()..end]);
    Ok((end, s.parse()?))
}

fn calc_total_length(buf: &[u8], end: usize, len: usize, prefix: &str) -> Result<usize, RespDecodeError> {
    let (end,len)=parse_length(buf, prefix)?;
    if end + 2 + len > buf.len() {
        return Err(RespDecodeError::NotComplete);
    }
    Ok(end + 2 + len)

}