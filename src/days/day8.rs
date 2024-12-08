use std::cmp::{max, min};
use std::collections::{HashMap, HashSet};
use std::path::Path;

fn get_antenna_by_freq(grid: &Vec<Vec<char>>) -> HashMap<char, Vec<(i32, i32)>> {
    // Collect the antenna positions by frequency
    let mut antenna_by_freq: HashMap<char, Vec<(i32, i32)>> = HashMap::new();

    for y in 0..grid.len() {
        for x in 0..grid[y].len() {
            if grid[y][x] != '.' {
                antenna_by_freq
                    .entry(grid[y][x])
                    .or_insert(Vec::new())
                    .push((x.try_into().unwrap(), y.try_into().unwrap()));
            }
        }
    }

    antenna_by_freq
}

pub fn part1(data_path: &Path) -> u32 {
    let text = std::fs::read_to_string(data_path).unwrap();

    // Read in the antenna grid
    let grid: Vec<Vec<char>> = text.lines().map(|l| l.chars().collect()).collect();
    let height_i32: i32 = grid.len().try_into().unwrap();
    let width_i32: i32 = grid[0].len().try_into().unwrap();

    let antenna_by_freq = get_antenna_by_freq(&grid);

    let mut antinodes: HashSet<(i32, i32)> = HashSet::new();

    for (_freq, positions) in &antenna_by_freq {
        // Calculate antinodes for all pairs of antenna's of this same freq
        for i in 0..positions.len() - 1 {
            for j in i + 1..positions.len() {
                let (x1, y1) = positions[i];
                let (x2, y2) = positions[j];
                let (dx, dy) = ((x1 - x2).abs(), (y1 - y2).abs());

                // There will either be a "left-up" and a "right-down" antinode
                // or a "left-down" and "right-up"
                let ((a1_x, a1_y), (a2_x, a2_y)) = if (x1 < x2 && y1 < y2) || (x2 < x1 && y2 < y1) {
                    (
                        (min(x1, x2) - dx, min(y1, y2) - dy),
                        (max(x1, x2) + dx, max(y1, y2) + dy),
                    )
                } else {
                    (
                        (min(x1, x2) - dx, max(y1, y2) + dy),
                        (max(x1, x2) + dx, min(y1, y2) - dy),
                    )
                };

                // Only add antinodes within the grid bounds
                if a1_x >= 0 && a1_y >= 0 && a1_x < width_i32 && a1_y < height_i32 {
                    antinodes.insert((a1_x, a1_y));
                }
                if a2_x >= 0 && a2_y >= 0 && a2_x < width_i32 && a2_y < height_i32 {
                    antinodes.insert((a2_x, a2_y));
                }
            }
        }
    }

    antinodes.len().try_into().unwrap()
}

pub fn part2(data_path: &Path) -> u32 {
    let text = std::fs::read_to_string(data_path).unwrap();

    // Read in the antenna grid
    let grid: Vec<Vec<char>> = text.lines().map(|l| l.chars().collect()).collect();
    let height_i32: i32 = grid.len().try_into().unwrap();
    let width_i32: i32 = grid[0].len().try_into().unwrap();

    let antenna_by_freq = get_antenna_by_freq(&grid);

    let mut antinodes: HashSet<(i32, i32)> = HashSet::new();

    for (_freq, positions) in &antenna_by_freq {
        // Calculate antinodes for all pairs of antenna's of this same freq
        for i in 0..positions.len() - 1 {
            for j in i + 1..positions.len() {
                let (x1, y1) = positions[i];
                let (x2, y2) = positions[j];
                let (dx, dy) = (x1 - x2, y1 - y2);

                // Add all grid positions as antinodes that are co-linear between each pair of antenna
                // first "walk left" from the first antenna (could be either)
                let (mut curr_x, mut curr_y) = (x1, y1);
                while curr_x >= 0 && curr_x < width_i32 && curr_y >= 0 && curr_y < height_i32 {
                    antinodes.insert((curr_x, curr_y));
                    curr_x -= dx;
                    curr_y -= dy;
                }

                // then "walk right" from the same antenna
                let (mut curr_x, mut curr_y) = (x1, y1);
                while curr_x >= 0 && curr_x < width_i32 && curr_y >= 0 && curr_y < height_i32 {
                    antinodes.insert((curr_x, curr_y));
                    curr_x += dx;
                    curr_y += dy;
                }
            }
        }
    }

    antinodes.len().try_into().unwrap()
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
............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............\n";
        let (_d, _f, test_path) = create_test_file(&test_input);
        let result = part1(&test_path);
        assert_eq!(result, 14);
    }

    #[test]
    fn test_part2() {
        let test_input = "\
............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............\n";
        let (_d, _f, test_path) = create_test_file(&test_input);
        let result = part2(&test_path);
        assert_eq!(result, 34);
    }
}
