mod Process;
use Process::*;

use std::{env, fs};

fn read_file(path: &str) -> String {
    fs::read_to_string(path).expect("读取文件失败")
}

#[warn(unused_imports)]
#[warn(dead_code)]
fn main() {
    // let args: Vec<String> = env::args().collect();

    // let default_content = "没有输入内容".to_string();
    // let content = if args.len() < 2 {
    //     &read_file("rc/default.txt")
    // } else {
    //     args.get(1).unwrap_or(&default_content)
    // };

    if let Ok(entries) = fs::read_dir("rc/") {
        for entry in entries {
            if let Ok(entry) = entry {
                if let Ok(content) = fs::read_to_string(&entry.path()) {
                    let mut tablet_content = TabletContent::new(&content);
                    show(&mut tablet_content);
                }
            }
        }
    }
}
