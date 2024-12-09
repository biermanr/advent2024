use std::collections::HashMap;
use std::path::Path;

fn expand_map(ns: &Vec<u32>) -> Vec<Option<u32>> {
    // Expand the fragmented memory representation into the full map
    let mut file_id = 0;
    let mut expanded_map: Vec<Option<u32>> = Vec::new();

    for (i, n) in ns.iter().enumerate() {
        if i % 2 == 0 {
            for _ in 0..*n {
                expanded_map.push(Some(file_id));
            }
            file_id += 1;
        } else {
            for _ in 0..*n {
                expanded_map.push(None);
            }
        }
    }
    expanded_map
}

fn checksum(m: Vec<Option<u32>>) -> u64 {
    // Return the checksum (must be a better way to do this)
    m.iter()
        .enumerate()
        .map(|(i, f_n)| {
            if let Some(n) = f_n {
                (i as u64) * (*n as u64)
            } else {
                0
            }
        })
        .sum()
}

pub fn part1(data_path: &Path) -> u64 {
    let text = std::fs::read_to_string(data_path).unwrap();

    // Convert chars to int following this stackoverflow
    // https://stackoverflow.com/questions/43983414/how-to-convert-a-rust-char-to-an-integer-so-that-1-becomes-1
    const RADIX: u32 = 10;
    let ns: Vec<u32> = text.chars().filter_map(|c| c.to_digit(RADIX)).collect();

    let mut expanded_map = expand_map(&ns);

    // De-fragment the map, keeping track of the left-most open and right-most used blocks
    let mut open_ind = 0;
    let mut block_ind = expanded_map.len() - 1;

    while open_ind < block_ind {
        if expanded_map[open_ind].is_some() {
            // Advance open_ind to the right until it's at an open block
            open_ind += 1;
        } else if expanded_map[block_ind].is_none() {
            // Advance block_ind to the left until it's at a file block
            block_ind -= 1;
        } else {
            // Swap the contents of the indices and advance both
            expanded_map[open_ind] = expanded_map[block_ind];
            expanded_map[block_ind] = None;
            open_ind += 1;
            block_ind -= 1;
        }
    }

    checksum(expanded_map)
}

pub fn part2(data_path: &Path) -> u64 {
    let text = std::fs::read_to_string(data_path).unwrap();

    const RADIX: u32 = 10;
    let ns: Vec<u32> = text.chars().filter_map(|c| c.to_digit(RADIX)).collect();

    // Get the start and end coordinates of each file
    let mut files: HashMap<u32, (u32, u32)> = HashMap::new();

    let mut cumulative_ind = 0;
    let mut file_id = 0;
    for (i, n) in ns.iter().enumerate() {
        if i % 2 == 0 {
            files.insert(file_id, (cumulative_ind, cumulative_ind + n - 1));
            file_id += 1;
        }
        cumulative_ind += n;
    }

    // Get the fully expanded map (maybe a smart way to avoid this)
    let mut expanded_map = expand_map(&ns);

    // De-fragment the map
    let max_file_id: u32 = (ns.len() / 2).try_into().unwrap();

    for file_id in (0..=max_file_id).rev() {
        //use the start and end ind of the file blocks
        let (f_start, f_end) = files.get(&file_id).unwrap();
        let f_start: usize = (*f_start).try_into().unwrap();
        let f_end: usize = (*f_end).try_into().unwrap();
        let f_blocks = f_end - f_start + 1;

        //find the left-most open span large enough to contain this file
        let mut open_ind: usize = 0;
        while open_ind < f_start {
            if expanded_map[open_ind].is_some() {
                open_ind += 1;
            } else {
                let mut num_contiguous_open = 0;
                let mut open_end_ind = open_ind;
                while open_ind < f_start && expanded_map[open_end_ind].is_none() {
                    num_contiguous_open += 1;
                    open_end_ind += 1;
                }

                if num_contiguous_open >= f_blocks {
                    break;
                } else {
                    open_ind += num_contiguous_open;
                }
            }
        }

        // If open_ind is less than the f_start, must have found a large enough open block
        if open_ind < f_start {
            for transfer_ind in 0..f_blocks {
                expanded_map[open_ind + transfer_ind] = expanded_map[f_start + transfer_ind];
                expanded_map[f_start + transfer_ind] = None;
            }
        }
    }

    checksum(expanded_map)
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
        let test_input = "2333133121414131402\n";
        let (_d, _f, test_path) = create_test_file(&test_input);
        let result = part1(&test_path);
        assert_eq!(result, 1928);
    }

    #[test]
    fn test_part2() {
        let test_input = "2333133121414131402\n";
        let (_d, _f, test_path) = create_test_file(&test_input);
        let result = part2(&test_path);
        assert_eq!(result, 2858);
    }
}
