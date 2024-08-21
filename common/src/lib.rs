use rand::{self, distributions::Distribution, seq::SliceRandom, Rng};
use std::{char, error::Error};

#[derive(Debug, PartialEq, Eq)]
pub struct RandomStringGenerator {
    pub count: u32,
    pub length: u32,
    pub include_special_chars: bool,
    pub include_numbers: bool,
    pub include_uppercase: bool,
}

impl Default for RandomStringGenerator {
    fn default() -> Self {
        RandomStringGenerator {
            count: 5,
            length: 32,
            include_numbers: true,
            include_special_chars: true,
            include_uppercase: true,
        }
    }
}

impl Distribution<char> for RandomStringGenerator {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> char {
        let char_set = Vec::from_iter(char::from(0)..=char::from(127));
        let char_set = char_set
            .into_iter()
            .filter(|character| {
                !character.is_whitespace()
                    && (self.include_numbers || !character.is_numeric())
                    && (self.include_uppercase || !character.is_ascii_uppercase())
                    && (self.include_special_chars || character.is_alphanumeric())
            })
            .collect::<Vec<char>>();

        *char_set.choose(rng).unwrap()
    }
}

impl RandomStringGenerator {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn generate(&self) -> Result<Vec<String>, Box<dyn Error>> {
        if let 0 = self.count {
            return Err("Number of strings cannot be 0".into());
        }

        if let 0 = self.length {
            return Err("Length of the string cannot be 0".into());
        }

        let mut rng = rand::thread_rng();
        let mut strings = Vec::new();
        for _ in 0..self.count {
            let string = self
                .sample_iter(&mut rng)
                .take(self.length.try_into()?)
                .map(char::from)
                .collect();

            strings.push(string);
        }

        Ok(strings)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default() {
        let string_generator = RandomStringGenerator::new();
        assert_eq!(
            RandomStringGenerator {
                count: 5,
                length: 32,
                include_numbers: true,
                include_special_chars: true,
                include_uppercase: true
            },
            string_generator
        );
    }

    #[test]
    fn count() {
        let mut string_generator = RandomStringGenerator::new();

        let mut result = string_generator.generate();
        assert_eq!(5, result.unwrap().len());

        string_generator.count = 10;
        result = string_generator.generate();
        assert_eq!(10, result.unwrap().len());

        string_generator.count = 0;
        result = string_generator.generate();
        assert_eq!(
            "Number of strings cannot be 0",
            result.unwrap_err().to_string()
        );
    }

    #[test]
    fn length() {
        let mut string_generator = RandomStringGenerator::new();

        let mut result = string_generator.generate();
        result
            .unwrap()
            .iter()
            .for_each(|string| assert_eq!(32, string.len()));

        string_generator.length = 10;
        result = string_generator.generate();
        result
            .unwrap()
            .iter()
            .for_each(|string| assert_eq!(10, string.len()));

        string_generator.length = 0;
        result = string_generator.generate();
        assert_eq!(
            "Length of the string cannot be 0",
            result.unwrap_err().to_string()
        );
    }

    fn setup() -> (Vec<String>, RandomStringGenerator) {
        let mut string_generator = RandomStringGenerator::new();
        string_generator.count = 100;
        string_generator.length = 100;

        (string_generator.generate().unwrap(), string_generator)
    }

    #[test]
    fn numbers() {
        let (mut strings, mut string_generator) = setup();

        assert_eq!(
            true,
            strings
                .iter()
                .any(|string| string.chars().any(|char| char.is_numeric()))
        );

        string_generator.include_numbers = false;
        strings = string_generator.generate().unwrap();
        assert_eq!(
            true,
            strings
                .iter()
                .all(|string| string.chars().all(|char| !char.is_numeric()))
        );
    }

    #[test]
    fn special_chars() {
        let (mut strings, mut string_generator) = setup();

        assert_eq!(
            true,
            strings
                .iter()
                .any(|string| string.chars().any(|char| !char.is_alphanumeric()))
        );

        string_generator.include_special_chars = false;
        strings = string_generator.generate().unwrap();
        assert_eq!(
            true,
            strings
                .iter()
                .all(|string| string.chars().all(|char| char.is_alphanumeric()))
        );
    }

    #[test]
    fn uppercase() {
        let (mut strings, mut string_generator) = setup();

        assert_eq!(
            true,
            strings
                .iter()
                .any(|string| string.chars().any(|char| char.is_ascii_uppercase()))
        );

        string_generator.include_uppercase = false;
        strings = string_generator.generate().unwrap();
        assert_eq!(
            true,
            strings
                .iter()
                .all(|string| string.chars().all(|char| !char.is_ascii_uppercase()))
        );
    }
}
