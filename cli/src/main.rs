use args::Args;
use clap::Parser;

mod args;
mod opt;
mod port;
mod server;
mod client;
mod common;

fn main() -> anyhow::Result<()> {
    pretty_env_logger::init();

    Args::parse().execute()?;
    Ok(())
}
