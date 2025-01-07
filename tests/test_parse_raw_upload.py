import pytest
import base64
import zlib
import json
import msgpack
from test_results_parser import parse_raw_upload, ParserError

class TestParsers:
    def test_junit(self):
        with open("tests/junit.xml", "b+r") as f:
            file_bytes = f.read()
            raw_upload = {
                "network": [
                    "a/b/c.py",
                ],
                "test_results_files": [
                    {
                        "filename": "junit.xml",
                        "format": "base64+compressed",
                        "data": base64.b64encode(zlib.compress(file_bytes)).decode(
                            "utf-8"
                        ),
                    }
                ]
            }
            json_bytes = json.dumps(raw_upload).encode("utf-8")
            parsing_infos, readable_files_bytes = parse_raw_upload(json_bytes)


            readable_files = bytes(readable_files_bytes)

            assert readable_files == f"""# path=junit.xml\n{file_bytes.decode()}\n<<<<<< EOF\n""".encode()
            

            assert parsing_infos[0]["framework"] == "Pytest"
            assert parsing_infos[0]["testruns"] == [
                {
                    "name": "test_junit[junit.xml--True]",
                    "classname": "tests.test_parsers.TestParsers",
                    "duration": 0.001,
                    "outcome": "failure",
                    "testsuite": "pytest",
                    "failure_message": """self = <test_parsers.TestParsers object at 0x102182d10>, filename = 'junit.xml', expected = '', check = True

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
                    "filename": None,
                    "build_url": None,
                    "computed_name": "tests.test_parsers.TestParsers::test_junit[junit.xml--True]",
                },
                {
                    "name": "test_junit[jest-junit.xml--False]",
                    "classname": "tests.test_parsers.TestParsers",
                    "duration": 0.064,
                    "outcome": "pass",
                    "testsuite": "pytest",
                    "failure_message": None,
                    "filename": None,
                    "build_url": None,
                    "computed_name": "tests.test_parsers.TestParsers::test_junit[jest-junit.xml--False]",
                }
            ]


    def test_json_error(self):
        with pytest.raises(RuntimeError):
            parse_raw_upload(b"whatever")

    def test_base64_error(self):
        raw_upload = {
            "network": [
                "a/b/c.py",
            ],
            "test_results_files": [
                {
                    "filename": "junit.xml",
                    "format": "base64+compressed",
                    "data": "whatever",
                }
            ]
        }
        json_bytes = json.dumps(raw_upload).encode("utf-8")
        with pytest.raises(RuntimeError):
            parse_raw_upload(json_bytes)

    def test_decompression_error(self):
        raw_upload = {
            "network": [
                "a/b/c.py",
            ],
            "test_results_files": [
                {
                    "filename": "junit.xml",
                    "format": "base64+compressed",
                    "data": base64.b64encode(b"whatever").decode("utf-8"),
                }
            ]
        }
        json_bytes = json.dumps(raw_upload).encode("utf-8")
        with pytest.raises(RuntimeError):
            parse_raw_upload(json_bytes)
            
    def test_parser_error(self):
        with open("tests/error.xml", "b+r") as f:
            file_bytes = f.read()
            raw_upload = {
                "network": [
                    "a/b/c.py",
                ],
                "test_results_files": [
                    {
                        "filename": "jest-junit.xml",
                        "format": "base64+compressed",
                        "data": base64.b64encode(zlib.compress(file_bytes)).decode(
                            "utf-8"
                        ),
                    }
                ]
            }
            json_bytes = json.dumps(raw_upload).encode("utf-8")
            with pytest.raises(RuntimeError):
                parse_raw_upload(json_bytes)



