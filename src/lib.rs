use std::collections::{HashMap, VecDeque};
use std::collections::HashSet;
use std::path::PathBuf;

pub type DefaultIx = usize;

mod file;
use file::get_yaml;
use file::walk;

mod node;
pub use node::{MetaData, Node};

/// to get info of all files
pub fn resolve(root: PathBuf, tags: Vec<String>, rel_to_node: Option<(String, usize)>)
        -> Result<(HashMap<DefaultIx, Node>, Option<usize>), &'static str> {
    eprintln!("rel_to_node: {:?}", rel_to_node);
    let mut set = HashSet::new();
    let mut rel_md = None;

    let mut opt_depth = None;

    for path in walk(root) {
        if let Some(contents) = get_yaml(path) {
            let md = MetaData::parse(contents);
            if let Some((title, depth)) = &rel_to_node {
                if title == &md.title {
                    if opt_depth.is_none() {
                        rel_md = Some(md);
                        opt_depth = Some(*depth);
                        continue;
                    } else {
                        eprintln!("title: {:?}", title);
                        return Err("Repeated titles appears");
                    }
                }
            } 
            if tags.is_empty() || tags.iter().any(|tag| md.with_tag(tag)) {
                eprintln!("title: {:?}", md.title);
                md.try_add_to_set(&mut set)?;
            }
        }
    }

    let mut map = HashMap::new();
    if let Some(md) = rel_md {
        Node::from(md).add_to_map(&mut map);
    }

    for md in set.into_iter() {
        Node::from(md).add_to_map(&mut map);
    }
    Ok((map, opt_depth))
}

#[test] fn t_resolve() {
    let root = PathBuf::from("../_published");
    let titles = resolve(root, vec![], None).unwrap()
                                .0.into_iter().map(|(_, e)| e.title()).collect::<Vec<_>>();
    eprintln!("titles: {:?}", titles);
    assert!(titles.contains(&String::from("古典代数几何：切锥")));
    assert!(!titles.contains(&String::from("cuspidal_curve_tan.svg")));
    todo!("test for tags");
}

#[test] fn t_resolve_rel() {
    let root = PathBuf::from("../_published");
    let (map, opt_depth) = resolve(root, vec![], Some(("局部微分几何：（余）切空间".into(), 1))).unwrap();
    let titles: HashSet<String> = map.iter().map(|(_, e)| e.title()).collect();
    eprintln!("titles: {:?}", titles);
    assert!(opt_depth.is_some());
    assert_eq!(map[&0].title(), "局部微分几何：（余）切空间");
    assert!(titles.contains(&String::from("一般微分几何：子流形")));
    assert!(titles.contains(&String::from("一般微分几何：坐标与流形")));
}

mod graph;
pub use graph::Graph;

impl Graph<String> {
    fn from_nodes(nodes: HashMap<DefaultIx, Node>) -> Graph<String> {
        let mut dot: Graph<String> = Graph::new_strict_digraph("rankdir=\"LR\";\n");

        // add nodes
        for (&id, node) in nodes.iter() {
            dot.add_node(id, node.title());
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
fn tree(all_nodes: HashMap<DefaultIx, Node>) -> Graph<String> {
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

/// produce the dot file around some node
fn rel_tree(data: HashMap<DefaultIx, Node>, depth: usize) -> Graph<String> {
    let mut output_nodes = HashMap::new();

    // start bfs for parents
    let mut queue = VecDeque::new();
    queue.push_back((0 as DefaultIx, 0));
    let mut _count = 0;
    while !queue.is_empty() {
        eprintln!("round {_count}");
        eprintln!("queue_1: {:?}", queue);
        let (node_id, node_depth) = queue.pop_front().unwrap();        
        let mut node = data[&node_id].clone();

        // add the parent nodes until reach the depth
        if node_depth < depth {
            for dep_name in node.body.dependencies.iter() {
                if let Some((&dep_id, _)) = data.iter().find(|(_, e)| &e.body.title == dep_name) {
                    queue.push_back((dep_id, node_depth + 1));
                    node.parents.insert(dep_id);
                    eprintln!("node.parents.values: {:?}", node.parents.iter().map(|e| data[e].title()).collect::<Vec<_>>());
                } else {
                    panic!("no such a dependency found")
                }
            }
            eprintln!("parents: {:?}", data[&node_id].parents);
        }
        output_nodes.insert(node_id, node);
        _count += 1;
    }
    
    // start bfs for children
    // impossible

    Graph::from_nodes(output_nodes)
}

#[test] fn t_rel_tree() {
    let root = PathBuf::from("../_published");
    let (data, opt_depth) =  resolve(root, vec![], Some(("局部微分几何：（余）切空间".into(), 2))).unwrap();
    eprintln!("----- start -----");
    let _g = rel_tree(data, opt_depth.unwrap());
    unimplemented!()
}

pub fn run(root: PathBuf, tags: Vec<String>, rel_to_node: Option<(String, usize)>) -> String {
    let (set, opt_depth) =  resolve(root, tags, rel_to_node).unwrap();
    let graph = if let Some(depth) = opt_depth {
        rel_tree(set, depth)
    } else {
        tree(set)
    };
    graph.to_dot_file()
}

// #[test] fn t_rel_tree() {
//     let root = PathBuf::from("../_published");
//     run(root, vec![], Some(("一般微分几何：子流形".into(), 1)));
    
//     unimplemented!("test for tags");
// }