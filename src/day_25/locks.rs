#[derive(Debug, PartialEq, Clone)]
struct Lock {
    pin_heights: Vec<i32>,
}

impl Lock {
    fn input_is_valid(input: &[String]) -> bool {
        input
            .first()
            .unwrap_or(&String::from("empty"))
            .chars()
            .all(|c| c == '#')
    }
}

impl<const N: usize> From<[&str; N]> for Lock {
    fn from(input: [&str; N]) -> Self {
        let strings: Vec<String> = input.iter().map(ToString::to_string).collect();

        Self::from(strings.as_slice())
    }
}

impl From<&[String]> for Lock {
    fn from(input: &[String]) -> Self {
        let mut pin_heights = vec![0; 5];

        for line in input {
            for (index, _) in line.chars().enumerate().filter(|(_, c)| *c == '#') {
                if let Some(height_so_far) = pin_heights.get_mut(index) {
                    *height_so_far += 1;
                }
            }
        }

        Lock { pin_heights }
    }
}

#[derive(Debug, PartialEq, Clone)]
struct Key {
    heights: Vec<i32>,
}

impl Key {
    fn input_is_valid(input: &[String]) -> bool {
        input
            .last()
            .unwrap_or(&String::from("empty"))
            .chars()
            .all(|c| c == '#')
    }
}

impl<const N: usize> From<[&str; N]> for Key {
    fn from(input: [&str; N]) -> Self {
        let strings: Vec<String> = input.iter().map(ToString::to_string).collect();

        Self::from(strings.as_slice())
    }
}

impl From<&[String]> for Key {
    fn from(input: &[String]) -> Self {
        let mut heights = vec![0; 5];

        for line in input {
            for (index, _) in line.chars().enumerate().filter(|(_, c)| *c == '#') {
                if let Some(height_so_far) = heights.get_mut(index) {
                    *height_so_far += 1;
                }
            }
        }

        Key { heights }
    }
}

#[derive(Debug, PartialEq, Clone)]
struct System {
    locks: Vec<Lock>,
    keys: Vec<Key>,
}

impl<const N: usize> From<[&str; N]> for System {
    fn from(input: [&str; N]) -> Self {
        let strings: Vec<String> = input.iter().map(ToString::to_string).collect();

        Self::from(strings.as_slice())
    }
}

impl From<&[String]> for System {
    fn from(input: &[String]) -> Self {
        let mut locks = Vec::new();
        let mut keys = Vec::new();

        for schematic in input.split(String::is_empty) {
            if Key::input_is_valid(schematic) {
                keys.push(Key::from(schematic));
            } else if Lock::input_is_valid(schematic) {
                locks.push(Lock::from(schematic));
            } else {
                panic!("{schematic:?} is not valid for `Key` or `Lock`!");
            }
        }

        System { locks, keys }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lock_input_is_valid() {
        assert!(Lock::input_is_valid(&[
            String::from("#####"),
            String::from("#.###"),
            String::from("#.###"),
            String::from("#.#.#"),
            String::from("..#.#"),
            String::from("....."),
            String::from("....."),
        ]));

        assert!(!Lock::input_is_valid(&[
            String::from("....."),
            String::from("#...."),
            String::from("##..."),
            String::from("##.#."),
            String::from("##.#."),
            String::from("##.#."),
            String::from("#####"),
        ]));

        assert!(!Lock::input_is_valid(&[]));
    }

    #[test]
    fn test_lock_from_str_array() {
        let expected = Lock {
            pin_heights: vec![4, 1, 5, 3, 5],
        };

        let result = Lock::from([
            "#####", "#.###", "#.###", "#.#.#", "..#.#", ".....", ".....",
        ]);

        assert_eq!(result, expected);
    }

    #[test]
    fn test_key_input_is_valid() {
        assert!(Key::input_is_valid(&[
            String::from("....."),
            String::from("#...."),
            String::from("##..."),
            String::from("##.#."),
            String::from("##.#."),
            String::from("##.#."),
            String::from("#####"),
        ]));

        assert!(!Key::input_is_valid(&[
            String::from("#####"),
            String::from("#.###"),
            String::from("#.###"),
            String::from("#.#.#"),
            String::from("..#.#"),
            String::from("....."),
            String::from("....."),
        ]));

        assert!(!Key::input_is_valid(&[]));
    }

    #[test]
    fn test_key_from_str_array() {
        let expected = Key {
            heights: vec![6, 5, 1, 4, 1],
        };

        let result = Key::from([
            ".....", "#....", "##...", "##.#.", "##.#.", "##.#.", "#####",
        ]);

        assert_eq!(result, expected);
    }

    #[test]
    fn test_system_from_str_array() {
        let expected_locks = vec![
            Lock {
                pin_heights: vec![1, 6, 4, 5, 4],
            },
            Lock {
                pin_heights: vec![2, 3, 1, 6, 4],
            },
        ];

        let expected_keys = vec![
            Key {
                heights: vec![6, 1, 3, 2, 4],
            },
            Key {
                heights: vec![5, 4, 5, 1, 3],
            },
            Key {
                heights: vec![4, 1, 3, 1, 2],
            },
        ];

        let expected = System {
            locks: expected_locks,
            keys: expected_keys,
        };

        let result = System::from([
            "#####", ".####", ".####", ".####", ".#.#.", ".#...", ".....", "", "#####", "##.##",
            ".#.##", "...##", "...#.", "...#.", ".....", "", ".....", "#....", "#....", "#...#",
            "#.#.#", "#.###", "#####", "", ".....", ".....", "#.#..", "###..", "###.#", "###.#",
            "#####", "", ".....", ".....", ".....", "#....", "#.#..", "#.#.#", "#####",
        ]);

        assert_eq!(result, expected);
    }
}
