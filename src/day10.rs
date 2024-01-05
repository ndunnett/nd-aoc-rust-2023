// use itertools::Itertools;
use std::collections::HashMap;

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, Ord, PartialOrd)]
enum Pipe {
    NS,
    EW,
    NE,
    NW,
    SW,
    SE,
    Any,
}

impl Pipe {
    fn from_char(c: char) -> Option<Pipe> {
        match c {
            '|' => Some(Pipe::NS),
            '-' => Some(Pipe::EW),
            'L' => Some(Pipe::NE),
            'J' => Some(Pipe::NW),
            '7' => Some(Pipe::SW),
            'F' => Some(Pipe::SE),
            'S' => Some(Pipe::Any),
            _ => None,
        }
    }

    fn to_char(self) -> char {
        match self {
            Pipe::NS => '║',
            Pipe::EW => '═',
            Pipe::NE => '╚',
            Pipe::NW => '╝',
            Pipe::SW => '╗',
            Pipe::SE => '╔',
            Pipe::Any => '╬',
        }
    }

    // fn neighbours(self) -> Vec<Point> {
    //     match self {
    //         Pipe::NS => vec![Point { x: 0, y: -1 }, Point { x: 0, y: 1 }],
    //         Pipe::EW => ,
    //         Pipe::NE => ,
    //         Pipe::NW => ,
    //         Pipe::SW => ,
    //         Pipe::SE => ,
    //         Pipe::Any => ,
    //     }
    // }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, Ord, PartialOrd)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn new(x: i32, y: i32) -> Point {
        Point { x, y }
    }

    fn from_usize(x: usize, y: usize) -> Point {
        Point::new(x as i32, y as i32)
    }
}

#[derive(Debug, Clone)]
struct Map {
    pipes: HashMap<Point, Pipe>,
    start: Point,
}

impl Map {
    fn parse(input: &str) -> Map {
        let mut start = Point::new(0, 0);

        let pipes = input
            .lines()
            .enumerate()
            .fold(HashMap::new(), |mut pipes, (y, line)| {
                for (x, c) in line.chars().enumerate() {
                    if let Some(p) = Pipe::from_char(c) {
                        if p == Pipe::Any {
                            start = Point::from_usize(x, y);
                        }

                        pipes.insert(Point::from_usize(x, y), p);
                    }
                }

                pipes
            });

        Map { start, pipes }
    }

    // fn is_connected(&self, a: &Point, b: &Point) -> bool {
    //     match a {}
    // }

    // fn reduce(&mut self) {
    //     let mut current = self.start;
    // }
}

fn part1(input: &str) -> u32 {
    let map = Map::parse(input);
    let h = input.lines().count();
    let w = input.lines().next().unwrap().len();

    println!("\n{}\n", input);

    for y in 0..h {
        let line = (0..w)
            .map(|x| {
                if let Some(p) = map.pipes.get(&Point { x, y }) {
                    p.to_char()
                } else {
                    ' '
                }
            })
            .collect::<String>();

        println!("{}", line);
    }

    4
}

fn part2(input: &str) -> u32 {
    let _ = input;
    0
}

#[allow(dead_code)]
pub fn puzzle() {
    let input = crate::input::load_input(10);
    println!("part 1 = {}", part1(&input));
    println!("part 2 = {}", part2(&input));
}

#[cfg(test)]
mod test {
    use super::{part1, part2};

    const INPUT1: &str = "-L|F7
7S-7|
L|7||
-L-J|
L|-JF";

    const INPUT2: &str = "..F7.
.FJ|.
SJ.L7
|F--J
LJ...";

    #[test]
    fn test_part1() {
        assert_eq!(part1(INPUT1), 4);
        assert_eq!(part1(INPUT2), 8);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(INPUT1), 0);
    }
}
