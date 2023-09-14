use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;

use walkdir::{DirEntry, WalkDir};
use serde::Deserialize;
use yaml_front_matter::YamlFrontMatter as YFM;

fn is_hidden(entry: &DirEntry) -> bool {
    // return false;
    entry.file_name()
         .to_str()
         .map(|s| s.starts_with(".") && s != ".")
         .unwrap_or(false)
}

fn walk(root : PathBuf) -> impl Iterator<Item = PathBuf> {
    let walker = WalkDir::new(root);
    walker.into_iter()
          .filter_entry(|e| !is_hidden(e))
          .map(|e| e.expect("walkdir::Error").path().to_path_buf())
          .filter(|e| e.is_file())
          .filter(|e| e.extension().unwrap() == "md")
}

#[derive(Deserialize, Default, Debug)]
struct ActIndex {
    title : String,
    date : String,
    description: String,
    preliminaries: HashSet<String>,
    tags : HashSet<String>
}
impl ActIndex {
    fn parse(s : String) -> ActIndex {
        YFM::parse::<ActIndex>(s.as_str()).unwrap().metadata
    }
}

#[test] fn t_actindex_parse() {
    let s = r"---
    title: '一般代数几何：概形族'
    date: 2023-02-03
    description:
    preliminaries: []
    tags:
      - 代数几何
      - 一般代数几何
    ---";
    let ai = ActIndex::parse(s.to_string());
    assert_eq!(ai.title, "一般代数几何：概形族");
    assert!(ai.tags.contains("代数几何"));
}

fn resolve(root : PathBuf) -> HashMap<String, ActIndex> {
    eprintln!("{:?}", "start");
    let mut res = HashMap::new();
    for path in walk(root) {
        eprintln!("path: {:?}", path);
        let buf_reader = BufReader::new(File::open(path).unwrap());
        let mut iterator = buf_reader.lines();
        if let Some(Ok(s)) = iterator.next() {
            if s.starts_with("---") {
                let mut contents = String::from("---");
                for line in iterator.filter_map(|e| e.ok()) {
                    eprintln!("line: {:?}", line);
                    contents.push_str("\n");
                    if line.starts_with("---") {
                        contents.push_str("---");
                        let actIndex = ActIndex::parse(contents);
                        let name = actIndex.title.clone();
                        res.insert(name, actIndex);
                        break;
                    } else {
                        contents.push_str(&line);
                    }
                }
            }
        }
        eprintln!("{:?}", res);
    }
    res
}

#[test] fn t_resolve() {
    let root = PathBuf::from("../_published");
    let ai = resolve(root);
    eprintln!("{:?}", ai.keys().collect::<Vec<_>>());
    assert!(ai.contains_key(&String::from("古典代数几何：切锥")));
    assert!(!ai.contains_key(&String::from("cuspidal_curve_tan.svg")));
}
