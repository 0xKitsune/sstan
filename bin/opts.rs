#[derive(Default)]
pub struct Opts {
    pub path: String,
    pub output: String,
    pub git: Option<String>,
    pub json: Option<String>,
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
        let json = args.json;
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
            json,
            optimizations,
            vulnerabilities,
            qa,
        }
    }
}
