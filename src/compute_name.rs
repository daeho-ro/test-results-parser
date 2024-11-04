use crate::testrun::{Framework, Testrun};
use crate::ComputeNameError;
use pyo3::prelude::*;
use quick_xml::escape::unescape;
use std::borrow::Cow;

fn compute_jest(_classname: &str, name: &str, _filename: Option<&str>) -> String {
    name.to_string()
}

fn compute_pytest(classname: &str, name: &str, filename: Option<&str>) -> String {
    match filename {
        Some(filename) => {
            let path_components = filename.split('/').count();

            let classname_components = classname.split(".");

            let actual_classname = classname_components
                .skip(path_components)
                .collect::<Vec<_>>()
                .join("::");

            format!("{}::{}::{}", filename, actual_classname, name).to_string()
        }
        None => format!("{}::{}", classname, name).to_string(),
    }
}

fn compute_vitest(classname: &str, name: &str, _filename: Option<&str>) -> String {
    format!("{} > {}", classname, name).to_string()
}

fn compute_phpunit(classname: &str, name: &str, _filename: Option<&str>) -> String {
    format!("{}::{}", classname, name).to_string()
}

fn get_name<'a>(testrun: &'a Testrun) -> PyResult<Cow<'a, str>> {
    unescape(&testrun.name)
        .map_err(|e| ComputeNameError::new_err(format!("Failed to unescape name: {}", e)))
}

fn get_classname<'a>(testrun: &'a Testrun) -> PyResult<Cow<'a, str>> {
    unescape(&testrun.classname)
        .map_err(|e| ComputeNameError::new_err(format!("Failed to unescape classname: {}", e)))
}

fn get_filename<'a>(testrun: &'a Testrun) -> PyResult<Option<Cow<'a, str>>> {
    testrun
        .filename
        .as_ref()
        .map(|filename| {
            unescape(&filename).map_err(|e| {
                ComputeNameError::new_err(format!("Failed to unescape filename: {}", e))
            })
        })
        .transpose()
}

#[pyfunction]
pub fn compute_name(testruns: Vec<Testrun>, framework: &Framework) -> PyResult<Vec<String>> {
    let mut names = Vec::new();

    for testrun in testruns {
        let name = get_name(&testrun)?;
        let classname = get_classname(&testrun)?;
        let filename = get_filename(&testrun)?;
        let computed_name = match framework {
            &Framework::Jest => compute_jest(&classname, &name, filename.as_deref()),
            &Framework::Pytest => compute_pytest(&classname, &name, filename.as_deref()),
            &Framework::Vitest => compute_vitest(&classname, &name, filename.as_deref()),
            &Framework::PHPUnit => compute_phpunit(&classname, &name, filename.as_deref()),
        };

        names.push(computed_name);
    }
    Ok(names)
}
