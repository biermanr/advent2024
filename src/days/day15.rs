use std::path::Path;

#[allow(dead_code)]
fn print_grid(grid: &Vec<Vec<char>>) {
    for y in 0..grid.len() {
        let row: String = grid[y].iter().collect();
        println!("{:?}", row);
    }
    println!("");
}

fn get_robot_loc(grid: &Vec<Vec<char>>) -> Option<(i32, i32)> {
    for y in 0..grid.len() {
        for x in 0..grid[y].len() {
            if grid[y][x] == '@' {
                return Some((x.try_into().unwrap(), y.try_into().unwrap()));
            }
        }
    }
    None
}

fn score_warehouse(grid: &Vec<Vec<char>>) -> u32 {
    let mut score = 0;
    for y in 0..grid.len() {
        for x in 0..grid[y].len() {
            if grid[y][x] == 'O' {
                score += x + 100 * y;
            }
        }
    }
    score.try_into().unwrap()
}
fn make_move(grid: &mut Vec<Vec<char>>, pos: (i32, i32), m: char) -> (i32, i32) {
    // Determine the direction of the attempted move
    let (dx, dy) = match m {
        '>' => (1, 0),
        'v' => (0, 1),
        '<' => (-1, 0),
        '^' => (0, -1),
        _ => unreachable!(),
    };

    // Look in the direction of the move until
    // either a '.' or a '#' are found
    // '.' means the move is possible, '#' means not
    // there can be boxes in the way, in which case the
    // first free square gets swapped with the directly
    // adjacent square to the robot, then robot moves
    //
    // don't have to check out bounds because the warehouse
    // is enclosed in walls

    // all this type conversion is very ugly
    let (x, y) = pos;
    let orig_x = x as usize;
    let orig_y = y as usize;
    let adj_x = (x + dx) as usize;
    let adj_y = (y + dy) as usize;
    let mut px = adj_x;
    let mut py = adj_y;

    while grid[py][px] == 'O' {
        px = ((px as i32) + dx) as usize;
        py = ((py as i32) + dy) as usize;
    }

    match grid[py][px] {
        '.' => {
            // shift boxes and robot, and return new position
            grid[py][px] = grid[adj_y][adj_x];
            grid[adj_y][adj_x] = '@';
            grid[orig_y][orig_x] = '.';
            (((adj_x) as i32), ((adj_y) as i32))
        }
        '#' => (orig_x as i32, orig_y as i32), //return the original position
        _ => unreachable!(),
    }
}

pub fn part1(data_path: &Path) -> u32 {
    let text = std::fs::read_to_string(data_path).unwrap();

    // Read in the warehouse grid
    let mut grid: Vec<Vec<char>> = text
        .lines()
        .take_while(|l| l.len() > 0)
        .map(|l| l.chars().collect())
        .collect();

    // Read in the moves
    let moves = text
        .lines()
        .skip_while(|l| l.len() > 0)
        .collect::<Vec<&str>>()
        .join("");

    // Find where the robot starts
    let mut pos = get_robot_loc(&grid).unwrap();

    // Perform all the moves one at a time
    for m in moves.chars() {
        //print_grid(&grid);
        pos = make_move(&mut grid, pos, m);
    }

    // Score the warehouse
    score_warehouse(&grid)
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
    fn test_part1_small_example() {
        let test_input = "\
########
#..O.O.#
##@.O..#
#...O..#
#.#.O..#
#...O..#
#......#
########

<^^>>>vv<v>>v<<\n";
        let (_d, _f, test_path) = create_test_file(&test_input);
        let result = part1(&test_path);
        assert_eq!(result, 2028);
    }
}
