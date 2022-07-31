#[cfg(not(feature = "metadata"))]
fn main() {
    // Update if needed
    println!("cargo:rerun-if-changed=src");
    println!("cargo:rerun-if-changed=build.rs");

    // Make empty file for documentation
    std::fs::File::create(std::env::var("OUT_DIR").unwrap() + "/rhai-sci-docs.md").unwrap();
}

#[cfg(feature = "metadata")]
fn main() {
    use itertools::Itertools;
    use rhai::{packages::Package, plugin::*, Engine, ScriptFnMetadata};
    use serde::{Deserialize, Serialize};
    use serde_json::Value;
    use std::collections::HashMap;
    use std::io::Write;

    #[derive(Serialize, Deserialize, Debug, Clone)]
    struct Function {
        pub access: String,
        pub baseHash: u128,
        pub fullHash: u128,
        pub name: String,
        pub namespace: String,
        pub numParams: usize,
        pub params: Option<Vec<HashMap<String, String>>>,
        pub signature: String,
        pub returnType: Option<String>,
        pub docComments: Option<Vec<String>>,
    }

    // Update if needed
    println!("cargo:rerun-if-changed=src");
    println!("cargo:rerun-if-changed=build.rs");

    // Make a file for documentation
    let mut doc_file =
        std::fs::File::create(std::env::var("OUT_DIR").unwrap() + "/rhai-sci-docs.md").unwrap();

    // Build an engine for doctests
    let mut engine = Engine::new();

    // Add custom functions from Rust
    let mut lib = Module::new();
    combine_with_exported_module!(&mut lib, "rhai_sci_matrix_function", matrix_functions);
    combine_with_exported_module!(&mut lib, "rhai_sci_miscellaneous_functions", misc_functions);
    combine_with_exported_module!(&mut lib, "rhai_sci_basic_stats", stats);
    combine_with_exported_module!(&mut lib, "rhai_sci_cumulative", cum_functions);
    combine_with_exported_module!(&mut lib, "rhai_sci_int_and_diff", int_and_diff);
    combine_with_exported_module!(&mut lib, "rhai_sci_assertions", assert_functions);
    combine_with_exported_module!(&mut lib, "rhai_sci_constants", constant_definitions);
    combine_with_exported_module!(&mut lib, "rhai_sci_sets", set_functions);
    combine_with_exported_module!(&mut lib, "rhai_sci_moving", moving_functions);
    combine_with_exported_module!(&mut lib, "rhai_sci_validate", validation_functions);
    engine.register_global_module(rhai::Shared::new(lib));

    // Extract metadata
    let mut json_fns = engine.gen_fn_metadata_to_json(false).unwrap();
    println!("{json_fns}");
    let v: HashMap<String, Vec<Function>> = serde_json::from_str(&json_fns).unwrap();
    for function in v["functions"].clone() {
        println!("{:?}", function);
    }

    let function_list = v["functions"].clone();

    // Write functions
    write!(doc_file, "# Functions\n This package provides a large variety of functions to help with scientific computing.\n").expect("Cannot write to {test_file}");
    let mut indented = false;
    for (idx, function) in function_list.iter().enumerate() {
        let mut function = function.clone();
        // Pull out basic info
        let name = function.name;
        if !name.starts_with("anon") {
            // let params = function.params.join(", ");

            let comments = match function.docComments {
                None => "".to_owned(),
                Some(strings) => strings.join("\n"),
            }
            .replace("///", "")
            .replace("/**", "")
            .replace("**/", "");

            let signature = function
                .signature
                .replace("Result<", "")
                .replace(", Box<EvalAltResult>>", "")
                .replace("&mut ", "");

            // Check if there are multiple arities, and if so add a header and indent
            if idx < function_list.len() - 1 {
                if name == function_list[idx + 1].name && !indented {
                    write!(doc_file, "## `{name}`\n").expect("Cannot write to {doc_file}");
                    indented = true;
                }
            }

            // Print definition with right level of indentation
            if indented {
                write!(doc_file, "### `{signature}`\n{comments}\n")
                    .expect("Cannot write to {doc_file}");
            } else {
                write!(doc_file, "## `{signature}`\n{comments}\n")
                    .expect("Cannot write to {doc_file}");
            }

            // End indentation when its time
            if idx != 0 && idx < function_list.len() - 1 {
                if name == function_list[idx - 1].name && name != function_list[idx + 1].name {
                    indented = false;
                }
            }

            // Run doc tests
            let code = comments.split("```").collect::<Vec<&str>>();
            for i in (1..code.len()).step_by(2) {
                let clean_code = code[i]
                    .replace("javascript", "")
                    .replace("typescript", "")
                    .replace("rhai", "");
                println!("{clean_code}");
                assert!(engine.eval::<bool>(&clean_code).unwrap());
            }
        }
    }
}

#[cfg(feature = "metadata")]
mod functions {
    include!("src/matrices_and_arrays.rs");
    include!("src/statistics.rs");
    include!("src/misc.rs");
    include!("src/cumulative.rs");
    include!("src/integration_and_differentiation.rs");
    include!("src/assertions.rs");
    include!("src/constants.rs");
    include!("src/sets.rs");
    include!("src/moving.rs");
    include!("src/validate.rs");
}

#[cfg(feature = "metadata")]
pub use functions::*;
