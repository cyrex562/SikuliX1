//! SikuliX Python bindings using PyO3

use pyo3::prelude::*;

mod py_region;
mod py_location;

use py_location::{PyLocation, PyOffset};
use py_region::PyRegion;

/// SikuliX Python module
#[pymodule]
fn sikulix(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<PyLocation>()?;
    m.add_class::<PyOffset>()?;
    m.add_class::<PyRegion>()?;

    Ok(())
}
