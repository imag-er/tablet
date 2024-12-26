use std::cmp::{Ordering::*, *};

#[derive(Clone,Debug)]
pub struct TabletContent {
    pub title: Vec<String>,
    pub content: Vec<Vec<String>>,
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
        // dbg!(&title);
        // dbg!(&content);
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
