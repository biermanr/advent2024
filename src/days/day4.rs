use std::path::Path;

fn check_pos(grid: &Vec<&str>, x: i64, y: i64, c: char) -> bool {
    // Check bounds
    if y < 0 || x < 0 {
        return false;
    }

    let x: usize = x.try_into().unwrap();
    let y: usize = y.try_into().unwrap();

    if y >= grid.len() || x >= grid[0].len() {
        return false;
    }

    if let Some(v) = grid[y].chars().nth(x) {
        v == c
    } else {
        false
    }
}

fn find_char_coords(grid: &Vec<&str>, c: char) -> Vec<(i64, i64)> {
    let mut xs = vec![];
    for y in 0..grid.len() {
        for x in 0..grid[0].len() {
            if grid[y].chars().nth(x).unwrap() == c {
                xs.push((x as i64, y as i64));
            }
        }
    }
    xs
}

fn count_mas(grid: &Vec<&str>, xy: (i64, i64)) -> u32 {
    let mut num_mas = 0;
    let dirs = vec![
        (0, 1),
        (1, 0),
        (0, -1),
        (-1, 0),
        (1, 1),
        (1, -1),
        (-1, 1),
        (-1, -1),
    ];
    let (x, y) = xy;
    let mas_chars = "MAS";

    for (dx, dy) in dirs {
        let mut curr_x = x;
        let mut curr_y = y;
        let mut found = true;
        for c in mas_chars.chars() {
            curr_x += dx;
            curr_y += dy;
            if !check_pos(&grid, curr_x, curr_y, c) {
                found = false;
                break;
            }
        }

        if found {
            num_mas += 1
        };
    }

    num_mas
}

fn count_crosses(grid: &Vec<&str>, xy: (i64, i64)) -> u32 {
    let (x, y) = xy;
    let mut num_crosses = 0;
    let directions = vec![(1, 1), (0, -2), (-2, 0), (0, 2)];
    let mas_chars_type1 = "MMSS";
    let mas_chars_type2 = "SSMM";
    let mas_chars_type3 = "MSSM";
    let mas_chars_type4 = "SMMS";
    let mas_chars_types = vec![
        mas_chars_type1,
        mas_chars_type2,
        mas_chars_type3,
        mas_chars_type4,
    ];

    for mas_chars in mas_chars_types {
        let mut curr_x = x;
        let mut curr_y = y;
        let mut found = true;

        for ((dx, dy), c) in directions.iter().zip(mas_chars.chars()) {
            curr_x += dx;
            curr_y += dy;
            if !check_pos(&grid, curr_x, curr_y, c) {
                found = false;
                break;
            }
        }
        if found {
            num_crosses += 1
        };
    }

    num_crosses
}

pub fn part1(data_path: &Path) -> u32 {
    let text = std::fs::read_to_string(data_path).unwrap();

    // Read in the XMAS grid
    let grid: Vec<&str> = text.lines().collect();

    let x_coords = find_char_coords(&grid, 'X');

    x_coords.iter().map(|&xy| count_mas(&grid, xy)).sum()
}

pub fn part2(data_path: &Path) -> u32 {
    let text = std::fs::read_to_string(data_path).unwrap();

    // Read in the XMAS grid
    let grid: Vec<&str> = text.lines().collect();

    // Find the A positions
    let a_coords = find_char_coords(&grid, 'A');

    a_coords.iter().map(|&xy| count_crosses(&grid, xy)).sum()
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
MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";
        let (_d, _f, test_path) = create_test_file(&test_input);
        let result = part1(&test_path);
        assert_eq!(result, 18);
    }

    #[test]
    fn test_part2() {
        let test_input = "\
.M.S......
..A..MSMS.
.M.S.MAA..
..A.ASMSM.
.M.S.M....
..........
S.S.S.S.S.
.A.A.A.A..
M.M.M.M.M.
..........\n";
        let (_d, _f, test_path) = create_test_file(&test_input);
        let result = part2(&test_path);
        assert_eq!(result, 9);
    }
}
