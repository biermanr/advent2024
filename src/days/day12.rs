use std::collections::HashSet;
use std::path::Path;

fn next_steps(grid: &Vec<Vec<char>>, pos: (usize, usize)) -> Vec<(usize, usize)> {
    let (x, y) = pos;
    let plant = grid[y][x];
    let mut valid_steps = Vec::new();

    //try stepping in all four cardinal directions to see if the plot is of the same plant type
    if x >= 1 && grid[y][x - 1] == plant {
        valid_steps.push((x - 1, y))
    }
    if x + 1 <= grid[0].len() - 1 && grid[y][x + 1] == plant {
        valid_steps.push((x + 1, y))
    }
    if y >= 1 && grid[y - 1][x] == plant {
        valid_steps.push((x, y - 1))
    }
    if y + 1 <= grid.len() - 1 && grid[y + 1][x] == plant {
        valid_steps.push((x, y + 1))
    }

    valid_steps
}

fn discover_region(grid: &mut Vec<Vec<char>>, pos: (usize, usize)) -> HashSet<(usize, usize)> {
    let mut visited = HashSet::new();
    let mut to_visit = vec![pos];

    while let Some((x, y)) = to_visit.pop() {
        // This plot was already visited, skip
        if grid[y][x] == '.' {
            continue;
        }

        // Add the next steps to the stack
        to_visit.extend(next_steps(&grid, (x, y)));

        // Mark this plot as visited
        visited.insert((x, y));
        grid[y][x] = '.';
    }

    visited
}

fn get_fence_segments(
    plots: &HashSet<(usize, usize)>,
) -> (Vec<((i32, i32), (i32, i32))>, Vec<((i32, i32), (i32, i32))>) {
    let mut vert_fence_segments = Vec::new();
    let mut horz_fence_segments = Vec::new();

    for &(x, y) in plots {
        let lt_same_plant = x >= 1 && plots.contains(&(x - 1, y));
        let rt_same_plant = plots.contains(&(x + 1, y));
        let up_same_plant = y >= 1 && plots.contains(&(x, y - 1));
        let dn_same_plant = plots.contains(&(x, y + 1));

        let x: i32 = x as i32;
        let y: i32 = y as i32;

        if !lt_same_plant {
            vert_fence_segments.push(((x, y), (x - 1, y)));
        }

        if !rt_same_plant {
            vert_fence_segments.push(((x, y), (x + 1, y)));
        }

        if !up_same_plant {
            horz_fence_segments.push(((x, y), (x, y - 1)));
        }

        if !dn_same_plant {
            horz_fence_segments.push(((x, y), (x, y + 1)));
        }
    }

    (vert_fence_segments, horz_fence_segments)
}

fn count_fences(fence_segments: &mut Vec<((i32, i32), (i32, i32))>, vertical: bool) -> usize {
    let mut num_fences = 0;
    while let Some(start_segment) = fence_segments.pop() {
        num_fences += 1;

        // Try and consume all the connected fence pieces to the start_segment
        // slow with how I'm using remove, could likely make "fence numbers" instead
        let mut segments_to_check = vec![start_segment];
        while let Some(((x1, y1), (x2, y2))) = segments_to_check.pop() {
            if vertical {
                let up_segment = ((x1, y1 - 1), (x2, y2 - 1));
                let dw_segment = ((x1, y1 + 1), (x2, y2 + 1));

                if let Some(ind) = fence_segments.iter().position(|&x| x == up_segment) {
                    segments_to_check.push(up_segment);
                    fence_segments.remove(ind);
                }

                if let Some(ind) = fence_segments.iter().position(|&x| x == dw_segment) {
                    segments_to_check.push(dw_segment);
                    fence_segments.remove(ind);
                }
            } else {
                let lt_segment = ((x1 - 1, y1), (x2 - 1, y2));
                let rt_segment = ((x1 + 1, y1), (x2 + 1, y2));

                if let Some(ind) = fence_segments.iter().position(|&x| x == lt_segment) {
                    segments_to_check.push(lt_segment);
                    fence_segments.remove(ind);
                }

                if let Some(ind) = fence_segments.iter().position(|&x| x == rt_segment) {
                    segments_to_check.push(rt_segment);
                    fence_segments.remove(ind);
                }
            }
        }
    }
    num_fences
}

pub fn part1(data_path: &Path) -> u32 {
    let text = std::fs::read_to_string(data_path).unwrap();

    let mut grid: Vec<Vec<char>> = text.lines().map(|l| l.chars().collect()).collect();

    let mut score = 0;

    // Look through the whole grid to find the regions and score them
    for y in 0..grid.len() {
        for x in 0..grid[y].len() {
            // Plot already part of region, skip
            if grid[y][x] == '.' {
                continue;
            }

            let plots = discover_region(&mut grid, (x, y));
            let (vert_fence_segments, horz_fence_segments) = get_fence_segments(&plots);
            score += (vert_fence_segments.len() + horz_fence_segments.len()) * plots.len()
        }
    }

    score.try_into().unwrap()
}

pub fn part2(data_path: &Path) -> u32 {
    let text = std::fs::read_to_string(data_path).unwrap();

    let orig_grid: Vec<Vec<char>> = text.lines().map(|l| l.chars().collect()).collect();
    let mut grid = orig_grid.clone();

    let mut score = 0;

    // Look through the whole grid to find the regions and score them
    for y in 0..grid.len() {
        for x in 0..grid[y].len() {
            // Plot already part of region, skip
            if grid[y][x] == '.' {
                continue;
            }

            let plots = discover_region(&mut grid, (x, y));
            let (mut vert_fence_segments, mut horz_fence_segments) = get_fence_segments(&plots);

            let num_vert_fences = count_fences(&mut vert_fence_segments, true);
            let num_horz_fences = count_fences(&mut horz_fence_segments, false);
            score += (num_vert_fences + num_horz_fences) * plots.len();
        }
    }

    score.try_into().unwrap()
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

    #[test]
    fn test_part2_example1() {
        let test_input = "\
AAAA
BBCD
BBCC
EEEC\n";
        let (_d, _f, test_path) = create_test_file(&test_input);
        let result = part2(&test_path);
        assert_eq!(result, 80);
    }

    #[test]
    fn test_part2_example2() {
        let test_input = "\
EEEEE
EXXXX
EEEEE
EXXXX
EEEEE\n";
        let (_d, _f, test_path) = create_test_file(&test_input);
        let result = part2(&test_path);
        assert_eq!(result, 236);
    }

    #[test]
    fn test_part2_example3() {
        let test_input = "\
OOOOO
OXOXO
OOOOO
OXOXO
OOOOO\n";
        let (_d, _f, test_path) = create_test_file(&test_input);
        let result = part2(&test_path);
        assert_eq!(result, 436);
    }

    #[test]
    fn test_part2_example4() {
        let test_input = "\
AAAAAA
AAABBA
AAABBA
ABBAAA
ABBAAA
AAAAAA\n";
        let (_d, _f, test_path) = create_test_file(&test_input);
        let result = part2(&test_path);
        assert_eq!(result, 368);
    }
}
