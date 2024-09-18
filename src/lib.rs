use pyo3::prelude::*;

#[path = "impl"]
pub mod module_f32 {
    type T = f32;
    mod pymod;
    pub use pymod::*;
}

#[path = "impl"]
pub mod module_f64 {
    type T = f64;
    mod pymod;
    pub use pymod::*;
}

#[pymodule]
fn pyo3_sub(m: &Bound<'_, PyModule>) -> PyResult<()> {
    module_f32::register_module(m, "f32")?;
    module_f64::register_module(m, "f64")?;
    Ok(())
}
