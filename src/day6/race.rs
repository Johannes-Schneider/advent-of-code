use std::error::Error;

use crate::string_functions::{all_to_u128, all_to_u32, split_and_clean};
use crate::GenericError;

#[derive(Debug, PartialEq)]
pub struct Race {
    time: u128,
    record_distance: u128,
    _sqrt: f64,
}

impl Race {
    pub fn parse_all(input: &str) -> Result<Vec<Race>, GenericError> {
        let lines = input.lines().collect::<Vec<&str>>();
        if lines.len() != 2 {
            return Err(GenericError::new("unable to parse races"));
        }

        let raw_times = split_and_clean(lines[0], " ");
        let raw_distances = split_and_clean(lines[1], " ");

        if raw_times.len() < 2 || raw_distances.len() != raw_times.len() {
            return Err(GenericError::new("unable to parse races"));
        }

        let times = all_to_u128(&raw_times[1..])?;
        let distances = all_to_u128(&raw_distances[1..])?;

        let races = (0..times.len())
            .map(|i| Race::new(times[i], distances[i]))
            .collect::<Vec<Race>>();
        return Ok(races);
    }

    fn new(time: u128, distance: u128) -> Race {
        let inner: f64 = time.pow(2) as f64 - 4.0f64 * distance as f64;
        let root: f64 = inner.sqrt();

        return Race {
            time,
            record_distance: distance,
            _sqrt: root,
        };
    }

    pub fn number_of_ways_to_win(&self) -> u128 {
        self.maximum_charge_time() - self.minimum_charge_time() + 1
    }

    fn minimum_charge_time(&self) -> u128 {
        let result: f64 = 0.5f64 * (self.time as f64 - self._sqrt);
        return result.ceil() as u128;
    }

    fn maximum_charge_time(&self) -> u128 {
        let result: f64 = 0.5f64 * (self.time as f64 + self._sqrt);
        return result.floor() as u128;
    }
}

#[cfg(test)]
mod tests {
    use crate::day6::race::Race;

    #[test]
    fn test_parse_all() {
        let input = "\
Time:      7  15   30
Distance:  9  40  200";
        let actual = Race::parse_all(input).unwrap();

        assert_eq!(
            actual,
            vec![Race::new(7, 9), Race::new(15, 40), Race::new(30, 200),]
        );
    }

    #[test]
    fn test_numbers_of_ways_to_win() {
        let sut = Race::new(7, 9);

        assert_eq!(sut.number_of_ways_to_win(), 4);
    }

    #[test]
    fn test_minimum_charge_time() {
        let sut = Race::new(7, 9);

        assert_eq!(sut.minimum_charge_time(), 2);
    }

    #[test]
    fn test_maximum_charge_time() {
        let sut = Race::new(7, 9);

        assert_eq!(sut.maximum_charge_time(), 5);
    }
}
