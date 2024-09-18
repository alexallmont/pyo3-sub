use pyo3::prelude::*;

#[path="private"]
pub mod module_f32 {
    type T = f32;
    static MODULE_NAME:&'static str = "module_f32";
    mod bindings;
    pub use bindings::*;
}

#[path="private"]
pub mod module_f64 {
    type T = f64;
    static MODULE_NAME:&'static str = "module_f64";
    mod bindings;
    pub use bindings::*;
}

/// Top-level module for diffsol bindings by type
#[pymodule]
fn pyo3_sub(m: &Bound<'_, PyModule>) -> PyResult<()> {
    module_f32::add_to_parent_module(m)?;
    module_f64::add_to_parent_module(m)?;
    Ok(())
}
