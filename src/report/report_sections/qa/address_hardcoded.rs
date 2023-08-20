pub fn report_section_content() -> String {
    String::from(
        r##" 
### Addresses should not be hardcoded
Consider assigning the address variables to immutable and initializing them in the constructor. 
    "##,
    )
}
