use super::T;
use super::MODULE_NAME;

use pyo3::prelude::*;

/// Problem does X
///
/// Longer docs expanding on why problem does X, how problem does X and when it
/// should be doing X.
#[pyclass]
struct Problem {
    _marker: std::marker::PhantomData<T>,
    _b: Option<Builder>,
}

/// Builder does Y
///
/// Y is the bees knees, really it is. More text here rambling on saying stuff
/// and then more just so long as it flows over a few lines.
#[pyclass]
struct Builder {
    _marker: std::marker::PhantomData<T>,
}

#[pymethods]
impl Builder {
    /// Create a new builder
    #[new]
    pub fn new() -> Builder {
        Builder { _marker: std::marker::PhantomData::<T> }
    }

    /// Get the problem
    fn get_problem(&self) -> Problem {
        Problem { _marker: std::marker::PhantomData::<T>, _b: None }
    }
}

pub fn add_to_parent_module(
    parent_module: &Bound<'_, PyModule>
) -> PyResult<()> {
    let m = PyModule::new_bound(parent_module.py(), &MODULE_NAME)?;

    m.add_class::<Builder>()?;
    m.add_class::<Problem>()?;

    // As the module is created programatically rather than #[pymodule], the
    // docs have to be set up manually rather than pyo3 parsing /// comments.
    m.setattr("__doc__", format!("diffsol wrapper for {} type", &MODULE_NAME))?;

    parent_module.add_submodule(&m)
}
