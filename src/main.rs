use dir_tree::run;

use std::path::PathBuf;
use std::process::Command;
use clap::Parser;

#[derive(Parser)]
#[command(version, about = "Produce the graph of posts", long_about = None)]
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
        
        let dot = run(path, cli.tags, rel_to_node);
        eprintln!("dot: \n{}", dot);

        let _output = if cfg!(target_os = "windows") {
            panic!("unimplemented!")
        } else {
            Command::new("sh")
                .arg("-c")
                .arg(format!("echo '{dot}' | dot -Tsvg > output.svg"))
                .output()
                .expect("failed to execute process")
        };
    }
}