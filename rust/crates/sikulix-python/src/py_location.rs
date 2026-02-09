//! Python bindings for Location and Offset

use pyo3::prelude::*;
use sikulix_core::{Location, Offset};

/// A point on the screen with x, y coordinates
#[pyclass(name = "Location")]
#[derive(Clone)]
pub struct PyLocation {
    inner: Location,
}

#[pymethods]
impl PyLocation {
    #[new]
    fn new(x: i32, y: i32) -> Self {
        Self {
            inner: Location::new(x, y),
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

    fn offset(&self, offset: &PyOffset) -> Self {
        Self {
            inner: self.inner.offset(offset.inner),
        }
    }

    fn distance_to(&self, other: &PyLocation) -> f64 {
        self.inner.distance_to(other.inner)
    }

    fn __repr__(&self) -> String {
        format!("Location({}, {})", self.inner.x, self.inner.y)
    }

    fn __str__(&self) -> String {
        self.__repr__()
    }
}

impl PyLocation {
    pub fn from_inner(loc: Location) -> Self {
        Self { inner: loc }
    }

    pub fn inner(&self) -> Location {
        self.inner
    }
}

/// A relative displacement with dx, dy components
#[pyclass(name = "Offset")]
#[derive(Clone)]
pub struct PyOffset {
    inner: Offset,
}

#[pymethods]
impl PyOffset {
    #[new]
    fn new(dx: i32, dy: i32) -> Self {
        Self {
            inner: Offset::new(dx, dy),
        }
    }

    #[getter]
    fn dx(&self) -> i32 {
        self.inner.dx
    }

    #[getter]
    fn dy(&self) -> i32 {
        self.inner.dy
    }

    fn __repr__(&self) -> String {
        format!("Offset({}, {})", self.inner.dx, self.inner.dy)
    }

    fn __str__(&self) -> String {
        self.__repr__()
    }
}

impl PyOffset {
    pub fn from_inner(offset: Offset) -> Self {
        Self { inner: offset }
    }

    pub fn inner(&self) -> Offset {
        self.inner
    }
}
