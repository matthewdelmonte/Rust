use clap::Parser;

/// simple program to greet a person
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// name of the person to greet
    #[arg(short, long)]
    name: String,

    /// number of times to greet
    #[arg(short, long, default_value_t = 1)]
    count: u8,
}

fn main() {
    let args = Args::parse();

    for _ in 0..args.count {
        println!("Hello, {}!", args.name);
    }
}

// to execute run this command: $ cargo run -- --name Matthew or "Matthew"
// to execute more than the default 1 greeting, $ cargo run -- --name "Matthew" --count 2