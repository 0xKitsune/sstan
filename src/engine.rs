use crate::analyzer::qa::{QualityAssuranceOutcome, QualityAssuranceTarget};

#[derive(Default)]
pub struct Engine {
    //TODO: pass in dir or file to analyze, or a default dir if none is passed in
    pub optimizations: Option<OptimizationModule>,
    pub vulnerabilities: Option<VulnerabilityModule>,
    pub qa: Option<QualityAssuranceModule>,
    //TODO: where to output options, etc maybe just make EngineOpts
}

//TODO: FIXME: revisit in the am, but the goal is to just be able to do engine.run().into_report() and have it generate a report for each module recursively

impl Engine {
    pub fn new() -> Self {
        Engine::default()
    }

    pub fn run(&self) {}
    pub fn into_report(&self) {
        //TODO: generate report for each module including test analysis and coverage
    }
}

impl Outcome for Vec<T: Outcome> {
    fn into_report(self) -> Report {
        for outcome in self.into_iter() {
            outcome.into_report(); //TODO: have to concat this
        }
    }
}

//TODO: also have trait for GPTReportSection or something

pub trait EngineModule<T: Outcome> {
    //TODO: define some traits for each component of the engine module,

    fn run(&mut self) -> Vec<T> {} //TODO: make this some generic that implements outcome
                                   //TODO: outcomes need to be .into() for the report format
                                   //TODO: the report should have a widget for big moving parts
}

//TODO: impl EngineModule for all modules
pub struct OptimizationModule {
    pub targets: Vec<QualityAssuranceTarget>,
    pub outcomes: Vec<OptimzationOutcome>,
}

pub struct VulnerabilityModule {
    pub targets: Vec<QualityAssuranceTarget>,
    pub outcomes: Vec<VulnerabilityOutcome>,
}

pub struct QualityAssuranceModule {
    pub targets: Vec<QualityAssuranceTarget>,
    pub outcomes: Vec<QualityAssuranceOutcome>,
}

pub struct TestAnalysisModule {
    //TODO: right now we can just run forge coverage. generate outcomes and call into report
}

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
