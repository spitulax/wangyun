use crate::{
    modern::Variants,
    prog::{baxter, display},
};
use clap::Parser;
use regexes::regexes;

mod middle;
mod modern;
mod old_bs;
mod old_zh;
mod prog;
mod regexes;
mod request;
mod utils;

#[derive(Debug, Parser)]
#[command(version, about)]
struct Args {
    /// Characters to look up
    chars: String,

    /// Show Middle Chinese rime info
    #[arg(short, long)]
    middle: bool,

    /// Show Old Chinese rime info
    #[arg(short, long)]
    old: bool,

    /// Show pronunciations of modern variants
    #[arg(short('M'), long)]
    modern: Vec<Variants>,

    /// Only print Baxter's Middle Chinese transcription
    #[arg(short, long)]
    baxter: bool,
}

fn main() -> reqwest::Result<()> {
    let args = Args::parse();

    if args.baxter {
        baxter(&args)?;
    } else {
        display(&args)?;
    }

    Ok(())
}
