use std::{fmt, fs};
use serde_json::Value;
use crate::JsonFragment;

impl fmt::Debug for JsonFragment {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let data_view_len = self.data.len().min(5);
        let data_preview = &self.data[..data_view_len];
        f.debug_struct("JsonFragment")
            .field("data", &DebugBytesPreview(data_preview, self.data.len() > data_view_len))
            .field("flags", &flags_to_vec(self.flags))
            .field("index", &self.index)
            .field("hash", &self.hash)
            .field("parent", &self.parent)
            .finish()
    }
}

struct DebugBytesPreview<'a>(&'a [u8], bool);
struct Ellipsis;

impl fmt::Debug for Ellipsis {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "...")
    }
}
impl<'a> fmt::Debug for DebugBytesPreview<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut list = f.debug_list();
        for byte in self.0 {
            list.entry(byte);
        }
        if self.1 {
            list.entry(&Ellipsis);
        }
        list.finish()
    }
}

fn flags_to_vec(flags_bitfield: u128) -> Vec<String> {
    let mut flags = vec![];
    if flags_bitfield & 0x01 != 0 {
        flags.push("null".to_string());
    }
    if flags_bitfield & 0x02 != 0 {
        flags.push("bool".to_string());
    }
    if flags_bitfield & 0x04 != 0 {
        flags.push("number".to_string());
    }
    if flags_bitfield & 0x08 != 0 {
        flags.push("string".to_string());
    }
    if flags_bitfield & 0x10 != 0 {
        flags.push("array".to_string());
    }
    if flags_bitfield & 0x20 != 0 {
        flags.push("object".to_string());
    }
    flags
}

pub(crate) fn get_flags(value: &Value) -> u128 {
    match value {
        Value::Null => 0x01,
        Value::Bool(_) => 0x02,
        Value::Number(_) => 0x04,
        Value::String(_) => 0x08,
        Value::Array(_) => 0x10,
        Value::Object(_) => 0x20,
    }
}
use md5;

pub fn hash_bytes(bytes: &[u8]) -> String {
    format!("{:x}", md5::compute(bytes))
}

pub trait IntoBytes {
    fn into_bytes(self) -> Vec<u8>;
}

impl IntoBytes for u8 {
    fn into_bytes(self) -> Vec<u8> {
        vec![self]
    }
}

impl IntoBytes for i32 {
    fn into_bytes(self) -> Vec<u8> {
        self.to_le_bytes().to_vec()
    }
}

impl IntoBytes for &str {
    fn into_bytes(self) -> Vec<u8> {
        self.as_bytes().to_vec()
    }
}

impl IntoBytes for u128 {
    fn into_bytes(self) -> Vec<u8> {
        self.to_le_bytes().to_vec()
    }
}
impl IntoBytes for String {
    fn into_bytes(self) -> Vec<u8> {
        self.into_bytes()
    }
}

impl IntoBytes for Vec<u8> {
    fn into_bytes(self) -> Vec<u8> {
        self
    }
}

impl IntoBytes for &Vec<u8> {
    fn into_bytes(self) -> Vec<u8> {
        self.clone()
    }
}

impl IntoBytes for &[u8] {
    fn into_bytes(self) -> Vec<u8> {
        self.to_vec()
    }
}

pub fn append_as_bytes<T: IntoBytes>(v: &mut Vec<u8>, value: T) {
    v.extend(value.into_bytes());
}

#[macro_export]
macro_rules! hash {
    ($($x:expr),*) => {{
        let mut v = Vec::new();
        $(
            append_as_bytes(&mut v, $x);
        )*
        hash_bytes(&v)
    }};
}



pub(crate) fn read_json_from_file(file_path: &str) -> serde_json::Result<Value> {
    let json = fs::read_to_string(file_path).unwrap();
    Ok(serde_json::from_str(&json)?)
}