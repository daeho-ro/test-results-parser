from typing import TypedDict, Literal

class Testrun(TypedDict):
    name: str
    classname: str
    duration: float | None
    outcome: Literal["pass", "failure", "skip", "error"]
    testsuite: str
    failure_message: str | None
    filename: str | None
    build_url: str | None
    computed_name: str

class ParsingInfo(TypedDict):
    framework: Literal["Pytest", "Jest", "Vitest", "PHPUnit"] | None
    testruns: list[Testrun]

def parse_raw_upload(raw_upload_bytes: bytes) -> tuple[list[ParsingInfo], bytes]: ...
