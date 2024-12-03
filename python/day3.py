import io
import re

def calc_mat_mul(s):
    muls = re.findall("mul\((\d{1,3}),(\d{1,3})\)",s)
    return sum(int(n1)*int(n2) for n1,n2 in muls)

def part1(f):
    return sum([calc_mat_mul(l) for l in f])

def part2(f):
    tot = 0
    l = ''.join(f.readlines())
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
    

#Tests
example_f = io.StringIO("xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))\n")
r1 = part1(example_f)
assert r1 == 161

example_f = io.StringIO("xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))\n")
r2 = part2(example_f)
assert r2 == 48


#Actual data
with open("data/day3.txt") as f:
    print(part1(f))

with open("data/day3.txt") as f:
    print(part2(f))
