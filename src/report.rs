pub mod optimizations;
pub mod qa;
pub mod vulnerabilities;
pub mod ctor;
use std::{path::PathBuf};

use crate::engine::{Engine};

pub struct ReportOutput {
    pub file_name: PathBuf,
    pub file_contents: String,
}

pub struct Report {
    pub preamble: ReportPreamble,
    pub description: String,
    pub summary: ReportSummary, //this is a struct that would define the charts/data visualizations
    pub table_of_contents: TableOfContents,
    pub vulnerability_report: ReportSection,
    pub optimization_report: ReportSection,
    pub qa_report: ReportSection,
}

pub struct ReportPreamble {
    pub title: String,
    pub logo: String,
    pub description: String,
    pub date: String,
    pub version: String,
    pub authors: Vec<String>,
}

//Table of Contents
pub struct TableOfContents {
    pub table_sections: Vec<TableSection>,
}

pub struct TableSection {
    pub title: String,
    pub subsections: Vec<TableFragment>,
}

pub struct TableFragment {
    pub identifier: Identifier, //TODO: this would be something that would define the item like [G-0], [G-1], etc
    pub title: String,
    pub instances: usize,
}

#[derive(Default)]
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

#[derive(Default)]
pub struct ReportSection {
    pub title: String,
    pub description: String,
    pub outcomes: Vec<ReportSectionFragment>,
}

#[derive(Default)]
pub struct ReportSectionFragment {
    pub identifier: Option<Identifier>, //TODO: this would be something that would define the item like [G-0], [G-1], etc
    pub instances: u32,
    pub title: String,
    pub description: String,
    pub outcomes: Vec<OutcomeReport>,
}

impl ReportSectionFragment {
    pub fn new(
        title: String,
        identifier: Option<Identifier>,
        description: String,
        instances: u32,
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

#[derive(Default)]
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

#[derive(Default)]
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

pub struct ReportSummary {
    pub charts: Vec<String>,
}

impl Into<ReportOutput> for Report {
    fn into(self) -> ReportOutput {
        todo!()
    }
}

//TODO: into report section for QA engine module, etc

//TODO: into table section for QA engine module

//TODO: need to define charts and data visualizations

//TODO: need into string for each report section

//TODO: after analysis and turning each module into report section, we need to populate nonces across all of the findings, then we can create the table of contents

impl Into<Report> for Engine {
    fn into(self) -> Report {
        //TODO: note that we will need to follow something like this order:
        // run the module
        // call into reportSection
        // populate nonces

        // do this for all modules ^^

        // call into table section for each module

        //Create the table of contents

        todo!()
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
            fragment.push_str(&format!("\n <details open> \n <summary> \n <a name={}>[<span style=\"color: blue;\">{}</span>]</a> <Strong>{}</Strong> Instances({}) \n </summary>",identifier,identifier,value.title,value.instances));
        } else {
            fragment.push_str(&format!(
                "\n <details open> \n <summary> \n <Strong>{}</Strong> Instances({}) \n </summary>",
                value.title, value.instances,
            ));
        }

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
