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
use std::path::Path;


struct Game {
    xa: u32,
    ya: u32,
    xb: u32,
    yb: u32,
    xp: u32,
    yp: u32,
    moves: [[(u32,u32); 100]; 100],
}

impl Game {
    fn new(a_button_line: &str, b_button_line: &str, prize_line: &str) -> Game {
        Game {
            xa: 0,
            ya: 0,
            xb: 0,
            yb: 0,
            xp: 0,
            yp: 0,
            moves: [[(0,0); 100]; 100],
        }
    }
}

pub fn part1(data_path: &Path) -> u32 {

    let text = std::fs::read_to_string(data_path).unwrap();
    for (i,line) in text.lines().enumerate() {
        match i%4 {
            0 => println!("Button A: {:?}",line),
            1 => println!("Button B: {:?}",line),
            2 => println!("Prize: {:?}",line),
            3 => println!("EMPTY: {:?}",line),
            _ => unreachable!(),
        }
    }

    let text = std::fs::read_to_string(data_path).unwrap();
    let lines: Vec<&str> = text.lines().collect();
    let q: Vec<_> = lines.chunks(4).map(|s| (s[0],s[1],s[2])).collect();
    println!("{:?}",q);

    let g = Game::new("woo","hoo","boo");
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
}