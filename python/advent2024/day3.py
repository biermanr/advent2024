import pathlib
import re

def calc_mat_mul(s):
    muls = re.findall("mul\((\d{1,3}),(\d{1,3})\)",s)
    return sum(int(n1)*int(n2) for n1,n2 in muls)

def part1(f_path: pathlib.Path) -> int:
    with open(f_path) as f:
        return sum([calc_mat_mul(l) for l in f])

def part2(f_path: pathlib.Path) -> int:
    with open(f_path) as f:
        l = ''.join(f.readlines())

    tot = 0
    state = True
    while len(l) > 0:
        if state:
            i = l.index("don't()") if "don't()" in l else len(l)
            calc_l,l = l[:i],l[i+7:]
            tot += calc_mat_mul(calc_l)
            state = False
        else:
            i = l.index("do()") if "do()" in l else len(l)
            l = l[i+4:]
            state = True
                
    return tot
