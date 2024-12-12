use std::collections::HashSet;
use std::path::Path;

//struct Region {
//    plant: char,
//    plots: HashSet<(usize, usize)>,
//}

fn walk_region(mut grid: Vec<Vec<char>>, pos: (usize, usize)) -> Vec<((usize, usize), usize)> {
    let (x, y) = pos;
    let plant = grid[y][x];

    let mut num_fences = 4;

    //Mark this position as visited and add it to the list
    //assuming there is no plant type named '.'
    grid[y][x] = '.';
    let mut valid_steps = Vec::new();

    //Check all four directions for the same plant
    if x >= 1 {
        if grid[y][x - 1] == plant {
            valid_steps.push((x - 1, y));
            num_fences -= 1;
        }
        if grid[y][x - 1] == '.' {
            num_fences -= 1;
        }
    }
    if x + 1 <= grid[0].len() - 1 {
        if grid[y][x + 1] == plant {
            valid_steps.push((x + 1, y));
            num_fences -= 1;
        }
        if grid[y][x + 1] == '.' {
            num_fences -= 1;
        }
    }
    if y >= 1 {
        if grid[y - 1][x] == plant {
            valid_steps.push((x, y - 1));
            num_fences -= 1;
        }
        if grid[y - 1][x] == '.' {
            num_fences -= 1;
        }
    }
    if y + 1 <= grid.len() - 1 {
        if grid[y + 1][x] == plant {
            valid_steps.push((x, y + 1));
            num_fences -= 1;
        }
        if grid[y + 1][x] == '.' {
            num_fences -= 1;
        }
    }

    let mut future_fences: Vec<((usize, usize), usize)> = valid_steps
        .iter()
        .flat_map(|s| walk_region(grid.clone(), *s))
        .collect();

    future_fences.push((pos, num_fences));
    future_fences
}

pub fn part1(data_path: &Path) -> u32 {
    let text = std::fs::read_to_string(data_path).unwrap();

    // Read in the antenna grid
    let grid: Vec<Vec<char>> = text.lines().map(|l| l.chars().collect()).collect();

    let mut visited_plots: HashSet<(usize, usize)> = HashSet::new();
    let mut total_score: usize = 0;

    for y in 0..grid.len() {
        for x in 0..grid[y].len() {
            if visited_plots.contains(&(x, y)) {
                continue;
            }

            let mut region = walk_region(grid.clone(), (x, y));

            // deduplicate region because my walking is bad
            region.sort();
            region.dedup();

            let score = region.iter().map(|(_, f)| f).sum::<usize>() * region.len();
            total_score += score;

            //println!("WORKING ON PLOT AT {:?}, type {:?}, with score {:?} with len {:?}",(x,y), grid[y][x], score, region.len());
            //println!("{:?}",region);

            let region_plots: HashSet<(usize, usize)> =
                region.iter().map(|((x, y), _)| (*x, *y)).collect();
            visited_plots.extend(region_plots);
        }
    }

    total_score.try_into().unwrap()
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
AAAA
BBCD
BBCC
EEEC\n";
        let (_d, _f, test_path) = create_test_file(&test_input);
        let result = part1(&test_path);
        assert_eq!(result, 140);
    }
}
