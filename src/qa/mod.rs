//TODO: we could have something here that creates the OptimizationOutcome enum

//optimizations!(
// (ConstructorOrder, "This is the description that will be used when into_report, maybe add an example or something")
// )

//TODO: have some trait that is implemented on each struct created like ConstructorOrder::find() -> Vec<OptimizationOutcome> or something
//TODO: maybe have a enum like pub enum Outcome{ QualityAssurance(QualityAssuranceOutcome), Vulnerability(VulnerabilityOutcome), Optimization(OptimizationOutcome) } and find() returns Outcome

//TODO: this would go in optimization
// pub trait OptimizationPattern {
//     fn find() -> Vec<OptimizationOutcome>;
// }

pub trait QAPattern {
    fn find() -> Vec<QualityAssuranceOutcome>;
}

// pub struct QualityAssuranceModule {
//     pub targets: Vec<QualityAssuranceTarget>,
//     pub outcomes: Vec<QualityAssuranceOutcome>,
// }

// pub type Outcome = HashMap<PathBuf, Vec<Loc>>;

// pub enum QualityAssuranceOutcome {
//     ConstructorOrderOutcome(Outcome),
//     //     PrivateVarsLeadingUnderscore(PrivateVarsLeadingUnderscoreOutcome),
//     //     PrivateFuncLeadingUnderscore(PrivateFuncLeadingUnderscoreOutcome),
//     //     ImportIdentifiers(ImportIdentifiersOutcome),
// }

//TODO: still have to figure this out
// pub trait IntoReport{
//     fn into_report(self) -> String;

// }
