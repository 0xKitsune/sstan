//Truncates mock report file on every test run
#[cfg(test)]
#[ctor::ctor]
fn init() {
    std::fs::File::options()
        .truncate(true)
        .write(true)
        .open("src/report/mock_report.md")
        .unwrap();
}
