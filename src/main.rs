use dir_tree::run;

use std::path::PathBuf;
use std::process::Command;

use clap::Parser;


#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    /// Optional path to operate on
    path: Option<PathBuf>,

    // /// Sets a custom config file
    // #[arg(short, long, value_name = "FILE")]
    // config: Option<PathBuf>,
}

fn main() {
	let cli = Cli::parse();

    // You can check the value provided by positional arguments, or option arguments
    if let Some(path) = cli.path {
        let dot = run(path);
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