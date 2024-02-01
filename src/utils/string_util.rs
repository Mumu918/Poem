use unicode_width::UnicodeWidthChar;

pub fn md5<T: AsRef<[u8]>>(s: T) -> String {
    let digest = md5::compute(s);
    format!("{:x}", digest)
}

/// 将字符串根据unicode的宽度截断
pub fn unicode_width_splice(s: &str, len: usize) -> String {
    let mut res = String::new();
    let mut len = len;
    let mut iter = s.chars().into_iter();
    loop {
        match iter.next() {
            Some(char) => {
                let width = UnicodeWidthChar::width(char).unwrap();
                if len < width {
                    break;
                }
                len -= width;
                res.push(char);
            }
            None => break,
        }
    }
    res
}
