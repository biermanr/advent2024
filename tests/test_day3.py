from advent2024 import day3

def test_py_day3_part1(tmp_path):
    f = tmp_path / "input.txt"
    input_data = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))"
    f.write_text(input_data)

    assert day3.part1(f) == 161

def test_py_day3_part2(tmp_path):
    f = tmp_path / "input.txt"
    input_data = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))"
    f.write_text(input_data)

    assert day3.part2(f) == 48