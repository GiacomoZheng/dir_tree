use clap::Parser;
// use dir_tree::walk;

/// Search for a pattern in a file and display the lines that contain it.
#[derive(Parser)]
struct Cli {
    // pattern: String,
    path: std::path::PathBuf,
}

fn main() {
	
}