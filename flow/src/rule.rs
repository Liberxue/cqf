use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;
use std::fs::File;
use std::io::Read;
use std::path::Path;
use thiserror::Error;
use std::path::PathBuf;

#[derive(Debug, Serialize, Deserialize)]
pub struct DecisionRef {
    pub id: String,
    pub kind: String,
    pub rules: String,
    pub inputs: Option<Vec<String>>,
    pub outputs: Option<Vec<String>>,
    pub sources: Vec<String>,
    pub targets: Vec<String>,
}

pub type Rule = HashMap<String, String>;

#[derive(Debug, Clone)]
pub struct Decision {
    pub id: String,
    pub kind: String,
    pub rules: Vec<Rule>,
    pub expression: String,
    pub function: String,
    pub inputs: Vec<String>,
    pub outputs: Vec<String>,
    pub sources: Vec<String>,
    pub targets: Vec<String>,
}

#[derive(Error, Debug)]
pub enum ReaderError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),
    #[error("CSV error: {0}")]
    Csv(#[from] csv::Error),
    #[error("Unknown file extension: {0}")]
    UnknownExtension(String),
    #[error("Invalid or missing variable: {0}")]
    InvalidVariable(String),
}

impl From<DecisionRef> for Decision {
    fn from(dec_ref: DecisionRef) -> Self {
        let DecisionRef {
            id,
            kind,
            rules,
            inputs,
            outputs,
            sources,
            targets,
        } = dec_ref;

        let rules_table = RulesReader::read_rules(&rules).unwrap_or_default();
        let function_content = if kind == "function" {
            std::fs::read_to_string(&rules).unwrap_or_default()
        } else {
            String::new()
        };

        Decision {
            id,
            kind,
            rules: rules_table,
            expression: rules,
            function: function_content,
            inputs: inputs.unwrap_or_default(),
            outputs: outputs.unwrap_or_default(),
            sources,
            targets,
        }
    }
}

pub struct RulesReader;

impl RulesReader {
    pub fn read_rules(path: &str) -> Result<Vec<Rule>, ReaderError> {
        let path = PathBuf::from(path);
        let extension = path.extension()
            .and_then(std::ffi::OsStr::to_str)
            .ok_or_else(|| ReaderError::UnknownExtension(path.to_string_lossy().to_string()))?;

        match extension {
            "json" => Self::read_rules_json(&path),
            "csv" => Self::read_rules_csv(&path),
            _ => Err(ReaderError::UnknownExtension(extension.to_string())),
        }
    }

    fn read_rules_json(path: &Path) -> Result<Vec<Rule>, ReaderError> {
        let mut file = File::open(path)?;
        let mut data = String::new();
        file.read_to_string(&mut data)?;
        let rules: Vec<Rule> = serde_json::from_str(&data)?;
        Ok(rules)
    }

    fn read_rules_csv(path: &Path) -> Result<Vec<Rule>, ReaderError> {
        let file = File::open(path)?;
        let mut rdr = csv::Reader::from_reader(file);
        let rules: Result<Vec<Rule>, csv::Error> = rdr.deserialize().collect();
        Ok(rules?)
    }
}

pub struct DecisionReader;

impl DecisionReader {
    pub async fn read_flow<P: AsRef<Path>>(path: P) -> Result<Vec<Decision>, ReaderError> {
        let mut file = File::open(path)?;
        let mut data = String::new();
        file.read_to_string(&mut data)?;
        let decision_refs: Vec<DecisionRef> = serde_json::from_str(&data)?;
        Ok(decision_refs.into_iter().map(Decision::from).collect())
    }

    pub async fn read_input<P: AsRef<Path>>(path: P) -> Result<Value, ReaderError> {
        let mut file = File::open(path)?;
        let mut data = String::new();
        file.read_to_string(&mut data)?;
        Ok(serde_json::from_str(&data)?)
    }

    pub async fn read_str(data: &str) -> Result<Value, ReaderError> {
        Ok(serde_json::from_str(data)?)
    }
}

pub async fn read_flow<P: AsRef<Path>>(path: P) -> Vec<Decision> {
    DecisionReader::read_flow(path).await.unwrap_or_default()
}

pub async fn read_input<P: AsRef<Path>>(path: P) -> Value {
    DecisionReader::read_input(path).await.unwrap_or_default()
}

pub async fn read_str(data: &str) -> Value {
    DecisionReader::read_str(data).await.unwrap_or_default()
}
