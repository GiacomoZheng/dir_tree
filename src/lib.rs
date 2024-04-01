use std::collections::HashSet;
use std::path::PathBuf;
use std::rc::Rc;
use std::hash::Hash;

mod file;
use file::get_yaml;
use file::walk;

use serde::Deserialize;
use yaml_front_matter::YamlFrontMatter as YFM;

#[derive(Deserialize, Default, Debug)]
pub struct ActIndex {
    pub title : String,
    pub date : String,
    pub description: String,
    pub dependencies: HashSet<String>,
    pub tags : HashSet<String>,
}
impl ActIndex {
    fn parse(s : String) -> ActIndex {
        YFM::parse::<ActIndex>(s.as_str()).unwrap().metadata
    }
}
impl PartialEq for ActIndex {
    fn eq(&self, other: &Self) -> bool {
        self.title == other.title
        //  && self.date == other.date && self.description == other.description && self.dependencies == other.dependencies && self.tags == other.tags
    }
}
impl Eq for ActIndex {}
impl Hash for ActIndex {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.title.hash(state);
    }
}
#[test] fn t_actindex_parse() {
    let s = r"---
    title: '一般代数几何：概形族'
    date: 2023-02-03
    description:
    dependencies: []
    tags:
      - 代数几何
      - 一般代数几何
    ---";
    let ai = ActIndex::parse(s.to_string());
    assert_eq!(ai.title, "一般代数几何：概形族");
    assert!(ai.tags.contains("代数几何"));
}

/// to get info of all files
pub fn resolve(root : PathBuf) -> HashSet<ActIndex> {
    let mut set = HashSet::new();
    for path in walk(root) {
        eprintln!("path: {:?}", path);
        if let Some(contents) = get_yaml(path) {
            if !set.insert(ActIndex::parse(contents)) {
                panic!("Repeated titles appears")
            }
        }
    }
    set
}

#[test] fn t_resolve() {
    let root = PathBuf::from("../_published");
    let titles = resolve(root).into_iter().map(|e| e.title.clone()).collect::<Vec<_>>();
    eprintln!("titles: {:?}", titles);
    assert!(titles.contains(&String::from("古典代数几何：切锥")));
    assert!(!titles.contains(&String::from("cuspidal_curve_tan.svg")));
}

mod graph;
pub use graph::{DotFile, DefaultIx};

#[derive(Clone)]
struct Node {
    body : Rc<Box<ActIndex>>,
    id : DefaultIx,
    parents : HashSet<DefaultIx>,
}
impl Node {
    fn title(&self) -> String {
        self.body.title.clone()
    }
}

/// to draw the tree of all files by dependencies
fn tree(data : Vec<ActIndex>) -> String {
    let mut vec_proto = Vec::new();
    for (id, ai) in data.into_iter().enumerate() {
        vec_proto.push(Node {
            body : Rc::new(Box::new(ai)),
            id : id as DefaultIx,
            parents : HashSet::new()
        })
    }

    let mut vec = vec_proto.clone();

    for node in vec.iter_mut() {
        eprintln!("title : {:?}", node.title());
        eprintln!("deps : {:?}", &node.body.dependencies);
        for dep in &node.body.dependencies {
            if let Some(index) = vec_proto.iter().position(|e| e.body.title == *dep) {
                node.parents.insert(index as DefaultIx);
            } else {
                
                panic!("no such a dependency found")
            }
        }
    }

    let mut dot: DotFile<String> = DotFile::new_strict_digraph("rankdir=\"LR\";");

    for i in 0..vec.len() {
        dot.add_node(i, vec[i].title());
    }

    for node in vec.iter() {
        for &j in node.parents.iter() {
            dot.add_edge(j, node.id);
        }
    }

    dot.to_string()
}

pub fn run(root : PathBuf) -> String {
    tree(resolve(root).into_iter().collect())
}