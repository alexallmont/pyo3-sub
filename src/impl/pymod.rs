use super::T;
use super::module_name;

use pyo3::prelude::*;

#[pyclass]
struct Problem {
    _marker: std::marker::PhantomData<T>,
    _b: Option<Builder>,
}

#[pyclass]
struct Builder {
    _marker: std::marker::PhantomData<T>,
}

#[pymethods]
impl Builder {
    #[new]
    pub fn new() -> Builder {
        Builder { _marker: std::marker::PhantomData::<T> }
    }

    fn get_problem(&self) -> Problem {
        Problem { _marker: std::marker::PhantomData::<T>, _b: None }
    }
}

pub fn add_to_parent_module(
    parent_module: &Bound<'_, PyModule>
) -> PyResult<()> {
    let m = PyModule::new_bound(parent_module.py(), &module_name())?;
    m.add_class::<Builder>()?;
    m.add_class::<Problem>()?;
    parent_module.add_submodule(&m)
}
