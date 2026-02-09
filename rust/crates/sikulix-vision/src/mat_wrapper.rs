//! Safe wrapper around OpenCV Mat with automatic memory management

use opencv::core::{Mat, MatTraitConst};
use opencv::prelude::*;
use std::fmt;

/// Safe wrapper around OpenCV Mat with RAII memory management
///
/// This wrapper ensures that:
/// - Mat memory is properly freed when dropped
/// - Mat is not copied accidentally (expensive operation)
/// - Thread-safe access is enforced
pub struct MatWrapper {
    /// The underlying OpenCV Mat
    /// We use Option to allow taking ownership during conversions
    inner: Option<Mat>,
}

impl MatWrapper {
    /// Create a new MatWrapper from an OpenCV Mat
    pub fn new(mat: Mat) -> Self {
        Self { inner: Some(mat) }
    }

    /// Create an empty MatWrapper
    pub fn empty() -> opencv::Result<Self> {
        Ok(Self {
            inner: Some(Mat::default()),
        })
    }

    /// Get a reference to the underlying Mat
    pub fn as_mat(&self) -> &Mat {
        self.inner.as_ref().expect("Mat was already consumed")
    }

    /// Get a mutable reference to the underlying Mat
    pub fn as_mat_mut(&mut self) -> &mut Mat {
        self.inner.as_mut().expect("Mat was already consumed")
    }

    /// Take ownership of the underlying Mat, consuming this wrapper
    pub fn into_mat(mut self) -> Mat {
        self.inner.take().expect("Mat was already consumed")
    }

    /// Check if the Mat is empty
    pub fn is_empty(&self) -> bool {
        self.as_mat().empty()
    }

    /// Get the Mat dimensions (width, height)
    pub fn size(&self) -> opencv::Result<(i32, i32)> {
        let size = self.as_mat().size()?;
        Ok((size.width, size.height))
    }

    /// Get the number of channels (1 for grayscale, 3 for BGR, 4 for BGRA)
    pub fn channels(&self) -> opencv::Result<i32> {
        Ok(self.as_mat().channels())
    }

    /// Get the Mat type (CV_8UC3, etc.)
    pub fn mat_type(&self) -> opencv::Result<i32> {
        Ok(self.as_mat().typ())
    }

    /// Clone the underlying Mat (expensive operation)
    pub fn clone_mat(&self) -> opencv::Result<MatWrapper> {
        let cloned = self.as_mat().try_clone()?;
        Ok(MatWrapper::new(cloned))
    }
}

impl fmt::Debug for MatWrapper {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(mat) = &self.inner {
            let size = mat.size().unwrap_or_default();
            let channels = mat.channels();
            let mat_type = mat.typ();
            f.debug_struct("MatWrapper")
                .field("width", &size.width)
                .field("height", &size.height)
                .field("channels", &channels)
                .field("type", &mat_type)
                .field("empty", &mat.empty())
                .finish()
        } else {
            f.debug_struct("MatWrapper")
                .field("consumed", &true)
                .finish()
        }
    }
}

impl Drop for MatWrapper {
    fn drop(&mut self) {
        // Mat implements Drop internally, so we just need to let it go
        // This is automatic, but we make it explicit for clarity
        if let Some(_mat) = self.inner.take() {
            // Mat's Drop implementation will free the memory
            tracing::trace!("Dropping MatWrapper and its underlying Mat");
        }
    }
}

// Prevent automatic cloning (Mat cloning is expensive)
// Users must explicitly call clone_mat()

#[cfg(test)]
mod tests {
    use super::*;
    use opencv::core::{Mat, CV_8UC3};

    #[test]
    fn test_create_empty() {
        let wrapper = MatWrapper::empty().unwrap();
        assert!(wrapper.is_empty());
    }

    #[test]
    fn test_create_from_mat() {
        let mat = Mat::new_rows_cols_with_default(100, 200, CV_8UC3, (0, 0, 0, 0).into()).unwrap();
        let wrapper = MatWrapper::new(mat);
        assert!(!wrapper.is_empty());
        assert_eq!(wrapper.size().unwrap(), (200, 100)); // width, height
    }

    #[test]
    fn test_mat_properties() {
        let mat = Mat::new_rows_cols_with_default(50, 75, CV_8UC3, (0, 0, 0, 0).into()).unwrap();
        let wrapper = MatWrapper::new(mat);

        assert_eq!(wrapper.size().unwrap(), (75, 50));
        assert_eq!(wrapper.channels().unwrap(), 3);
        assert!(!wrapper.is_empty());
    }

    #[test]
    fn test_clone_mat() {
        let mat = Mat::new_rows_cols_with_default(10, 20, CV_8UC3, (255, 128, 64, 0).into()).unwrap();
        let wrapper = MatWrapper::new(mat);

        let cloned = wrapper.clone_mat().unwrap();
        assert_eq!(cloned.size().unwrap(), wrapper.size().unwrap());
        assert_eq!(cloned.channels().unwrap(), wrapper.channels().unwrap());
    }

    #[test]
    fn test_into_mat() {
        let mat = Mat::new_rows_cols_with_default(10, 20, CV_8UC3, (0, 0, 0, 0).into()).unwrap();
        let wrapper = MatWrapper::new(mat);

        let size_before = wrapper.size().unwrap();
        let mat_back = wrapper.into_mat();

        let size_after = mat_back.size().unwrap();
        assert_eq!(size_before, (size_after.width, size_after.height));
    }

    #[test]
    fn test_debug_format() {
        let mat = Mat::new_rows_cols_with_default(10, 20, CV_8UC3, (0, 0, 0, 0).into()).unwrap();
        let wrapper = MatWrapper::new(mat);

        let debug_str = format!("{:?}", wrapper);
        assert!(debug_str.contains("MatWrapper"));
        assert!(debug_str.contains("width"));
        assert!(debug_str.contains("height"));
    }
}
