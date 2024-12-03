from advent2024 import day1
import pytest

@pytest.fixture
def test_lists_path(tmp_path):
    f = tmp_path / "input.txt"
    input_data = (
        "3   4\n"
        "4   3\n"
        "2   5\n"
        "1   3\n"
        "3   9\n"
        "3   3\n"
    )
    f.write_text(input_data)
    return f

def test_py_day1_part1(test_lists_path):
    assert day1.part1(test_lists_path) == 11

def test_py_day1_part2(test_lists_path):
    assert day1.part2(test_lists_path) == 31