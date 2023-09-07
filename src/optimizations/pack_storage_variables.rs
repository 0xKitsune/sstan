use std::collections::HashMap;
use std::path::PathBuf;

use super::{OptimizationOutcome, OptimizationPattern, PackStorageVariables};
use solang_parser::pt::CodeLocation;
use solang_parser::{self, pt::SourceUnit};

use crate::engine::{EngineError, Outcome, Pushable};
use crate::extractors::{compound::StorageVariableExtractor, Extractor};
use crate::utils;

impl OptimizationPattern for PackStorageVariables {
    fn find(source: &mut HashMap<PathBuf, SourceUnit>) -> Result<OptimizationOutcome, EngineError> {
        let mut outcome = Outcome::new();
        for (path_buf, source_unit) in source {
            let target_nodes = StorageVariableExtractor::extract(source_unit)?;
            let mut variable_sizes: Vec<u16> = vec![];

            for node in target_nodes.clone() {
                variable_sizes.push(utils::get_type_size(node.ty));
            }
            //Cache a version of variable sizes that is unordered
            let unordered_variable_sizes = variable_sizes.clone();

            //Sort the variable sizes
            variable_sizes.sort();

            //If the ordered version is smaller than the unordered, add loc
            if utils::storage_slots_used(unordered_variable_sizes)
                > utils::storage_slots_used(variable_sizes)
            {
                outcome.push_or_insert(
                    path_buf.clone(),
                    target_nodes[0].loc(),
                    target_nodes[0].to_string(),
                )
            }
        }
        Ok(OptimizationOutcome::PackStorageVariables(outcome))
    }
}
mod test {
    use std::{fs::File, io::Write};

    use crate::{
        optimizations::{OptimizationPattern, PackStorageVariables},
        report::ReportSectionFragment,
        utils::MockSource,
    };

    #[test]
    fn test_pack_storage_variables_optimization() -> eyre::Result<()> {
        // Optimal packing
        let contract = r#"
    contract Contract {
        uint256 num0;
        uint256 num1;
        uint256 num2;
        bool bool0;
        bool bool1;
    }
    "#;
        let mut source_0 = MockSource::new().add_source("pack_storage_0.sol", contract);
        let optimization_locations = PackStorageVariables::find(&mut source_0.source)?;
        assert_eq!(optimization_locations.len(), 0);

        // Cannot pack better, 2x bytes24 don't fit in a slot
        let contract = r#"
        contract Contract {
            bytes24 b0;
            uint256 num0;
            bytes24 b1;
        }
        "#;

        let mut source_1 = MockSource::new().add_source("pack_storage_1.sol", contract);
        let optimization_locations = PackStorageVariables::find(&mut source_1.source)?;
        assert_eq!(optimization_locations.len(), 0);

        // Cannot pack better, bool are stored with uint8 so cannot move bo1
        let contract = r#"
        contract Contract {
            bytes28 b0;
            uint8 num0;
            uint8 num1;
            uint8 num2;
            bool bo0;
            uint256 num3;
            bool bo1;
        }
        "#;

        let mut source_2 = MockSource::new().add_source("pack_storage_2.sol", contract);
        let optimization_locations = PackStorageVariables::find(&mut source_2.source)?;
        assert_eq!(optimization_locations.len(), 0);

        // Suboptimal, can be packed better
        let contract = r#"
    contract Contract {
        uint256 num0;
        uint256 num1;
        bool bool0;
        uint256 num2;
        bool bool1;
    }
    "#;

        let mut source_3 = MockSource::new().add_source("pack_storage_3.sol", contract);
        let optimization_locations = PackStorageVariables::find(&mut source_3.source)?;
        assert_eq!(optimization_locations.len(), 1);

        // Suboptimal, can be packed better (owner,bool,num0);
        let contract = r#"
    contract Contract {
        address owner;
        uint256 num0;
        bool bool0;
    }
    "#;

        let mut source_4 = MockSource::new().add_source("pack_storage_4.sol", contract);
        let optimization_locations = PackStorageVariables::find(&mut source_4.source)?;
        assert_eq!(optimization_locations.len(), 1);

        // Suboptimal, can be packed better (owner,num1,b0,num0)
        let contract = r#"
        contract Contract {
            address owner; // 160 bits
            uint256 num0;  // 256 bits
            bytes4 b0;     // 32 bits
            uint64 num1;   // 64 bits
        }
        "#;

        let mut source_5 = MockSource::new().add_source("pack_storage_5.sol", contract);
        let optimization_locations = PackStorageVariables::find(&mut source_5.source)?;
        assert_eq!(optimization_locations.len(), 1);
        let report: Option<ReportSectionFragment> = optimization_locations.into();
        if let Some(report) = report {
            let mut f = File::options()
                .append(true)
                .open("optimization_report_sections.md")?;
            writeln!(&mut f, "{}", &String::from(report))?;
        }
        Ok(())
    }
}
