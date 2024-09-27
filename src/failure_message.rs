use std::{cmp::max, sync::OnceLock};

use pyo3::prelude::*;
use regex::Regex;
use serde::Serialize;
use tera::{Context, Tera};


#[pyfunction]
pub fn escape_message(failure_message: &str) -> String {
    /* 
    Escapes characters that will break Markdown Templating.
     */
    let mut e = String::new();
    for c in failure_message.chars() {
        match c {
            '\r' => {}
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

        // we are looking for the 3rd slash (0-indexed) from the back
        if let Some((third_last_slash_idx, _)) = filepath.rmatch_indices('/').nth(2) {
            new.push_str(&failure_message[last_match..m.start()]);
            new.push_str("...");
            new.push_str(&filepath[third_last_slash_idx..]);
        } else {
            new.push_str(&failure_message[last_match..m.end()]);
        }
        last_match = m.end();
    }
    new.push_str(&failure_message[last_match..]);

    new
}

#[derive(FromPyObject, Debug, Clone)]
pub struct Failure {
    name: String,
    testsuite: String,
    failure_message: Option<String>,
    duration: f64,
    build_url: Option<String>,
}
#[derive(FromPyObject, Debug)]
pub struct MessagePayload {
    passed: i32,
    failed: i32,
    skipped: i32,
    failures: Vec<Failure>,
}

#[derive(Serialize)]
struct TemplateContext {
    num_tests: i32,
    num_failed: i32,
    num_passed: i32,
    num_skipped: i32,
    num_output: i32,
    failures: Vec<TemplateFailure>,
}

impl TemplateContext {
    fn new(
        num_tests: i32,
        num_failed: i32,
        num_passed: i32,
        num_skipped: i32,
        failures: Vec<TemplateFailure>,
    ) -> Self {
        let num_output: i32 = failures.len().try_into().unwrap();
        Self { num_tests, num_failed, num_passed, num_skipped, num_output, failures }
    }
}

#[derive(Serialize)]
struct TemplateFailure {
    test_suite: String,
    test_name: String,
    duration: String,
    backticks: String,
    build_url: Option<String>,
    stack_trace: Vec<String>,
}

impl TemplateFailure {
    fn new(
        test_suite: String, 
        test_name: String, 
        duration: String, 
        raw_num_backticks: usize, 
        build_url: Option<String>,
        stack_trace: Vec<String>
    ) -> Self {
        let num_backticks = max(raw_num_backticks + 1, 3);
        let backticks = String::from("`".repeat(num_backticks));
        Self { test_suite, test_name, duration, backticks, build_url, stack_trace }
    }
}

fn longest_repeated_substring(s: String, target: char) -> usize {
    let mut max_length = 0;
    let mut current_length = 0;

    for c in s.chars() {
        if c == target {
            current_length += 1;
            max_length = max_length.max(current_length);
        } else {
            current_length = 0; // Reset when the character doesn't match
        }
    }

    max_length
}

#[pyfunction]
pub fn build_message(payload: MessagePayload) -> String {
    let tera = Tera::new("templates/**/*").unwrap();
    let failed: i32 = payload.failed;
    let passed: i32 = payload.passed;
    let skipped: i32 = payload.skipped;

    let completed = failed + passed + skipped;

    let mut sorted_failures: Vec<Failure> = payload.failures.to_vec();
    sorted_failures.sort_by(|a, b| a.duration.partial_cmp(&b.duration).unwrap());

    let mut template_failures: Vec<TemplateFailure> = Vec::new();
    sorted_failures.truncate(3);
    for failure in sorted_failures.iter_mut() {
        let failure_message = match failure.failure_message.as_ref() {
            Some(x) => String::from(x),
            _ => String::from("No failure message available"),
        };
        let stack_trace_lines: Vec<String> = failure_message
            .split('\n')
            .map(|s| escape_message(s).to_string())
            .collect();
        let num_backticks: usize = longest_repeated_substring(failure_message, '`'); 
        let temp: TemplateFailure = TemplateFailure::new(
            failure.testsuite.clone(),
            failure.name.clone(), 
            format!("{:.3}", failure.duration),
            num_backticks,
            failure.build_url.clone(),
            stack_trace_lines,
        );
        template_failures.push(temp);
    };

    let template_context = TemplateContext::new(
        completed, failed, passed, skipped, template_failures,
    );
    
    let message = tera.render(
        "test_results_message.md", 
        &Context::from_serialize(&template_context).unwrap())
        .unwrap();
    message
}
