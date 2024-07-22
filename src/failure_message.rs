use std::sync::OnceLock;

use pyo3::prelude::*;
use regex::Regex;

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
pub fn build_message(payload: MessagePayload) -> String {
    use std::fmt::Write;
    let mut message = String::from("### :x: Failed Test Results:\n");

    let failed: i32 = payload.failed;
    let passed: i32 = payload.passed;
    let skipped: i32 = payload.skipped;

    let completed = failed + passed + skipped;
    writeln!(&mut message, "Completed {completed} tests with **`{failed} failed`**, {passed} passed and {skipped} skipped.").unwrap();
    message.push_str("<details><summary>View the full list of failed tests</summary>\n\n");
    message.push_str("| **Test Description** | **Failure message** |\n");
    message.push_str("| :-- | :-- |\n");

    for fail in payload.failures {
        message.push_str("| <pre>");
        write!(
            &mut message,
            "Testsuite:<br>{}<br><br>Test name:<br>{}<br>",
            fail.testsuite, fail.name
        )
        .unwrap();
        message.push_str("</pre> | <pre>");

        match fail.failure_message {
            None => message.push_str("No failure message available"),
            Some(x) => message.push_str(&escape_failure_message(&shorten_file_paths(&x))),
        }
        message.push_str("</pre> |\n");
    }

    message
}
