use args::Args;
use clap::Parser;

mod args;
mod client;
mod common;
mod opt;
mod port;
mod server;

fn main() -> anyhow::Result<()> {
    pretty_env_logger::init();

    Args::parse().execute()?;
    Ok(())
}
