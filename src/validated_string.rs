use std::ops::Deref;

use anyhow::{Context, Result};
use pyo3::{FromPyObject, IntoPyObject};
use serde::{Deserialize, Serialize};

// String that is validated to be less than 1000 characters

#[derive(
    Debug, Default, Clone, PartialEq, Eq, Serialize, Deserialize, FromPyObject, IntoPyObject,
)]
#[serde(transparent)]
#[pyo3(transparent)]
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
}

impl Deref for ValidatedString {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl TryFrom<String> for ValidatedString {
    type Error = anyhow::Error;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::from_string(value).context("Error converting String to ValidatedString")
    }
}

impl TryFrom<&str> for ValidatedString {
    type Error = anyhow::Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::from_string(value.to_string()).context("Error converting &str to ValidatedString")
    }
}
