mod robots;

use crate::util::file_reader::to_string_vector;

use robots::Robots;

pub fn run() {
    let input = to_string_vector("inputs/day_14.txt").expect("Something went wrong with Day 14!");

    let robots = Robots::new(&input, 101, 103);

    println!("Day 14 Part 1: {:?}", part_1(&robots));
    println!("Day 14 Part 2: {:?}", part_2(&robots));
}

fn part_1(robots: &Robots) -> usize {
    robots.safety_factor_after(100)
}

fn part_2(robots: &Robots) -> i32 {
    let seconds = robots.seconds_until_max_line_length();

    let mut clone = robots.clone();

    clone.simulate(seconds);

    println!("{clone}");

    seconds
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let input = to_string_vector("test_inputs/day_14.txt").unwrap();

        let robots = Robots::new(&input, 11, 7);

        assert_eq!(part_1(&robots), 12);
    }
}
