/*
I think this claw machine game can be solved with a dynamic programming approach
where we make a 100 by 100 matrix since that's the maximum number of times
each button can be pressed.

The x-axis is the number of times the A button is pressed
The y-axis is the number of times the B button is pressed
The matrix entries are the current X and Y positions of the claw
so for example in this game:

    Button A: X+1, Y+1
    Button B: X+2, Y+4
    Prize: X=5, Y=9

We start with a matrix of zeros:
  B
3 |(0,0) (0,0) (0,0) (0,0)
2 |(0,0) (0,0) (0,0) (0,0)
1 |(0,0) (0,0) (0,0) (0,0)
0 |(0,0) (0,0) (0,0) (0,0)
  ------------------------- A
   0     1     2     3


Then since it costs 3 tokens to press A and 1 token to press B,
we'll start by trying to do a "majority-B" approach and start filling
out the upper triangle of the table

  B
3 |(0,0) (0,0) (0,0) (0,0)
2 |(4,8) (0,0) (0,0) (0,0)
1 |(2,4) (0,0) (0,0) (0,0)
0 |(0,0) (0,0) (0,0) (0,0)
  ------------------------- A
   0     1     2     3

At  (A=0, B=2) if we fill in (A=0, B=3) then we'll get (6,12) which is too big, so we instead
try (A=1, B=2) which gives us the prize

  B
3 |(0,0) (0,0) (0,0) (0,0)
2 |(4,8) (5,9) (0,0) (0,0)
1 |(2,4) (0,0) (0,0) (0,0)
0 |(0,0) (0,0) (0,0) (0,0)
  ------------------------- A
   0     1     2     3

And since we've always tried to push button B rather than A, we know this is the least
expensive option.

I'm not sure if this is really dynamic programming or not.
Feels like it since it's a 2D matrix that we're filling in
*/
use regex::Regex;
use std::path::Path;

const MAX_PRESSES: usize = 100;

#[derive(Debug)]
struct Game {
    a_button: (i64, i64),
    b_button: (i64, i64),
    prize: (i64, i64),
}

impl Game {
    fn new(a_button: (i64, i64), b_button: (i64, i64), prize: (i64, i64)) -> Game {
        Game {
            a_button: a_button,
            b_button: b_button,
            prize: prize,
        }
    }

    fn numeric_solve(&mut self) -> Option<(usize, usize)> {
        let b_press_numer = self.a_button.0 * self.prize.1 - self.prize.0 * self.a_button.1;
        let b_press_denom = self.a_button.0 * self.b_button.1 - self.b_button.0 * self.a_button.1;

        // Special case to handle where a_button and b_button are "on the same line"
        // which would cause the b_press_denom to be 0 and cause an issue, for example
        // (1,2) and (4,8) in which case we'll check a different way
        //
        // ok, they didn't include any of these cases :)
        if b_press_denom == 0 {
            println!("THEY WERE MEAN!! {:?},{:?}", self.a_button, self.b_button);
        }

        if b_press_numer % b_press_denom != 0 {
            None
        } else {
            let b_presses = b_press_numer / b_press_denom;

            let a_press_numer = self.prize.0 - b_presses * self.b_button.0;
            let a_press_denom = self.a_button.0;

            if a_press_numer % a_press_denom != 0 {
                None
            } else {
                let a_presses = a_press_numer / a_press_denom;
                Some((a_presses as usize, b_presses as usize))
            }
        }
    }

    fn dp_solve(&mut self) -> Option<(usize, usize)> {
        let mut presses = vec![(0, 0)];
        let mut moves = [[(0, 0); MAX_PRESSES + 1]; MAX_PRESSES + 1];
        let mut visited = [[false; MAX_PRESSES + 1]; MAX_PRESSES + 1];

        while let Some((a, b)) = presses.pop() {
            // Found the prize, return the num of A and B presses
            if moves[a][b] == self.prize {
                return Some((a, b));
            }

            visited[a][b] = true;

            let (x, y) = moves[a][b];

            // Calculate the position if using one more B press
            if a < MAX_PRESSES {
                let new_x = x + self.a_button.0;
                let new_y = y + self.a_button.1;
                let not_too_big = new_x <= self.prize.0 && new_y <= self.prize.1;

                if not_too_big && !visited[a + 1][b] {
                    moves[a + 1][b] = (new_x, new_y);
                    presses.push((a + 1, b));
                }
            }

            // Calculate the position if using one more B press
            // NOTE, doing this second so it's at the top of the stack
            //       because b presses are cheaper
            if b < MAX_PRESSES {
                let new_x = x + self.b_button.0;
                let new_y = y + self.b_button.1;
                let not_too_big = new_x <= self.prize.0 && new_y <= self.prize.1;

                if not_too_big && !visited[a][b + 1] {
                    moves[a][b + 1] = (new_x, new_y);
                    presses.push((a, b + 1));
                }
            }
        }
        None
    }
}

fn parse_xy(s: &str) -> (i64, i64) {
    let xy_re = Regex::new(r"X.(\d+).*Y.(\d+)").unwrap();
    let m = &xy_re.captures_iter(s).next().unwrap();
    let x = m[1].parse().unwrap();
    let y = m[2].parse().unwrap();
    (x, y)
}

pub fn part1(data_path: &Path) -> u64 {
    let text = std::fs::read_to_string(data_path).unwrap();
    let lines: Vec<&str> = text.lines().collect();
    let mut games: Vec<_> = lines
        .chunks(4)
        .map(|s| Game::new(parse_xy(s[0]), parse_xy(s[1]), parse_xy(s[2])))
        .collect();

    let button_presses: Vec<_> = games.iter_mut().filter_map(|g| g.dp_solve()).collect();
    let cost: usize = button_presses.iter().map(|(a, b)| a * 3 + b).sum();
    cost.try_into().unwrap()
}

pub fn part2(data_path: &Path) -> u64 {
    let text = std::fs::read_to_string(data_path).unwrap();
    let lines: Vec<&str> = text.lines().collect();
    let mut games: Vec<_> = lines
        .chunks(4)
        .map(|s| Game::new(parse_xy(s[0]), parse_xy(s[1]), parse_xy(s[2])))
        .collect();

    // Add the 10000000000000 offsets
    let offset = 10000000000000;
    for g in &mut games {
        g.prize = (g.prize.0 + offset, g.prize.1 + offset);
    }

    let button_presses: Vec<_> = games.iter_mut().filter_map(|g| g.numeric_solve()).collect();
    let cost: usize = button_presses.iter().map(|(a, b)| a * 3 + b).sum();
    cost.try_into().unwrap()
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
Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400

Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=12748, Y=12176

Button A: X+17, Y+86
Button B: X+84, Y+37
Prize: X=7870, Y=6450

Button A: X+69, Y+23
Button B: X+27, Y+71
Prize: X=18641, Y=10279\n";
        let (_d, _f, test_path) = create_test_file(&test_input);
        let result = part1(&test_path);
        assert_eq!(result, 480);
    }

    #[test]
    fn test_part2() {
        let test_input = "\
Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400

Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=12748, Y=12176

Button A: X+17, Y+86
Button B: X+84, Y+37
Prize: X=7870, Y=6450

Button A: X+69, Y+23
Button B: X+27, Y+71
Prize: X=18641, Y=10279\n";
        let (_d, _f, test_path) = create_test_file(&test_input);
        let result = part2(&test_path);
        assert_eq!(result, 875318608908);
    }
}
