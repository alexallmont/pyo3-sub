#[path = "./problem.rs"]
mod problem;

use super::T;

use problem::Problem;

use pyo3::prelude::*;

#[pyclass]
pub struct Builder {
    pub _marker: std::marker::PhantomData<T>,
}

#[pymethods]
impl Builder {
    #[new]
    pub fn new() -> Builder {
        Builder { _marker: std::marker::PhantomData::<T> }
    }

    fn get_problem(&self) -> Problem {
        Problem { _marker: std::marker::PhantomData::<T> }
    }
}