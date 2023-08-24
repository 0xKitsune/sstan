use std::collections::HashSet;

use solang_parser::pt::{Loc, SourceUnit};

use crate::analyzer::extractors::{compound::InterfaceExtractor, Extractor};

pub fn interfaces_namespace(source_unit: &mut SourceUnit) -> eyre::Result<HashSet<Loc>> {
    let mut qa_locations: HashSet<Loc> = HashSet::new();
    let interfaces = InterfaceExtractor::extract(source_unit)?;
    for interface in interfaces.iter() {
        if let Some(identifier) = &interface.name {
            if !identifier.name.starts_with("I") {
                qa_locations.insert(interface.loc);
            }
        }
    }
    Ok(qa_locations)
}
#[test]
fn test_interface_namespace() {
    let file_contents_1 = r#"
    interface IContract {}

    interface Contract0 {
        function foo() external returns (uint256 x);
    }
    "#;

    let mut source_unit_1 = solang_parser::parse(file_contents_1, 0).unwrap().0;
    let qa_locations_1 = interfaces_namespace(&mut source_unit_1);
    assert_eq!(qa_locations_1.unwrap().len(), 1);
}
