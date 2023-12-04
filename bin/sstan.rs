// use report::generation::generate_report;
// use sstan::analyzer;
// use sstan::report;
// use std::{fs, process};

use std::{fs, io::Write, process};

use clap::Parser;

use sstan::{
    engine::Engine,
    optimizations::OptimizationTarget,
    qa::QualityAssuranceTarget,
    report::{JsonReport, Report},
    utils::{str_to_optimization, str_to_qa, str_to_vulnerability},
    vulnerabilities::VulnerabilityTarget,
};

pub const DEFAULT_PATH: &str = "./src";

#[macro_use]
extern crate colour;
fn main() -> eyre::Result<()> {
    let opts = Opts::new();

    let mut engine = Engine::new(
        &opts.path,
        opts.git,
        opts.vulnerabilities,
        opts.optimizations,
        opts.qa,
    );

    //Populate the modules
    engine.run()?;
    //Generate the report struct
    let report = Report::from(engine);
    //Generate the report string & write to the output path.
    //Write to json
    std::fs::File::create("sstan.json")?.write_all(
        serde_json::to_string(&JsonReport::from(report.clone()))
            .unwrap()
            .as_bytes(),
    )?;
    //Write to markdown
    std::fs::File::create("sstan.md")?.write_all(String::from(report).as_bytes())?;

    Ok(())
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
        help = "Path to the directory to write the report to. The default directory is `./`"
    )]
    pub output: Option<String>,
    #[clap(short, long, help = "Github repository link")]
    pub git: Option<String>,
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
    pub output: String,
    pub git: Option<String>,
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
            let toml_str =
                fs::read_to_string(toml_path).expect("Could not read toml file to string");

            let sstan_toml: SstanToml =
                toml::from_str(&toml_str).expect("Could not convert toml contents to sstanToml");
            (
                sstan_toml
                    .optimizations
                    .iter()
                    .filter_map(|f| str_to_optimization(f))
                    .collect::<Vec<OptimizationTarget>>(),
                sstan_toml
                    .vulnerabilities
                    .iter()
                    .filter_map(|f| str_to_vulnerability(f))
                    .collect::<Vec<VulnerabilityTarget>>(),
                sstan_toml
                    .vulnerabilities
                    .iter()
                    .filter_map(|f| str_to_qa(f))
                    .collect::<Vec<QualityAssuranceTarget>>(),
            )
        } else {
            (
                OptimizationTarget::all_targets(),
                VulnerabilityTarget::all_targets(),
                QualityAssuranceTarget::all_targets(),
            )
        };

        let output = if let Some(output) = args.output {
            output
        } else {
            "".into()
        };
        //Github repo link to the root
        let git = args.git;

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
            output,
            git,
            optimizations,
            vulnerabilities,
            qa,
        }
    }
}
