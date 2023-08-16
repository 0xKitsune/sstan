use std::collections::HashSet;

use solang_parser::pt::{self, Loc, SourceUnit};

use crate::analyzer::extractors::{primitive::AssignmentExtractor, Extractor};

pub fn addresses_hardcoded(source_unit: &mut SourceUnit) -> eyre::Result<HashSet<Loc>> {
    let mut qa_locations: HashSet<Loc> = HashSet::new();
    let assignments = AssignmentExtractor::extract(source_unit)?;
    for assignment in assignments.iter() {
        if let pt::Expression::Assign(loc, expr_0, _) = assignment {
            //Assignment to address literal
            if matches!(expr_0.as_ref(), pt::Expression::AddressLiteral(_, _)) {
                qa_locations.insert(*loc);
            }
        }
    }
    Ok(qa_locations)
}

#[test]
fn test_hardcoded_address() {
    let file_contents_1 = r#"
    contract Contract0 {
        address public owner;
        address hardcoded = "0x01111111111111";
        constructor(address _owner) {
            owner = _owner;
            address hardcoded = address(0x01111111111111);
        }
    }
    "#;

    let mut source_unit_1 = solang_parser::parse(file_contents_1, 0).unwrap().0;
    let qa_locations_1 = addresses_hardcoded(&mut source_unit_1);
    assert_eq!(qa_locations_1.unwrap().len(), 1);
}
