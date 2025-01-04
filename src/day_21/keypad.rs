use std::collections::HashMap;
use std::fmt;

#[derive(Debug, Default, PartialEq, Eq, Clone, Hash)]
struct SearchNode {
    col: usize,
    row: usize,
    buttons_to_press: Vec<DirectionPadButton>,
}

impl SearchNode {
    fn new(col: usize, row: usize) -> Self {
        SearchNode {
            col,
            row,
            buttons_to_press: Vec::new(),
        }
    }

    #[must_use]
    fn moves_towards(self, end_col: usize, end_row: usize) -> Vec<Self> {
        let mut moves = Vec::new();

        if self.col < end_col {
            moves.push(self.move_from(DirectionPadButton::Right));
        }

        if self.row < end_row {
            moves.push(self.move_from(DirectionPadButton::Up));
        }

        if end_row < self.row {
            moves.push(self.move_from(DirectionPadButton::Down));
        }

        if end_col < self.col {
            moves.push(self.move_from(DirectionPadButton::Left));
        }

        moves
    }

    fn move_from(&self, added_button: DirectionPadButton) -> Self {
        let mut new_buttons_to_press = self.buttons_to_press.clone();

        let (new_col, new_row) = match added_button {
            DirectionPadButton::A => (self.col, self.row),
            DirectionPadButton::Up => (self.col, self.row + 1),
            DirectionPadButton::Right => (self.col + 1, self.row),
            DirectionPadButton::Down => (self.col, self.row - 1),
            DirectionPadButton::Left => (self.col - 1, self.row),
        };

        new_buttons_to_press.push(added_button);

        SearchNode {
            col: new_col,
            row: new_row,
            buttons_to_press: new_buttons_to_press,
        }
    }

    fn is_at(&self, col: usize, row: usize) -> bool {
        (self.col, self.row) == (col, row)
    }
}

#[derive(Debug, Default, PartialEq, Eq, Copy, Clone, Hash)]
pub enum DirectionPadButton {
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

    fn possible_presses_to_traverse_and_press(self, to: Self) -> Vec<Vec<Self>> {
        Self::possible_presses_to_traverse_and_press_from_usizes(
            usize::from(self),
            usize::from(to),
            (0, 1),
        )
    }

    fn possible_presses_to_traverse_and_press_from_usizes(
        from: usize,
        to: usize,
        avoid: (usize, usize),
    ) -> Vec<Vec<Self>> {
        let mut result = Vec::new();

        let (start_row, start_col) = (from / 3, from % 3);
        let (end_row, end_col) = (to / 3, to % 3);

        let mut search_nodes = vec![SearchNode::new(start_col, start_row)];

        while let Some(node) = search_nodes.pop() {
            if node.is_at(end_col, end_row) {
                let mut presses = node.buttons_to_press.clone();

                presses.push(DirectionPadButton::A);

                result.push(presses);
            }

            if !node.is_at(avoid.0, avoid.1) {
                search_nodes.extend(node.moves_towards(end_col, end_row));
            }
        }

        result
    }
}

impl fmt::Display for DirectionPadButton {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let as_string = match self {
            Self::A => String::from('A'),
            Self::Up => String::from('^'),
            Self::Right => String::from('>'),
            Self::Down => String::from('v'),
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

impl From<DirectionPadButton> for usize {
    fn from(input: DirectionPadButton) -> Self {
        match input {
            DirectionPadButton::Left => 0,
            DirectionPadButton::Down => 1,
            DirectionPadButton::Right => 2,
            DirectionPadButton::Up => 4,
            DirectionPadButton::A => 5,
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct DirectionPad {
    press_costs_from_controller: HashMap<(DirectionPadButton, DirectionPadButton), u64>,
}

impl DirectionPad {
    pub fn lowest_pad_of(number_of_pads: usize) -> Self {
        let mut current_pad = DirectionPad::default();

        for _ in 1..number_of_pads {
            current_pad = DirectionPad::from(&current_pad);
        }

        current_pad
    }

    pub fn cost_to_press(&self, buttons_to_press: &[DirectionPadButton]) -> u64 {
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
            let minimum_cost_from_controller = from
                .possible_presses_to_traverse_and_press(to_press)
                .iter()
                .map(|presses| controller.cost_to_press(presses))
                .min()
                .unwrap();

            // let cost_from_controller =
            //     controller.cost_to_press(&from.presses_to_traverse_and_press(to_press));

            press_costs_from_controller.insert((from, to_press), minimum_cost_from_controller);
        }

        DirectionPad {
            press_costs_from_controller,
        }
    }
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum NumberPadButton {
    A,
    Zero,
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
}

impl NumberPadButton {
    pub fn possible_presses_to_traverse_and_press(self, to: Self) -> Vec<Vec<DirectionPadButton>> {
        DirectionPadButton::possible_presses_to_traverse_and_press_from_usizes(
            usize::from(self),
            usize::from(to),
            (0, 0),
        )
    }
}

impl fmt::Display for NumberPadButton {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let as_string = match self {
            Self::Zero => String::from('1'),
            Self::A => String::from('2'),
            Self::One => String::from('3'),
            Self::Two => String::from('4'),
            Self::Three => String::from('5'),
            Self::Four => String::from('6'),
            Self::Five => String::from('7'),
            Self::Six => String::from('8'),
            Self::Seven => String::from('9'),
            Self::Eight => String::from("10"),
            Self::Nine => String::from("11"),
        };

        write!(f, "{as_string}")
    }
}

impl From<char> for NumberPadButton {
    fn from(input: char) -> Self {
        match input {
            'A' => Self::A,
            '0' => Self::Zero,
            '1' => Self::One,
            '2' => Self::Two,
            '3' => Self::Three,
            '4' => Self::Four,
            '5' => Self::Five,
            '6' => Self::Six,
            '7' => Self::Seven,
            '8' => Self::Eight,
            '9' => Self::Nine,
            _ => panic!("Cannot convert '{input}' to `NumberPadButton`!"),
        }
    }
}

impl From<NumberPadButton> for usize {
    fn from(input: NumberPadButton) -> Self {
        match input {
            NumberPadButton::Zero => 1,
            NumberPadButton::A => 2,
            NumberPadButton::One => 3,
            NumberPadButton::Two => 4,
            NumberPadButton::Three => 5,
            NumberPadButton::Four => 6,
            NumberPadButton::Five => 7,
            NumberPadButton::Six => 8,
            NumberPadButton::Seven => 9,
            NumberPadButton::Eight => 10,
            NumberPadButton::Nine => 11,
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
    fn test_cost_to_press_with_double_nested_costs_from_029a_example() {
        let buttons_to_press: Vec<DirectionPadButton> = "<A^A>^^AvvvA"
            .chars()
            .map(DirectionPadButton::from)
            .collect();

        let double_nested_dpad = DirectionPad::lowest_pad_of(3);

        assert_eq!(double_nested_dpad.cost_to_press(&buttons_to_press), 68);
    }

    #[test]
    fn test_direction_pad_button_possible_presses_to_traverse_and_press_a_to_left() {
        let expected = vec![
            vec![
                DirectionPadButton::Left,
                DirectionPadButton::Down,
                DirectionPadButton::Left,
                DirectionPadButton::A,
            ],
            vec![
                DirectionPadButton::Down,
                DirectionPadButton::Left,
                DirectionPadButton::Left,
                DirectionPadButton::A,
            ],
        ];

        let result =
            DirectionPadButton::A.possible_presses_to_traverse_and_press(DirectionPadButton::Left);

        assert_eq!(result, expected);
    }

    #[test]
    fn test_direction_pad_button_possible_presses_to_traverse_and_press_left_to_a() {
        let expected = vec![
            vec![
                DirectionPadButton::Right,
                DirectionPadButton::Up,
                DirectionPadButton::Right,
                DirectionPadButton::A,
            ],
            vec![
                DirectionPadButton::Right,
                DirectionPadButton::Right,
                DirectionPadButton::Up,
                DirectionPadButton::A,
            ],
        ];

        let result =
            DirectionPadButton::Left.possible_presses_to_traverse_and_press(DirectionPadButton::A);

        assert_eq!(result, expected);
    }

    #[test]
    fn test_number_pad_button_possible_presses_to_traverse_and_press_7_to_a() {
        let expected = vec![
            vec![
                DirectionPadButton::Down,
                DirectionPadButton::Down,
                DirectionPadButton::Right,
                DirectionPadButton::Down,
                DirectionPadButton::Right,
                DirectionPadButton::A,
            ],
            vec![
                DirectionPadButton::Down,
                DirectionPadButton::Down,
                DirectionPadButton::Right,
                DirectionPadButton::Right,
                DirectionPadButton::Down,
                DirectionPadButton::A,
            ],
            vec![
                DirectionPadButton::Down,
                DirectionPadButton::Right,
                DirectionPadButton::Down,
                DirectionPadButton::Down,
                DirectionPadButton::Right,
                DirectionPadButton::A,
            ],
            vec![
                DirectionPadButton::Down,
                DirectionPadButton::Right,
                DirectionPadButton::Down,
                DirectionPadButton::Right,
                DirectionPadButton::Down,
                DirectionPadButton::A,
            ],
            vec![
                DirectionPadButton::Down,
                DirectionPadButton::Right,
                DirectionPadButton::Right,
                DirectionPadButton::Down,
                DirectionPadButton::Down,
                DirectionPadButton::A,
            ],
            vec![
                DirectionPadButton::Right,
                DirectionPadButton::Down,
                DirectionPadButton::Down,
                DirectionPadButton::Down,
                DirectionPadButton::Right,
                DirectionPadButton::A,
            ],
            vec![
                DirectionPadButton::Right,
                DirectionPadButton::Down,
                DirectionPadButton::Down,
                DirectionPadButton::Right,
                DirectionPadButton::Down,
                DirectionPadButton::A,
            ],
            vec![
                DirectionPadButton::Right,
                DirectionPadButton::Down,
                DirectionPadButton::Right,
                DirectionPadButton::Down,
                DirectionPadButton::Down,
                DirectionPadButton::A,
            ],
            vec![
                DirectionPadButton::Right,
                DirectionPadButton::Right,
                DirectionPadButton::Down,
                DirectionPadButton::Down,
                DirectionPadButton::Down,
                DirectionPadButton::A,
            ],
        ];

        let result =
            NumberPadButton::Seven.possible_presses_to_traverse_and_press(NumberPadButton::A);

        assert_eq!(result, expected);
    }

    #[test]
    fn test_number_pad_button_possible_presses_to_traverse_and_press() {
        let numbers = [
            NumberPadButton::Zero,
            NumberPadButton::Two,
            NumberPadButton::Nine,
            NumberPadButton::A,
        ];

        let expected = vec![
            vec![vec![DirectionPadButton::Left, DirectionPadButton::A]],
            vec![vec![DirectionPadButton::Up, DirectionPadButton::A]],
            vec![
                vec![
                    DirectionPadButton::Up,
                    DirectionPadButton::Up,
                    DirectionPadButton::Right,
                    DirectionPadButton::A,
                ],
                vec![
                    DirectionPadButton::Up,
                    DirectionPadButton::Right,
                    DirectionPadButton::Up,
                    DirectionPadButton::A,
                ],
                vec![
                    DirectionPadButton::Right,
                    DirectionPadButton::Up,
                    DirectionPadButton::Up,
                    DirectionPadButton::A,
                ],
            ],
            vec![vec![
                DirectionPadButton::Down,
                DirectionPadButton::Down,
                DirectionPadButton::Down,
                DirectionPadButton::A,
            ]],
        ];

        let mut result = Vec::new();
        let mut current_number = NumberPadButton::A;

        for number in numbers {
            result.push(current_number.possible_presses_to_traverse_and_press(number));

            current_number = number;
        }

        assert_eq!(result, expected);
    }
}
