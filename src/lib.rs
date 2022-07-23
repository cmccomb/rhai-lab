#![warn(clippy::all)]
#![warn(missing_docs)]
#![warn(rustdoc::missing_doc_code_examples)]
#![warn(clippy::missing_docs_in_private_items)]
#![doc = include_str!("../README.md")]
#![doc = include_str!(concat!(env!("OUT_DIR"), "/rhai-sci-docs.md"))]
#![doc = include_str!("../docs/highlight.html")]

use rhai::{def_package, packages::Package, plugin::*, Engine, EvalAltResult};
use rhai_rand::RandomPackage;
mod matrix;
use matrix::matrix_functions;
mod basic_statistics;
pub use basic_statistics::stats;
mod utils;
pub use utils::util_functions;

def_package! {
    /// Package for scientific computing
    pub SciPackage(lib) {

        // Load random package
        RandomPackage::init(lib);

        combine_with_exported_module!(lib, "matrix", matrix_functions);
        combine_with_exported_module!(lib, "stats", stats);
        combine_with_exported_module!(lib, "utils", util_functions);

        // Load scripts
        let engine = Engine::new();
        let ast = engine.compile(include_str!(concat!(env!("OUT_DIR"), "/rhai-sci-compiled.txt"))).unwrap();
        let my_module = Module::eval_ast_as_new(rhai::Scope::new(), &ast, &engine).unwrap();
        lib.fill_with(&my_module);
    }
}

/// This provides the ability to easily evaluate a line (or lines) of code without explicitly
/// setting up a script engine
/// ```
/// use rhai_sci::eval;
/// use rhai::FLOAT;
/// print!("{:?}", eval::<FLOAT>("let x = max(5, 2); x + min(3, 72)"));
/// ```
pub fn eval<T: Clone + 'static>(script: &str) -> Result<T, Box<EvalAltResult>> {
    let mut engine = Engine::new();
    engine.register_global_module(SciPackage::new().as_shared_module());
    engine.eval::<T>(script)
}
