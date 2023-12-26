use regex::{Match, Regex};
use std::ops::Range;

#[allow(dead_code)]
fn part1(input: &str) -> u64 {
    let re = Regex::new(r"\d+").unwrap();
    let parse_number = |m: Match| m.as_str().parse::<u64>().unwrap();

    let seeds: Vec<u64> = re
        .find_iter(input.lines().next().unwrap())
        .map(parse_number)
        .collect();

    let mut maps: Vec<Vec<(Range<u64>, i64)>> = Vec::new();

    for line in input.lines().skip(1) {
        if line.contains(':') {
            maps.push(Vec::new());
        } else {
            let numbers = re.find_iter(line).map(parse_number).collect::<Vec<_>>();

            if numbers.len() == 3 {
                let (dst, src, n) = (numbers[0], numbers[1], numbers[2]);
                maps.last_mut()
                    .unwrap()
                    .push((src..src + n + 1, dst as i64 - src as i64));
            }
        }
    }

    let locations: Vec<u64> = seeds
        .iter()
        .map(|&seed| {
            maps.iter().fold(seed, |acc, vec| {
                vec.iter()
                    .find(|(range, _)| range.contains(&acc))
                    .map_or(acc, |&(_, offset)| (acc as i64 + offset) as u64)
            })
        })
        .collect();

    *locations.iter().min().unwrap()
}

// #[allow(dead_code)]
// fn part2(input: &str) -> u64 {
// }

#[allow(dead_code)]
pub fn puzzle() {
    let input = crate::input::load_input(5);
//     let input = "seeds: 79 14 55 13

// seed-to-soil map:
// 50 98 2
// 52 50 48

// soil-to-fertilizer map:
// 0 15 37
// 37 52 2
// 39 0 15

// fertilizer-to-water map:
// 49 53 8
// 0 11 42
// 42 0 7
// 57 7 4

// water-to-light map:
// 88 18 7
// 18 25 70

// light-to-temperature map:
// 45 77 23
// 81 45 19
// 68 64 13

// temperature-to-humidity map:
// 0 69 1
// 1 0 69

// humidity-to-location map:
// 60 56 37
// 56 93 4";
    println!("part 1 = {}", part1(&input));
    // println!("part 2 = {}", part2(input));
}
