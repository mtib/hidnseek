use std::str;

/// Split a message into <code> and <content>
pub fn split_str(msg: &str) -> Option<(&str, &str)> {
    if msg.len() > 4 {
        Some((&msg[..3], &msg[4..]))
    } else {
        None
    }
}

/// Split a message into <code> and <content>
pub fn split_u8(data: &[u8]) -> Option<(&str, &str)> {
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
                start = k + 1;
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

#[cfg(test)]
mod tests {
    use super::*;
    use std::fmt;

    const TO_TEST: [(&'static str, &'static str, &'static str); 4] = [
        ("800 Chat message", "800", "Chat message"),
        ("200 Login", "200", "Login"),
        ("201 Username", "201", "Username"),
        ("code content content content", "cod", " content content content"),
    ];

    type Splitter<'a, A> = Fn(A) -> Option<(&'a str, &'a str)>;

    #[test]
    fn test_split_str() {
        check_split_iter(TO_TEST.to_vec(), &split_str);
    }

    #[test]
    fn test_split_u8() {
        let to_test_iter = TO_TEST.iter().map(|&(x,y,z)| (x.as_bytes(),y,z)).collect::<Vec<(&[u8], &str, &str)>>();
        check_split_iter(to_test_iter, &split_u8);
    }

    fn check_split_iter<T: fmt::Debug + Copy>(i: Vec<(T, &str, &str)>, s: &Splitter<T>) {
        let mut it = i.iter();
        while let Some(&(t, code, content)) = it.next() {
            if let Some((res_code, res_content)) = s(t) {
                assert_eq!(res_code, code);
                assert_eq!(res_content, content);
            } else {
                panic!("Failed testing {:?}, expected {}:{}", t, code, content);
            }
        }
    }

    #[test]
    fn test_trim() {
        let test_arrs = [
            ([0,0,20,0,0], (2,3)),
            ([0,10,20,0,0], (1,3)),
            ([10,0,20,0,0], (0,3)),
            ([0,0,20,0,0], (2,3)),
            ([0,0,20,0,10], (2,5)),
            ([0,0,20,0,0], (2,3)),
            ([1,1,1,1,1], (0,5)),
            ([0,0,1,1,1], (2,5)),
        ];
        let mut test_iter = test_arrs.iter();
        while let Some(&(a, r)) = test_iter.next() {
            assert_eq!(trim(&a), r);
        }
    }
}
