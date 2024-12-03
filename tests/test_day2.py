from advent2024 import day2
import pytest

@pytest.fixture
def test_report_path(tmp_path):
    f = tmp_path / "input.txt"
    input_data = (
        "7 6 4 2 1\n"
        "1 2 7 8 9\n"
        "9 7 6 2 1\n"
        "1 3 2 4 5\n"
        "8 6 4 4 1\n"
        "1 3 6 7 9\n"
    )
    f.write_text(input_data)
    return f

def test_py_day2_part1(test_report_path):
    assert day2.part1(test_report_path) == 2

def test_py_day2_part2(test_report_path):
    assert day2.part2(test_report_path) == 4