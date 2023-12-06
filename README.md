# sstan [![Github Actions][gha-badge]][gha]
[gha]: https://github.com/0xKitsune/sstan/actions
[gha-badge]: https://github.com/0xKitsune/sstan/actions/workflows/ci.yml/badge.svg

# Table of Contents
- [Installation](#installation)
- [Usage](#usage)
- [Identified Issues](https://github.com/0xKitsune/sstan/tree/main/docs)
  - [⚡Optimizations](https://github.com/0xKitsune/sstan/blob/main/docs/identified-optimizations.md)
  - [🪲Vulnerabilities](https://github.com/0xKitsune/sstan/blob/main/docs/identified-vulnerabilities.md)
  - [👍Quality Assurance](https://github.com/0xKitsune/sstan/blob/main/docs/identified-quality-assurance.md)
- [Example Reports](https://github.com/0xKitsune/sstan-reports)
- [Contributing](#contributing)


&nbsp;
# Installation
First, make sure that you have [Rust installed](https://www.rust-lang.org/tools/install). Then you can choose either of the installation methods by entering the corresponding command in your terminal below.

&nbsp;
### Install from source
```
git clone https://github.com/0xKitsune/sstan &&
cd sstan &&
cargo install --path .
```

&nbsp;
# Usage
Now that you have sstan installed, you can use the `sstan` command from anywhere in your terminal. By default, sstan looks for a `./contracts` directory and analyzes every file within the folder. If you would like to specify the directory sstan should use, you can pass the `--path` flag (ex. `sstan --path <path_to_dir>`). 

In the default configuration, sstan runs analysis for every [currently included Optimization, Vulnerability and QA](https://github.com/0xKitsune/sstan#currently-identified-optimizations-vulnerabilities-and-qa), however if you would like to run analysis for select patterns, you can create a `.toml` file for your custom configuration.  Check out the [default sstan.toml configuration](https://github.com/0xKitsune/sstan/blob/main/sstan.toml) for reference. After creating a custom `.toml` file, make sure to pass the `--toml` flag when running sstan (ex. `sstan --toml <path_to_toml_file>`).

Once sstan runs its analysis, a report will be generated and output as `sstan_report.md`.

At any point you can use `sstan --help` to see a list of all commands and options.

```
Usage: sstan [OPTIONS]

Options:
  -p, --path <PATH>  Path to the directory containing the files sstan will analyze. The default directory is `./contracts`
  -t, --toml <TOML>  Path to the toml file containing the sstan configuration when not using the default settings.
  -g, --git <URL> Path to the root of the git repository.
  -h, --help         Print help information
```

&nbsp;
# Contributing
Check out [Contributing.md](https://github.com/0xKitsune/sstan/blob/main/docs/Contributing.md) for adding new features.
