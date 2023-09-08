// use report::generation::generate_report;
// use sstan::analyzer;
// use sstan::report;
// use std::{fs, process};

use std::{fs, process};

use clap::Parser;
use sstan::{
    engine::Engine,
    optimizations::{self, OptimizationTarget},
    qa::{self, QualityAssuranceTarget},
    vulnerabilities::VulnerabilityTarget,
};

pub const DEFAULT_PATH: &str = "./src";

#[macro_use]
extern crate colour;
fn main() -> eyre::Result<()> {
    let opts = Opts::new();

    let engine = Engine::new(opts.vulnerabilities, opts.optimizations, opts.qa);

    engine.run();

    //     let vulnerabilities = vulnerabilities::analyze_dir(&opts.path, opts.vulnerabilities)?;
    //     let optimizations = optimizations::analyze_dir(&opts.path, opts.optimizations)?;
    //     let qa = qa::analyze_dir(&opts.path, opts.qa)?;

    //     generate_report(vulnerabilities, optimizations, qa);
    //     Ok(())
    // }

    todo!()
}

#[derive(Parser, Debug)]
#[clap(
    name = "sstan",
    about = "A Solidity static analyzer to identify contract vulnerabilities and gas efficiencies."
)]

pub struct Args {
    #[clap(
        short,
        long,
        help = "Path to the directory containing the files sstan will analyze. The default directory is `./src`"
    )]
    pub path: Option<String>,

    #[clap(
        short,
        long,
        help = "Path to the toml file containing the sstan configuration when not using the default settings."
    )]
    pub toml: Option<String>,
}

#[derive(Default)]
pub struct Opts {
    pub path: String,
    vulnerabilities: Vec<VulnerabilityTarget>,
    optimizations: Vec<OptimizationTarget>,
    qa: Vec<QualityAssuranceTarget>,
}

#[derive(serde::Deserialize, Debug)]
pub struct SstanToml {
    pub path: String,
    pub optimizations: Vec<String>,
    pub vulnerabilities: Vec<String>,
    pub qa: Vec<String>,
}

impl Opts {
    pub fn new() -> Opts {
        let args = Args::parse();

        let (optimizations, vulnerabilities, qa) = if let Some(toml_path) = args.toml {
            // let toml_str =
            //     fs::read_to_string(toml_path).expect("Could not read toml file to string");

            // let sstan_toml: SstanToml =
            //     toml::from_str(&toml_str).expect("Could not convert toml contents to sstanToml");

            // (
            //     sstan_toml
            //         .optimizations
            //         .iter()
            //         .map(|f| str_to_optimization(f))
            //         .collect::<Vec<OptimizationTarget>>(),
            //     sstan_toml
            //         .vulnerabilities
            //         .iter()
            //         .map(|f| str_to_vulnerability(f))
            //         .collect::<Vec<VulnerabilityTarget>>(),
            //     sstan_toml
            //         .vulnerabilities
            //         .iter()
            //         .map(|f| str_to_qa(f))
            //         .collect::<Vec<QualityAssuranceTarget>>(),
            // )

            (
                OptimizationTarget::all_targets(),
                VulnerabilityTarget::all_targets(),
                QualityAssuranceTarget::all_targets(),
            )
        } else {
            (
                OptimizationTarget::all_targets(),
                VulnerabilityTarget::all_targets(),
                QualityAssuranceTarget::all_targets(),
            )
        };

        let path = if let Some(path) = args.path {
            path
        } else {
            match fs::read_dir(DEFAULT_PATH) {
                Ok(_) => {}

                Err(_) => {
                    yellow!(
                        "Error when reading the target contracts directory.
If the `--path` flag is not passed, sstan will look for `./src` by default.
To fix this, either add a `./contracts` directory or provide `--path <path_to_contracts_dir>\n"
                    );
                    process::exit(1)
                }
            }
            DEFAULT_PATH.into()
        };

        Opts {
            path,
            optimizations,
            vulnerabilities,
            qa,
        }
    }
}
