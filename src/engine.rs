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
        let entry = self.entry(path).or_insert(vec![]);
        entry.push((loc, snippet));
    }
}

//TODO: this is just a placeholder, we will need to update this
#[derive(Error, Debug)]
pub enum EngineError {
    #[error("Error while extracting source units")]
    ExtractionError(#[from] ExtractionError),
    #[error("Error while running regex")]
    RegexError(#[from] regex::Error),
    #[error("Error while parsing int")]
    ParseIntError(#[from] std::num::ParseIntError),
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

//TODO: also have trait for GPTReportSection or something

//TODO: FIXME: we can have the appendix generated for specific outcomes, have a trait that can get implemented to generate appendix
pub trait EngineModule<T> {
    fn run(&mut self, source: &mut HashMap<PathBuf, SourceUnit>) -> Result<T, EngineError>;
}
impl From<Engine> for Report {
    fn from(engine: Engine) -> Report {
        let mut report = Report::default();
        report.vulnerability_report = ReportSection::from(engine.vulnerabilities.outcomes);
        report.optimization_report = ReportSection::from(engine.optimizations.outcomes);
        report.qa_report = ReportSection::from(engine.qa.outcomes);
        //Set the github url for the repo
        report.git_url = engine.git_url;
        let table_sections = vec![
            TableSection::from(&report.vulnerability_report),
            TableSection::from(&report.optimization_report),
            TableSection::from(&report.qa_report),
        ];

        report.table_of_contents = TableOfContents::new(table_sections);

        report
    }
}

//TODO: impl EngineModule for all modules
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

// pub struct TestAnalysisModule {
//     //TODO: right now we can just run forge coverage. generate outcomes and call into report
// }

//TODO: each module will also implement Report and have an into_report() method.

//TODO: really we need to build something like an AST for areport{

//
// Report Section {
// Scope, etc
// Charts
// Vulnerabilties(Vec<VulnerabilityOutcome>) //maybe have some internal type that is able to have a title and description, it would be cool to be able to run an analysis on everything and then show the findings and generate the report.
// Optimizations(Vec<OptimizationOutcome>)
// QualityAssurance(Vec<QualityAssuranceOutcome>
// Appendix or something
// }
// }

// Each of these should implement to x trait that transforms it into a markdown report

// At the end of the day, each Outcome in the vec of outcomes should be a specific finding with all instances, highlighting line numbers, blocks of code and a short description for the finding.
// There should be some way to pass a flag that either populates the description with gpt or not. There should be a description for the section, then all the blocks of code and the findings. Maybe also, itll link line numbers and then link to github as well as an appendix.
// Maybe though it should just have the code blocks right there probably the latter

// NOTE: so basically, each outcome should have a list of all instances, and then transform into a title and a description. If you want, the description could be gpt and we could write a prompt in the trait.
// Though with this approach, we would need to feed the whole contract in first, and then give the specific finding and a prompt before hand
