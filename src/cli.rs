use clap::Parser;

pub fn parse_args() -> Args {
    Args::parse()
}

/// Utility to convert a Jacoco coverage report to a Cobertura coverage report.
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// Input file with the Jacoco report
    #[arg(short, long)]
    pub input_file: String,

    /// Output file to write the Cobertura report to
    #[arg(short, long)]
    pub output_file: String,

    /// The absolute paths to the source roots (i.e. the folders where the source files mentioned
    /// in the input report are located.
    #[arg(long)]
    pub source_root: Vec<String>,

    /// Whether to ignore a missing input file
    #[arg(long)]
    pub ignore_missing: bool,
}

#[cfg(test)]
mod tests {
    use clap::CommandFactory;
    use crate::cli::Args;

    #[test]
    fn verify_cli() {
        Args::command().debug_assert()
    }
}
