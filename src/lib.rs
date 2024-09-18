use pyo3::prelude::*;

#[path = "impl"]
pub mod module_f32 {
    type T = f32;
    fn module_name() -> String { "f32".to_string() }
    mod pymod;
    pub use pymod::*;
}

#[path = "impl"]
pub mod module_f64 {
    type T = f64;
    fn module_name() -> String { "f64".to_string() }
    mod pymod;
    pub use pymod::*;
}

#[pymodule]
fn pyo3_sub(m: &Bound<'_, PyModule>) -> PyResult<()> {
    module_f32::add_to_parent_module(m)?;
    module_f64::add_to_parent_module(m)?;
    Ok(())
}
