pub type DefaultIx = usize;

use std::collections::{HashMap, HashSet};
use std::path::PathBuf;

mod file;
use file::get_yaml;
use file::walk;

mod node;
pub use node::Node;

/// to get info of all files
pub fn resolve(root: PathBuf) -> HashMap<DefaultIx, Node> {
    let mut set = HashSet::new();

    for path in walk(root) {
        if let Some(contents) = get_yaml(path) {
            let node = Node::parse(contents);
            node.try_add_to_set(&mut set).unwrap();
        }
    }

    let mut map = HashMap::new();

    for node in set.into_iter() {
        node.add_to_map(&mut map);
    }
    map
}

#[test] fn t_resolve() {
    let root = PathBuf::from("../_published");
    let titles = resolve(root).into_iter().map(|(_, e)| e.title()).collect::<Vec<_>>();
    eprintln!("titles: {:?}", titles);
    assert!(titles.contains(&String::from("古典代数几何：切锥")));
    assert!(!titles.contains(&String::from("cuspidal_curve_tan.svg")));
}

mod graph;
pub use graph::Graph;

impl Graph<String> {
    fn from_nodes(nodes: HashMap<DefaultIx, Node>) -> Graph<String> {
        let mut dot: Graph<String> = Graph::new_strict_digraph();

        // add nodes
        for (&id, node) in nodes.iter() {
            if node.is_root() {
                dot.add_root(id, node.title());
            } else {
                dot.add_node(id, node.title());
            }
        }
        // add edges
        for (&id, node) in nodes.iter() {
            for &j in node.parents.iter() {
                dot.add_edge(j, id);
            }
        }
        dot
    }
}

/// produce the dot file of all files by dependencies
pub fn tree(all_nodes: HashMap<DefaultIx, Node>) -> Graph<String> {
    let mut output_nodes = all_nodes.clone();

    for (_, node) in output_nodes.iter_mut() {
        eprintln!("title: {:?}", node.title());
        eprintln!("deps: {:?}", &node.body.dependencies);
        for dep_name in &node.body.dependencies {
            if let Some((&dep_id, _)) = all_nodes.iter().find(|&(_, e)| e.body.title == *dep_name) {
                node.parents.insert(dep_id as DefaultIx);
            } else {
                panic!("no such a dependency found")
            }
        }
    }

    Graph::from_nodes(output_nodes)
}