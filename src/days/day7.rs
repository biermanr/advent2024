// TODO fix lots of code duplication
use std::collections::VecDeque;
use std::path::Path;

fn concat_nums(a: usize, b: usize) -> usize {
    format!("{}{}", a, b).parse().unwrap()
}

fn valid_equation(target: usize, mut ns: VecDeque<usize>, test: usize, is_part2: bool) -> bool {
    if let Some(n) = ns.pop_front() {
        // Recursive case
        if test > target {
            // Can return early if test is already too large
            false
        } else {
            // Ugly, but on the first iteration should have test = 1 for mult
            let test_mult_branch = if test == 0 { 1 } else { test };

            // If the sum of recur calls is greater than 0, at least one must be ok
            let mut any_valid = valid_equation(target, ns.clone(), test + n, is_part2);
            any_valid |= valid_equation(target, ns.clone(), test_mult_branch * n, is_part2);

            if is_part2 {
                any_valid |= valid_equation(target, ns.clone(), concat_nums(test, n), is_part2);
            }
            any_valid
        }
    } else {
        // Base case where there are no more numbers
        test == target
    }
}
fn parse_input(data_path: &Path) -> Vec<(usize, VecDeque<usize>)> {
    let text = std::fs::read_to_string(data_path).unwrap();

    let equations: Vec<(usize, VecDeque<usize>)> = text
        .lines()
        .map(|l| l.split_once(": ").unwrap())
        .map(|(t, ns)| {
            (
                t.parse().unwrap(),
                ns.split(" ").map(|n| n.parse().unwrap()).collect(),
            )
        })
        .collect();

    equations
}

pub fn part1(data_path: &Path) -> u64 {
    let equations = parse_input(data_path);

    let summed_results: usize = equations
        .iter()
        .filter(|(test, ns)| valid_equation(*test, ns.clone(), 0, false))
        .map(|(test, _)| test)
        .sum();

    summed_results.try_into().unwrap()
}

pub fn part2(data_path: &Path) -> u64 {
    let equations = parse_input(data_path);

    let summed_results: usize = equations
        .iter()
        .filter(|(test, ns)| valid_equation(*test, ns.clone(), 0, true))
        .map(|(test, _)| test)
        .sum();

    summed_results.try_into().unwrap()
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
        let test_input = "\
190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20\n";
        let (_d, _f, test_path) = create_test_file(&test_input);
        let result = part1(&test_path);
        assert_eq!(result, 3749);
    }

    #[test]
    fn test_part2() {
        let test_input = "\
190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20\n";
        let (_d, _f, test_path) = create_test_file(&test_input);
        let result = part2(&test_path);
        assert_eq!(result, 11387);
    }
}
