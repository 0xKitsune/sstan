//Truncates mock report file on every test run
#[cfg(test)]
#[ctor::ctor]
fn init() {
    std::fs::File::options()
        .truncate(true)
        .write(true)
        .open("src/report/mocks/qa_report_sections.md")
        .unwrap();
    std::fs::File::options()
        .truncate(true)
        .write(true)
        .open("src/report/mocks/vulnerability_report_sections.md")
        .unwrap();
    std::fs::File::options()
        .truncate(true)
        .write(true)
        .open("src/report/mocks/qa_table_sections.md")
        .unwrap();
}
