from test_results_parser import compute_name, Framework, Testrun, Outcome


def test_compute_name():
    assert compute_name(
        [
            Testrun(
                name="test_junit[junit.xml--True]",
                testsuite="pytest",
                classname="tests.test_parsers.TestParsers",
                filename="tests/test_parsers.py",
                duration=0.1,
                outcome=Outcome.Pass,
            )
        ],
        Framework.Pytest,
    ) == ["tests/test_parsers.py::TestParsers::test_junit[junit.xml--True]"]

    assert compute_name(
        [
            Testrun(
                name="test_junit[junit.xml--True]",
                testsuite="pytest",
                classname="tests.test_parsers.TestParsers",
                duration=0.1,
                outcome=Outcome.Pass,
            )
        ],
        Framework.Pytest,
    ) == ["tests.test_parsers.TestParsers::test_junit[junit.xml--True]"]

    assert compute_name(
        [
            Testrun(
                name="it does the thing &gt; it does the thing",
                testsuite="jest",
                classname="it does the thing &gt; it does the thing",
                duration=0.1,
                outcome=Outcome.Pass,
            )
        ],
        Framework.Jest,
    ) == ["it does the thing > it does the thing"]

    assert compute_name(
        [
            Testrun(
                name="it does the thing &gt; it does the thing",
                testsuite="vitest",
                classname="tests/thing.js",
                duration=0.1,
                outcome=Outcome.Pass,
            )
        ],
        Framework.Vitest,
    ) == ["tests/thing.js > it does the thing > it does the thing"]

    assert compute_name(
        [
            Testrun(
                name="test1",
                testsuite="phpunit",
                classname="class.className",
                duration=0.1,
                outcome=Outcome.Pass,
            )
        ],
        Framework.PHPUnit,
    ) == ["class.className::test1"]
