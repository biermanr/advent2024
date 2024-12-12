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

fn count_fence_segments(plots: &HashSet<(usize, usize)>) -> u32 {
    let mut total_fence_segments = 0;

    for &(x, y) in plots {
        let mut shared_fences = 0;

        if x >= 1 && plots.contains(&(x - 1, y)) {
            shared_fences += 1;
        }
        if y >= 1 && plots.contains(&(x, y - 1)) {
            shared_fences += 1;
        }
        if plots.contains(&(x + 1, y)) {
            shared_fences += 1;
        }
        if plots.contains(&(x, y + 1)) {
            shared_fences += 1;
        }

        total_fence_segments += 4 - shared_fences;
    }

    total_fence_segments
}

fn count_fences(plots: HashSet<(usize, usize)>, width: usize, height: usize) -> u32 {
    // a fence segment is either horizontal or vertical
    // For example, the 4 fences below could be described by the below
    //
    // B B B
    //  +-+
    // B|A|B
    //  +-+
    // B B B
    //
    // would be (1,1,V), (2,1,V), (1,1,H), (1,2,H)
    //
    // can keep the vertical and horizontal fences separate
    // keep track of which plot each fence belongs to to try and solve the inner-fence issue

    let mut vert_fences: Vec<(usize, usize, (usize,usize))> = Vec::new();
    let mut horz_fences: Vec<(usize, usize, (usize,usize))> = Vec::new();

    for &(x, y) in &plots {
        // if at the left-side of the map or the plot to the left is a diff plant
        // then there will be a LEFT horizontal fence segment, similar for UP fence
        if x == 0 || !plots.contains(&(x - 1, y)) {
            vert_fences.push((x, y,(x,y)));
        }

        if y == 0 || !plots.contains(&(x, y - 1)) {
            horz_fences.push((x, y,(x,y)));
        }

        // check RIGHT and DOWN
        if x == width - 1 || !plots.contains(&(x + 1, y)) {
            vert_fences.push((x + 1, y,(x,y)));
        }
        if y == height - 1 || !plots.contains(&(x, y + 1)) {
            horz_fences.push((x, y + 1,(x,y)));
        }
    }

    // sort the fence segments and try to combine them
    vert_fences.sort_by(|a, b| a.0.cmp(&b.0).then(a.1.cmp(&b.1)));

    let mut num_vert_fences = 1;
    let (mut pfx, mut pfy, _) = vert_fences[0]; //take the first fence
    for (fx, fy, (x,y)) in &vert_fences[1..] {
        // checking if the previous fence was the same x and one y above
        // and making sure that the plots are connected for the internal fence issue
        let connected = *fx == pfx && *fy == pfy+1 && *x > 0 && plots.contains(&(*x-1, *y));
        if !connected {
            num_vert_fences += 1;
        }
        pfx = *x;
        pfy = *y;
    }

    // sort the fence segments and try to combine them
    horz_fences.sort_by(|a, b| a.1.cmp(&b.1).then(a.0.cmp(&b.0)));

    let mut num_horz_fences = 1;
    let (mut pfx, mut pfy, _) = horz_fences[0]; //take the first fence
    for (fx, fy, (x,y)) in &horz_fences[1..] {
        // not part of the same fence
        let connected = *fx == pfx+1 && *fy == pfy && *y > 0 && plots.contains(&(*x, *y-1));
        if !connected {
            num_horz_fences += 1;
        }
        pfx = *x;
        pfy = *y;
    }

    num_vert_fences + num_horz_fences
}

pub fn part1(data_path: &Path) -> u32 {
    let text = std::fs::read_to_string(data_path).unwrap();

    let mut grid: Vec<Vec<char>> = text.lines().map(|l| l.chars().collect()).collect();

    let mut score = 0;

    // Look through the whole grid to find the regions and score themk
    for y in 0..grid.len() {
        for x in 0..grid[y].len() {
            // Plot already part of region, skip
            if grid[y][x] == '.' {
                continue;
            }

            let plots = discover_region(&mut grid, (x, y));
            let fence_segments = count_fence_segments(&plots);
            score += fence_segments*(plots.len() as u32)
        }
    }

    score
}

pub fn part2(data_path: &Path) -> u32 {
    let text = std::fs::read_to_string(data_path).unwrap();

    let mut grid: Vec<Vec<char>> = text.lines().map(|l| l.chars().collect()).collect();
    let height = grid.len();
    let width = grid[0].len();

    let mut score = 0;

    // Look through the whole grid to find the regions and score them
    for y in 0..grid.len() {
        for x in 0..grid[y].len() {
            // Plot already part of region, skip
            if grid[y][x] == '.' {
                continue;
            }

            let plots = discover_region(&mut grid, (x, y));
            let num_plots = plots.len() as u32;
            let num_fences = count_fences(plots, width, height);
            score += num_fences * num_plots;
        }
    }

    score
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
