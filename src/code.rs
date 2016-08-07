use std::str;

pub fn split_str<'a>(msg: &'a str) -> Option<(&'a str, &'a str)> {
    if msg.len() > 4 {
        Some((&msg[..3], &msg[4..]))
    } else {
        None
    }
}

pub fn split_u8<'a>(data: &'a [u8]) -> Option<(&'a str, &'a str)> {
    match str::from_utf8(data) {
        Ok(s) => split_str(s),
        Err(_) => None,
    }
}
