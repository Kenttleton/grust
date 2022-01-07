use clap::{Parser};

#[derive(Parser, Debug)]
#[clap(about, author, version)]
struct Args {
    #[clap(short, long)]
    name: String,
    #[clap(short, long, default_value_t = 1)]
    count: u8,
}

fn main() {
    let args = Args::parse();
    for _ in 0..args.count {
        println!("Hello, {}!", args.name);
    }
}
