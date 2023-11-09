use std::{fs, io::Write, process};

use clap::Parser;

use sstan::{
    engine::Engine, optimizations::OptimizationTarget, qa::QualityAssuranceTarget, report::Report,
    vulnerabilities::VulnerabilityTarget,
};

pub const DEFAULT_PATH: &str = "./src";

extern crate colour;
fn main() -> eyre::Result<()> {
    let mut engine = Engine::new(
        "./bin",
        Some("https://github.com/0xKitsune/sstan/blob/main".to_string()),
        vec![],
        vec![OptimizationTarget::ReadStorageInForLoop],
        vec![],
    );

    //Populate the modules
    engine.run()?;
    //Generate the report struct
    let report = Report::from(engine);
    //Generate the report string & write to the output path.
    std::fs::File::create("sstan.md")?.write_all(String::from(report).as_bytes())?;

    Ok(())
}
