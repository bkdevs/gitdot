use clap::Parser;

use gitdot_cli::Args;

fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    for _ in 0..args.count {
        println!("Hello {}!", args.name);
    }

    Ok(())
}
