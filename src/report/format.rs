use crate::report::types::{Classification, OutcomeReport, Rank, ReportSectionFragment};

//Report Fragment Formatting
impl From<ReportSectionFragment> for String {
    fn from(value: ReportSectionFragment) -> String {
        let identifier: String = value.identifier.into();
        let mut fragment = format!("\n <details open> \n <summary> \n <a name={}>[<span style=\"color: blue;\">{}</span>]</a> <Strong>{}</Strong> Instances({}) \n </summary>",
            identifier,
            identifier,
            value.title,
            value.instances,
        );

        fragment.push_str(&format!(" \n {} \n", value.description));

        fragment.push_str(
            &value
                .outcomes
                .iter()
                .map(|outcome| String::from(outcome))
                .collect::<Vec<String>>()
                .join("\n"),
        );

        fragment.push_str(" \n </details>");

        fragment
    }
}

impl From<&OutcomeReport> for String {
    fn from(outcome_report: &OutcomeReport) -> String {
        format!(
            "\n <span style=\"color: green;\">File: </span> {} {}-{} \n ```solidity \n {} \n ```",
            outcome_report.file_name,
            outcome_report.line_numbers.0,
            outcome_report.line_numbers.1,
            outcome_report.snippet
        )
    }
}

impl From<Classification> for String {
    fn from(value: Classification) -> String {
        match value {
            Classification::Vulnerability(rank)
            | Classification::QA(rank)
            | Classification::Optimization(rank) => match rank {
                Rank::High(ident)
                | Rank::Medium(ident)
                | Rank::Low(ident)
                | Rank::NonCritical(ident) => ident,
            },
        }
    }
}
