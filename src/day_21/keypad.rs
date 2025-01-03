use std::collections::HashMap;
use std::fmt;

#[derive(Debug, Default, PartialEq, Eq, Copy, Clone, Hash)]
enum DirectionPadButton {
    #[default]
    A,
    Up,
    Right,
    Down,
    Left,
}

impl DirectionPadButton {
    fn all_pairs() -> Vec<(Self, Self)> {
        vec![
            (Self::A, Self::A),
            (Self::A, Self::Up),
            (Self::A, Self::Right),
            (Self::A, Self::Down),
            (Self::A, Self::Left),
            (Self::Up, Self::A),
            (Self::Up, Self::Up),
            (Self::Up, Self::Right),
            (Self::Up, Self::Down),
            (Self::Up, Self::Left),
            (Self::Right, Self::A),
            (Self::Right, Self::Up),
            (Self::Right, Self::Right),
            (Self::Right, Self::Down),
            (Self::Right, Self::Left),
            (Self::Down, Self::A),
            (Self::Down, Self::Up),
            (Self::Down, Self::Right),
            (Self::Down, Self::Down),
            (Self::Down, Self::Left),
            (Self::Left, Self::A),
            (Self::Left, Self::Up),
            (Self::Left, Self::Right),
            (Self::Left, Self::Down),
            (Self::Left, Self::Left),
        ]
    }

    fn presses_to_traverse_and_press(self, to: Self) -> Vec<Self> {
        let mut presses = match (self, to) {
            (Self::A, Self::A)
            | (Self::Up, Self::Up)
            | (Self::Right, Self::Right)
            | (Self::Down, Self::Down)
            | (Self::Left, Self::Left) => Vec::new(),
            (Self::A, Self::Up) | (Self::Right, Self::Down) | (Self::Down, Self::Left) => {
                vec![Self::Left]
            }
            (Self::A, Self::Right) | (Self::Up, Self::Down) => {
                vec![Self::Down]
            }
            (Self::Up, Self::A) | (Self::Down, Self::Right) | (Self::Left, Self::Down) => {
                vec![Self::Right]
            }
            (Self::Right, Self::A) | (Self::Down, Self::Up) => {
                vec![Self::Up]
            }
            (Self::A, Self::Down) | (Self::Up, Self::Left) => {
                vec![Self::Down, Self::Left]
            }
            (Self::Down, Self::A) | (Self::Left, Self::Up) => {
                vec![Self::Right, Self::Up]
            }
            (Self::A, Self::Left) => {
                vec![Self::Down, Self::Left, Self::Left]
            }
            (Self::Up, Self::Right) => {
                vec![Self::Down, Self::Right]
            }
            (Self::Right, Self::Up) => {
                vec![Self::Up, Self::Left]
            }
            (Self::Right, Self::Left) => {
                vec![Self::Left, Self::Left]
            }
            (Self::Left, Self::A) => {
                vec![Self::Right, Self::Right, Self::Up]
            }
            (Self::Left, Self::Right) => {
                vec![Self::Right, Self::Right]
            }
        };

        presses.push(Self::A);

        presses
    }
}

impl fmt::Display for DirectionPadButton {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let as_string = match self {
            Self::A => String::from('A'),
            Self::Up => String::from('^'),
            Self::Right => String::from('>'),
            Self::Down => String::from('V'),
            Self::Left => String::from('<'),
        };

        write!(f, "{as_string}")
    }
}

impl From<char> for DirectionPadButton {
    fn from(input: char) -> Self {
        match input {
            'A' => Self::A,
            '^' => Self::Up,
            '>' => Self::Right,
            'V' | 'v' => Self::Down,
            '<' => Self::Left,
            _ => panic!("Cannot convert '{input}' to `DirectionPadButton`!"),
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
struct DirectionPad {
    press_costs_from_controller: HashMap<(DirectionPadButton, DirectionPadButton), u64>,
}

impl DirectionPad {
    fn lowest_pad_of(number_of_pads: usize) -> Self {
        let mut current_pad = DirectionPad::default();

        for _ in 1..number_of_pads {
            current_pad = DirectionPad::from(&current_pad);
        }

        current_pad
    }

    fn cost_to_press(&self, buttons_to_press: &[DirectionPadButton]) -> u64 {
        let mut result = 0;
        let mut current_button = DirectionPadButton::A;

        for next_button in buttons_to_press {
            let cost_to_traverse_and_press = self
                .press_costs_from_controller
                .get(&(current_button, *next_button))
                .copied()
                .unwrap();

            result += cost_to_traverse_and_press;
            current_button = *next_button;
        }

        result
    }
}

impl Default for DirectionPad {
    fn default() -> Self {
        let press_costs_from_controller = DirectionPadButton::all_pairs()
            .into_iter()
            .map(|pair| (pair, 1))
            .collect();

        DirectionPad {
            press_costs_from_controller,
        }
    }
}

impl From<&DirectionPad> for DirectionPad {
    fn from(controller: &Self) -> Self {
        let mut press_costs_from_controller = HashMap::new();

        for (from, to_press) in DirectionPadButton::all_pairs() {
            let cost_from_controller =
                controller.cost_to_press(&from.presses_to_traverse_and_press(to_press));

            press_costs_from_controller.insert((from, to_press), cost_from_controller);
        }

        DirectionPad {
            press_costs_from_controller,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cost_to_press_with_default_costs() {
        let buttons_to_press = [
            DirectionPadButton::Down,
            DirectionPadButton::Left,
            DirectionPadButton::A,
        ];

        let user_dpad = DirectionPad::default();

        assert_eq!(user_dpad.cost_to_press(&buttons_to_press), 3);
    }

    #[test]
    fn test_cost_to_press_with_single_nested_costs() {
        let buttons_to_press = [
            DirectionPadButton::Down,
            DirectionPadButton::Left,
            DirectionPadButton::Left,
            DirectionPadButton::A,
        ];

        let nested_dpad = DirectionPad::lowest_pad_of(2);

        assert_eq!(nested_dpad.cost_to_press(&buttons_to_press), 10);
    }

    #[test]
    fn test_cost_to_press_with_double_nested_costs() {
        let buttons_to_press = [DirectionPadButton::Left, DirectionPadButton::A];

        let double_nested_dpad = DirectionPad::lowest_pad_of(3);

        assert_eq!(double_nested_dpad.cost_to_press(&buttons_to_press), 18);
    }

    #[test]
    fn test_cost_to_press_with_double_nested_costs_from_029A_example() {
        let buttons_to_press: Vec<DirectionPadButton> = "<A^A>^^AvvvA"
            .chars()
            .map(DirectionPadButton::from)
            .collect();

        let double_nested_dpad = DirectionPad::lowest_pad_of(3);

        assert_eq!(double_nested_dpad.cost_to_press(&buttons_to_press), 68);
    }
}
