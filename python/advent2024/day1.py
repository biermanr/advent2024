import pathlib
import collections

def part1(f_path: pathlib.Path) -> int:
    with open(f_path) as f:
        l,r = zip(*[map(int,line.split()) for line in f])
        diff_sums = sum([abs(l-r) for l,r in zip(sorted(l),sorted(r))])
        return diff_sums

def part2(f_path: pathlib.Path) -> int:
    with open(f_path) as f:
        l,r = zip(*[map(int,line.split()) for line in f])
        c = collections.Counter(r)
        similarity_score = sum([c[l]*l for l in l])
        return similarity_score
