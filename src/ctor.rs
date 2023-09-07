//Truncates mock report file on every test run
#[cfg(test)]
#[ctor::ctor]
fn init() {
    std::fs::File::options()
        .truncate(true)
        .write(true)
        .open("qa_report_sections.md")
        .unwrap();
    std::fs::File::options()
        .truncate(true)
        .write(true)
        .open("vulnerability_report_sections.md")
        .unwrap();
    std::fs::File::options()
        .truncate(true)
        .write(true)
        .open("qa_table_sections.md")
        .unwrap();
}
