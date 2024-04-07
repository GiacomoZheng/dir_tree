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
	nodes : Vec<(DefaultIx, N)>,
	edges : Vec<(DefaultIx, DefaultIx)>,
	config : String,
}
impl<N : Debug + Display> Graph<N> {
    pub fn to_dot_file(&self) -> String {
        let mut output = String::from(format!("{} {{\n", self.graph_type));

		output.push_str(&self.config);

		for (i, node) in self.nodes.iter() {
			output.push_str(&format!("    {i} [label=\"{}\"]\n", node));
		}
		
		for (i, j) in self.edges.iter() {
			output.push_str(&format!("    {i} -> {j}\n"));
		}

		output.push_str("}");

		output
    }
}
impl<N> Graph<N> {
	fn new(t : GraphType, config : &str) -> Self {
		Self {
			graph_type: t,
			nodes: Vec::new(),
			edges: Vec::new(),
			config : config.to_string()
		}
	}
	pub fn new_strict_digraph(config : &str) -> Self {
		Self::new(GraphType::DiGraph {is_strict : true}, config)
	}
	pub fn new_strict_graph(config : &str) -> Self {
		Self::new(GraphType::Graph {is_strict : true}, config)
	}

	pub fn add_node(&mut self, id : DefaultIx, info : N) {
		self.nodes.push((id, info));
	}
	pub fn add_edge(&mut self, i : DefaultIx, j : DefaultIx) {
		self.edges.push((i, j));
	}
}

#[test] fn dot_file() {
	let mut graph: Graph<String> = Graph::new_strict_digraph("");
	graph.add_node(0, "A".to_string());
	graph.add_node(1, "B".to_string());

	graph.add_edge(0, 1);

	assert_eq!(graph.to_dot_file(), "strict digraph {\n    0 [label=\"A\"]\n    1 [label=\"B\"]\n    0 -> 1\n}".to_string())
}