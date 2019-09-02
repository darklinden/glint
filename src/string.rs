use unic_segment::Graphemes;

pub fn len(s: &str) -> usize {
    Graphemes::new(s).count()
}

pub fn to_byte_offset<'a>(s: &'a str, grapheme_offset: usize) -> usize {
    let mut byte_offset = 0;

    for item in Graphemes::new(s).take(grapheme_offset) {
        byte_offset += item.len();
    }

    byte_offset
}

pub fn split_at<'a>(s: &'a str, grapheme_offset: usize) -> (&'a str, &'a str) {
    let mut byte_offset = 0;

    for item in Graphemes::new(s).take(grapheme_offset) {
        byte_offset += item.len();
    }

    (&s[0..byte_offset], &s[byte_offset..])
}