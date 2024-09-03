use pyo3::prelude::*;

use quick_xml::events::attributes::Attributes;
use quick_xml::events::{BytesStart, Event};
use quick_xml::reader::Reader;

use crate::testrun::{check_testsuites_name, Outcome, ParsingInfo, Testrun};
use crate::ParserError;

struct RelevantAttrs {
    classname: Option<String>,
    name: Option<String>,
    time: Option<String>,
    file: Option<String>,
}

// from https://gist.github.com/scott-codecov/311c174ecc7de87f7d7c50371c6ef927#file-cobertura-rs-L18-L31
fn get_relevant_attrs(attributes: Attributes) -> PyResult<RelevantAttrs> {
    let mut rel_attrs: RelevantAttrs = RelevantAttrs {
        time: None,
        classname: None,
        name: None,
        file: None,
    };
    for attribute in attributes {
        let attribute = attribute
            .map_err(|e| ParserError::new_err(format!("Error parsing attribute: {}", e)))?;
        let bytes = attribute.value.into_owned();
        let value = String::from_utf8(bytes)?;
        match attribute.key.into_inner() {
            b"time" => rel_attrs.time = Some(value),
            b"classname" => rel_attrs.classname = Some(value),
            b"name" => rel_attrs.name = Some(value),
            b"file" => rel_attrs.file = Some(value),
            _ => {}
        }
    }
    Ok(rel_attrs)
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

fn populate(rel_attrs: RelevantAttrs, testsuite: String) -> PyResult<Testrun> {
    let classname = rel_attrs
        .classname
        .ok_or(ParserError::new_err("No classname found"))?;
    let name = rel_attrs
        .name
        .ok_or(ParserError::new_err("No name found"))?;

    let duration = rel_attrs
        .time
        .ok_or(ParserError::new_err("No duration found"))?
        .parse()?;

    Ok(Testrun {
        name,
        classname,
        duration,
        outcome: Outcome::Pass,
        testsuite,
        failure_message: None,
        filename: rel_attrs.file,
    })
}

#[pyfunction]
pub fn parse_junit_xml(file_bytes: &[u8]) -> PyResult<ParsingInfo> {
    let mut reader = Reader::from_reader(file_bytes);
    reader.config_mut().trim_text(true);

    let mut testruns: Vec<Testrun> = Vec::new();
    let mut saved_testrun: Option<Testrun> = None;

    let mut curr_testsuite = String::new();
    let mut in_failure: bool = false;

    let mut buf = Vec::new();

    let mut testsuites_name: Option<String> = None;

    loop {
        match reader.read_event_into(&mut buf) {
            Err(e) => {
                return Err(ParserError::new_err(format!(
                    "Error parsing XML at position: {} {:?}",
                    reader.buffer_position(),
                    e
                )))
            }
            Ok(Event::Eof) => {
                break;
            }
            Ok(Event::Start(e)) => match e.name().as_ref() {
                b"testcase" => {
                    let rel_attrs = get_relevant_attrs(e.attributes())?;
                    saved_testrun = Some(populate(rel_attrs, curr_testsuite.clone())?);
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
                b"testsuites" => {
                    testsuites_name = get_attribute(&e, "name")?;
                }
                _ => {}
            },
            Ok(Event::End(e)) => {
                match e.name().as_ref() {
                    b"testcase" => match saved_testrun {
                        Some(testrun) => {
                            testruns.push(testrun);
                            saved_testrun = None;
                        }
                        None => return Err(ParserError::new_err(
                            "Met testcase closing tag without first meeting testcase opening tag"
                                .to_string(),
                        )),
                    },
                    b"failure" => in_failure = false,
                    _ => (),
                }
            }
            Ok(Event::Empty(e)) => {
                if e.name().as_ref() == b"testcase" {
                    let rel_attrs = get_relevant_attrs(e.attributes())?;
                    let testrun = populate(rel_attrs, curr_testsuite.clone())?;
                    testruns.push(testrun);
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

    let mut framework = None;
    for testrun in &testruns {
        if let Some(matched_framework) = testrun.framework() {
            framework = Some(matched_framework);
            break;
        }
    }

    if framework.is_none() {
        if let Some(name) = testsuites_name {
            framework = check_testsuites_name(&name);
        }
    }

    Ok(ParsingInfo {
        framework,
        testruns,
    })
}
