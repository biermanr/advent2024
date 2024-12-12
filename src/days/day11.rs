use cached::proc_macro::cached;
use std::path::Path;

fn next_step(stone: usize) -> Vec<usize> {
    if stone == 0 {
        return vec![1];
    }

    let num_digits = stone.ilog10() as usize;

    if num_digits % 2 == 1 {
        let str_digits = stone.to_string();
        let p1: usize = str_digits[..num_digits / 2 + 1].parse().unwrap();
        let p2: usize = str_digits[num_digits / 2 + 1..].parse().unwrap();
        vec![p1, p2]
    } else {
        vec![stone * 2024]
    }
}

#[cached]
fn num_ending_stones(stone: usize, num_steps: u8) -> usize {
    if num_steps == 0 {
        // Base case, on the last step
        1
    } else {
        // Recursive case
        next_step(stone)
            .iter()
            .map(|&s| num_ending_stones(s, num_steps - 1))
            .sum()
    }
}

pub fn part1(data_path: &Path) -> u32 {
    let text = std::fs::read_to_string(data_path).unwrap();

    let mut stones: Vec<usize> = text
        .split_whitespace()
        .filter_map(|n| n.parse().ok())
        .collect();

    for _ in 0..25 {
        stones = stones.iter().flat_map(|&s| next_step(s)).collect();
    }

    stones.len().try_into().unwrap()
}

pub fn part2(data_path: &Path) -> u64 {
    let text = std::fs::read_to_string(data_path).unwrap();

    let stones: Vec<usize> = text
        .split_whitespace()
        .filter_map(|n| n.parse().ok())
        .collect();

    stones
        .iter()
        .map(|&s| num_ending_stones(s, 75))
        .sum::<usize>()
        .try_into()
        .unwrap()
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
        let test_input = "125 17\n";
        let (_d, _f, test_path) = create_test_file(&test_input);
        let result = part1(&test_path);
        assert_eq!(result, 55312);
    }

    #[test]
    fn test_part2() {
        let test_input = "125 17\n";
        let (_d, _f, test_path) = create_test_file(&test_input);
        let result = part2(&test_path);
        assert_eq!(result, 65601038650482);
    }
}
