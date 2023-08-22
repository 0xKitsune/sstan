use std::{collections::HashMap, path::PathBuf};

// use crate::analyzer::qa::{QualityAssuranceOutcome, QualityAssuranceTarget};
use solang_parser::pt::Loc;
use thiserror::Error;

use crate::{
    extractors::ExtractionError,
    qa::{QualityAssuranceOutcome, QualityAssuranceTarget},
};

pub type Report = String;
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
}

#[derive(Default)]
pub struct Engine {
    //TODO: pass in dir or file to analyze, or a default dir if none is passed in
    // pub optimizations: Option<OptimizationModule>,
    // pub vulnerabilities: Option<VulnerabilityModule>,
    pub qa: Option<QualityAssuranceModule>,
    //TODO: where to output options, etc maybe just make EngineOpts
}

//TODO: FIXME: revisit in the am, but the goal is to just be able to do engine.run().into_report() and have it generate a report for each module recursively

impl Engine {
    pub fn new() -> Self {
        Engine::default()
    }

    pub fn run(&self) {}
}

#[derive(Debug)]
pub enum OptimizationOutcome {
    CacheLoopVariable(Outcome),
}

//TODO: also have trait for GPTReportSection or something

//TODO: FIXME: we can have the appendix generated for specific outcomes, have a trait that can get implemented to generate appendix
pub trait EngineModule<T: Into<Report>> {
    fn run(&mut self) -> Vec<T>;
}

//TODO: impl EngineModule for all modules
// pub struct OptimizationModule {
//     pub targets: Vec<QualityAssuranceTarget>,
//     pub outcomes: Vec<OptimzationOutcome>,
// }

// pub struct VulnerabilityModule {
//     pub targets: Vec<QualityAssuranceTarget>,
//     pub outcomes: Vec<VulnerabilityOutcome>,
// }

pub struct QualityAssuranceModule {
    pub targets: Vec<QualityAssuranceTarget>,
    pub outcomes: Vec<QualityAssuranceOutcome>,
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
