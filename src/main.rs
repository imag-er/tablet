mod strproc;
use std::io::Read;
use std::fs;
use strproc::display::*;
use strproc::tablet_content::*;

#[warn(unused_imports)]
#[warn(dead_code)]
fn main() {
    // read from stdin
    let mut content = String::new();
    let _ = std::io::stdin().read_to_string(&mut content);

    let mut tablet_content = TabletContent::new(&content);
    show(&mut tablet_content);

}

#[allow(dead_code)]
fn demo(){
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