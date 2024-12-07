use std::collections::HashSet;
use std::path::Path;

#[derive(Debug, Hash, PartialEq, Eq, Clone)]
struct GuardPosition {
    x: usize,
    y: usize,
    orientation: char,
}

fn next_pos(grid: &Vec<Vec<char>>, guard_pos: &GuardPosition) -> Option<(usize, usize)> {
    let (dx, dy) = match guard_pos.orientation {
        '^' => (0, -1),
        '>' => (1, 0),
        'v' => (0, 1),
        '<' => (-1, 0),
        _ => (0, 0), //TODO make directions enum so I don't have to do this
    };

    //TODO all of this is ugly. Issue is negative signs with usize types
    let dx: i64 = dx.try_into().unwrap();
    let dy: i64 = dy.try_into().unwrap();
    let curr_x: i64 = guard_pos.x.try_into().unwrap();
    let curr_y: i64 = guard_pos.y.try_into().unwrap();

    //check if taking a step leaves the area
    if curr_x + dx < 0 || curr_y + dy < 0 {
        return None;
    }

    let new_x: usize = (curr_x + dx).try_into().unwrap();
    let new_y: usize = (curr_y + dy).try_into().unwrap();

    if new_y >= grid.len() || new_x >= grid[0].len() {
        return None;
    }

    //otherwise take a step if unblocked
    if grid[new_y][new_x] == '#' {
        Some((guard_pos.x, guard_pos.y))
    } else {
        Some((new_x, new_y))
    }
}

fn next_guard_position(grid: &Vec<Vec<char>>, guard_pos: &GuardPosition) -> Option<GuardPosition> {
    let (new_x, new_y) = next_pos(&grid, &guard_pos)?;

    let new_guard_pos = if guard_pos.x == new_x && guard_pos.y == new_y {
        // If the next_pos is the same as the previous pos, then need to rotate
        let new_orientation = match guard_pos.orientation {
            '^' => '>',
            '>' => 'v',
            'v' => '<',
            '<' => '^',
            _ => '?', //TODO make directions enum so I don't have to do this
        };

        GuardPosition {
            x: guard_pos.x,
            y: guard_pos.y,
            orientation: new_orientation,
        }
    } else {
        //Take a step
        GuardPosition {
            x: new_x,
            y: new_y,
            orientation: guard_pos.orientation,
        }
    };

    Some(new_guard_pos)
}

pub fn part1(data_path: &Path) -> u32 {
    let text = std::fs::read_to_string(data_path).unwrap();

    // Read in the guard grid
    let grid: Vec<Vec<char>> = text.lines().map(|l| l.chars().collect()).collect();

    let directions = HashSet::from(['^', '>', 'v', '<']);

    // Find guard's starting position and orientation
    //
    // Got from ChatGPT, need to understand what's happening :(
    // what's being `move`d?
    // apparently `move` is transfering ownership of y and row into the inner map from flat_map
    // still don't really understand what problem move is avoiding
    let (x, y, orientation) = grid
        .iter()
        .enumerate()
        .flat_map(|(y, row)| row.iter().enumerate().map(move |(x, &c)| (x, y, c)))
        .filter(|(_x, _y, c)| directions.contains(c))
        .next()
        .unwrap();

    let mut guard = GuardPosition {
        x: x,
        y: y,
        orientation: orientation,
    };

    let mut guard_positions: HashSet<GuardPosition> = HashSet::new();
    let mut guard_squares: HashSet<(usize, usize)> = HashSet::new();

    while !guard_positions.contains(&guard) {
        guard_positions.insert(guard.clone());
        guard_squares.insert((guard.x, guard.y));

        //TODO make this less ugly
        if let Some(g) = next_guard_position(&grid, &guard) {
            guard = g;
        } else {
            break;
        };
    }

    // Return length of all unique guard squares
    guard_squares.len().try_into().unwrap()
}

pub fn part2(data_path: &Path) -> u32 {
    let text = std::fs::read_to_string(data_path).unwrap();

    // Read in the guard grid
    let mut grid: Vec<Vec<char>> = text.lines().map(|l| l.chars().collect()).collect();

    let directions = HashSet::from(['^', '>', 'v', '<']);

    let (start_x, start_y, start_orientation) = grid
        .iter()
        .enumerate()
        .flat_map(|(y, row)| row.iter().enumerate().map(move |(x, &c)| (x, y, c)))
        .filter(|(_x, _y, c)| directions.contains(c))
        .next()
        .unwrap();

    let mut num_loops = 0;

    //Brute force, just try changing every non-obstacle into an obstacle and
    //count how many result in loops
    for y in 0..grid.len() {
        for x in 0..grid[y].len() {
            if grid[y][x] != '.' {
                continue;
            };

            // pretend there's an obstacle here
            grid[y][x] = '#';

            let mut guard = GuardPosition {
                x: start_x,
                y: start_y,
                orientation: start_orientation,
            };

            let mut guard_positions: HashSet<GuardPosition> = HashSet::new();

            let mut in_a_loop = true;
            while !guard_positions.contains(&guard) {
                guard_positions.insert(guard.clone());

                if let Some(g) = next_guard_position(&grid, &guard) {
                    guard = g;
                } else {
                    in_a_loop = false;
                    break;
                };
            }

            if in_a_loop {
                num_loops += 1;
            }

            // remove the added obstacle
            grid[y][x] = '.';
        }
    }

    num_loops
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
....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...\n";
        let (_d, _f, test_path) = create_test_file(&test_input);
        let result = part1(&test_path);
        assert_eq!(result, 41);
    }

    #[test]
    fn test_part2() {
        let test_input = "\
....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...\n";
        let (_d, _f, test_path) = create_test_file(&test_input);
        let result = part2(&test_path);
        assert_eq!(result, 6);
    }
}
