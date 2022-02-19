use clap::Parser;

pub fn parse_args() -> Args {
    Args::parse()
}

/// Utility to convert a Jacoco coverage report to a Cobertura coverage report.
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
pub struct Args {
    /// Input file with the Jacoco report
    #[clap(short, long)]
    pub input_file: String,

    /// Output file to write the Cobertura report to
    #[clap(short, long)]
    pub output_file: String,

    /// The absolute paths to the source roots (i.e. the folders where the source files mentioned
    /// in the input report are located.
    #[clap(long)]
    pub source_root: Vec<String>,

    /// Whether to ignore a missing input file
    #[clap(long)]
    pub ignore_missing: bool,
}
