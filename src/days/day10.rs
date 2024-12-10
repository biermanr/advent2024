use std::collections::HashSet;
use std::path::Path;

fn get_trailheads(grid: &Vec<Vec<u32>>) -> Vec<(usize, usize)> {
    let mut trailheads = vec![];
    for y in 0..grid.len() {
        for x in 0..grid[y].len() {
            if grid[y][x] == 0 {
                trailheads.push((x, y));
            }
        }
    }
    trailheads
}

fn next_steps(grid: &Vec<Vec<u32>>, pos: (usize, usize)) -> Vec<(usize, usize)> {
    let (x, y) = pos;
    let height = grid[y][x];
    let mut valid_steps = Vec::new();

    //try stepping in all four cardinal directions to see if the height is one-higher than current
    if x >= 1 && grid[y][x - 1] == height + 1 {
        valid_steps.push((x - 1, y))
    }
    if x + 1 <= grid[0].len() - 1 && grid[y][x + 1] == height + 1 {
        valid_steps.push((x + 1, y))
    }
    if y >= 1 && grid[y - 1][x] == height + 1 {
        valid_steps.push((x, y - 1))
    }
    if y + 1 <= grid.len() - 1 && grid[y + 1][x] == height + 1 {
        valid_steps.push((x, y + 1))
    }

    valid_steps
}

fn get_trail_tails(
    grid: &Vec<Vec<u32>>,
    pos: (usize, usize),
    mut trail_tails: HashSet<(usize, usize)>,
) -> HashSet<(usize, usize)> {
    let height = grid[pos.1][pos.0];

    if height == 9 {
        // Basecase: we've found a trailhead!
        trail_tails.insert(pos);
    } else {
        for next_pos in next_steps(&grid, pos) {
            trail_tails.extend(get_trail_tails(&grid, next_pos, trail_tails.clone()));
        }
    }
    trail_tails
}

fn num_distinct_hikes(grid: &Vec<Vec<u32>>, pos: (usize, usize)) -> u32 {
    let height = grid[pos.1][pos.0];

    if height == 9 {
        // Basecase: we've found a trailhead!
        1
    } else {
        next_steps(&grid, pos)
            .iter()
            .map(|&next_pos| num_distinct_hikes(&grid, next_pos))
            .sum()
    }
}

pub fn part1(data_path: &Path) -> u32 {
    // Create grid of u32 from input
    let text = std::fs::read_to_string(data_path).unwrap();
    const RADIX: u32 = 10;
    let grid: Vec<Vec<u32>> = text
        .lines()
        .map(|l| l.chars().filter_map(|c| c.to_digit(RADIX)).collect())
        .collect();

    // Get all the trailheads
    let trailheads = get_trailheads(&grid);

    // Score each trailhead and return the sum
    let scores: Vec<u32> = trailheads
        .iter()
        .map(|th| get_trail_tails(&grid, *th, HashSet::new()).len() as u32)
        .collect();
    scores.iter().sum()
}

pub fn part2(data_path: &Path) -> u32 {
    // Create grid of u32 from input
    let text = std::fs::read_to_string(data_path).unwrap();
    const RADIX: u32 = 10;
    let grid: Vec<Vec<u32>> = text
        .lines()
        .map(|l| l.chars().filter_map(|c| c.to_digit(RADIX)).collect())
        .collect();

    // Get all the trailheads
    let trailheads = get_trailheads(&grid);

    // ???
    let scores: Vec<u32> = trailheads
        .iter()
        .map(|th| num_distinct_hikes(&grid, *th))
        .collect();
    scores.iter().sum()
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
89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732\n";
        let (_d, _f, test_path) = create_test_file(&test_input);
        let result = part1(&test_path);
        assert_eq!(result, 36);
    }

    #[test]
    fn test_part2() {
        let test_input = "\
89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732\n";
        let (_d, _f, test_path) = create_test_file(&test_input);
        let result = part2(&test_path);
        assert_eq!(result, 81);
    }
}
