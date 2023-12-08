use crate::GenericError;

#[derive(Debug, PartialEq)]
pub struct ScratchCard {
    pub id: u32,
    winning_numbers: Vec<u32>,
    player_numbers: Vec<u32>,
    pub number_of_matches: u32,
}

impl ScratchCard {
    pub fn parse_all(input: &str) -> Result<Vec<ScratchCard>, GenericError> {
        let mut result: Vec<ScratchCard> = Vec::new();

        for line in input.lines() {
            result.push(ScratchCard::parse(line)?);
        }

        return Ok(result);
    }

    pub fn parse(input: &str) -> Result<ScratchCard, GenericError> {
        if !input.is_ascii() {
            return Err(GenericError::new(
                "the input must consist of ascii chars only",
            ));
        }

        let (id, remaining) = ScratchCard::extract_game_id(input)?;
        let (winning_numbers, player_numbers) =
            ScratchCard::extract_winning_and_player_numbers(remaining)?;

        let number_of_matches = player_numbers
            .iter()
            .filter(|c| winning_numbers.contains(c))
            .count() as u32;

        return Ok(ScratchCard {
            id,
            winning_numbers,
            player_numbers,
            number_of_matches,
        });
    }

    fn extract_game_id(input: &str) -> Result<(u32, &str), GenericError> {
        let parts = ScratchCard::split_and_clean(input, ":");
        if parts.len() != 2 {
            return Err(GenericError::new("cannot extract game id from input"));
        }

        let sub_parts = ScratchCard::split_and_clean(parts[0], " ");
        if sub_parts.len() != 2 {
            return Err(GenericError::new("cannot extract game id from input"));
        }

        let maybe_id = sub_parts[1].parse::<u32>();
        if maybe_id.is_err() {
            return Err(GenericError::new("cannot extract game id from input"));
        }

        return Ok((maybe_id.unwrap(), parts[1]));
    }

    fn extract_winning_and_player_numbers(
        input: &str,
    ) -> Result<(Vec<u32>, Vec<u32>), GenericError> {
        let parts = ScratchCard::split_and_clean(input, "|");
        if parts.len() != 2 {
            return Err(GenericError::new("cannot extract number from input"));
        }

        return Ok((
            ScratchCard::extract_numbers(parts[0])?,
            ScratchCard::extract_numbers(parts[1])?,
        ));
    }

    fn extract_numbers(input: &str) -> Result<Vec<u32>, GenericError> {
        let parts = ScratchCard::split_and_clean(input, " ");

        let mut result: Vec<u32> = Vec::new();
        for part in parts {
            let maybe_number = part.parse::<u32>();
            if maybe_number.is_err() {
                return Err(GenericError::new("cannot convert input into number"));
            }

            let number = maybe_number.unwrap();
            if result.contains(&number) {
                continue;
            }

            result.push(number);
        }

        return Ok(result);
    }

    fn split_and_clean<'a>(input: &'a str, separator: &str) -> Vec<&'a str> {
        let mut parts: Vec<&str> = input.trim().split(separator).map(|s| s.trim()).collect();
        parts.retain(|s| !s.is_empty());

        return parts;
    }

    pub fn points(&self) -> u32 {
        if self.number_of_matches < 1 {
            return 0;
        }

        return 2u32.pow(self.number_of_matches - 1);
    }
}

#[cfg(test)]
mod tests {
    use crate::day4::scratch_card::ScratchCard;

    #[test]
    fn test_parse() {
        let input = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53";
        let actual = ScratchCard::parse(input).unwrap();

        assert_eq!(
            actual,
            ScratchCard {
                id: 1,
                winning_numbers: vec![41, 48, 83, 86, 17],
                player_numbers: vec![83, 86, 6, 31, 17, 9, 48, 53],
                number_of_matches: 4
            }
        );
    }

    #[test]
    fn test_extract_game_id() {
        let input = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53";
        let actual = ScratchCard::extract_game_id(input).unwrap();

        assert_eq!(actual, (1, "41 48 83 86 17 | 83 86  6 31 17  9 48 53"))
    }

    #[test]
    fn test_extract_winning_and_player_numbers() {
        let input = "41 48 83 86 17 | 83 86  6 31 17  9 48 53";
        let actual = ScratchCard::extract_winning_and_player_numbers(input).unwrap();

        assert_eq!(
            actual,
            (vec![41, 48, 83, 86, 17], vec![83, 86, 6, 31, 17, 9, 48, 53])
        );
    }

    #[test]
    fn test_extract_numbers() {
        let input = "41 48 83 86 17";
        let actual = ScratchCard::extract_numbers(input).unwrap();

        assert_eq!(actual, vec![41, 48, 83, 86, 17]);
    }

    #[test]
    fn test_extract_numbers_with_duplicates() {
        let input = "41 41 41 34";
        let actual = ScratchCard::extract_numbers(input).unwrap();

        assert_eq!(actual, vec![41, 34]);
    }
}
