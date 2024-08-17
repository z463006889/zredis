use std::collections::{HashMap, HashSet};
use anyhow::Result;
use std::ops::Deref;
mod encode;
mod decode;

pub use encode::*;
pub use decode::*;

pub enum RespFrame {
    SimpleString(SimpleString),
    Error(SimpleError),
    Integer(i64),
    BulkString(BulkString),
    NullBulkString(RespNullBulkString),
    Array(RespArray),
    NullArray(RespNullArray),
    Null(RespNull),
    Boolean(bool),
    Double(f64),
    Map(HashMap<String,RespFrame>),
    Set(HashSet<RespFrame>),
}

pub struct SimpleString(String);
pub struct RespNullBulkString;
pub struct SimpleError(String);
pub struct BulkString(Vec<u8>);
pub struct RespNull;
pub struct RespNullArray;

pub struct RespArray(Vec<RespFrame>);

pub trait RespEncode {
    fn encode(self) -> Vec<u8>;
}

pub trait RespDecode {
    fn decode(buf:Self) -> Result<RespFrame,String>;
}

impl  Deref for SimpleString {
    type Target = String;
    fn deref(&self) -> &Self::Target {
        &self.0
    } 
}

impl Deref for SimpleError {
    type Target = String;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl  Deref for BulkString {
    type Target = Vec<u8>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Deref for RespArray {
    type Target=Vec<RespFrame>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

