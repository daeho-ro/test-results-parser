use pyo3::prelude::*;

use quick_xml::events::attributes::Attributes;
use quick_xml::events::{BytesStart, Event};
use quick_xml::reader::Reader;
use std::collections::HashMap;

use crate::testrun::{Outcome, Testrun};
use crate::ParserError;

// from https://gist.github.com/scott-codecov/311c174ecc7de87f7d7c50371c6ef927#file-cobertura-rs-L18-L31
fn attributes_map(attributes: Attributes) -> PyResult<HashMap<String, String>> {
    let mut attr_map = HashMap::new();
    for attribute in attributes {
        let attribute = attribute
            .map_err(|e| ParserError::new_err(format!("Error parsing attribute: {}", e)))?;
        let bytes = attribute.value.into_owned();
        let value = String::from_utf8(bytes)?;
        let key = String::from_utf8(attribute.key.into_inner().to_vec())?;
        attr_map.insert(key, value);
    }
    Ok(attr_map)
}

fn get_attribute(e: &BytesStart, name: &str) -> PyResult<Option<String>> {
    let attr = if let Some(message) = e
        .try_get_attribute(name)
        .map_err(|e| ParserError::new_err(format!("Error parsing attribute: {}", e)))?
    {
        Some(String::from_utf8(message.value.to_vec())?)
    } else {
        None
    };
    Ok(attr)
}

fn populate(attr_hm: &HashMap<String, String>, testsuite: String) -> PyResult<Testrun> {
    let name = format!(
        "{}\x1f{}",
        attr_hm
            .get("classname")
            .ok_or(ParserError::new_err("No classname found"))?,
        attr_hm
            .get("name")
            .ok_or(ParserError::new_err("No name found"))?
    );

    let duration = attr_hm
        .get("time")
        .ok_or(ParserError::new_err("No duration found"))?
        .parse()?;

    Ok(Testrun {
        name,
        duration,
        outcome: Outcome::Pass,
        testsuite,
        failure_message: None,
    })
}

#[pyfunction]
pub fn parse_junit_xml(file_bytes: &[u8]) -> PyResult<Vec<Testrun>> {
    let mut reader = Reader::from_reader(file_bytes);
    reader.config_mut().trim_text(true);

    let mut list_of_test_runs = Vec::new();
    let mut saved_testrun: Option<Testrun> = None;

    let mut curr_testsuite = String::new();
    let mut in_failure: bool = false;

    let mut buf = Vec::new();
    loop {
        match reader.read_event_into(&mut buf) {
            Err(e) => {
                break Err(ParserError::new_err(format!(
                    "Error parsing XML at position: {} {:?}",
                    reader.buffer_position(),
                    e
                )))
            }
            Ok(Event::Eof) => {
                break Ok(list_of_test_runs);
            }
            Ok(Event::Start(e)) => match e.name().as_ref() {
                b"testcase" => {
                    let attr_hm = attributes_map(e.attributes())?;
                    saved_testrun = Some(populate(&attr_hm, curr_testsuite.clone())?);
                }
                b"skipped" => {
                    let testrun = saved_testrun
                        .as_mut()
                        .ok_or(ParserError::new_err("Error accessing saved testrun"))?;
                    testrun.outcome = Outcome::Skip;
                }
                b"error" => {
                    let testrun = saved_testrun
                        .as_mut()
                        .ok_or(ParserError::new_err("Error accessing saved testrun"))?;
                    testrun.outcome = Outcome::Error;
                }
                b"failure" => {
                    let testrun = saved_testrun
                        .as_mut()
                        .ok_or(ParserError::new_err("Error accessing saved testrun"))?;
                    testrun.outcome = Outcome::Failure;

                    testrun.failure_message = get_attribute(&e, "message")?;
                    in_failure = true;
                }
                b"testsuite" => {
                    curr_testsuite = get_attribute(&e, "name")?
                        .ok_or(ParserError::new_err("Error getting name".to_string()))?;
                }
                _ => {}
            },
            Ok(Event::End(e)) => match e.name().as_ref() {
                b"testcase" => {
                    list_of_test_runs.push(
                        saved_testrun
                            .ok_or(ParserError::new_err("Error accessing saved testrun"))?,
                    );
                    saved_testrun = None;
                }
                b"failure" => in_failure = false,
                _ => (),
            },
            Ok(Event::Empty(e)) => {
                if e.name().as_ref() == b"testcase" {
                    let attr_hm = attributes_map(e.attributes())?;
                    list_of_test_runs.push(populate(&attr_hm, curr_testsuite.clone())?);
                }
            }
            Ok(Event::Text(x)) => {
                if in_failure {
                    let testrun = saved_testrun
                        .as_mut()
                        .ok_or(ParserError::new_err("Error accessing saved testrun"))?;

                    let mut xml_failure_message = x.into_owned();
                    xml_failure_message.inplace_trim_end();
                    xml_failure_message.inplace_trim_start();

                    testrun.failure_message =
                        Some(String::from_utf8(xml_failure_message.as_ref().to_vec())?);
                }
            }

            // There are several other `Event`s we do not consider here
            _ => (),
        }
        buf.clear()
    }
}
