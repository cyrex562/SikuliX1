//! Python bindings for Region

use pyo3::prelude::*;
use sikulix_core::Region;

use crate::py_location::PyLocation;

/// A rectangular area on the screen
#[pyclass(name = "Region")]
#[derive(Clone)]
pub struct PyRegion {
    inner: Region,
}

#[pymethods]
impl PyRegion {
    #[new]
    fn new(x: i32, y: i32, w: i32, h: i32) -> Self {
        Self {
            inner: Region::new(x, y, w, h),
        }
    }

    #[getter]
    fn x(&self) -> i32 {
        self.inner.x
    }

    #[getter]
    fn y(&self) -> i32 {
        self.inner.y
    }

    #[getter]
    fn w(&self) -> i32 {
        self.inner.w
    }

    #[getter]
    fn h(&self) -> i32 {
        self.inner.h
    }

    fn center(&self) -> PyLocation {
        PyLocation::from_inner(self.inner.center())
    }

    fn contains(&self, loc: &PyLocation) -> bool {
        self.inner.contains(loc.inner())
    }

    fn area(&self) -> i32 {
        self.inner.area()
    }

    fn __repr__(&self) -> String {
        format!(
            "Region({}, {}, {}, {})",
            self.inner.x, self.inner.y, self.inner.w, self.inner.h
        )
    }

    fn __str__(&self) -> String {
        self.__repr__()
    }
}

impl PyRegion {
    pub fn from_inner(region: Region) -> Self {
        Self { inner: region }
    }

    pub fn inner(&self) -> Region {
        self.inner
    }
}
