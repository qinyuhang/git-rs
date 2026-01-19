use clap::Parser;

#[derive(Parser)]
#[command(name = "git-rs")]
#[command(about = "A Git implemented in Rust", long_about = None)]
struct Cli {
    #[arg(required = true)]
    command: String,
}

fn main() {
    let cli = Cli::parse();
    println!("Command: {}", cli.command);
}
