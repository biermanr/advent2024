from advent2024 import advent2024
import argparse
import time
import contextlib

# stolen from Henry https://github.com/henryiii/aoc2024/blob/main/python/problems.py
@contextlib.contextmanager
def timer():
    start = time.perf_counter()
    yield
    print(f"({(time.perf_counter() - start)*1000:.2f}ms)")

def main():
    parser = argparse.ArgumentParser(description="Run all days of Advent of Code 2024")
    parser.add_argument("--data", type=str, default="data", help="Folder containing input data files like data/ where it expects to find data/day1.txt data/day2.txt etc.")
    args = parser.parse_args()

    days = [
        advent2024.day1_part1, advent2024.day1_part2,
        advent2024.day2_part1, advent2024.day2_part2,
        advent2024.day3_part1, advent2024.day3_part2,
        advent2024.day4_part1, advent2024.day4_part2,
        advent2024.day5_part1, advent2024.day5_part2,
        advent2024.day6_part1, advent2024.day6_part2,
        advent2024.day7_part1, advent2024.day7_part2,
        advent2024.day8_part1, advent2024.day8_part2,
        advent2024.day9_part1, advent2024.day9_part2,
        advent2024.day10_part1, advent2024.day10_part2,
        advent2024.day11_part1, advent2024.day11_part2,
        advent2024.day12_part1, advent2024.day12_part2,
        advent2024.day13_part1, advent2024.day13_part2,
    ]

    for i, func in enumerate(days):
        day = i//2 + 1
        part = i%2 + 1

        with timer():
            result = func(f"{args.data}/day{day}.txt")
            print(f"Day {day} part {part} result {result}", end=" ")

if __name__ == "__main__":
    main()