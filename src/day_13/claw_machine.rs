use crate::util::point_2d::Point2d;

use std::string::ToString;

#[derive(Debug, PartialEq)]
pub struct ClawMachine {
    a_button: Point2d<i64>,
    b_button: Point2d<i64>,
    prize_location: Point2d<i64>,
}

impl ClawMachine {
    pub fn adjust_prize_location(&mut self, adjustment: i64) {
        self.prize_location += Point2d::new(adjustment, adjustment);
    }

    pub fn cost_to_get_prize(&self, a_button_cost: i64, b_button_cost: i64) -> Option<i64> {
        self.number_of_button_presses_to_prize()
            .map(|(a, b)| (a * a_button_cost + b * b_button_cost))
    }

    fn number_of_button_presses_to_prize(&self) -> Option<(i64, i64)> {
        let b_presses_numerator =
            self.a_button.x * self.prize_location.y - self.a_button.y * self.prize_location.x;
        let b_presses_denominator =
            self.a_button.x * self.b_button.y - self.a_button.y * self.b_button.x;

        let can_press_b = b_presses_numerator % b_presses_denominator == 0;

        let b_presses = b_presses_numerator / b_presses_denominator;

        let a_presses_numerator = self.prize_location.x - self.b_button.x * b_presses;
        let a_presses_denominator = self.a_button.x;

        let can_press_a = a_presses_numerator % a_presses_denominator == 0;

        let a_presses = a_presses_numerator / a_presses_denominator;

        if can_press_a && can_press_b {
            Some((a_presses, b_presses))
        } else {
            None
        }
    }
}

impl<const N: usize> From<[&str; N]> for ClawMachine {
    fn from(input: [&str; N]) -> Self {
        Self::from(
            &input
                .into_iter()
                .map(ToString::to_string)
                .collect::<Vec<String>>(),
        )
    }
}

impl From<&Vec<String>> for ClawMachine {
    fn from(input: &Vec<String>) -> Self {
        Self::from(input.as_slice())
    }
}

impl From<&[String]> for ClawMachine {
    fn from(input: &[String]) -> Self {
        let data: Vec<Point2d<i64>> = input
            .iter()
            .filter_map(|line| line.split_once(": "))
            .filter_map(|(_, rhs)| rhs.split_once(", "))
            .map(|(x_def, y_def)| (x_def.split_at(2).1, y_def.split_at(2).1))
            .map(|(x_str, y_str)| Point2d::new(x_str.parse().unwrap(), y_str.parse().unwrap()))
            .collect();

        ClawMachine {
            a_button: *data.first().unwrap(),
            b_button: *data.get(1).unwrap(),
            prize_location: *data.get(2).unwrap(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_claw_machine_from_str_array() {
        let expected = ClawMachine {
            a_button: Point2d::new(94, 34),
            b_button: Point2d::new(22, 67),
            prize_location: Point2d::new(8_400, 5_400),
        };

        let result = ClawMachine::from([
            "Button A: X+94, Y+34",
            "Button B: X+22, Y+67",
            "Prize: X=8400, Y=5400",
        ]);

        assert_eq!(result, expected);
    }

    #[test]
    fn test_number_of_button_presses_to_prize() {
        let machine_1 = ClawMachine::from([
            "Button A: X+94, Y+34",
            "Button B: X+22, Y+67",
            "Prize: X=8400, Y=5400",
        ]);
        let machine_2 = ClawMachine::from([
            "Button A: X+26, Y+66",
            "Button B: X+67, Y+21",
            "Prize: X=12748, Y=12176",
        ]);
        let machine_3 = ClawMachine::from([
            "Button A: X+17, Y+86",
            "Button B: X+84, Y+37",
            "Prize: X=7870, Y=6450",
        ]);
        let machine_4 = ClawMachine::from([
            "Button A: X+69, Y+23",
            "Button B: X+27, Y+71",
            "Prize: X=18641, Y=10279",
        ]);

        assert_eq!(
            machine_1.number_of_button_presses_to_prize(),
            Some((80, 40))
        );
        assert_eq!(machine_2.number_of_button_presses_to_prize(), None);
        assert_eq!(
            machine_3.number_of_button_presses_to_prize(),
            Some((38, 86))
        );
        assert_eq!(machine_4.number_of_button_presses_to_prize(), None);
    }
}
