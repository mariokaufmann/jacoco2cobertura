use serde::Serialize;

#[derive(Serialize)]
#[serde(rename = "coverage")]
pub struct CoberturaCoverage {
    pub timestamp: f64,
    pub sources: CoberturaSources,
    pub packages: Vec<CoberturaPackage>,
    #[serde(rename = "line-rate")]
    pub line_rate: String,
    #[serde(rename = "branch-rate")]
    pub branch_rate: String,
    pub complexity: String,
}

#[derive(Serialize)]
pub struct CoberturaSources {
    pub source: Vec<CoberturaSource>,
}

#[derive(Serialize)]
pub struct CoberturaSource {
    #[serde(rename = "$value")]
    pub value: String,
}

#[derive(Serialize)]
pub struct CoberturaPackage {
    pub name: String,
    pub classes: CoberturaClasses,
    #[serde(rename = "line-rate")]
    pub line_rate: String,
    #[serde(rename = "branch-rate")]
    pub branch_rate: String,
    pub complexity: String,
}

#[derive(Serialize)]
pub struct CoberturaClasses {
    pub class: Vec<CoberturaClass>,
}

#[derive(Serialize)]
pub struct CoberturaClass {
    pub name: String,
    pub methods: CoberturaMethods,
    pub lines: CoberturaLines,
    #[serde(rename = "filename")]
    pub file_name: String,
    #[serde(rename = "line-rate")]
    pub line_rate: String,
    #[serde(rename = "branch-rate")]
    pub branch_rate: String,
    pub complexity: String,
}

#[derive(Serialize)]
pub struct CoberturaMethods {
    pub method: Vec<CoberturaMethod>,
}

#[derive(Serialize)]
pub struct CoberturaMethod {
    pub name: String,
    pub signature: String,
    pub lines: CoberturaLines,
    #[serde(rename = "line-rate")]
    pub line_rate: String,
    #[serde(rename = "branch-rate")]
    pub branch_rate: String,
    pub complexity: String,
}

#[derive(Serialize)]
pub struct CoberturaLines {
    pub line: Vec<CoberturaLine>,
}

#[derive(Serialize)]
pub struct CoberturaLine {
    pub number: u32,
    pub hits: u32,
    pub branch: bool,
    #[serde(rename = "condition-coverage")]
    pub condition_coverage: Option<String>,
    pub conditions: CoberturaConditions,
}

#[derive(Serialize)]
pub struct CoberturaConditions {
    pub condition: Vec<CoberturaCondition>,
}

#[derive(Serialize)]
pub struct CoberturaCondition {
    pub number: u32,
    #[serde(rename = "type")]
    pub condition_type: String,
    pub coverage: String,
}
