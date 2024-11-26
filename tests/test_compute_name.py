from test_results_parser import compute_name, Framework


def test_compute_name():
    assert compute_name(
        name="test_junit[junit.xml--True]",
        classname="tests.test_parsers.TestParsers",
        filename="tests/test_parsers.py",
        framework=Framework.Pytest,
    ) == "tests/test_parsers.py::TestParsers::test_junit[junit.xml--True]"

    assert compute_name(
        name="test_junit[junit.xml--True]",
        classname="tests.test_parsers.TestParsers",
        framework=Framework.Pytest,
    ) == "tests.test_parsers.TestParsers::test_junit[junit.xml--True]"

    assert compute_name(
        name="it does the thing &gt; it does the thing",
        classname="it does the thing &gt; it does the thing",
        framework=Framework.Jest,
    ) == "it does the thing > it does the thing"

    assert compute_name(
        name="it does the thing &gt; it does the thing",
        classname="tests/thing.js",
        framework=Framework.Vitest,
    ) == "tests/thing.js > it does the thing > it does the thing"

    assert compute_name(
        name="test1",
        classname="class.className",
        framework=Framework.PHPUnit,
    ) == "class.className::test1"
