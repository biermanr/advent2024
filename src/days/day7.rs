use std::collections::VecDeque;
use std::path::Path;

fn validate_equation(target: usize, mut ns: VecDeque<usize>, test: usize) -> usize {
    if let Some(n) = ns.pop_front() {
        if test > target {
            // Can return early if test is already too large
            0
        } else {
            // Ugly, but on the first iteration shoulhd have test = 1 for mult
            let test_mult_branch = if test == 0 { 1 } else { test };

            // If the sum of recur calls is greater than 0, at least one must be ok
            if validate_equation(target, ns.clone(), test + n)
                + validate_equation(target, ns, test_mult_branch * n)
                > 0
            {
                target
            } else {
                0
            }
        }
    } else {
        // Base case where there are no more numbers
        if test == target {
            target
        } else {
            0
        }
    }
}

pub fn part1(data_path: &Path) -> u64 {
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

    let summed_results: usize = equations
        .iter()
        .map(|(test, ns)| validate_equation(*test, ns.clone(), 0))
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
}
