use std::{fs::File, io::Write, path::Path};

use serde::Serialize;

/// 写入数据到目标地址中
pub fn write_json_to<T: Serialize>(target: &Path, data: &T) {
    let mut opened_file = File::create(target).unwrap();
    let json_string = serde_json::to_string(data).unwrap();
    let _ = opened_file.write(json_string.as_bytes());
}
