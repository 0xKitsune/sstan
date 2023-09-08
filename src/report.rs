use std::path::PathBuf;

use crate::engine::{Engine, OptimizationModule, QualityAssuranceModule, VulnerabilityModule};
#[derive(Default, Clone)]
pub struct ReportOutput {
    pub file_name: PathBuf,
    pub file_contents: String,
}
#[derive(Default, Clone)]

pub struct Report {
    pub preamble: ReportPreamble,
    pub description: String,
    pub summary: ReportSummary, //this is a struct that would define the charts/data visualizations
    pub table_of_contents: TableOfContents,
    pub vulnerability_report: ReportSection,
    pub optimization_report: ReportSection,
    pub qa_report: ReportSection,
}

impl From<Report> for String {
    fn from(report: Report) -> Self {
        format!(
            "{}\n{}\n{}\n{}\n{}\n{}\n{}\n",
            String::from(report.preamble),
            report.description,
            String::from(report.summary),
            String::from(report.table_of_contents),
            String::from(report.vulnerability_report),
            String::from(report.optimization_report),
            String::from(report.qa_report),
        )
    }
}
#[derive(Default, Clone)]
pub struct ReportPreamble {
    pub title: String,
    pub logo: String,
    pub description: String,
    pub date: String,
    pub version: String,
    pub authors: Vec<String>,
}

impl From<ReportPreamble> for String {
    fn from(preamble: ReportPreamble) -> Self {
        format!(
            "# {} \n\n ({})\n\n{}\n\n{}\n\n{}\n\n{}\n\n",
            preamble.title,
            preamble.logo,
            preamble.description,
            preamble.date,
            preamble.version,
            preamble.authors.join(", "),
        )
    }
}
#[derive(Default, Clone)]

//Table of Contents
pub struct TableOfContents {
    pub table_sections: Vec<TableSection>,
}

impl From<TableOfContents> for String {
    fn from(toc: TableOfContents) -> Self {
        toc.table_sections
            .iter()
            .map(|section| String::from(section))
            .collect::<Vec<String>>()
            .join("\n")
    }
}
#[derive(Default, Clone)]

pub struct TableSection {
    pub title: String,
    pub subsections: Vec<TableFragment>,
}

impl From<&TableSection> for String {
    fn from(section: &TableSection) -> Self {
        format!(
            "# {}\n\n{}\n\n",
            section.title,
            section
                .subsections
                .iter()
                .map(|subsection| String::from(subsection))
                .collect::<Vec<String>>()
                .join("\n\n")
        )
    }
}
#[derive(Default, Clone)]
pub struct TableFragment {
    pub identifier: Option<Identifier>, //TODO: this would be something that would define the item like [G-0], [G-1], etc
    pub title: String,
    pub instances: usize,
}

impl TableFragment {
    pub fn new(title: String, identifier: Option<Identifier>, instances: usize) -> Self {
        Self {
            title,
            identifier,
            instances,
        }
    }
}

#[derive(Default, Copy, Clone)]
pub struct Identifier {
    pub classification: Classification,
    pub nonce: usize,
}

impl Identifier {
    pub fn new(classification: Classification, nonce: usize) -> Self {
        Self {
            classification,
            nonce,
        }
    }
}

#[derive(Default, Clone)]
pub struct ReportSection {
    pub title: String,
    pub description: String,
    pub outcomes: Vec<ReportSectionFragment>,
}

#[derive(Default, Clone)]
pub struct ReportSectionFragment {
    pub identifier: Option<Identifier>, //TODO: this would be something that would define the item like [G-0], [G-1], etc
    pub instances: usize,
    pub title: String,
    pub description: String,
    pub outcomes: Vec<OutcomeReport>,
}

impl ReportSectionFragment {
    pub fn new(
        title: String,
        identifier: Option<Identifier>,
        description: String,
        instances: usize,
    ) -> Self {
        Self {
            title,
            identifier,
            description,
            instances,
            outcomes: vec![],
        }
    }
}

#[derive(Default, Copy, Clone)]

// This is used as classifications for both vulns and gas opts
pub enum Classification {
    #[default]
    VulnerabilityHigh,
    VulnerabilityMedium,
    VulnerabilityLow,
    OptimizationHigh,
    OptimizationMedium,
    OptimizationLow,
    NonCritical,
}

impl Classification {
    pub fn identifier(&self) -> String {
        match self {
            Classification::VulnerabilityHigh => "H".to_string(),
            Classification::VulnerabilityMedium => "M".to_string(),
            Classification::VulnerabilityLow => "L".to_string(),
            Classification::NonCritical => "NC".to_string(),
            Classification::OptimizationHigh
            | Classification::OptimizationMedium
            | Classification::OptimizationLow => "G".to_string(),
        }
    }
}

#[derive(Default, Clone)]

pub struct OutcomeReport {
    pub file_name: String,
    pub line_numbers: (usize, usize), //if the same line number then we just compile report as one number
    pub snippet: String,
    //TODO: whatever else that we want
}

impl OutcomeReport {
    pub fn new(file_name: String, line_numbers: (usize, usize), snippet: String) -> Self {
        Self {
            file_name,
            line_numbers,
            snippet,
        }
    }
}

#[derive(Default, Clone)]
pub struct ReportSummary {
    pub charts: Vec<String>,
}

impl From<ReportSummary> for String {
    fn from(summary: ReportSummary) -> Self {
        format!("# Summary\n\n{}\n\n", summary.charts.join("\n\n"))
    }
}

impl From<Report> for ReportOutput {
    fn from(_value: Report) -> Self {
        todo!()
    }
}

impl From<QualityAssuranceModule> for ReportSection {
    fn from(value: QualityAssuranceModule) -> Self {
        ReportSection {
            title: "Vulnerabilities".to_string(),
            description: String::new(),
            outcomes: value
                .outcomes
                .into_iter()
                .map(|f| (f.classification(), Option::<ReportSectionFragment>::from(f)))
                .filter(|(_, fragment)| fragment.is_some())
                .enumerate()
                .map(
                    |(nonce, (classification, fragment))| ReportSectionFragment {
                        identifier: Some(Identifier::new(classification, nonce)),
                        ..fragment.unwrap()
                    },
                )
                .collect::<Vec<ReportSectionFragment>>(),
        }
    }
}

impl From<OptimizationModule> for ReportSection {
    fn from(value: OptimizationModule) -> Self {
        ReportSection {
            title: "Optimizations".to_string(),
            description: String::new(),
            outcomes: value
                .outcomes
                .into_iter()
                .map(|f| (f.classification(), Option::<ReportSectionFragment>::from(f)))
                .filter(|(_, fragment)| fragment.is_some())
                .enumerate()
                .map(
                    |(nonce, (classification, fragment))| ReportSectionFragment {
                        identifier: Some(Identifier::new(classification, nonce)),
                        ..fragment.unwrap()
                    },
                )
                .collect::<Vec<ReportSectionFragment>>(),
        }
    }
}

impl From<VulnerabilityModule> for ReportSection {
    fn from(value: VulnerabilityModule) -> Self {
        ReportSection {
            title: "Quality Assurance".to_string(),
            description: String::new(),
            outcomes: value
                .outcomes
                .into_iter()
                .map(|f| (f.classification(), Option::<ReportSectionFragment>::from(f)))
                .filter(|(_, fragment)| fragment.is_some())
                .enumerate()
                .map(
                    |(nonce, (classification, fragment))| ReportSectionFragment {
                        identifier: Some(Identifier::new(classification, nonce)),
                        ..fragment.unwrap()
                    },
                )
                .collect::<Vec<ReportSectionFragment>>(),
        }
    }
}

impl From<ReportSection> for String {
    fn from(value: ReportSection) -> Self {
        let mut fragment: String = String::new();
        fragment.push_str(&format!(
            "\n <details open> \n <summary> \n <Strong>{}</Strong> Instances({}) \n </summary>",
            value.title,
            value.outcomes.len()
        ));
        fragment.push_str(&format!(" \n {} \n", value.description));

        fragment.push_str(
            &value
                .outcomes
                .iter()
                .map(String::from)
                .collect::<Vec<String>>()
                .join("\n"),
        );

        fragment.push_str(" \n </details>");

        fragment
    }
}

impl From<ReportSection> for TableSection {
    fn from(value: ReportSection) -> Self {
        TableSection {
            title: value.title,
            subsections: value
                .outcomes
                .into_iter()
                .map(|outcome| TableFragment::from(&outcome))
                .collect::<Vec<TableFragment>>(),
        }
    }
}

//Report Fragment Formatting
impl From<ReportSectionFragment> for String {
    fn from(value: ReportSectionFragment) -> String {
        let mut fragment: String = String::new();
        if let Some(identifier) = value.identifier {
            let identifier: String = format!(
                "[{}-{}]",
                identifier.classification.identifier(),
                identifier.nonce
            );
            fragment.push_str(&format!("\n <details open> \n <summary> \n <a name={}>[<span style=\"color: blue;\">{}</span>]</a> <font size=\"5\"> {} - Instances: {} </font> \n </summary>",identifier,identifier,value.title,value.instances));
        } else {
            fragment.push_str(&format!(
                "\n <details open> \n <summary> \n <h2> {} - Instances: {} </h2>\n </summary>",
                value.title, value.instances,
            ));
        }

        fragment.push_str(&format!(" \n &nbsp; {} \n", value.description));
        fragment.push_str("\n &nbsp;");
        fragment.push_str(
            &value
                .outcomes
                .iter()
                .map(String::from)
                .collect::<Vec<String>>()
                .join("\n &nbsp;"),
        );

        fragment.push_str(" \n </details>");

        fragment
    }
}

//Report Fragment Formatting
impl From<&ReportSectionFragment> for String {
    fn from(value: &ReportSectionFragment) -> String {
        let mut fragment: String = String::new();
        if let Some(identifier) = value.identifier {
            let identifier: String = format!(
                "[{}-{}]",
                identifier.classification.identifier(),
                identifier.nonce
            );
            fragment.push_str(&format!("\n <details open> \n <summary> \n <a name={}>[<span style=\"color: blue;\">{}</span>]</a> <Strong>{}</Strong> - Instances: {} \n </summary>",identifier,identifier,value.title,value.instances));
        } else {
            fragment.push_str(&format!(
                "\n <details open> \n <summary> \n <Strong>{}</Strong> - Instances: {} \n </summary>",
                value.title, value.instances,
            ));
        }

        fragment.push_str(&format!(" \n    > {} \n", value.description));

        fragment.push_str(
            &value
                .outcomes
                .iter()
                .map(String::from)
                .collect::<Vec<String>>()
                .join("\n"),
        );

        fragment.push_str(" \n </details>");

        fragment
    }
}

impl From<&ReportSectionFragment> for TableFragment {
    fn from(value: &ReportSectionFragment) -> TableFragment {
        if let Some(ident) = &value.identifier {
            TableFragment::new(
                value.title.to_string(),
                Some(Identifier {
                    classification: ident.classification,
                    nonce: ident.nonce,
                }),
                value.instances,
            )
        } else {
            TableFragment::new(value.title.to_string(), None, value.instances)
        }
    }
}

impl From<&TableFragment> for String {
    fn from(value: &TableFragment) -> String {
        let mut fragment: String = String::new();
        if let Some(identifier) = value.identifier {
            let identifier: String = format!(
                "[{}-{}]",
                identifier.classification.identifier(),
                identifier.nonce
            );
            fragment.push_str(&format!("\n <details open> \n <summary> \n <a name={}>[<span style=\"color: blue;\">{}</span>]</a> <Strong>{}</Strong> - Instances: {} \n </summary>",identifier,identifier,value.title,value.instances));
        } else {
            fragment.push_str(&format!(
                "\n <details open> \n <summary> \n <Strong>{}</Strong> - Instances: {} \n </summary>",
                value.title, value.instances,
            ));
        }

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
