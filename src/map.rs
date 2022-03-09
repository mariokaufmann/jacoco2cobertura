use anyhow::Context;

use crate::cobertura::{
    CoberturaClass, CoberturaClasses, CoberturaCondition, CoberturaConditions, CoberturaCoverage,
    CoberturaLine, CoberturaLines, CoberturaMethod, CoberturaMethods, CoberturaPackage,
    CoberturaSource, CoberturaSources,
};
use crate::jacoco::{
    JacocoClass, JacocoCounter, JacocoLine, JacocoMethod, JacocoPackage, JacocoSourcefile,
};
use crate::JacocoReport;

struct CounterInfo {
    line_rate: String,
    branch_rate: String,
    complexity: String,
}

pub fn map(
    jacoco_report: JacocoReport,
    source_roots: Vec<String>,
) -> anyhow::Result<CoberturaCoverage> {
    let packages: anyhow::Result<Vec<CoberturaPackage>> = jacoco_report
        .packages
        .into_iter()
        .map(map_package)
        .collect();
    let packages = packages.context("Could not map Cobertura package.")?;
    let counter_info = map_counter_info(&jacoco_report.counters);

    Ok(CoberturaCoverage {
        timestamp: (jacoco_report.session_info.start as f64) / 1000_f64,
        sources: map_source_roots(source_roots),
        packages,
        complexity: counter_info.complexity,
        line_rate: counter_info.line_rate,
        branch_rate: counter_info.branch_rate,
    })
}

fn map_source_roots(sources: Vec<String>) -> CoberturaSources {
    let source_roots = if sources.is_empty() {
        vec![CoberturaSource {
            value: ".".to_owned(),
        }]
    } else {
        sources
            .into_iter()
            .map(|source| CoberturaSource { value: source })
            .collect()
    };
    CoberturaSources {
        source: source_roots,
    }
}

fn map_package(jacoco_package: JacocoPackage) -> anyhow::Result<CoberturaPackage> {
    let name = map_name(&jacoco_package.name);
    let classes: anyhow::Result<Vec<CoberturaClass>> = jacoco_package
        .classes
        .into_iter()
        .map(|class| map_class(class, &jacoco_package.source_files))
        .collect();
    let classes = classes.context("Could not map Cobertura class.")?;

    let counter_info = map_counter_info(&jacoco_package.counters);

    Ok(CoberturaPackage {
        name,
        classes: CoberturaClasses { class: classes },
        complexity: counter_info.complexity,
        line_rate: counter_info.line_rate,
        branch_rate: counter_info.branch_rate,
    })
}

fn map_class(
    jacoco_class: JacocoClass,
    jacoco_source_files: &[JacocoSourcefile],
) -> anyhow::Result<CoberturaClass> {
    let name = map_name(&jacoco_class.name);
    let file_name = map_cobertura_file_name(&jacoco_class.source_file_name, &jacoco_class.name)
        .context("Could not Cobertura file name.")?;
    let counter_info = map_counter_info(&jacoco_class.counters);
    let source_file = jacoco_source_files
        .iter()
        .find(|file| file.name.eq(&jacoco_class.source_file_name))
        .with_context(|| {
            format!(
                "Did not find source file element ({}).",
                &jacoco_class.source_file_name
            )
        })?;

    let class_jacoco_source_lines = &source_file.lines;

    let lines = class_jacoco_source_lines.iter().map(map_line).collect();

    let mut methods = Vec::new();
    let mut method_line_numbers: Vec<u32> = jacoco_class
        .methods
        .iter()
        .map(|method| method.line)
        .collect();
    method_line_numbers.sort_unstable();
    method_line_numbers.dedup();

    for method in jacoco_class.methods.iter() {
        let next_line = get_next_method_line(method.line, &method_line_numbers);
        let lines: Vec<&JacocoLine> = class_jacoco_source_lines
            .iter()
            .filter(|line| is_relevant_line(method, next_line, line))
            .collect();
        let cobertura_method = map_method(method, &lines);
        methods.push(cobertura_method);
    }

    Ok(CoberturaClass {
        name,
        file_name,
        lines: CoberturaLines { line: lines },
        methods: CoberturaMethods { method: methods },
        complexity: counter_info.complexity,
        line_rate: counter_info.line_rate,
        branch_rate: counter_info.branch_rate,
    })
}

/** Determines whether the given line is relevant for the given method. */
fn is_relevant_line(
    jacoco_method: &JacocoMethod,
    next_jacoco_line: Option<u32>,
    line: &JacocoLine,
) -> bool {
    if let Some(next_jacoco_line) = next_jacoco_line {
        return line.number >= jacoco_method.line && line.number < next_jacoco_line;
    }
    line.number >= jacoco_method.line
}

fn get_next_method_line(line: u32, lines: &[u32]) -> Option<u32> {
    let index = lines
        .iter()
        .position(|current_line| *current_line == line)?;
    let line = lines.get(index + 1)?;
    Some(*line)
}

fn map_method(
    jacoco_method: &JacocoMethod,
    method_jacoco_lines: &[&JacocoLine],
) -> CoberturaMethod {
    let counter_info = map_counter_info(&jacoco_method.counters);
    let lines = method_jacoco_lines
        .iter()
        .map(|line| map_line(*line))
        .collect();

    CoberturaMethod {
        name: jacoco_method.name.clone(),
        signature: jacoco_method.description.clone(),
        lines: CoberturaLines { line: lines },
        complexity: counter_info.complexity,
        line_rate: counter_info.line_rate,
        branch_rate: counter_info.branch_rate,
    }
}

fn map_line(jacoco_line: &JacocoLine) -> CoberturaLine {
    let hits = if jacoco_line.covered_instructions > 0 {
        1
    } else {
        0
    };

    let conditions;
    let branch;
    let condition_coverage;

    let total_branches = jacoco_line.covered_branches + jacoco_line.missed_branches;
    if total_branches > 0 {
        let percentage = format!(
            "{}%",
            (((100 * jacoco_line.covered_branches) as f64) / (total_branches as f64)).round()
                as u32
        );
        condition_coverage = Some(format!(
            "{} ({}/{})",
            percentage, jacoco_line.covered_branches, total_branches
        ));
        branch = true;

        let condition = CoberturaCondition {
            number: 0,
            condition_type: "jump".to_owned(),
            coverage: percentage,
        };
        conditions = vec![condition];
    } else {
        conditions = Vec::new();
        branch = false;
        condition_coverage = None;
    }

    CoberturaLine {
        number: jacoco_line.number,
        hits,
        branch,
        conditions: CoberturaConditions {
            condition: conditions,
        },
        condition_coverage,
    }
}

fn map_counter_info(counters: &[JacocoCounter]) -> CounterInfo {
    let mut line_rate: Option<f64> = None;
    let mut branch_rate: Option<f64> = None;
    let mut complexity: Option<f64> = None;

    for counter in counters {
        let total = (counter.covered + counter.missed) as f64;
        match counter.counter_type.as_str() {
            "COMPLEXITY" => complexity = Some(total),
            "BRANCH" => branch_rate = Some((counter.covered as f64) / total),
            "LINE" => line_rate = Some((counter.covered as f64) / total),
            _ => {}
        }
    }

    CounterInfo {
        line_rate: map_optional_float(line_rate),
        branch_rate: map_optional_float(branch_rate),
        complexity: map_optional_float(complexity),
    }
}

fn map_name(name: &str) -> String {
    name.replace("/", ".")
}

fn map_optional_float(value: Option<f64>) -> String {
    let value = value.unwrap_or(0.0);
    format!("{}", value)
}

fn map_cobertura_file_name(jacoco_source_file_name: &str, name: &str) -> anyhow::Result<String> {
    let extension_position = jacoco_source_file_name.rfind('.').with_context(|| {
        format!(
            "Could not extract source file extension from file name ({}).",
            jacoco_source_file_name
        )
    })?;
    let extension = &jacoco_source_file_name[extension_position..];
    Ok(format!("{}{}", name, extension))
}
