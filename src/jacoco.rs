use serde::Deserialize;

#[derive(Deserialize)]
pub struct JacocoReport {
    #[serde(rename = "sessioninfo")]
    pub session_info: Vec<JacocoSessionInfo>,
    #[serde(rename = "package", default)]
    pub packages: Vec<JacocoPackage>,
    #[serde(rename = "counter", default)]
    pub counters: Vec<JacocoCounter>,
}

#[derive(Deserialize)]
pub struct JacocoSessionInfo {
    pub start: u64,
}

#[derive(Deserialize)]
pub struct JacocoPackage {
    pub name: String,
    #[serde(rename = "class", default)]
    pub classes: Vec<JacocoClass>,
    #[serde(rename = "counter", default)]
    pub counters: Vec<JacocoCounter>,
    #[serde(rename = "sourcefile", default)]
    pub source_files: Vec<JacocoSourcefile>,
}

#[derive(Deserialize)]
pub struct JacocoClass {
    pub name: String,
    #[serde(rename = "sourcefilename")]
    pub source_file_name: String,
    #[serde(rename = "method", default)]
    pub methods: Vec<JacocoMethod>,
    #[serde(rename = "counter", default)]
    pub counters: Vec<JacocoCounter>,
}

#[derive(Deserialize)]
pub struct JacocoMethod {
    pub name: String,
    #[serde(rename = "desc")]
    pub description: String,
    pub line: u32,
    #[serde(rename = "counter", default)]
    pub counters: Vec<JacocoCounter>,
}

#[derive(Deserialize)]
pub struct JacocoCounter {
    #[serde(rename = "type")]
    pub counter_type: String,
    pub missed: u32,
    pub covered: u32,
}

#[derive(Deserialize)]
pub struct JacocoSourcefile {
    pub name: String,
    #[serde(rename = "line", default)]
    pub lines: Vec<JacocoLine>,
    #[serde(rename = "counter", default)]
    pub counters: Vec<JacocoCounter>,
}

#[derive(Deserialize)]
pub struct JacocoLine {
    #[serde(rename = "nr")]
    pub number: u32,
    #[serde(rename = "mi")]
    pub missed_instructions: u32,
    #[serde(rename = "ci")]
    pub covered_instructions: u32,
    #[serde(rename = "mb")]
    pub missed_branches: u32,
    #[serde(rename = "cb")]
    pub covered_branches: u32,
}
