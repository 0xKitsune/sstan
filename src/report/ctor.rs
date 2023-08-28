//Truncates mock report file on every test run
#[cfg(test)]
#[ctor::ctor]
fn init() {
    std::fs::File::options()
        .truncate(true)
        .write(true)
        .open("src/report/mocks/qa_report_sections.md")
        .unwrap();
}
