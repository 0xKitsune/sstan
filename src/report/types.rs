//Report
pub type SstanReport = String;
pub struct Report {
    pub title: String,
    pub outcome_summary: OutcomeSummary, //this is a struct that would define the charts/data visualizations
    pub table_of_contents: TableOfContents,
    pub vuln_report: ReportSection,
    pub gas_report: ReportSection,
    pub qa_report: ReportSection,
}

//Table of Contents
pub struct TableOfContents {
    pub table_sections: Vec<TableSection>,
}
pub struct TableSection {
    pub title: String,
    pub description: String,
    pub subsections: Vec<TableFragments>,
}

pub struct TableFragments {
    pub title: String,
    pub identifier: Classification,
    pub outcomes: Vec<OutcomeReport>,
}

//Report Section
pub struct ReportSection {
    pub title: String,
    pub description: String,
    pub outcomes: Vec<ReportSectionFragment>,
}

pub struct ReportSectionFragment {
    pub title: String,
    pub identifier: Classification,
    pub description: String,
    pub instances: u32,
    pub outcomes: Vec<OutcomeReport>,
}

impl ReportSectionFragment {
    pub fn new(
        title: String,
        identifier: Classification,
        description: String,
        instances: u32,
    ) -> Self {
        Self {
            title: String::new(),
            identifier: Classification::Vulnerability(Rank::High(String::new())),
            description: String::new(),
            instances: 0,
            outcomes: Vec::new(),
        }
    }
}

pub enum Classification {
    Vulnerability(Rank),
    Optimization(Rank),
    QA(Rank),
}

pub enum Rank {
    High(String),
    Medium(String),
    Low(String),
    NonCritical(String),
}

pub struct OutcomeReport {
    pub file_name: String,
    pub line_numbers: (i32, i32), //if the same line number then we just compile report as one number
    pub snippet: String,
}

impl OutcomeReport {
    pub fn new(file_name: String, line_numbers: (usize, usize), snippet: String) -> Self {
        Self {
            file_name: String::new(),
            line_numbers: (0, 0),
            snippet: String::new(),
        }
    }
}

pub struct OutcomeSummary {
    pub charts: Vec<String>,
}

impl Into<SstanReport> for Report {
    fn into(self) -> SstanReport {
        todo!()
    }
}
