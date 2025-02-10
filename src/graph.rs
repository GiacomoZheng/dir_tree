use std::fmt::{Debug, Display};

use super::DefaultIx;

#[derive(Debug)]
enum GraphType {
	Graph {is_strict : bool},
	DiGraph {is_strict : bool},
}
impl Display for GraphType {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			GraphType::Graph {is_strict : true} => write!(f, "strict graph"),
			GraphType::Graph {is_strict : false} => write!(f, "graph"),
			GraphType::DiGraph {is_strict : true} => write!(f, "strict digraph"),
			GraphType::DiGraph {is_strict : false} => write!(f, "digraph"),
		}
	}
}


#[derive(Debug)]
pub struct Graph<N> {
    graph_type : GraphType,
	nodes : Vec<(DefaultIx, N, String)>,
	edges : Vec<(DefaultIx, DefaultIx)>
}
impl<N : Debug + Display> Graph<N> {
    pub fn to_dot_file(&self, config : &str) -> String {
        let mut output = String::from(format!("{} {{\n", self.graph_type));

		output.push_str(&format!("{}\n", config));

		for (i, node, layout) in self.nodes.iter() {
			output.push_str(&format!("    {i} [label=\"{}\", {}]\n", node, layout));
		}
		
		for (i, j) in self.edges.iter() {
			output.push_str(&format!("    {i} -> {j}\n"));
		}

		output.push_str("}");

		output
    }
}
impl<N> Graph<N> {
	fn new(t : GraphType) -> Self {
		Self {
			graph_type: t,
			nodes: Vec::new(),
			edges: Vec::new()
		}
	}
	pub fn new_strict_digraph() -> Self {
		Self::new(GraphType::DiGraph {is_strict : true})
	}
	pub fn new_strict_graph() -> Self {
		Self::new(GraphType::Graph {is_strict : true})
	}

	pub fn add_node(&mut self, id : DefaultIx, info : N) {
		self.nodes.push((id, info, String::from("shape=ellipse,")));
	}
	pub fn add_root(&mut self, id : DefaultIx, info : N) {
		self.nodes.push((id, info, String::from("shape=ellipse, peripheries=2,")));
	}
	pub fn add_edge(&mut self, i : DefaultIx, j : DefaultIx) {
		self.edges.push((i, j));
	}
}

#[test] fn dot_file() {
	let mut graph: Graph<String> = Graph::new_strict_digraph();
	graph.add_node(0, "A".to_string());
	graph.add_node(1, "B".to_string());

	graph.add_edge(0, 1);
	let mut dot_str = graph.to_dot_file("");
	dot_str.retain(|c| !c.is_whitespace());

	assert_eq!(dot_str, "strictdigraph{0[label=\"A\"]1[label=\"B\"]0->1}".to_string())
}