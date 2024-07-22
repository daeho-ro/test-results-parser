use pyo3::exceptions::PyException;
use pyo3::prelude::*;

mod failure_message;
mod junit;
mod pytest_reportlog;
mod testrun;
mod vitest_json;

pyo3::create_exception!(test_results_parser, ParserError, PyException);

/// A Python module implemented in Rust.
#[pymodule]
fn test_results_parser(py: Python, m: &Bound<PyModule>) -> PyResult<()> {
    m.add("ParserError", py.get_type_bound::<ParserError>())?;
    m.add_class::<testrun::Testrun>()?;
    m.add_class::<testrun::Outcome>()?;

    m.add_function(wrap_pyfunction!(junit::parse_junit_xml, m)?)?;
    m.add_function(wrap_pyfunction!(
        pytest_reportlog::parse_pytest_reportlog,
        m
    )?)?;
    m.add_function(wrap_pyfunction!(vitest_json::parse_vitest_json, m)?)?;
    m.add_function(wrap_pyfunction!(failure_message::build_message, m)?)?;
    m.add_function(wrap_pyfunction!(
        failure_message::escape_failure_message,
        m
    )?)?;
    m.add_function(wrap_pyfunction!(failure_message::shorten_file_paths, m)?)?;

    Ok(())
}
