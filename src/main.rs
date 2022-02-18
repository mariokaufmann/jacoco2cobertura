use std::path::Path;

use anyhow::{anyhow, Context};

use crate::jacoco::JacocoReport;

mod cli;
mod cobertura;
mod jacoco;
mod map;

fn main() -> anyhow::Result<()> {
    let args = cli::parse_args();

    let input_file_path = Path::new(&args.input_file);
    if !input_file_path.exists() && !args.ignore_missing {
        return Err(anyhow!("Input file is missing."));
    }

    let input_text =
        std::fs::read_to_string(&args.input_file).context("Could not read input file.")?;
    let jacoco_report: JacocoReport =
        quick_xml::de::from_str(&input_text).context("Could not parse Jacoco report.")?;
    let cobertura_report =
        map::map(jacoco_report).context("Could not map Jacoco report to Cobertura report.")?;

    let output = quick_xml::se::to_string(&cobertura_report)
        .context("Could not serialize Cobertura report.")?;
    std::fs::write(&args.output_file, output)
        .context("Could not write Cobertura report to output file.")?;

    Ok(())
}
