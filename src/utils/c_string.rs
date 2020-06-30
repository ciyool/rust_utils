use std::str::from_utf8;
use v_htmlescape::escape;

/// *************************************************************
/// utf8 bytes to &str
/// *************************************************************
pub fn bytes_to_str(v: &[u8]) -> &str {
    let v = from_utf8(&v).unwrap_or_default();
    if v.len() < 2 {
        return "{}";
    }
    v
}

/// *************************************************************
/// Html 转义 （html -> string）
/// *************************************************************
pub fn html_to_string(v: String) -> String {
    let v = escape(v.as_str());
    format!("{}", v)
}

/// *************************************************************
/// 16进制字符串转 i32（ 0x325255 -> 3297877 ）
/// *************************************************************
pub fn str_to_i32_radix(str: &str) -> std::result::Result<i32, std::num::ParseIntError> {
    let str = str.trim().trim_start_matches("0x");
    //    println!(" new str: {}", str);
    i32::from_str_radix(str, 16)
}
