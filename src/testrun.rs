use pyo3::prelude::*;

use serde::Serialize;

static FRAMEWORKS: &[(&'static str, &'static str)] = &[
    ("pytest", "Pytest"),
    ("vitest", "Vitest"),
    ("jest", "Jest"),
    ("phpunit", "PHPUnit"),
];

static EXTENSIONS: &[(&str, &str)] = &[(".py", "Pytest"), (".php", "PHPUnit")];

fn check_substring_before_word_boundary(string: &str, substring: &str) -> bool {
    if let Some((_, suffix)) = string.to_lowercase().split_once(substring) {
        return suffix
            .chars()
            .next()
            .map_or(true, |first_char| !first_char.is_alphanumeric());
    }
    false
}

pub fn check_testsuites_name(testsuites_name: &str) -> Option<&'static str> {
    FRAMEWORKS
        .iter()
        .filter_map(|(name, framework)| {
            check_substring_before_word_boundary(testsuites_name, name)
                .then_some(framework)
                .map(|framework| *framework)
        })
        .next()
}

// i can't seem to get  pyo3(from_item_all) to work when IntoPyObject is also being derived
#[derive(FromPyObject, IntoPyObject, Clone, Debug, Serialize, PartialEq)]
pub struct Testrun {
    #[pyo3(item)]
    pub name: String,
    #[pyo3(item)]
    pub classname: String,
    #[pyo3(item)]
    pub duration: Option<f64>,
    #[pyo3(item)]
    pub outcome: String,
    #[pyo3(item)]
    pub testsuite: String,
    #[pyo3(item)]
    pub failure_message: Option<String>,
    #[pyo3(item)]
    pub filename: Option<String>,
    #[pyo3(item)]
    pub build_url: Option<String>,
    #[pyo3(item)]
    pub computed_name: Option<String>,
}

impl Testrun {
    pub fn framework(&self) -> Option<&'static str> {
        for (name, framework) in FRAMEWORKS {
            if check_substring_before_word_boundary(&self.testsuite, name) {
                return Some(framework);
            }
        }

        for (extension, framework) in EXTENSIONS {
            if check_substring_before_word_boundary(&self.classname, extension)
                || check_substring_before_word_boundary(&self.name, extension)
            {
                return Some(framework);
            }

            if let Some(message) = &self.failure_message {
                if check_substring_before_word_boundary(message, extension) {
                    return Some(framework);
                }
            }

            if let Some(filename) = &self.filename {
                if check_substring_before_word_boundary(filename, extension) {
                    return Some(framework);
                }
            }
        }
        None
    }
}

#[derive(Clone, Debug, Serialize, IntoPyObject)]
pub struct ParsingInfo {
    pub framework: Option<&'static str>,
    pub testruns: Vec<Testrun>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_detect_framework_testsuites_name_no_match() {
        let f = check_testsuites_name("whatever");
        assert_eq!(f, None)
    }

    #[test]
    fn test_detect_framework_testsuites_name_match() {
        let f = check_testsuites_name("jest tests");
        assert_eq!(f, Some("Jest"))
    }

    #[test]
    fn test_detect_framework_testsuite_name() {
        let t = Testrun {
            classname: "".to_string(),
            name: "".to_string(),
            duration: None,
            outcome: "pass".to_string(),
            testsuite: "pytest".to_string(),
            failure_message: None,
            filename: None,
            build_url: None,
            computed_name: None,
        };
        assert_eq!(t.framework(), Some("Pytest"))
    }

    #[test]
    fn test_detect_framework_filenames() {
        let t = Testrun {
            classname: "".to_string(),
            name: "".to_string(),
            duration: None,
            outcome: "pass".to_string(),
            testsuite: "".to_string(),
            failure_message: None,
            filename: Some(".py".to_string()),
            build_url: None,
            computed_name: None,
        };
        assert_eq!(t.framework(), Some("Pytest"))
    }

    #[test]
    fn test_detect_framework_example_classname() {
        let t = Testrun {
            classname: ".py".to_string(),
            name: "".to_string(),
            duration: None,
            outcome: "pass".to_string(),
            testsuite: "".to_string(),
            failure_message: None,
            filename: None,
            build_url: None,
            computed_name: None,
        };
        assert_eq!(t.framework(), Some("Pytest"))
    }

    #[test]
    fn test_detect_framework_example_name() {
        let t = Testrun {
            classname: "".to_string(),
            name: ".py".to_string(),
            duration: None,
            outcome: "pass".to_string(),
            testsuite: "".to_string(),
            failure_message: None,
            filename: None,
            build_url: None,
            computed_name: None,
        };
        assert_eq!(t.framework(), Some("Pytest"))
    }

    #[test]
    fn test_detect_framework_failure_messages() {
        let t = Testrun {
            classname: "".to_string(),
            name: "".to_string(),
            duration: None,
            outcome: "pass".to_string(),
            testsuite: "".to_string(),
            failure_message: Some(".py".to_string()),
            filename: None,
            build_url: None,
            computed_name: None,
        };
        assert_eq!(t.framework(), Some("Pytest"))
    }

    #[test]
    fn test_detect_build_url() {
        let t = Testrun {
            classname: "".to_string(),
            name: "".to_string(),
            duration: None,
            outcome: "pass".to_string(),
            testsuite: "".to_string(),
            failure_message: Some(".py".to_string()),
            filename: None,
            build_url: Some("https://example.com/build_url".to_string()),
            computed_name: None,
        };
        assert_eq!(t.framework(), Some("Pytest"))
    }
}
