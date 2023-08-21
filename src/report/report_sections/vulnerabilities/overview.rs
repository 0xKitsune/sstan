pub fn report_section_content(total_vulnerabilities: usize) -> String {
    format!("# Vulnerabilities - (Total Vulnerabilities {})\n
The following sections detail the high, medium and low severity vulnerabilities found throughout the codebase.\n\n<br>\n
", total_vulnerabilities)
}
