use anyhow::Result;
use clap::Parser;

#[derive(Parser)]
#[clap(about = "Convert Rust enums to C tagged unions.", version = "0.1")]
struct Args {
    /// A Rust source file containing an enum.
    infile: String,
    #[clap(short, long)]
    /// Path to the output file to write C code.
    outfile: Option<String>,
    /// Format the outputted C code.
    #[clap(short, long, default_value_t = true)]
    format: bool,
}

fn main() -> Result<()> {
    let args: Args = Args::parse();
    tagmap::convert(&args.infile, args.outfile.as_deref(), args.format)?;
    Ok(())
}
