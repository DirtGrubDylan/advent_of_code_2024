use std::str::FromStr;

#[derive(Debug, PartialEq, Clone)]
pub struct SecretNumber {
    value: isize,
}

impl SecretNumber {
    pub fn delta_and_buy_values(&self, first_n_values: usize) -> Vec<(isize, isize)> {
        self.clone()
            .take(first_n_values)
            .collect::<Vec<isize>>()
            .windows(2)
            .map(|window| (window.first().unwrap() % 10, window.get(1).unwrap() % 10))
            .map(|(x, y)| (y - x, y))
            .collect()
    }

    fn get_next_value(&self) -> isize {
        let mut next_value = self.value;

        // Multiply by 64 and XOR with current secret value
        next_value ^= self.value << 6;

        // Modulo with 16777216 (2^24 meaning keep only the last 24 bits)
        next_value %= 16_777_216;

        // Divide by 32 and XOR result with current secret value
        //
        // Note: Since the modulo, the number is only 19 bits long. So no need to mod again here.
        next_value ^= next_value >> 5;

        // Multiply by 2048 and XOR result with current secret value
        next_value ^= next_value << 11;

        // Modulo with 16777216 (2^24 meaning keep only the last 24 bits)
        next_value % 16_777_216
    }
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct SecretNumberParseError;

impl FromStr for SecretNumber {
    type Err = SecretNumberParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        s.parse()
            .map(|value| SecretNumber { value })
            .map_err(|_| SecretNumberParseError)
    }
}

impl From<isize> for SecretNumber {
    fn from(value: isize) -> Self {
        SecretNumber { value }
    }
}

impl Iterator for SecretNumber {
    type Item = isize;

    fn next(&mut self) -> Option<Self::Item> {
        let result = Some(self.value);

        self.value = self.get_next_value();

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_iterator_first_11() {
        let secret_number = SecretNumber::from(123);

        let expected = vec![
            123, 15_887_950, 16_495_136, 527_345, 704_524, 1_553_684, 12_683_156, 11_100_544,
            12_249_484, 7_753_432, 5_908_254,
        ];

        let result: Vec<isize> = secret_number.into_iter().take(11).collect();

        assert_eq!(result, expected);
    }

    #[test]
    fn test_iterator_2000th_step() {
        let secret_number = SecretNumber::from(1);

        assert_eq!(secret_number.into_iter().nth(2_000), Some(8_685_429));
    }

    #[test]
    fn test_delta_and_buy_values() {
        let secret_number = SecretNumber::from(123);

        let expected = vec![
            (-3, 0),
            (6, 6),
            (-1, 5),
            (-1, 4),
            (0, 4),
            (2, 6),
            (-2, 4),
            (0, 4),
            (-2, 2),
        ];

        let result = secret_number.delta_and_buy_values(10);

        assert_eq!(result, expected);
    }
}
