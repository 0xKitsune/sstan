use std::collections::HashSet;

use solang_parser::pt::{self, Loc, SourceUnit};

use crate::analyzer::extractors::{primitive::NumberLiteralExtractor, Extractor};

pub fn large_multiples_of_ten(source_unit: &mut SourceUnit) -> eyre::Result<HashSet<Loc>> {
    let mut qa_locations: HashSet<Loc> = HashSet::new();
    let number_literals = NumberLiteralExtractor::extract(source_unit)?;
    //Get all variables that are larger than 1000000, and divisible by 10.
    for number_literal in number_literals.iter() {
        if let pt::Expression::NumberLiteral(loc, str_number, _, _) = number_literal {
            //TODO: Use u256's here to prevent potential overflow. Or alternatively rug Integer.
            let number = str_number.to_string().parse::<u128>()?;
            if number % 10 == 0 && number > 1000000 {
                qa_locations.insert(*loc);
            }
        }
    }
    Ok(qa_locations)
}

#[test]
fn test_import_identifiers() {
    let file_contents_1 = r#"
    contract Contract0 {
        address public owner;
        uint x = 1e7;
        constructor(address _owner) {
            owner = _owner;
        }
    }
    "#;

    let file_contents_2 = r#"
    contract Contract0 {
        address public owner;
        uint x = 10000000;
        constructor(address _owner) {
            require(_owner != address(0), "Owner cannot be zero address");
            owner = _owner;
        }
    }
    "#;

    let mut source_unit_1 = solang_parser::parse(file_contents_1, 0).unwrap().0;
    let mut source_unit_2 = solang_parser::parse(file_contents_2, 0).unwrap().0;
    let qa_locations_1 = large_multiples_of_ten(&mut source_unit_1);
    let qa_locations_2 = large_multiples_of_ten(&mut source_unit_2);
    assert_eq!(qa_locations_1.unwrap().len(), 0);
    assert_eq!(qa_locations_2.unwrap().len(), 1);
}
