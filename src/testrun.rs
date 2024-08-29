use std::fmt::Display;

use pyo3::class::basic::CompareOp;
use pyo3::{prelude::*, pyclass};

#[derive(Clone, Copy, Debug, PartialEq)]
#[pyclass(eq, eq_int)]
pub enum Outcome {
    Pass,
    Error,
    Failure,
    Skip,
}

#[pymethods]
impl Outcome {
    #[new]
    fn new(value: &str) -> Self {
        match value {
            "pass" => Outcome::Pass,
            "failure" => Outcome::Failure,
            "error" => Outcome::Error,
            "skip" => Outcome::Skip,
            _ => Outcome::Failure,
        }
    }

    fn __str__(&self) -> &str {
        match &self {
            Outcome::Pass => "pass",
            Outcome::Failure => "failure",
            Outcome::Error => "error",
            Outcome::Skip => "skip",
        }
    }
}

impl Display for Outcome {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            Outcome::Pass => write!(f, "Pass"),
            Outcome::Failure => write!(f, "Failure"),
            Outcome::Error => write!(f, "Error"),
            Outcome::Skip => write!(f, "Skip"),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
#[pyclass]
pub struct Testrun {
    #[pyo3(get, set)]
    pub name: String,
    #[pyo3(get, set)]
    pub classname: String,
    #[pyo3(get, set)]
    pub duration: f64,
    #[pyo3(get, set)]
    pub outcome: Outcome,
    #[pyo3(get, set)]
    pub testsuite: String,
    #[pyo3(get, set)]
    pub failure_message: Option<String>,
    #[pyo3(get, set)]
    pub filename: Option<String>,
}

impl Testrun {
    pub fn empty() -> Testrun {
        Testrun {
            name: "".into(),
            classname: "".into(),
            duration: 0.0,
            outcome: Outcome::Pass,
            testsuite: "".into(),
            failure_message: None,
            filename: None,
        }
    }
}

#[pymethods]
impl Testrun {
    #[new]
    #[pyo3(signature = (name, classname, duration, outcome, testsuite, failure_message=None, filename=None))]
    fn new(
        name: String,
        classname: String,
        duration: f64,
        outcome: Outcome,
        testsuite: String,
        failure_message: Option<String>,
        filename: Option<String>,
    ) -> Self {
        Self {
            name,
            classname,
            duration,
            outcome,
            testsuite,
            failure_message,
            filename,
        }
    }

    fn __repr__(&self) -> String {
        format!(
            "({}, {}, {}, {}, {}, {:?}, {:?})",
            self.name,
            self.classname,
            self.outcome,
            self.duration,
            self.testsuite,
            self.failure_message,
            self.filename,
        )
    }

    fn __richcmp__(&self, other: &Self, op: CompareOp) -> PyResult<bool> {
        match op {
            CompareOp::Eq => Ok(self.name == other.name
                && self.classname == other.classname
                && self.outcome == other.outcome
                && self.duration == other.duration
                && self.testsuite == other.testsuite
                && self.failure_message == other.failure_message),
            _ => todo!(),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
#[pyclass(eq, eq_int)]
pub enum Framework {
    Pytest,
    Vitest,
    Jest,
    PHPUnit,
}

#[pymethods]
impl Framework {
    #[new]
    #[pyo3(signature = (value=None))]
    fn new(value: Option<&str>) -> Self {
        match value {
            Some("pytest") => Framework::Pytest,
            Some("vitest") => Framework::Vitest,
            Some("jest") => Framework::Jest,
            Some("phpunit") => Framework::PHPUnit,
            Some(_) => panic!("this should not occur"), // TODO error message here
            None => panic!("this should not occur"),
        }
    }

    fn __str__(&self) -> &str {
        match &self {
            Framework::Pytest => "pytest",
            Framework::Vitest => "vitest",
            Framework::Jest => "jest",
            Framework::PHPUnit => "phpunit",
        }
    }
}

impl Display for Framework {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            Framework::Pytest => write!(f, "Pytest"),
            Framework::Vitest => write!(f, "Vitest"),
            Framework::Jest => write!(f, "Jest"),
            Framework::PHPUnit => write!(f, "PHPUnit"),
        }
    }
}

#[derive(Clone, Debug)]
#[pyclass]
pub struct ParsingInfo {
    #[pyo3(get, set)]
    pub framework: Option<Framework>,
    #[pyo3(get, set)]
    pub testruns: Vec<Testrun>,
}

#[pymethods]
impl ParsingInfo {
    #[new]
    #[pyo3(signature = (framework, testruns))]
    fn new(framework: Option<Framework>, testruns: Vec<Testrun>) -> Self {
        Self {
            framework,
            testruns,
        }
    }

    fn __repr__(&self) -> String {
        format!("({:?}, {:?})", self.framework, self.testruns)
    }

    fn __richcmp__(&self, other: &Self, op: CompareOp) -> PyResult<bool> {
        match op {
            CompareOp::Eq => {
                Ok(self.framework == other.framework && self.testruns == other.testruns)
            }
            _ => todo!(),
        }
    }
}
