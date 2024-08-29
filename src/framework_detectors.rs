use regex::Regex;

use crate::testrun::Framework;

fn gen_reg(s: &str) -> Regex {
    Regex::new(format!(r"(?i){}(\W|$)", s).as_str()).unwrap()
}

fn apply_reg(rl: &[(Regex, Framework)], v: Vec<String>) -> Option<Framework> {
    for (r, f) in rl {
        for s in v.iter() {
            if r.is_match(s) {
                return Some(*f);
            }
        }
    }
    None
}

fn get_framework_names() -> [(Regex, Framework); 4] {
    [
        (gen_reg("pytest"), Framework::Pytest),
        (gen_reg("jest"), Framework::Jest),
        (gen_reg("vitest"), Framework::Vitest),
        (gen_reg("phpunit"), Framework::PHPUnit),
    ]
}

fn get_file_extensions() -> [(Regex, Framework); 2] {
    [
        (gen_reg(".py"), Framework::Pytest),
        (gen_reg(".php"), Framework::PHPUnit),
    ]
}

// i want it to iterate through running certain regexes on all
pub fn detect_framework(
    testsuites_name: String,
    mut testsuite_names: Vec<String>,
    mut filenames: Vec<String>,
    example_class_name: String,
    example_test_name: String,
    failure_messages: Vec<String>,
) -> Option<Framework> {
    let framework_names = get_framework_names();
    testsuite_names.insert(0, testsuites_name);
    match apply_reg(&framework_names, testsuite_names) {
        Some(f) => return Some(f),
        None => {}
    };

    // is there a better way to do something like this
    let file_extensions = get_file_extensions();
    filenames.push(example_class_name);
    filenames.push(example_test_name);
    filenames.extend(failure_messages.into_iter());

    match apply_reg(&file_extensions, filenames) {
        Some(f) => return Some(f),
        None => {}
    };

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_detect_framework_empty() {
        assert_eq!(
            detect_framework(
                "".to_string(),
                vec![],
                vec![],
                "".to_string(),
                "".to_string(),
                vec![]
            ),
            None
        );
    }

    #[test]
    fn test_detect_framework_testsuites_name() {
        assert_eq!(
            detect_framework(
                "jest tests".to_string(),
                vec![],
                vec![],
                "".to_string(),
                "".to_string(),
                vec![]
            ),
            Some(Framework::Jest)
        );
    }

    #[test]
    fn test_detect_framework_testsuite_names() {
        assert_eq!(
            detect_framework(
                "".to_string(),
                vec!["pytest".to_string()],
                vec![],
                "".to_string(),
                "".to_string(),
                vec![]
            ),
            Some(Framework::Pytest)
        );
    }

    #[test]
    fn test_detect_framework_filenames() {
        assert_eq!(
            detect_framework(
                "".to_string(),
                vec![],
                vec![".py".to_string()],
                "".to_string(),
                "".to_string(),
                vec![]
            ),
            Some(Framework::Pytest)
        );
    }

    #[test]
    fn test_detect_framework_example_classname() {
        assert_eq!(
            detect_framework(
                "".to_string(),
                vec![],
                vec![],
                ".py".to_string(),
                "".to_string(),
                vec![]
            ),
            Some(Framework::Pytest)
        );
    }

    #[test]
    fn test_detect_framework_example_name() {
        assert_eq!(
            detect_framework(
                "".to_string(),
                vec![],
                vec![],
                "".to_string(),
                ".py".to_string(),
                vec![]
            ),
            Some(Framework::Pytest)
        );
    }
    #[test]
    fn test_detect_framework_failure_messages() {
        assert_eq!(
            detect_framework(
                "".to_string(),
                vec![],
                vec![],
                "".to_string(),
                "".to_string(),
                vec![".py".to_string()]
            ),
            Some(Framework::Pytest)
        );
    }
}
