use solang_parser::pt::{Loc, SourceUnit};
use std::{collections::HashMap, path::PathBuf, vec};
use thiserror::Error;

use crate::{
    extractors::ExtractionError,
    optimizations::{OptimizationOutcome, OptimizationTarget},
    qa::{QualityAssuranceOutcome, QualityAssuranceTarget},
    report::{Report, ReportSection, TableOfContents, TableSection},
    utils,
    vulnerabilities::{VulnerabilityOutcome, VulnerabilityTarget},
};

// pub type Report = String;
pub type Snippet = String;
pub type Outcome = HashMap<PathBuf, Vec<(Loc, Snippet)>>;

//TODO: FIXME: maybe update this name
pub trait Pushable {
    fn push_or_insert(&mut self, path: PathBuf, loc: Loc, snippet: Snippet);
}

impl Pushable for Outcome {
    fn push_or_insert(&mut self, path: PathBuf, loc: Loc, snippet: Snippet) {
        let entry = self.entry(path).or_default();
        entry.push((loc, snippet));
    }
}

#[derive(Error, Debug)]
pub enum EngineError {
    #[error("Error while extracting source units")]
    ExtractionError(#[from] ExtractionError),
    #[error("Error while running regex")]
    RegexError(#[from] regex::Error),
    #[error("Error while parsing int")]
    ParseIntError(#[from] std::num::ParseIntError),
    #[error("Unrecognized pattern")]
    UnrecognizedPattern(String),
}

#[derive(Default)]
pub struct Engine {
    pub source: HashMap<PathBuf, SourceUnit>,
    pub git_url: Option<String>,
    pub optimizations: OptimizationModule,
    pub vulnerabilities: VulnerabilityModule,
    pub qa: QualityAssuranceModule,
}
impl Engine {
    pub fn new(
        path: &str,
        git_url: Option<String>,
        vulnerabilities: Vec<VulnerabilityTarget>,
        optimizations: Vec<OptimizationTarget>,
        qa: Vec<QualityAssuranceTarget>,
    ) -> Self {
        let mut source = HashMap::new();
        //write logic to parse all source unitis from the path and extract
        utils::extract_source(path, &mut source).unwrap();
        Engine {
            source,
            git_url,
            optimizations: OptimizationModule {
                targets: optimizations,
                outcomes: vec![],
            },
            vulnerabilities: VulnerabilityModule {
                targets: vulnerabilities,
                outcomes: vec![],
            },
            qa: QualityAssuranceModule {
                targets: qa,
                outcomes: vec![],
            },
        }
    }

    pub fn run(&mut self) -> Result<(), EngineError> {
        //Run the vulnerability module
        if !self.vulnerabilities.targets.is_empty() {
            self.vulnerabilities.outcomes = self.vulnerabilities.run(&mut self.source)?;
        }
        //Run the optimization module
        if !self.optimizations.targets.is_empty() {
            self.optimizations.outcomes = self.optimizations.run(&mut self.source)?;
        }
        //Run the QA module
        if !self.qa.targets.is_empty() {
            self.qa.outcomes = self.qa.run(&mut self.source)?;
        }
        Ok(())
    }
}

pub trait EngineModule<T> {
    fn run(&mut self, source: &mut HashMap<PathBuf, SourceUnit>) -> Result<T, EngineError>;
}
impl From<Engine> for Report {
    fn from(engine: Engine) -> Report {
        let mut report = Report {
            vulnerability_report: ReportSection::from(engine.vulnerabilities.outcomes),
            optimization_report: ReportSection::from(engine.optimizations.outcomes),
            qa_report: ReportSection::from(engine.qa.outcomes),
            git_url: engine.git_url,
            ..Default::default()
        };

        let table_sections = vec![
            TableSection::from(&report.vulnerability_report),
            TableSection::from(&report.optimization_report),
            TableSection::from(&report.qa_report),
        ];

        report.table_of_contents = TableOfContents::new(table_sections);

        report
    }
}

#[derive(Default)]
pub struct OptimizationModule {
    pub targets: Vec<OptimizationTarget>,
    pub outcomes: Vec<OptimizationOutcome>,
}
#[derive(Default)]
pub struct VulnerabilityModule {
    pub targets: Vec<VulnerabilityTarget>,
    pub outcomes: Vec<VulnerabilityOutcome>,
}
#[derive(Default)]
pub struct QualityAssuranceModule {
    pub targets: Vec<QualityAssuranceTarget>,
    pub outcomes: Vec<QualityAssuranceOutcome>,
}

impl EngineModule<Vec<QualityAssuranceOutcome>> for QualityAssuranceModule {
    fn run(
        &mut self,
        source: &mut HashMap<PathBuf, SourceUnit>,
    ) -> Result<Vec<QualityAssuranceOutcome>, EngineError> {
        let mut outcomes = vec![];
        for target in self.targets.iter() {
            outcomes.push(target.find(source)?);
        }

        Ok(outcomes)
    }
}

impl EngineModule<Vec<OptimizationOutcome>> for OptimizationModule {
    fn run(
        &mut self,
        source: &mut HashMap<PathBuf, SourceUnit>,
    ) -> Result<Vec<OptimizationOutcome>, EngineError> {
        let mut outcomes = vec![];
        for target in self.targets.iter() {
            outcomes.push(target.find(source)?);
        }

        Ok(outcomes)
    }
}

impl EngineModule<Vec<VulnerabilityOutcome>> for VulnerabilityModule {
    fn run(
        &mut self,
        source: &mut HashMap<PathBuf, SourceUnit>,
    ) -> Result<Vec<VulnerabilityOutcome>, EngineError> {
        let mut outcomes = vec![];
        for target in self.targets.iter() {
            outcomes.push(target.find(source)?);
        }
        Ok(outcomes)
    }
}
