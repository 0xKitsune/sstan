use std::path::PathBuf;

use crate::{
    optimizations::OptimizationOutcome, qa::QualityAssuranceOutcome, utils::read_lines,
    vulnerabilities::VulnerabilityOutcome,
};
#[derive(Default, Clone)]
pub struct ReportOutput {
    pub file_name: PathBuf,
    pub file_contents: String,
}
#[derive(Clone, Default)]
pub struct Report {
    pub preamble: ReportPreamble,
    pub git_url: Option<String>,
    pub description: String,
    pub summary: ReportSummary, //this is a struct that would define the charts/data visualizations
    pub table_of_contents: TableOfContents,
    pub vulnerability_report: ReportSection,
    pub optimization_report: ReportSection,
    pub qa_report: ReportSection,
}

impl Report {
    //Converts a report section into a string
    pub fn string_from_report_section(&self, report_section: ReportSection) -> String {
        let mut fragment: String = String::new();
        fragment.push_str(&format!(
            "\n## {} - Total: {}",
            report_section.title,
            report_section.outcomes.len()
        ));
        fragment.push_str(&format!(" \n{}\n", report_section.description));

        fragment.push_str(
            &report_section
                .outcomes
                .iter()
                .map(|f| self.string_from_report_section_fragment(f))
                .collect::<Vec<String>>()
                .join("\n"),
        );

        fragment.push_str("\n");

        fragment
    }
    //Converts a report section fragment into a string
    pub fn string_from_report_section_fragment(
        &self,
        report_section_fragment: &ReportSectionFragment,
    ) -> String {
        let mut fragment: String = String::new();
        let identifier: String = format!(
            "[{}-{}]",
            report_section_fragment
                .identifier
                .classification
                .identifier(),
            report_section_fragment.identifier.nonce
        );
        fragment.push_str(&format!("\n ### <a name={}></a> {} {} - Instances: {} \n",identifier,identifier,report_section_fragment.title,report_section_fragment.instances));

        fragment.push_str(&format!("\n {} \n", report_section_fragment.description));

        fragment.push_str("--- \n");

        fragment.push_str(
            &report_section_fragment
                .outcomes
                .iter()
                .map(|o| self.string_from_report_outcome(o))
                .collect::<Vec<String>>()
                .join("\n"),
        );

        fragment.push_str("\n\n --- \n");

        fragment
    }

    //Converts a report outcome into a string
    pub fn string_from_report_outcome(&self, report_outcome: &OutcomeReport) -> String {
        let mut snippet = String::new();
        //Git url should be formatted as base_url/blob/branch
        if let Some(url) = &self.git_url {
            snippet.push_str(&format!(
                "\n[File:{}#L{}]({}{}#L{}) \n",
                report_outcome.file_name,
                report_outcome.line_numbers.0,
                url,
                &report_outcome
                    .file_path
                    .as_path()
                    .as_os_str()
                    .to_str()
                    .unwrap()[1..], //./src/*/**  -> /src/*/**
                report_outcome.line_numbers.0
            ));
        } else {
            snippet.push_str(&format!(
                "\nFile:{}#L{}\n",
                report_outcome.file_name, report_outcome.line_numbers.0
            ));
        }
        snippet.push_str("```solidity\n");
        if let Ok(lines) = read_lines(report_outcome.file_path.as_path()) {
            for (i, line) in lines.enumerate() {
                if let Ok(l) = line {
                    if i + 1 >= report_outcome.line_numbers.0 && i < report_outcome.line_numbers.1 {
                        snippet.push_str(&format!("{}:{}\n", i, l));
                    }
                }
            }
        }
        snippet.push_str("``` \n\n");

        snippet
    }
}

impl From<Report> for String {
    fn from(report: Report) -> Self {
        //TODO: This is clone city, try to make this not clone city
        format!(
            "{}\n{}\n{}\n{}\n{}\n{}\n{}\n",
            String::from(report.preamble.clone()),
            report.description.clone(),
            String::from(report.summary.clone()),
            String::from(report.table_of_contents.clone()),
            report.string_from_report_section(report.vulnerability_report.clone()),
            report.string_from_report_section(report.optimization_report.clone()),
            report.string_from_report_section(report.qa_report.clone()),
        )
    }
}
#[derive(Clone)]
pub struct ReportPreamble {
    pub title: String,
    pub logo: String,
    pub description: String,
    pub date: String,
    pub version: String,
    pub authors: Vec<String>,
}

impl Default for ReportPreamble {
    fn default() -> Self {
        Self {
            title: "Sstan".to_string(),
            logo: String::default(),
            description: "TODO: add description".to_string(),
            date: "TODO: add date".to_string(),
            version: "v0.1.0".to_string(),
            authors: vec!["0x00face".to_string(), "0xOsiris".to_string()],
        }
    }
}

impl From<ReportPreamble> for String {
    fn from(preamble: ReportPreamble) -> Self {
        format!(
            "# {} - {} \n\n --- \n ## Authors: {} \n --- \n {}",
            preamble.title,
            preamble.version,
            preamble.authors.join(", "),
            preamble.description,
        )
    }
}
#[derive(Default, Clone)]

//Table of Contents
pub struct TableOfContents {
    pub table_sections: Vec<TableSection>,
}

impl TableOfContents {
    pub fn new(table_sections: Vec<TableSection>) -> Self {
        Self { table_sections }
    }
}

impl From<TableOfContents> for String {
    fn from(toc: TableOfContents) -> Self {
        toc.table_sections
            .iter()
            .map(String::from)
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
            "# {} \n\n | Classification | Title | Instances | \n |:-------:|:---------|:-------:| {}",
            section.title,
            section
                .subsections
                .iter()
                .map(String::from)
                .collect::<Vec<String>>()
                .join("")
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
    pub identifier: Identifier, //TODO: this would be something that would define the item like [G-0], [G-1], etc
    pub instances: usize,
    pub title: String,
    pub description: String,
    pub outcomes: Vec<OutcomeReport>,
}

impl ReportSectionFragment {
    pub fn new(
        title: String,
        identifier: Identifier,
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
    pub file_path: PathBuf,
}

impl OutcomeReport {
    pub fn new(
        file_name: String,
        line_numbers: (usize, usize),
        snippet: String,
        file_path: PathBuf,
    ) -> Self {
        Self {
            file_name,
            line_numbers,
            snippet,
            file_path,
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

impl From<Vec<QualityAssuranceOutcome>> for ReportSection {
    fn from(value: Vec<QualityAssuranceOutcome>) -> Self {
        ReportSection {
            title: "Quality Assurance".to_string(),
            description: String::new(),
            outcomes: value
                .into_iter()
                .map(|f| (f.classification(), Option::<ReportSectionFragment>::from(f)))
                .filter(|(_, fragment)| fragment.is_some())
                .enumerate()
                .map(
                    |(nonce, (classification, fragment))| ReportSectionFragment {
                        identifier: Identifier::new(classification, nonce),
                        ..fragment.unwrap()
                    },
                )
                .collect::<Vec<ReportSectionFragment>>(),
        }
    }
}

impl From<Vec<OptimizationOutcome>> for ReportSection {
    fn from(value: Vec<OptimizationOutcome>) -> Self {
        ReportSection {
            title: "Optimizations".to_string(),
            description: String::new(),
            outcomes: value
                .into_iter()
                .map(|f| (f.classification(), Option::<ReportSectionFragment>::from(f)))
                .filter(|(_, fragment)| fragment.is_some())
                .enumerate()
                .map(
                    |(nonce, (classification, fragment))| ReportSectionFragment {
                        identifier: Identifier::new(classification, nonce),
                        ..fragment.unwrap()
                    },
                )
                .collect::<Vec<ReportSectionFragment>>(),
        }
    }
}

impl From<Vec<VulnerabilityOutcome>> for ReportSection {
    fn from(value: Vec<VulnerabilityOutcome>) -> Self {
        ReportSection {
            title: "Vulnerabilities".to_string(),
            description: String::new(),
            outcomes: value
                .into_iter()
                .map(|f| (f.classification(), Option::<ReportSectionFragment>::from(f)))
                .filter(|(_, fragment)| fragment.is_some())
                .enumerate()
                .map(
                    |(nonce, (classification, fragment))| ReportSectionFragment {
                        identifier: Identifier::new(classification, nonce),
                        ..fragment.unwrap()
                    },
                )
                .collect::<Vec<ReportSectionFragment>>(),
        }
    }
}

impl From<&ReportSection> for TableSection {
    fn from(value: &ReportSection) -> Self {
        TableSection {
            title: format!("<h3>{}</h3>", value.title.clone()),
            subsections: value
                .outcomes
                .clone()
                .into_iter()
                .map(|outcome| TableFragment::from(&outcome))
                .collect::<Vec<TableFragment>>(),
        }
    }
}

impl From<&ReportSectionFragment> for TableFragment {
    fn from(value: &ReportSectionFragment) -> TableFragment {
        TableFragment::new(
            value.title.to_string(),
            Some(Identifier {
                classification: value.identifier.classification,
                nonce: value.identifier.nonce,
            }),
            value.instances,
        )
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
            fragment.push_str(&format!(
                "\n | [{}](#{}) | <Strong>{}</Strong> | {} |",
                identifier, identifier, value.title, value.instances
            ));
        } else {
            fragment.push_str(&format!(
                "\n <details open> \n <summary> \n <Strong>{}</Strong> - Instances: {} \n </summary>",
                value.title, value.instances,
            ));
        }
        fragment
    }
}
