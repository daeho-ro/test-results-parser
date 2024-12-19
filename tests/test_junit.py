import pytest
from test_results_parser import (
    Framework,
    Outcome,
    ParsingInfo,
    Testrun,
    parse_junit_xml,
)


class TestParsers:
    @pytest.mark.parametrize(
        "filename,expected",
        [
            (
                "./tests/junit.xml",
                ParsingInfo(
                    Framework.Pytest,
                    [
                        Testrun(
                            name="test_junit[junit.xml--True]",
                            classname="tests.test_parsers.TestParsers",
                            duration=0.001,
                            outcome=Outcome.Failure,
                            testsuite="pytest",
                            failure_message="""self = <test_parsers.TestParsers object at 0x102182d10>, filename = 'junit.xml', expected = '', check = True

    @pytest.mark.parametrize(
        "filename,expected,check",
        [("junit.xml", "", True), ("jest-junit.xml", "", False)],
    )
    def test_junit(self, filename, expected, check):
        with open(filename) as f:
            junit_string = f.read()
            res = parse_junit_xml(junit_string)
            print(res)
            if check:
>               assert res == expected
E               AssertionError: assert [{'duration': '0.010', 'name': 'tests.test_parsers.TestParsers.test_junit[junit.xml-]', 'outcome': 'failure'}, {'duration': '0.063', 'name': 'tests.test_parsers.TestParsers.test_junit[jest-junit.xml-]', 'outcome': 'pass'}] == ''

tests/test_parsers.py:16: AssertionError""",
                            filename=None,
                            computed_name="tests.test_parsers.TestParsers::test_junit[junit.xml--True]",
                        ),
                        Testrun(
                            name="test_junit[jest-junit.xml--False]",
                            classname="tests.test_parsers.TestParsers",
                            duration=0.064,
                            outcome=Outcome.Pass,
                            testsuite="pytest",
                            failure_message=None,
                            filename=None,
                            computed_name="tests.test_parsers.TestParsers::test_junit[jest-junit.xml--False]",
                        ),
                    ],
                ),
            ),
            (
                "./tests/junit-no-testcase-timestamp.xml",
                ParsingInfo(
                    Framework.Pytest,
                    [
                        Testrun(
                            name="test_junit[junit.xml--True]",
                            classname="tests.test_parsers.TestParsers",
                            duration=0.186,
                            outcome=Outcome.Failure,
                            testsuite="pytest",
                            failure_message="""aaaaaaa""",
                            filename=None,
                            computed_name="tests.test_parsers.TestParsers::test_junit[junit.xml--True]",
                        ),
                        Testrun(
                            name="test_junit[jest-junit.xml--False]",
                            classname="tests.test_parsers.TestParsers",
                            duration=0.186,
                            outcome=Outcome.Pass,
                            testsuite="pytest",
                            failure_message=None,
                            filename=None,
                            computed_name="tests.test_parsers.TestParsers::test_junit[jest-junit.xml--False]",
                        ),
                    ],
                ),
            ),
            (
                "./tests/junit-nested-testsuite.xml",
                ParsingInfo(
                    Framework.Pytest,
                    [
                        Testrun(
                            name="test_junit[junit.xml--True]",
                            classname="tests.test_parsers.TestParsers",
                            duration=0.186,
                            outcome=Outcome.Failure,
                            testsuite="nested_testsuite",
                            failure_message="""aaaaaaa""",
                            filename=None,
                            computed_name=None,
                        ),
                        Testrun(
                            name="test_junit[jest-junit.xml--False]",
                            classname="tests.test_parsers.TestParsers",
                            duration=0.186,
                            outcome=Outcome.Pass,
                            testsuite="pytest",
                            failure_message=None,
                            filename=None,
                            computed_name="tests.test_parsers.TestParsers::test_junit[jest-junit.xml--False]",
                        ),
                    ],
                ),
            ),
            (
                "./tests/jest-junit.xml",
                ParsingInfo(
                    Framework.Jest,
                    [
                        Testrun(
                            name="Title when rendered renders pull title",
                            classname="Title when rendered renders pull title",
                            duration=0.036,
                            outcome=Outcome.Pass,
                            testsuite="Title",
                            failure_message=None,
                            filename=None,
                            computed_name="Title when rendered renders pull title",
                        ),
                        Testrun(
                            name="Title when rendered renders pull author",
                            classname="Title when rendered renders pull author",
                            duration=0.005,
                            outcome=Outcome.Pass,
                            testsuite="Title",
                            failure_message=None,
                            filename=None,
                            computed_name="Title when rendered renders pull author",
                        ),
                        Testrun(
                            name="Title when rendered renders pull updatestamp",
                            classname="Title when rendered renders pull updatestamp",
                            duration=0.002,
                            outcome=Outcome.Pass,
                            testsuite="Title",
                            failure_message=None,
                            filename=None,
                            computed_name="Title when rendered renders pull updatestamp",
                        ),
                        Testrun(
                            name="Title when rendered for first pull request renders pull title",
                            classname="Title when rendered for first pull request renders pull title",
                            duration=0.006,
                            outcome=Outcome.Pass,
                            testsuite="Title",
                            failure_message=None,
                            filename=None,
                            computed_name="Title when rendered for first pull request renders pull title",
                        ),
                    ],
                ),
            ),
            (
                "./tests/vitest-junit.xml",
                ParsingInfo(
                    Framework.Vitest,
                    [
                        Testrun(
                            name="first test file &gt; 2 + 2 should equal 4",
                            classname="__tests__/test-file-1.test.ts",
                            duration=0.01,
                            outcome=Outcome.Failure,
                            testsuite="__tests__/test-file-1.test.ts",
                            failure_message="""AssertionError: expected 5 to be 4 // Object.is equality
 ❯ __tests__/test-file-1.test.ts:20:28""",
                            filename=None,
                            computed_name="__tests__/test-file-1.test.ts > first test file > 2 + 2 should equal 4",
                        ),
                        Testrun(
                            name="first test file &gt; 4 - 2 should equal 2",
                            classname="__tests__/test-file-1.test.ts",
                            duration=0,
                            outcome=Outcome.Pass,
                            testsuite="__tests__/test-file-1.test.ts",
                            failure_message=None,
                            filename=None,
                            computed_name="__tests__/test-file-1.test.ts > first test file > 4 - 2 should equal 2",
                        ),
                    ],
                ),
            ),
            (
                "./tests/empty_failure.junit.xml",
                ParsingInfo(
                    None,
                    [
                        Testrun(
                            name="test.test works",
                            classname="test.test",
                            duration=0.234,
                            outcome=Outcome.Pass,
                            testsuite="test",
                            failure_message=None,
                            filename="./test.rb",
                        ),
                        Testrun(
                            name="test.test fails",
                            classname="test.test",
                            duration=1,
                            outcome=Outcome.Failure,
                            testsuite="test",
                            failure_message="TestError",
                            filename="./test.rb",
                        ),
                    ],
                ),
            ),
            (
                "./tests/phpunit.junit.xml",
                ParsingInfo(
                    Framework.PHPUnit,
                    [
                        Testrun(
                            name="test1",
                            classname="class.className",
                            duration=0.1,
                            outcome=Outcome.Pass,
                            testsuite="Thing",
                            failure_message=None,
                            filename="/file1.php",
                            computed_name="class.className::test1",
                        ),
                        Testrun(
                            name="test2",
                            classname="",
                            duration=0.1,
                            outcome=Outcome.Pass,
                            testsuite="Thing",
                            failure_message=None,
                            filename="/file1.php",
                            computed_name="::test2",
                        ),
                    ],
                ),
            ),
            (
                "./tests/ctest.xml",
                ParsingInfo(
                    None,
                    [
                        Testrun(
                            name="a_unit_test",
                            classname="a_unit_test",
                            duration=33.4734,
                            outcome=Outcome.Failure,
                            testsuite="Linux-c++",
                            failure_message="Failed",
                            filename=None,
                        )
                    ],
                ),
            ),
            (
                "./tests/no-testsuite-name.xml",
                ParsingInfo(
                    None,
                    [
                        Testrun(
                            name="a_unit_test",
                            classname="a_unit_test",
                            duration=33.4734,
                            outcome=Outcome.Failure,
                            testsuite="",
                            failure_message="Failed",
                            filename=None,
                        )
                    ],
                ),
            ),
            (
                "./tests/testsuites.xml",
                ParsingInfo(
                    None,
                    [],
                ),
            ),
            (
                "./tests/skip-error.junit.xml",
                ParsingInfo(
                    Framework.Pytest,
                    [
                        Testrun(
                            name="test_subtract",
                            classname="tests.test_math.TestMath",
                            duration=0.1,
                            outcome=Outcome.Error,
                            testsuite="pytest",
                            failure_message="hello world",
                            filename=None,
                            computed_name="tests.test_math.TestMath::test_subtract",
                        ),
                        Testrun(
                            name="test_multiply",
                            classname="tests.test_math.TestMath",
                            duration=0.1,
                            outcome=Outcome.Error,
                            testsuite="pytest",
                            failure_message=None,
                            filename=None,
                            computed_name="tests.test_math.TestMath::test_multiply",
                        ),
                        Testrun(
                            name="test_add",
                            classname="tests.test_math.TestMath",
                            duration=0.1,
                            outcome=Outcome.Skip,
                            testsuite="pytest",
                            failure_message=None,
                            filename=None,
                            computed_name="tests.test_math.TestMath::test_add",
                        )
                    ],
                ),
            ),
        ],
    )
    def test_junit(self, filename, expected):
        with open(filename, "b+r") as f:
            res = parse_junit_xml(f.read())
            assert res.framework == expected.framework
            assert len(res.testruns) == len(expected.testruns)
            for restest, extest in zip(res.testruns, expected.testruns):
                print(
                    restest.classname,
                    restest.duration,
                    restest.filename,
                    restest.name,
                    restest.outcome,
                    restest.testsuite,
                )
                print(
                    extest.classname,
                    extest.duration,
                    extest.filename,
                    extest.name,
                    extest.outcome,
                    extest.testsuite,
                )
                assert restest == extest
