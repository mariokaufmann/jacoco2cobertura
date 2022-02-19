use std::io::Write;
use std::path::Path;

use anyhow::{anyhow, Context};

use crate::jacoco::JacocoReport;

mod cli;
mod cobertura;
mod jacoco;
mod map;

fn convert(input_file: &str, output_file: &str, source_root: Vec<String>) -> anyhow::Result<()> {
    let input_text = std::fs::read_to_string(input_file).context("Could not read input file.")?;
    let jacoco_report: JacocoReport =
        quick_xml::de::from_str(&input_text).context("Could not parse Jacoco report.")?;
    let cobertura_report = map::map(jacoco_report, source_root)
        .context("Could not map Jacoco report to Cobertura report.")?;

    let output = quick_xml::se::to_string(&cobertura_report)
        .context("Could not serialize Cobertura report.")?;
    let mut file = std::fs::File::create(output_file).context("Could not create output file.")?;
    file.write_all("<?xml version=\"1.0\" ?>".as_bytes())
        .context("Could not write header into output file.")?;
    file.write_all(output.as_bytes())
        .context("Could not write report into output file.")?;

    Ok(())
}

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

    convert(&args.input_file, &args.output_file, args.source_root)?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use super::*;

    #[test]
    fn map() {
        let mut directory = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        directory.push("fixtures");
        let mut input_file_path = directory.clone();
        input_file_path.push("jacoco_input.xml");
        let input_file_path = input_file_path.as_os_str().to_str().unwrap();

        convert(
            input_file_path,
            "output.xml",
            vec!["project/src/main/java".to_owned()],
        )
        .unwrap();

        let mut expected_output_file_path = directory;
        expected_output_file_path.push("expected_cobertura_output.xml");
        let expected_output_file_path = expected_output_file_path.as_os_str().to_str().unwrap();
        let expected_text = std::fs::read_to_string(expected_output_file_path).unwrap();
        let actual_text = std::fs::read_to_string("output.xml").unwrap();

        std::fs::remove_file("output.xml").unwrap();

        assert_eq!(actual_text, expected_text);
    }
}
