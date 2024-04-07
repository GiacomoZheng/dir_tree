use std::path::PathBuf;
use walkdir::{DirEntry, WalkDir};

fn is_hidden(entry: &DirEntry) -> bool {
    entry.file_name()
         .to_str()
         .map(|s| s.starts_with(".") && s != ".")
         .unwrap_or(false)
}
fn is_deleted(entry: &DirEntry) -> bool {
    entry.file_name()
         .to_str()
         .map(|s| s.ends_with("_deleted"))
         .unwrap_or(false)
}

/// to get the path of all files
pub fn walk(root : PathBuf) -> impl Iterator<Item = PathBuf> {
    let walker = WalkDir::new(root);
    walker.into_iter()
          .filter_entry(|e| !is_hidden(e) && !is_deleted(e))
          .map(|e| e.expect("walkdir::Error").path().to_path_buf())
          .filter(|e| e.is_file())
          .filter(|e| e.extension().unwrap() == "md")
}

/// to get yaml info of a file
pub fn get_yaml(path : PathBuf) -> Option<String> {
    use std::fs::File;
    use std::io::{BufRead, BufReader};

    let buf_reader = BufReader::new(File::open(path).unwrap());
    let mut iterator = buf_reader.lines();
    if let Some(Ok(s)) = iterator.next() {
        if s.starts_with("---") {
            let mut contents = String::from("---");
            for line in iterator.filter_map(|e| e.ok()) {
                // eprintln!("line: {:?}", line);
                contents.push_str("\n");
                if line.starts_with("---") {
                    contents.push_str("---");
                    return Some(contents);
                } else {
                    contents.push_str(&line);
                }
            }
        }
    }
    None
}

