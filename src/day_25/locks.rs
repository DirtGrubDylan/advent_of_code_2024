#[derive(Debug, PartialEq, Clone)]
struct Lock {
    pin_heights: Vec<i32>,
}

impl Lock {
    fn key_has_overlap(&self, key: &Key) -> bool {
        key.heights
            .iter()
            .zip(self.pin_heights.iter())
            .any(|(key_height, pin_height)| *key_height > (5 - *pin_height))
    }

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
        let mut pin_heights = vec![-1; 5];

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
        let mut heights = vec![-1; 5];

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
pub struct System {
    locks: Vec<Lock>,
    keys: Vec<Key>,
}

impl System {
    pub fn number_of_keys_that_fit_without_overlap(&self) -> usize {
        let mut result = 0;

        for key in &self.keys {
            for lock in &self.locks {
                if !lock.key_has_overlap(key) {
                    result += 1;
                }
            }
        }

        result
    }
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
            pin_heights: vec![3, 0, 4, 2, 4],
        };

        let result = Lock::from([
            "#####", "#.###", "#.###", "#.#.#", "..#.#", ".....", ".....",
        ]);

        assert_eq!(result, expected);
    }

    #[test]
    fn test_lock_key_has_overlap() {
        let lock = Lock::from([
            "#####", ".####", ".####", ".####", ".#.#.", ".#...", ".....",
        ]);

        let key_1 = Key::from([
            ".....", "#....", "#....", "#...#", "#.#.#", "#.###", "#####",
        ]);
        let key_2 = Key::from([
            ".....", ".....", "#.#..", "###..", "###.#", "###.#", "#####",
        ]);
        let key_3 = Key::from([
            ".....", ".....", ".....", "#....", "#.#..", "#.#.#", "#####",
        ]);

        assert!(lock.key_has_overlap(&key_1));
        assert!(lock.key_has_overlap(&key_2));
        assert!(!lock.key_has_overlap(&key_3));
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
            heights: vec![5, 4, 0, 3, 0],
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
                pin_heights: vec![0, 5, 3, 4, 3],
            },
            Lock {
                pin_heights: vec![1, 2, 0, 5, 3],
            },
        ];

        let expected_keys = vec![
            Key {
                heights: vec![5, 0, 2, 1, 3],
            },
            Key {
                heights: vec![4, 3, 4, 0, 2],
            },
            Key {
                heights: vec![3, 0, 2, 0, 1],
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

    #[test]
    fn test_number_of_keys_that_fit_without_overlap() {
        let system = System::from([
            "#####", ".####", ".####", ".####", ".#.#.", ".#...", ".....", "", "#####", "##.##",
            ".#.##", "...##", "...#.", "...#.", ".....", "", ".....", "#....", "#....", "#...#",
            "#.#.#", "#.###", "#####", "", ".....", ".....", "#.#..", "###..", "###.#", "###.#",
            "#####", "", ".....", ".....", ".....", "#....", "#.#..", "#.#.#", "#####",
        ]);

        assert_eq!(system.number_of_keys_that_fit_without_overlap(), 3);
    }
}
