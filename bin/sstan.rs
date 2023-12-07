use clap::Parser;
use sstan::{
    engine::Engine,
    optimizations::OptimizationTarget,
    qa::QualityAssuranceTarget,
    report::{JsonReport, Report},
    vulnerabilities::VulnerabilityTarget,
};
use std::{fs, io::Write, str::FromStr};

pub const DEFAULT_PATH: &str = "./src";

#[derive(Parser, Debug)]
#[clap(
    name = "sstan",
    about = "A Solidity static analyzer to identify contract optimizations, vulnerabilities and QA patterns."
)]

pub struct Args {
    #[clap(
        short,
        long,
        help = "Path to the root directory to analyze. The default directory is `./src`"
    )]
    pub path: Option<String>,
    #[clap(
        short,
        long,
        help = "Path to the directory where the report will be written. The default directory is `./`"
    )]
    pub output: Option<String>,
    #[clap(
        short,
        long,
        help = "Github repository link for the codebase being analyzed (e.g `https://github.com/repo/blob/main`). This will create hyperlinks to line numbers within the final report."
    )]
    pub git: Option<String>,
    #[clap(
        short,
        long,
        help = "Path to `.toml` file containing a custom sstan configuration."
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
                toml::from_str(&toml_str).expect("Could not convert toml contents to SstanToml");
            (
                sstan_toml
                    .optimizations
                    .iter()
                    .map(|f| OptimizationTarget::from_str(f).expect("Unrecognized optimization"))
                    .collect::<Vec<OptimizationTarget>>(),
                sstan_toml
                    .vulnerabilities
                    .iter()
                    .map(|f| VulnerabilityTarget::from_str(f).expect("Unrecognized vulnerability"))
                    .collect::<Vec<VulnerabilityTarget>>(),
                sstan_toml
                    .qa
                    .iter()
                    .map(|f| QualityAssuranceTarget::from_str(f).expect("Unrecognized qa pattern"))
                    .collect::<Vec<QualityAssuranceTarget>>(),
            )
        } else {
            (
                OptimizationTarget::all_targets(),
                VulnerabilityTarget::all_targets(),
                QualityAssuranceTarget::all_targets(),
            )
        };

        Opts {
            path: args.path.unwrap_or(DEFAULT_PATH.into()),
            output: args.output.unwrap_or_default(),
            git: args.git,
            optimizations,
            vulnerabilities,
            qa,
        }
    }
}

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
