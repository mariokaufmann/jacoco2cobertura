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
    #[serde(rename = "@start")]
    pub start: u64,
}

#[derive(Deserialize)]
pub struct JacocoPackage {
    #[serde(rename = "@name")]
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
    #[serde(rename = "@name")]
    pub name: String,
    #[serde(rename = "@sourcefilename")]
    pub source_file_name: String,
    #[serde(rename = "method", default)]
    pub methods: Vec<JacocoMethod>,
    #[serde(rename = "counter", default)]
    pub counters: Vec<JacocoCounter>,
}

#[derive(Deserialize)]
pub struct JacocoMethod {
    #[serde(rename = "@name")]
    pub name: String,
    #[serde(rename = "@desc")]
    pub description: String,
    #[serde(rename = "@line")]
    pub line: u32,
    #[serde(rename = "counter", default)]
    pub counters: Vec<JacocoCounter>,
}

#[derive(Deserialize)]
pub struct JacocoCounter {
    #[serde(rename = "@type")]
    pub counter_type: String,
    #[serde(rename = "@missed")]
    pub missed: u32,
    #[serde(rename = "@covered")]
    pub covered: u32,
}

#[derive(Deserialize)]
pub struct JacocoSourcefile {
    #[serde(rename = "@name")]
    pub name: String,
    #[serde(rename = "line", default)]
    pub lines: Vec<JacocoLine>,
    #[allow(dead_code)]
    #[serde(rename = "counter", default)]
    pub counters: Vec<JacocoCounter>,
}

#[derive(Deserialize)]
pub struct JacocoLine {
    #[serde(rename = "@nr")]
    pub number: u32,
    #[allow(dead_code)]
    #[serde(rename = "@mi")]
    pub missed_instructions: u32,
    #[serde(rename = "@ci")]
    pub covered_instructions: u32,
    #[serde(rename = "@mb")]
    pub missed_branches: u32,
    #[serde(rename = "@cb")]
    pub covered_branches: u32,
}
