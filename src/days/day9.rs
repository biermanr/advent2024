use std::path::Path;

pub fn part1(data_path: &Path) -> u64 {
    let text = std::fs::read_to_string(data_path).unwrap();

    // Convert chars to int following this stackoverflow
    // https://stackoverflow.com/questions/43983414/how-to-convert-a-rust-char-to-an-integer-so-that-1-becomes-1
    const RADIX: u32 = 10;
    let ns: Vec<u32> = text.chars().filter_map(|c| c.to_digit(RADIX)).collect();

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

    // Return the checksum (must be a better way to do this)
    expanded_map
        .iter()
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

pub fn part2(data_path: &Path) -> u64 {
    let text = std::fs::read_to_string(data_path).unwrap();

    // Convert chars to int following this stackoverflow
    // https://stackoverflow.com/questions/43983414/how-to-convert-a-rust-char-to-an-integer-so-that-1-becomes-1
    const RADIX: u32 = 10;
    let ns: Vec<u32> = text.chars().filter_map(|c| c.to_digit(RADIX)).collect();

    println!("{:?}",ns);
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
