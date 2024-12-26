
use std::{cmp::Ordering::*, cmp::*};

pub struct TabletContent {
    title: Vec<String>,
    content: Vec<Vec<String>>,
}

impl std::fmt::Display for TabletContent {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:#?}\n\n{:#?}", self.title, self.content[0])
    }
}
impl TabletContent {
    fn get_title(content: &String, split: &str) -> Vec<String> {
        // get the first line of content
        // 标题行
        let title_line = content
            .lines()
            .next()
            .map(|s| s.trim().to_string())
            .unwrap_or("无法解析标题".to_string());

        // 划分为单词
        title_line
            .split(split)
            .filter(|s| s.len() > 0)
            .map(|s| s.trim().to_string())
            .collect()
    }

    fn get_content(content: &String, split: &str) -> Vec<Vec<String>> {
        // get the rest of the content and split it by line
        content
            .lines()
            .skip(1)
            .map(|line| {
                line.split(split)
                    .map(str::to_string)
                    .filter(|s| s.len() > 0)
                    .collect()
            })
            .collect()
    }

    pub fn new(tablet_content: &String) -> TabletContent {
        let mut title = TabletContent::get_title(tablet_content, " ");
        let mut content = TabletContent::get_content(tablet_content, " ");

        // old: 使用grid search ,尝试用 ' ' 或 "  " 划分 , 无法处理出现空项的问题
        // new (TODO): 考虑标题栏和内容栏都为空格的位置作为split的索引

        // dbg!(title.len());
        // dbg!(content[0].len());
        match title.len().cmp(&content[0].len()) {
            Less => content = TabletContent::get_content(tablet_content, "  "),
            Greater => title = TabletContent::get_title(tablet_content, "  "),
            Equal => return TabletContent { title, content },
        }

        // dbg!(title.len());

        match title.len().cmp(&content[0].len()) {
            Less | Greater => {
                content = TabletContent::get_content(tablet_content, "  ");
                title = TabletContent::get_title(tablet_content, "  ");
                TabletContent { title, content }
            }
            Equal => TabletContent { title, content },
        }
    }
}

#[warn(unused_variables)]
pub fn show(content: &mut TabletContent) {
    let size = term_size::dimensions().unwrap();
    let threshold = truncate_strings(&mut content.title, size.0);

    println!("{}", content.title.join(" "));
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
