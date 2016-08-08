use std::str;

/// Split a message into <code> and <content>
pub fn split_str<'a>(msg: &'a str) -> Option<(&'a str, &'a str)> {
    if msg.len() > 4 {
        Some((&msg[..3], &msg[4..]))
    } else {
        None
    }
}

/// Split a message into <code> and <content>
pub fn split_u8<'a>(data: &'a [u8]) -> Option<(&'a str, &'a str)> {
    match str::from_utf8(data) {
        Ok(s) => split_str(s),
        Err(_) => None,
    }
}

/// remove leading or trailing \0 \u{0}
pub fn trim(data: &[u8]) -> (usize, usize) {
    let mut start = 0;
    let mut end = data.len();
    let mut switch = false;
    for (k, &v) in data.iter().enumerate() {
        // println!("{} {} {} {}", k, v, start, end);
        if v == 0 {
            if !switch {
                start = k;
            }
        } else {
            switch = true;
            end = k;
        }
    }
    assert!(start <= end, "error inside trim function");
    (start, if end >= data.len() {
            data.len()
        } else {
            end + 1
        }
    )
}
