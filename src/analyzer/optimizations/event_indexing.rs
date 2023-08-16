use std::collections::HashSet;

use solang_parser::pt::{self, Loc, SourceUnit};

use crate::analyzer::extractors::{primitive::EventExtractor, Extractor};

pub fn event_indexing_optimization(source_unit: &mut SourceUnit) -> eyre::Result<HashSet<Loc>> {
    let mut optimization_locations: HashSet<Loc> = HashSet::new();
    let events = EventExtractor::extract(source_unit)?;

    //Accumulate the number of indexed events, and the number of non-array indexed parameters.
    for event in events.iter() {
        let mut indexed_events_count = 0;
        let mut non_array_indexed_parameter_count = 0;
        let event_parameters = event.fields.clone();
        for event_parameter in event_parameters.iter() {
            if event_parameter.indexed {
                indexed_events_count += 1;
            }

            if !matches!(event_parameter.ty, pt::Expression::ArraySlice(..)) {
                non_array_indexed_parameter_count += 1;
            }
        }

        if non_array_indexed_parameter_count >= 3 && indexed_events_count < 3 {
            optimization_locations.insert(event.loc.clone());
        } else if non_array_indexed_parameter_count < 3
            && indexed_events_count != non_array_indexed_parameter_count
        {
            optimization_locations.insert(event.loc.clone());
        }
    }
    Ok(optimization_locations)
}

#[test]
fn test_immutable_variables_optimization() {
    let file_contents = r#"
    
    pragma solidity >= 0.8.0;
    contract Contract {

        event IsNotOptimized(address addr1, address indexed addr2);
        event IsOptimized(address indexed addr1, address indexed addr2, address indexed addr3);
        event AlsoIsNotOptimized(address addr1, address indexed addr2, address indexed addr3);
        
    }
 
    "#;
    let mut source_unit = solang_parser::parse(file_contents, 0).unwrap().0;

    let optimization_locations = event_indexing_optimization(&mut source_unit);
    assert_eq!(optimization_locations.unwrap().len(), 2)
}
