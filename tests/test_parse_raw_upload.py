import pytest
import base64
import zlib
import json
from test_results_parser import parse_raw_upload
class TestParsers:
    def test_junit(self, snapshot):
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


            assert snapshot("bin") == readable_files
            assert snapshot("json") == parsing_infos



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



