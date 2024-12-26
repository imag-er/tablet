use crate::*;
use std::cmp::*;

pub fn show(content: &TabletContent) {
    // make a copy of content
    let mut content = content.clone();
    let size = term_size::dimensions().unwrap();
    let _threshold = truncate_strings(&mut content.title, size.0);

    content
        .content
        .iter_mut()
        .for_each(|ss| truncate_strings(ss, size.0));

    println!("{}", content.title.join(" "));
    content
        .content
        .iter()
        .for_each(|s| println!("{}", s.join(" ")));
}

fn truncate_strings(strings: &mut Vec<String>, line_width: usize) {
    let n = strings.len();
    let mut total_length = strings.iter().map(|s| s.len()).sum::<usize>();

    // 最大长度作为threshold
    let mut threshold = strings.iter().max_by_key(|s| s.len()).unwrap().len();

    while total_length + n - 1 > line_width {
        threshold -= 1;
        total_length = strings
            .iter()
            .map(|s| min(s.len(), threshold))
            .sum::<usize>();
    }

    // 截断字符串
    for s in strings.iter_mut() {
        if s.len() > threshold {
            *s = s[..threshold].to_string();
        }
    }
}
