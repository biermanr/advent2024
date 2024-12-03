import pathlib

def validate_report(report: list[str]) -> bool:
    nums = [int(n) for n in report]
    diffs = [l-r for (l,r) in zip(nums,nums[1:])]

    # Invalid if some diffs are negative and some are positive
    min_diff,max_diff = min(diffs),max(diffs)
    if min_diff*max_diff < 0:
        return False
    
    # Invalid if the absolute value of the min diff is < 1 or abs value of max diff is > 3
    abs_min,abs_max = min(abs(min_diff),abs(max_diff)),max(abs(min_diff),abs(max_diff))
    if abs_min < 1 or abs_max > 3:
        return False
    
    return True

def part1(f_path: pathlib.Path) -> int:
    with open(f_path) as f:
        num_valid_reports = sum([validate_report(l.strip().split()) for l in f])

    return num_valid_reports

def part2(f_path: pathlib.Path) -> int:
    num_valid_reports = 0

    with open(f_path) as f:
        for l in f:
            report = l.strip().split()
            valid = validate_report(report)
            for i in range(len(report)):
                if valid:
                    break

                valid = validate_report(report[:i] + report[i+1:])

            num_valid_reports += valid


    return num_valid_reports