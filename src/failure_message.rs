use std::sync::OnceLock;

use pyo3::{prelude::*, types::PyString};
use regex::Regex;

use crate::helpers::s;

#[pyfunction]
pub fn escape_failure_message(failure_message: &str) -> String {
    let mut e = String::new();
    for c in failure_message.chars() {
        match c {
            '\"' => e.push_str("&quot;"),
            '\'' => e.push_str("&apos;"),
            '<' => e.push_str("&lt;"),
            '>' => e.push_str("&gt;"),
            '&' => e.push_str("&amp;"),
            '\r' => {}
            '\n' => e.push_str("<br>"),
            c => e.push(c),
        }
    }
    e
}

#[pyfunction]
pub fn shorten_file_paths(failure_message: &str) -> String {
    static SHORTEN_PATH_PATTERN: OnceLock<Regex> = OnceLock::new();
    /*
    Examples of strings that match:

    /path/to/file.txt
    /path/to/file
    /path/to
    path/to:1:2
    /path/to/file.txt:1:2

    Examples of strings that don't match:

    path
    file.txt
    */
    let re = SHORTEN_PATH_PATTERN
        .get_or_init(|| Regex::new(r"(?:\/*[\w\-]+\/)+(?:[\w\.]+)(?::\d+:\d+)*").unwrap());

    let mut new = String::with_capacity(failure_message.len());
    let mut last_match = 0;
    for caps in re.captures_iter(failure_message) {
        let m = caps.get(0).unwrap();
        let filepath = m.as_str();

        if let Some((third_last_slash_idx, _)) = filepath.rmatch_indices('/').nth(3) {
            new.push_str(".../");
            new.push_str(&filepath[third_last_slash_idx..]);
        } else {
            new.push_str(&failure_message[last_match..m.end()]);
        }
        last_match = m.end();
    }
    new.push_str(&failure_message[last_match..]);

    new
}

fn generate_test_description(testsuite: &String, name: &String) -> String {
    format!(
        "Testsuite:<br>{}<br><br>Test name:<br>{}<br>",
        testsuite, name
    )
}

fn generate_failure_info(failure_message: &Option<String>) -> String {
    match failure_message {
        None => s("No failure message available"),
        Some(x) => escape_failure_message(&shorten_file_paths(x)),
    }
}

#[derive(FromPyObject, Debug)]
pub struct Failure {
    name: String,
    testsuite: String,
    failure_message: Option<String>,
}
#[derive(FromPyObject, Debug)]
pub struct MessagePayload {
    passed: i32,
    failed: i32,
    skipped: i32,
    failures: Vec<Failure>,
}

#[pyfunction]
pub fn build_message(py: Python<'_>, payload: MessagePayload) -> PyResult<&PyString> {
    let mut message: Vec<String> = Vec::new();
    let header = s("### :x: Failed Test Results: ");
    message.push(header);

    let failed: i32 = payload.failed;
    let passed: i32 = payload.passed;
    let skipped: i32 = payload.skipped;

    let completed = failed + passed + skipped;
    let results_summary = format!(
        "Completed {} tests with **`{} failed`**, {} passed and {} skipped.",
        completed, failed, passed, skipped
    );
    message.push(results_summary);
    let details_beginning = [
        s("<details><summary>View the full list of failed tests</summary>"),
        s(""),
        s("| **Test Description** | **Failure message** |"),
        s("| :-- | :-- |"),
    ];
    message.append(&mut details_beginning.to_vec());

    let failures = payload.failures;
    for fail in failures {
        let name = &fail.name;
        let testsuite = &fail.testsuite;
        let failure_message = &fail.failure_message;
        let test_description = generate_test_description(name, testsuite);
        let failure_information = generate_failure_info(failure_message);
        let single_test_row = format!(
            "| <pre>{}</pre> | <pre>{}</pre> |",
            test_description, failure_information
        );
        message.push(single_test_row);
    }

    Ok(PyString::new(py, &message.join("\n")))
}
