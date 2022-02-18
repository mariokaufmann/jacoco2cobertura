use clap::Parser;

pub fn parse_args() -> Args {
    Args::parse()
}

/// Utility to convert a Jacoco coverage report to a Cobertura coverage report.
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None, )]
pub struct Args {
    /// Input file with the Jacoco report
    #[clap(short, long)]
    pub input_file: String,

    /// Output file to write the Cobertura report to
    #[clap(short, long)]
    pub output_file: String,

    /// Whether to ignore a missing input file
    #[clap(long)]
    pub ignore_missing: bool,
}
