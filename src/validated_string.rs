use std::{convert::Infallible, ops::Deref};

use anyhow::{Context, Result};
use pyo3::{
    types::{PyAnyMethods, PyString},
    Bound, FromPyObject, IntoPyObject, PyAny, PyResult, Python,
};
use serde::{Deserialize, Serialize};

// String that is validated to be less than 1000 characters

#[derive(Debug, Default, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(transparent)]
pub struct ValidatedString {
    value: String,
}

impl ValidatedString {
    pub fn from_string(value: String) -> Result<Self> {
        if value.len() > 1000 {
            anyhow::bail!("string is too long");
        }
        Ok(Self { value })
    }

    pub fn from_str(value: &str) -> Result<Self> {
        Self::from_string(value.to_string())
    }
}

impl Deref for ValidatedString {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl From<String> for ValidatedString {
    fn from(value: String) -> Self {
        Self::from_string(value).unwrap()
    }
}

impl From<&str> for ValidatedString {
    fn from(value: &str) -> Self {
        Self::from_str(value).unwrap()
    }
}

impl<'py> IntoPyObject<'py> for ValidatedString {
    type Target = PyString;
    type Output = Bound<'py, Self::Target>;
    type Error = Infallible;

    fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
        Ok(PyString::new(py, &self.value))
    }
}

impl FromPyObject<'_> for ValidatedString {
    fn extract_bound(obj: &Bound<'_, PyAny>) -> PyResult<Self> {
        let s = obj.extract::<String>()?;
        Ok(Self::from_string(s).context("Error converting PyString to ValidatedString")?)
    }
}
