use super::T;

use pyo3::prelude::*;

#[pyclass]
pub struct Problem {
    pub _marker: std::marker::PhantomData<T>,
}
