use pyo3::prelude::*;

#[path="private/create_binding.rs"]
mod create_binding;

create_binding!(f32, module_f32);
create_binding!(f64, module_f64);

/// Top-level module for diffsol bindings by type
#[pymodule]
fn pyo3_sub(m: &Bound<'_, PyModule>) -> PyResult<()> {
    module_f32::add_to_parent_module(m)?;
    module_f64::add_to_parent_module(m)?;
    Ok(())
}
