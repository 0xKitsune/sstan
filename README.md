# sstan [![Github Actions][gha-badge]][gha]
[gha]: https://github.com/0xKitsune/sstan/actions
[gha-badge]: https://github.com/0xKitsune/sstan/actions/workflows/ci.yml/badge.svg

`sstan` is a Solidity static analyzer specifically designed for the [Code4Arena Bot Races](https://code4rena.com/register/bot). With the constantly changing landscape of the bot races, this design prioritizes DevX, using an [Extractor pattern]() and macros to enable extremely quick development times when implementing new patterns. `sstan` comes "out of the box" with patterns to identify 50+ optimizations, vulnerabilities and QA patterns.

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
First, make sure that you have [Rust installed](https://www.rust-lang.org/tools/install). Then you can enter the following commands in your terminal.

```
git clone https://github.com/0xKitsune/sstan &&
cd sstan &&
cargo install --path .
```

&nbsp;
# Usage
Now that you have `sstan` installed, you can use the `sstan` command from anywhere in your terminal. By default, sstan looks for a `./src` directory and analyzes every file within the folder. If you would like to specify the directory `sstan` should target, you can pass the `--path` flag (ex. `sstan --path <path_to_dir>`). 

In the default configuration, sstan runs analysis for every [currently included optimization, vulnerability and QA pattern](https://github.com/0xKitsune/sstan#currently-identified-optimizations-vulnerabilities-and-qa), however if you would like to run analysis for select patterns, you can create a `.toml` file for your custom configuration.  You can use the [default sstan.toml configuration](https://github.com/0xKitsune/sstan/blob/main/sstan.toml) for reference. After creating a custom `.toml` file, make sure to pass the `--toml` flag when running `sstan` (ex. `sstan --toml <path_to_toml_file>`).

```
Usage: sstan [OPTIONS]

Options:
  -p, --path <PATH>      Path to the root directory to analyze. The default directory is `./src`
  -o, --output <OUTPUT>  Path to the directory where the report will be written. The default directory is `./`
  -g, --git <GIT>        Github repository link for the codebase being analyzed (e.g `https://github.com/repo/blob/main`). This will create hyperlinks to line numbers within the final report.
  -t, --toml <TOML>      Path to `.toml` file containing a custom sstan configuration.
  -h, --help             Print help
```

&nbsp;
# Contributing
Check out [Contributing.md](https://github.com/0xKitsune/sstan/blob/main/docs/Contributing.md) for adding new features.
