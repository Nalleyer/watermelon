use clap::Parser;

#[derive(Debug, Parser)]
#[clap(about, version, author)]
pub struct Args {
    pub file: String,
}