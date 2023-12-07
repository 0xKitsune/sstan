# Contributing To `sstan`
Thanks for checking out the `Contribution.md`! Contributions are welcomed and encouraged. Below are the guidelines for contributions.

1.) Before starting to work on a PR, check the Github issues as well as the PRs to make sure that someone has not already PRed the addition you are thinking of contributing. If someone has already started work on a specific issue, feel free to send a message in the issue thread to see what the status of the PR is. 

2.) Open up a Github issue for the contribution. Feel free to ask any questions about the implementation or different parts of the codebase. This is a great place to refine ideas before implementing the changes and submitting a PR.

3.) PR to `main` and link the Github issue. From there, the PR will be reviewed and any edits that are necessary will be suggested. Once all edits are complete and the CI pipeline clears, your contribution will be merged!


The repository can seem a little dense in some parts but adding a new optimization, vulnerability or QA pattern is simple! Below is a quick walkthrough on how to add new patterns.

<br>


## Extractors
`sstan` uses "Extractors" to extract target nodes from the AST generated from a Solidity file. There are two types of extractors; primitive and compound. Primitive extractors extract a simple type from the AST like a `FunctionDefinition`, `FunctionCall` or `StructDefinition`. Compound extractors extract complex types, typically utilizing one or more primitive extractors under the hood. For example, The `ConstructorExtractor` extracts all constructors from a Solidity file. The `ConstructorExtractor` first uses the `FunctionExtractor` to get all functions from the file, then filtering all functions except for the constructors.

```rust
compound_extractor!(ConstructorExtractor, FunctionDefinition);

#[allow(clippy::unnecessary_filter_map)]
impl<V: Visitable> Extractor<V, FunctionDefinition> for ConstructorExtractor {
    fn extract(v: &mut V) -> Result<Vec<FunctionDefinition>, ExtractionError> {
        let functions = FunctionExtractor::extract(v)?;
        let constructors = functions
            .iter()
            .filter(|function| FunctionTy::Constructor == function.ty)
            .cloned()
            .collect::<Vec<FunctionDefinition>>();
        Ok(constructors)
    }
}
```

Under the hood extractors leverage a visitor pattern based on the [visitor pattern used in Foundry](https://github.com/foundry-rs/foundry/blob/master/crates/fmt/src/visit.rs#L9-L384) which parses through an AST, visiting each node in the tree. For an in depth look at the `Extractor` trait, you can [check out the source code](https://github.com/0xKitsune/sstan/blob/main/src/extractors/mod.rs). If you would like to browse through all the available extractors, you can [check out the docs](). If the extractor you are looking for is not available, feel free to make a PR! Please do not hesitate to open a Github issue, we'll be happy to help with any questions or comments. 

Using extractors makes extracting all instances of a specific node or pattern very straightforward. For example `AssignmentExtractor::extract(ast_node)` gets all assignments from a node, `MutableStorageVariableExtractor::extract(ast_node)` gets all mutable storage variables and `EventExtractor::extract(ast_node)` gets all event definitions. 

Let's take a quick look at extractors in action within our first `OptimizationPattern`. For now, don't worry about what an optimization pattern is, we will cover this in just a moment. The following optimization pattern looks for instances of `keccak256()` outside an assembly block, noting that using `keccak256()` with inline assembly is more gas efficient. Note that the function first extracts all function calls with the `FunctionCallExtractor`, then inserting the finding into the outcomes if the function name is `keccak256` (`sstan` uses the [solang-parser](https://crates.io/crates/solang-parser) which considers built-in functions as function calls).

```rust
impl OptimizationPattern for SolidityKeccak256 {
    fn find(source: &mut HashMap<PathBuf, SourceUnit>) -> Result<OptimizationOutcome, EngineError> {
        let mut outcome = Outcome::new();
        for (path_buf, source_unit) in source {
            //Extract the target nodes from the source_unit
            let function_call_nodes = FunctionCallExtractor::extract(source_unit)?;

            //For each target node that was extracted, check for the optimization patterns
            for node in function_call_nodes {
                //Can unwrap because FunctionCall is an expression
                if let pt::Expression::FunctionCall(_, box_expression, _) = node.clone() {
                    if let pt::Expression::Variable(variable) = *box_expression {
                        if variable.name == "keccak256" {
                            outcome.push_or_insert(path_buf.clone(), node.loc(), node.to_string());
                        }
                    }
                }
            }
        }
        //Return the identified optimization locations
        Ok(OptimizationOutcome::SolidityKeccak256(outcome))
    }
}
```

<br>

## Adding a Pattern

Patterns are classified into three categories which are optimizations, vulnerabilities and QA patterns. Let's take a look at how to add a new pattern by using optimizations as an example.

All optimizations are located in `src/analyzer/optimizations`. Here you will see a file for each of the optimizations that `sstan` looks for. To add a new optimization, start by adding a new file in this directory.

Optimizations must implement the `OptimizationPattern` trait which specifies one trait method, `find()`. Using the `keccak256` optimization mentioned above, let's take a look at the `find()` method. This function takes `HashMap<PathBuf, SourceUnit>` where each `SourceUnit` is the AST representation of a Solidity file, and returns an `OptimizationOutcome`.

```rust
impl OptimizationPattern for SolidityKeccak256 {
    fn find(source: &mut HashMap<PathBuf, SourceUnit>) -> Result<OptimizationOutcome, EngineError> {

    // Your logic here

    }
}
```

Now this is where the codebase becomes a bit complex, but this is also where all the magic happens. Within `src/optimizations/mod.rs`, there is the [optimization!](https://github.com/0xKitsune/sstan/blob/main/src/optimizations/mod.rs#L41-L171) macro which takes a few arguments defining the struct representing the optimization, an identifier string, the gas savings per finding, the report title, description of the finding and the degree of gas savings. Once this is specified, the pattern will then be included in the report.


```rust
optimization!(
    (
        SolidityKeccak256, // Struct name
        "solidity_keccak256", // String identitfier used when specifying the pattern in custom `sstan.toml` configurations
        82, // Gas savings per finding
        "Use assembly to hash instead of Solidity", // Report title
        "Hashing is a safe operation to perform in assembly, and it is cheaper than Solidity's `keccak256` function.", // Report description
        Classification::OptimizationMedium // Gas saving classification
    )
)
```

This is how `sstan` achieves extremely fast dev times to implement new patterns. Simply implement the `OptimizationPattern` trait and add the pattern to the macro without having to touch any other part of the codebase. This same approach exists for the `VulnerabilityPattern` and `QAPattern` traits with the only difference being the arguments passed into the respective macros.

```rust
vulnerability!(
    (
        DoubleCasting, // Struct name
        "double_casting", // String identitfier used when specifying the pattern in custom `sstan.toml` configurations
        "Double Casting", // Report title
        "Avoid double casting as it may introduce unexpected truncations/rounding errors among other issues.", // Report description
        Classification::VulnerabilityLow // Severity classification
    ),
)

quality_assurance!(
    (
        ConstructorOrder, // Struct name
        "constructor_order", // String identitfier used when specifying the pattern in custom `sstan.toml` configurations
        "Constructor should be listed before any other function", // Report title
        "Consider changing the order of the functions so that the constructor is listed first" // Report description
    ),
)


```

With that, you are now equipped to start implementing patterns at the speed of light. Happy racing!
