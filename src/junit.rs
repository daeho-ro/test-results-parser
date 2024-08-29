use pyo3::prelude::*;

use quick_xml::events::attributes::Attributes;
use quick_xml::events::{BytesStart, Event};
use quick_xml::reader::Reader;

use crate::framework_detectors::detect_framework;
use crate::testrun::{Outcome, ParsingInfo, Testrun};
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

    let mut list_of_test_runs: Vec<Testrun> = Vec::new();
    let mut saved_testrun: Option<Testrun> = None;

    let mut curr_testsuite = String::new();
    let mut in_failure: bool = false;

    let mut buf = Vec::new();

    // set when we parse the testsuites tag
    let mut testsuites_name: String = "".to_string();

    // set every time we finish parsing a testcase
    let mut testsuite_names: Vec<String> = Vec::new();
    let mut filenames = Vec::new();
    let mut failure_messages = Vec::new();

    // set the first time we finish parsing a testcase
    let mut example_class_name = "".to_string();
    let mut example_test_name = "".to_string();

    let mut on_testcase_finish = |testrun: Testrun, ts: String| {
        testsuite_names.push(ts.clone());
        match testrun.filename.clone() {
            Some(x) => {
                filenames.push(x);
            }
            None => {}
        }
        match testrun.failure_message.clone() {
            Some(x) => {
                failure_messages.push(x);
            }
            None => {}
        }
        if example_class_name == "" {
            example_class_name = testrun.classname.clone();
        }
        if example_test_name == "" {
            example_test_name = (testrun.name).clone();
        }
        list_of_test_runs.push(testrun);
    };

    let loop_result =
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
                        testsuites_name = match get_attribute(&e, "name")? {
                            None => "".to_string(),
                            Some(x) => x,
                        };
                    }
                    _ => {}
                },
                Ok(Event::End(e)) => match e.name().as_ref() {
                    b"testcase" => match saved_testrun {
                        Some(testrun) => {
                            on_testcase_finish(testrun, curr_testsuite.clone());
                            saved_testrun = None;
                        }
                        None => return Err(ParserError::new_err(
                            "Met testcase closing tag without first meeting testcase opening tag"
                                .to_string(),
                        )),
                    },
                    b"failure" => in_failure = false,
                    _ => (),
                },
                Ok(Event::Empty(e)) => {
                    if e.name().as_ref() == b"testcase" {
                        let rel_attrs = get_relevant_attrs(e.attributes())?;
                        let testrun = populate(rel_attrs, curr_testsuite.clone())?;
                        on_testcase_finish(testrun, curr_testsuite.clone());
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
        };

    match loop_result {
        Ok(testruns) => {
            // detect framework based on all the information we'e collected so far
            let framework = detect_framework(
                testsuites_name,
                testsuite_names,
                filenames,
                example_class_name,
                example_test_name,
                failure_messages,
            );

            Ok(ParsingInfo {
                framework,
                testruns,
            })
        }
        Err(x) => Err(x),
    }
}
