use std::fmt;
use std::str::FromStr;

use crate::util::point_2d::Point2d;

#[derive(Debug, Default, PartialEq, Clone, Copy)]
struct Robot {
    location: Point2d<i32>,
    velocity: Point2d<i32>,
}

impl Robot {
    fn simulate(&mut self, seconds: i32, max_x: i32, max_y: i32) {
        self.location.x = (self.location.x + self.velocity.x * seconds).rem_euclid(max_x);
        self.location.y = (self.location.y + self.velocity.y * seconds).rem_euclid(max_y);
    }
}

#[derive(Debug, PartialEq)]
struct RobotParseError {}

impl FromStr for Robot {
    type Err = RobotParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (lhs, rhs) = s.split_once(' ').unwrap();

        let location = lhs
            .replace("p=", "")
            .split_once(',')
            .map(|(a, b)| Point2d::new(a.parse().unwrap(), b.parse().unwrap()))
            .unwrap();
        let velocity = rhs
            .replace("v=", "")
            .split_once(',')
            .map(|(a, b)| Point2d::new(a.parse().unwrap(), b.parse().unwrap()))
            .unwrap();

        Ok(Robot { location, velocity })
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct Robots {
    data: Vec<Robot>,
    x_size: i32,
    y_size: i32,
}

impl Robots {
    pub fn new(input: &[String], x_size: i32, y_size: i32) -> Self {
        Robots {
            data: input
                .iter()
                .filter_map(|line| Robot::from_str(line).ok())
                .collect(),
            x_size,
            y_size,
        }
    }

    pub fn safety_factor_after(&self, seconds: i32) -> usize {
        let mut clone = self.clone();

        clone.simulate(seconds);

        clone.quadrants().iter().map(Vec::len).product()
    }

    fn simulate(&mut self, seconds: i32) {
        for robot in &mut self.data {
            robot.simulate(seconds, self.x_size, self.y_size);
        }
    }

    fn quadrants(&self) -> Vec<Vec<Robot>> {
        let mut first_quadrant = Vec::new();
        let mut second_quadrant = Vec::new();
        let mut third_quadrant = Vec::new();
        let mut fourth_quadrant = Vec::new();

        let middle_col = self.x_size / 2;
        let middle_row = self.y_size / 2;

        for robot in &self.data {
            if (robot.location.x < middle_col) && (robot.location.y < middle_row) {
                first_quadrant.push(*robot);
            } else if (robot.location.x > middle_col) && (robot.location.y < middle_row) {
                second_quadrant.push(*robot);
            } else if (robot.location.x < middle_col) && (robot.location.y > middle_row) {
                third_quadrant.push(*robot);
            } else if (robot.location.x > middle_col) && (robot.location.y > middle_row) {
                fourth_quadrant.push(*robot);
            }
        }

        vec![
            first_quadrant,
            second_quadrant,
            third_quadrant,
            fourth_quadrant,
        ]
    }
}

impl fmt::Display for Robots {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut map = vec![
            vec![0; usize::try_from(self.x_size).unwrap()];
            usize::try_from(self.y_size).unwrap()
        ];

        for robot in &self.data {
            let map_value = map
                .get_mut(usize::try_from(robot.location.y).unwrap())
                .and_then(|row| row.get_mut(usize::try_from(robot.location.x).unwrap()));

            if let Some(value) = map_value {
                *value += 1;
            }
        }

        let map_str = map
            .iter()
            .map(|row| {
                row.iter()
                    .map(|&value| {
                        if value == 0 {
                            String::from(".")
                        } else {
                            value.to_string()
                        }
                    })
                    .collect::<String>()
            })
            .collect::<Vec<String>>()
            .join("\n");

        write!(f, "{map_str}")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_robot_fromstr() {
        let expected = Robot {
            location: Point2d::new(0, 4),
            velocity: Point2d::new(3, -3),
        };

        let result = Robot::from_str("p=0,4 v=3,-3");

        assert_eq!(result, Ok(expected));
    }

    #[test]
    fn test_robot_simulate() {
        let (max_x, max_y) = (11, 7);
        let mut robot = Robot::from_str("p=2,4 v=2,-3").unwrap();

        let expected_location = Point2d::new(1, 3);

        robot.simulate(5, max_x, max_y);

        assert_eq!(robot.location, expected_location);
    }

    #[test]
    fn test_robots_quadrants() {
        let input = [
            String::from("p=0,4 v=3,-3"),
            String::from("p=6,3 v=-1,-3"),
            String::from("p=10,3 v=-1,2"),
            String::from("p=2,0 v=2,-1"),
            String::from("p=0,0 v=1,3"),
            String::from("p=3,0 v=-2,-2"),
            String::from("p=7,6 v=-1,-3"),
            String::from("p=3,0 v=-1,-2"),
            String::from("p=9,3 v=2,3"),
            String::from("p=7,3 v=-1,2"),
            String::from("p=2,4 v=2,-3"),
            String::from("p=9,5 v=-3,-3"),
        ];

        let robots = Robots::new(&input, 11, 7);

        let expected_first_quadrant = vec![
            Robot::from_str("p=2,0 v=2,-1").unwrap(),
            Robot::from_str("p=0,0 v=1,3").unwrap(),
            Robot::from_str("p=3,0 v=-2,-2").unwrap(),
            Robot::from_str("p=3,0 v=-1,-2").unwrap(),
        ];
        let expected_second_quadrant = Vec::new();
        let expected_third_quadrant = vec![
            Robot::from_str("p=0,4 v=3,-3").unwrap(),
            Robot::from_str("p=2,4 v=2,-3").unwrap(),
        ];
        let expected_fourth_quadrant = vec![
            Robot::from_str("p=7,6 v=-1,-3").unwrap(),
            Robot::from_str("p=9,5 v=-3,-3").unwrap(),
        ];

        let expected = vec![
            expected_first_quadrant,
            expected_second_quadrant,
            expected_third_quadrant,
            expected_fourth_quadrant,
        ];

        let result = robots.quadrants();

        assert_eq!(result, expected);
    }

    #[test]
    fn test_robots_safety_factor_after() {
        let input = [
            String::from("p=0,4 v=3,-3"),
            String::from("p=6,3 v=-1,-3"),
            String::from("p=10,3 v=-1,2"),
            String::from("p=2,0 v=2,-1"),
            String::from("p=0,0 v=1,3"),
            String::from("p=3,0 v=-2,-2"),
            String::from("p=7,6 v=-1,-3"),
            String::from("p=3,0 v=-1,-2"),
            String::from("p=9,3 v=2,3"),
            String::from("p=7,3 v=-1,2"),
            String::from("p=2,4 v=2,-3"),
            String::from("p=9,5 v=-3,-3"),
        ];

        let robots = Robots::new(&input, 11, 7);

        assert_eq!(robots.safety_factor_after(100), 12);
    }
}
