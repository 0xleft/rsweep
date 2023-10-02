mod sweeper;

extern crate clap;
extern crate tokio;

use clap::Parser;
use sweeper::sweeper::Sweeper;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    range: String,

    #[arg(short, long, default_value = "all")]
    ports: String,

    #[arg(short, long, default_value = "5")]
    threads: u32,

    #[arg(short, long, default_value = "false")]
    verbose: bool,
}

#[tokio::main]
async fn main() {
    let args = Args::parse();

    if args.verbose {
        println!("{:?}", args);
    }

    let sweeper = Sweeper::new(
        args.range,
        args.ports,
        args.threads,
        args.verbose,
    );

    sweeper.run().await;
}