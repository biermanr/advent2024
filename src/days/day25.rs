use std::path::Path;

pub fn part1(data_path: &Path) -> u32 {
    let text = std::fs::read_to_string(data_path).unwrap();

    // Read in the lock and keys
    let raw_lock_keys: Vec<Vec<char>> = text
        .lines()
        .filter(|l| l.len() > 0)
        .map(|l| l.chars().collect())
        .collect();

    // Process the lock and keys in groups of 7
    // process locks and keys differently
    let processed_lock_keys: Vec<(&str, Vec<usize>)> = raw_lock_keys
        .chunks(7)
        .map(|ls| match ls[0][0] {
            '#' => {
                let mut heights = Vec::new();
                for t in 0..5 {
                    let mut height = 0;
                    for h in 0..5 {
                        if ls[5 - h][t] == '#' {
                            height = 5 - h;
                            break;
                        }
                    }
                    heights.push(height);
                }
                ("lock", heights)
            }
            '.' => {
                let mut heights = Vec::new();
                for t in 0..5 {
                    let mut height = 0;
                    for h in 1..6 {
                        if ls[h][t] == '#' {
                            height = 6 - h;
                            break;
                        }
                    }
                    heights.push(height);
                }
                ("key", heights)
            }
            _ => unreachable!(),
        })
        .collect();

    let locks: Vec<&Vec<usize>> = processed_lock_keys
        .iter()
        .filter(|(s, _)| *s == "lock")
        .map(|(_, l)| l)
        .collect();

    let keys: Vec<&Vec<usize>> = processed_lock_keys
        .iter()
        .filter(|(s, _)| *s == "key")
        .map(|(_, l)| l)
        .collect();

    let mut num_pairs = 0;
    for lock in &locks {
        for key in &keys {
            let mut ok = true;
            for i in 0..5 {
                if lock[i]+key[i] > 5 {
                    ok = false;
                    break;
                }
            }
            if ok {
                num_pairs += 1;
            }
        }
    }

    num_pairs
}

pub fn part2(data_path: &Path) -> u32 {
    0
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
#####
.####
.####
.####
.#.#.
.#...
.....

#####
##.##
.#.##
...##
...#.
...#.
.....

.....
#....
#....
#...#
#.#.#
#.###
#####

.....
.....
#.#..
###..
###.#
###.#
#####

.....
.....
.....
#....
#.#..
#.#.#
#####\n";
        let (_d, _f, test_path) = create_test_file(&test_input);
        let result = part1(&test_path);
        assert_eq!(result, 3);
    }
}
