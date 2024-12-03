use regex::Regex;
use std::cmp::min;
use std::path::Path;

fn calc_line_mul_sums(line: &str) -> u32 {
    let outer_re = Regex::new(r"(mul\(\d{1,3},\d{1,3}\))").unwrap();
    let inner_re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();

    let mut mul_sums = 0;

    for cap in outer_re.captures_iter(line) {
        for inner_cap in inner_re.captures_iter(&cap[1]) {
            let n1: u32 = inner_cap[1].parse().unwrap();
            let n2: u32 = inner_cap[2].parse().unwrap();
            mul_sums += n1 * n2;
        }
    }

    mul_sums
}

pub fn part1(data_path: &Path) -> u32 {
    let text = std::fs::read_to_string(data_path).unwrap();
    text.lines().map(calc_line_mul_sums).sum()
}

pub fn part2(data_path: &Path) -> u32 {
    let text = std::fs::read_to_string(data_path).unwrap();

    let mut mul_sums = 0;

    // Very tricky, the input should be treated as just one line
    // for some reason didn't matter for part1
    let joined_lines = text
        .lines()
        .map(|s| s.to_string())
        .collect::<Vec<String>>()
        .join("");

    let mut l = &joined_lines[..]; //[..] to get &str instead of &String ???

    let mut state = true;
    while !l.is_empty() {
        if state {
            let change_ind = l.find("don't()").unwrap_or(l.len());
            mul_sums += calc_line_mul_sums(&l[..change_ind]);
            l = &l[min(change_ind + 7, l.len())..]; //"don't()" is len 7
            state = false;
        } else {
            let change_ind = l.find("do()").unwrap_or(l.len());
            l = &l[min(change_ind + 4, l.len())..]; //"do()" is len 4
            state = true;
        }
    }
    mul_sums
}

#[cfg(test)]
mod tests {

    use super::*;
    use std::fs::File;
    use std::io::Write;
    use std::path::PathBuf;
    use tempfile::tempdir;

    fn create_test_file(test_input: &str) -> (tempfile::TempDir, File, PathBuf) {
        let temp_dir = tempdir().unwrap();
        let f_path = temp_dir.path().join("test_input.txt");
        let mut temp_file = File::create(f_path.clone()).unwrap();
        write!(temp_file, "{}", test_input).unwrap();

        // have to return dir and file so they don't go out of scope
        (temp_dir, temp_file, f_path)
    }

    #[test]
    fn test_part1() {
        let test_input =
            "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))\n";
        let (_d, _f, path) = create_test_file(test_input);
        let result = part1(&path);
        assert_eq!(result, 161);
    }

    #[test]
    fn test_part2() {
        let test_input =
            "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))\n";
        let (_d, _f, path) = create_test_file(test_input);
        let result = part2(&path);
        assert_eq!(result, 48);
    }
}
