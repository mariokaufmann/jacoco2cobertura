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
    if !input_file_path.exists() {
        return if args.ignore_missing {
            Ok(())
        } else {
            Err(anyhow!("Input file is missing."))
        };
    }

    let input_text =
        std::fs::read_to_string(&args.input_file).context("Could not read input file.")?;
    let jacoco_report: JacocoReport =
        quick_xml::de::from_str(&input_text).context("Could not parse Jacoco report.")?;
    let cobertura_report = map::map(jacoco_report, args.source_root)
        .context("Could not map Jacoco report to Cobertura report.")?;

    let output = quick_xml::se::to_string(&cobertura_report)
        .context("Could not serialize Cobertura report.")?;
    std::fs::write(&args.output_file, output)
        .context("Could not write Cobertura report to output file.")?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn map() {
        let mut directory = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        directory.push("fixtures");
        let mut input_file_path = directory.clone();
        input_file_path.push("jacoco_input.xml");
        let input_text = std::fs::read_to_string(input_file_path).unwrap();
        let jacoco_report: JacocoReport = quick_xml::de::from_str(&input_text).unwrap();
        let cobertura_report =
            map::map(jacoco_report, vec!["project/src/main/java".to_owned()]).unwrap();
        let mut output_file_path = directory;
        output_file_path.push("expected_cobertura_output.xml");
        let output_text = quick_xml::se::to_string(&cobertura_report).unwrap();

        let expected_text = std::fs::read_to_string(output_file_path).unwrap();

        assert_eq!(output_text, expected_text);
    }
}
