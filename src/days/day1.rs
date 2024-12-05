use std::cmp::max;
use std::collections::HashMap;
use std::path::Path;

pub fn part1(data_path: &Path) -> u32 {
    // Read entire file contents at once
    let text = std::fs::read_to_string(data_path).unwrap();

    // Convert lines to numeric
    let nums: Vec<Vec<i32>> = text
        .lines()
        .map(|l| l.split_whitespace().map(|n| n.parse().unwrap()).collect())
        .collect();

    // Collect the first and second value of each row into two sorted vectors
    let mut v1: Vec<i32> = nums.iter().map(|ns| ns[0]).collect();
    let mut v2: Vec<i32> = nums.iter().map(|ns| ns[1]).collect();
    v1.sort();
    v2.sort();

    // Perform the difference and sum
    let diff: i32 = v1
        .iter()
        .zip(v2.iter())
        .map(|(l, r)| max(l - r, r - l))
        .sum();

    diff.try_into().unwrap()
}

pub fn part2(data_path: &Path) -> u32 {
    let text = std::fs::read_to_string(data_path).unwrap();

    let nums: Vec<Vec<i32>> = text
        .lines()
        .map(|l| l.split_whitespace().map(|n| n.parse().unwrap()).collect())
        .collect();

    let v1: Vec<i32> = nums.iter().map(|ns| ns[0]).collect();
    let v2: Vec<i32> = nums.iter().map(|ns| ns[1]).collect();

    let mut v2_counts: HashMap<i32, i32> = HashMap::new();
    for v in v2 {
        *v2_counts.entry(v).or_insert(0) += 1;
    }

    let diff: i32 = v1
        .iter()
        .map(|v| *v2_counts.entry(*v).or_insert(0) * v)
        .sum();
    diff.try_into().unwrap()
}

// Test the run function
#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::io::Write;
    use std::path::PathBuf;
    use tempfile::tempdir;

    fn create_test_file() -> (tempfile::TempDir, File, PathBuf) {
        let test_input = "\
3   4
4   3
2   5
1   3
3   9
3   3";

        let temp_dir = tempdir().unwrap();
        let f_path = temp_dir.path().join("test_input.txt");
        let mut temp_file = File::create(f_path.clone()).unwrap();
        write!(temp_file, "{}", test_input).unwrap();

        // have to return dir and file so they don't go out of scope
        (temp_dir, temp_file, f_path)
    }

    #[test]
    fn test_part1() {
        let (_d, _f, test_path) = create_test_file();
        let result = part1(&test_path);
        assert_eq!(result, 11);
    }

    #[test]
    fn test_part2() {
        let (_d, _f, test_path) = create_test_file();
        let result = part2(&test_path);
        assert_eq!(result, 31);
    }
}
