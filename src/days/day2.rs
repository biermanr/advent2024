use std::path::{Path, PathBuf};
use std::cmp::{min, max};

pub fn part1(data_path: &Path) -> u32 {
    // Read in the entire file
    let text = std::fs::read_to_string(data_path).unwrap();

    let reports: Vec<Vec<i32>> = text.lines()
        .map(|l| l.split_whitespace().map(|n| n.parse().unwrap()).collect())
        .collect();

    let mut num_safe_reports = 0;

    for r in reports {
        let pairs = r.iter().zip(r.iter().skip(1));
        let diffs: Vec<i32> = pairs.map(|(x,y)| x-y).collect();

        let min_diff = *diffs.iter().min().unwrap();
        let max_diff = *diffs.iter().max().unwrap();

        // Skip if they have different signs
        if min_diff * max_diff < 0 { continue };

        let (abs_min, abs_max) = (
            min(min_diff.abs(), max_diff.abs()), 
            max(min_diff.abs(), max_diff.abs()), 
        );

        // Skip if the min diff is too small
        if abs_min < 1 { continue };

        // Skip if the max diff is too big
        if abs_max > 3 { continue };

        num_safe_reports += 1;
    }

    num_safe_reports    
}

pub fn part2(data_path: &Path){
    println!("Hello!");
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;
    use std::io::Write;
    use std::fs::File;

    fn create_test_file() -> (tempfile::TempDir,File,PathBuf) {
        let test_input = "\
7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9\n";

        let temp_dir = tempdir().unwrap();
        let f_path = temp_dir.path().join("test_input.txt");
        let mut temp_file = File::create(f_path.clone()).unwrap();
        write!(temp_file, "{}", test_input).unwrap();

        // have to return dir and file so they don't go out of scope
        (temp_dir, temp_file, f_path)
    }

    #[test]
    fn test_part1(){
        let (_d,_f,test_path) = create_test_file();
        let result = part1(&test_path);
        assert_eq!(result,2);
    }

    #[test]
    fn test_part2(){
        let (_d,_f,test_path) = create_test_file();
        part2(&test_path);
        assert_eq!(0,0);
    }
}