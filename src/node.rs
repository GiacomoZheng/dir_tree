use std::fmt::Debug;
use std::rc::Rc;
use std::hash::Hash;
use std::collections::{HashSet, HashMap};

use super::DefaultIx;

use serde::Deserialize;
use yaml_front_matter::YamlFrontMatter as YFM;

#[derive(Deserialize, Default, Debug)]
pub struct MetaData {
    pub title: String,
    pub date: String,
    pub description: String,
    pub dependencies: HashSet<String>,
    pub tags: HashSet<String>,
}
// impl PartialEq for MetaData {
//     fn eq(&self, other: &Self) -> bool {
//         self.title == other.title
//         //  && self.date == other.date && self.description == other.description && self.dependencies == other.dependencies && self.tags == other.tags
//     }
// }
// impl Hash for MetaData {
//     fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
//         self.title.hash(state);
//     }
// }
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
    let md: MetaData = YFM::parse(s).unwrap().metadata;
    assert_eq!(md.title, "一般代数几何：概形族");
    assert!(md.tags.contains("代数几何"));
}

#[derive(Clone, Debug)]
pub struct Node {
    pub body: Rc<Box<MetaData>>,
    pub parents: HashSet<DefaultIx>,
}
impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        self.title() == other.title()
        //  && self.date == other.date && self.description == other.description && self.dependencies == other.dependencies && self.tags == other.tags
    }
}
impl Eq for Node {}
impl Hash for Node {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.title().hash(state);
    }
}
impl Node {
    pub fn from(md: MetaData) -> Node {
        Node {
            body: Rc::new(Box::new(md)),
            parents: HashSet::new()
        }
    }

    pub fn parse(yfm: String) -> Node {
        Node::from(YFM::parse(yfm.as_str()).unwrap().metadata)
    }

    pub fn title(&self) -> String {
        self.body.title.clone()
    }

    pub fn try_add_to_set(self, set: &mut HashSet<Node>) -> Result<(), &'static str> {
        if !set.insert(self) {
            Err("Repeated titles appears")
        } else {
            Ok(())
        }
    }

	pub fn add_to_map(self, nodes: &mut HashMap<DefaultIx, Node>) {
        nodes.insert(nodes.len(), self.clone());
    }
}