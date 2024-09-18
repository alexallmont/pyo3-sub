use super::T;

use pyo3::prelude::*;

#[path = "./builder.rs"]
mod builder;

#[path = "./problem.rs"]
mod problem;

pub fn register_module(
    parent_module: &Bound<'_, PyModule>,
    name: &str
) -> PyResult<()> {
    let m = PyModule::new_bound(parent_module.py(), name)?;
    m.add_class::<builder::Builder>()?;
    m.add_class::<problem::Problem>()?;
    parent_module.add_submodule(&m)
}
