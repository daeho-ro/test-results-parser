use crate::testrun::Framework;
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

fn unescape_str(s: &str) -> Cow<'_, str> {
    unescape(s).unwrap_or_else(|_| Cow::Borrowed(s))
}

#[pyfunction(signature = (classname, name, framework, filename=None))]
pub fn compute_name(
    classname: &str,
    name: &str,
    framework: &Framework,
    filename: Option<&str>,
) -> String {
    let compute = match framework {
        Framework::Jest => compute_jest,
        Framework::Pytest => compute_pytest,
        Framework::Vitest => compute_vitest,
        Framework::PHPUnit => compute_phpunit,
    };

    let name = unescape_str(name);
    let classname = unescape_str(classname);
    let filename = filename.map(|f| unescape_str(f));

    compute(&classname, &name, filename.as_deref())
}
