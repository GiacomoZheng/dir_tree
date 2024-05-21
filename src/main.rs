use dir_tree::{resolve, tree};

use std::fs::File;
use std::io::Write;
use std::path::PathBuf;
use std::process::Command;
use clap::Parser;

#[derive(Parser)]
#[command(version, about = "Produce the tree of posts", long_about = None)]
#[command(next_line_help = false)]
struct Cli {
    /// Optional path to operate on
    path: Option<PathBuf>,
    /// Filter the tags select, leave it empty for select all
    #[arg(short, long, value_name = "TAGS")]
    tags: Vec<String>, // WARN: 有点小问题，filter掉一些tag的话图就连不起来了。
    /// Output the graph around this node, stop at that depth unless it is zero
    #[arg(short, long, value_name = "NODE")]
    rel : Option<String>,
    #[arg(short, long, value_name = "INT", default_value_t = 0)]
    depth : usize,
}

pub fn run(root: PathBuf, config : &str) -> String {
    tree(resolve(root)).to_dot_file(config)
}

fn main() {
	let cli = Cli::parse();

    if let Some(path) = cli.path {
        let rel_to_node = match cli.rel {
            Some(node) => Some((node, cli.depth)),
            None => {
                eprintln!("Warning: depth only works for rel version");
                None
            }
        };
        
        let dot = run(path, "rankdir=\"LR\"");
        let mut f = File::create("output.dot").unwrap();
        f.write(dot.as_bytes()).unwrap();
        eprintln!("dot: \n{}", dot);

        // "echo '{dot}' | dot -Tsvg > output.svg"
    }
}