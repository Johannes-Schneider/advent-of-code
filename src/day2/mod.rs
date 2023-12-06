use std::cmp::max;
use std::error::Error;
use std::fmt::{Debug, Display, Formatter};
use std::fs;

use advent_of_code::GenericError;

struct CubeCollection {
    red: u32,
    green: u32,
    blue: u32,
}

impl CubeCollection {
    fn parse(input: &str) -> Result<CubeCollection, GenericError> {
        let mut parts = input.split(",").map(|s| s.trim()).collect::<Vec<&str>>();
        parts.retain(|s| s.len() > 0);

        let mut red: Option<u32> = None;
        let mut green: Option<u32> = None;
        let mut blue: Option<u32> = None;

        for part in parts {
            let mut sub_parts = part.split(" ").map(|s| s.trim()).collect::<Vec<&str>>();
            sub_parts.retain(|s| s.len() > 0);

            if sub_parts.len() % 2 != 0 {
                return Err(GenericError::new("input does not match the assumed format"));
            }

            for i in (0..sub_parts.len()).step_by(2) {
                let maybe_amount = sub_parts[i].parse::<u32>();
                let color = sub_parts[i + 1];

                if maybe_amount.is_err() {
                    return Err(GenericError::new("cannot extract amount from given input"));
                }

                match color {
                    "red" => {
                        if red.is_some() {
                            return Err(GenericError::new("red cubes are revealed more than once"));
                        }

                        red = Some(maybe_amount.unwrap());
                    }
                    "green" => {
                        if green.is_some() {
                            return Err(GenericError::new(
                                "green cubes are revealed more than once",
                            ));
                        }

                        green = Some(maybe_amount.unwrap());
                    }
                    "blue" => {
                        if blue.is_some() {
                            return Err(GenericError::new(
                                "blue cubes are revealed more than once",
                            ));
                        }

                        blue = Some(maybe_amount.unwrap());
                    }
                    _ => {
                        return Err(GenericError::new("unknown color"));
                    }
                }
            }
        }

        Ok(CubeCollection {
            red: red.unwrap_or(0),
            green: green.unwrap_or(0),
            blue: blue.unwrap_or(0),
        })
    }
}

impl PartialEq<Self> for CubeCollection {
    fn eq(&self, other: &Self) -> bool {
        return self.red == other.red && self.green == other.green && self.blue == other.blue;
    }
}

impl Display for CubeCollection {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} red, {} green, {} blue",
            self.red, self.green, self.blue
        )
    }
}

impl Debug for CubeCollection {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} red, {} green, {} blue",
            self.red, self.green, self.blue
        )
    }
}

impl Eq for CubeCollection {}

struct Game {
    id: u32,
    reveals: Vec<CubeCollection>,
}

impl Game {
    fn parse(input: &str) -> Result<Game, GenericError> {
        let mut parts = input
            .trim()
            .split(":")
            .map(|s| s.trim())
            .collect::<Vec<&str>>();
        parts.retain(|s| s.len() > 0);

        if parts.len() != 2 {
            return Err(GenericError::new(
                "the input does not match the assumed format",
            ));
        }

        return Ok(Game {
            id: Game::extract_game_id(parts[0])?,
            reveals: Game::extract_cube_reveals(parts[1])?,
        });
    }

    fn extract_game_id(input: &str) -> Result<u32, GenericError> {
        let mut parts = input
            .trim()
            .split(" ")
            .map(|s| s.trim())
            .collect::<Vec<&str>>();
        parts.retain(|s| s.len() > 0);

        if parts.len() != 2 {
            return Err(GenericError::new("unable to extract game id"));
        }

        let maybe_id = parts[1].parse::<u32>();
        if maybe_id.is_err() {
            return Err(GenericError::new("the game id is not a number"));
        }

        return Ok(maybe_id.unwrap());
    }

    fn extract_cube_reveals(input: &str) -> Result<Vec<CubeCollection>, GenericError> {
        let mut parts = input
            .trim()
            .split(";")
            .map(|s| s.trim())
            .collect::<Vec<&str>>();
        parts.retain(|s| s.len() > 0);

        if parts.len() < 1 {
            return Err(GenericError::new("game does not include any cube reveals"));
        }

        let mut result: Vec<CubeCollection> = Vec::new();
        for part in parts {
            result.push(CubeCollection::parse(part)?);
        }

        return Ok(result);
    }

    fn is_possible_with_bag(&self, bag: &CubeCollection) -> bool {
        let seen_cubes = self.maximum_number_of_seen_cubes_per_color();
        return bag.red >= seen_cubes.red
            && bag.green >= seen_cubes.green
            && bag.blue >= seen_cubes.blue;
    }

    fn maximum_number_of_seen_cubes_per_color(&self) -> CubeCollection {
        let mut seen_red: u32 = 0;
        let mut seen_green: u32 = 0;
        let mut seen_blue: u32 = 0;

        for r in &self.reveals {
            seen_red = max(seen_red, r.red);
            seen_green = max(seen_green, r.green);
            seen_blue = max(seen_blue, r.blue);
        }

        return CubeCollection {
            red: seen_red,
            green: seen_green,
            blue: seen_blue,
        };
    }
}

impl PartialEq<Self> for Game {
    fn eq(&self, other: &Self) -> bool {
        return self.id == other.id && self.reveals == other.reveals;
    }
}

impl Debug for Game {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Game {}: {:?}", self.id, self.reveals)
    }
}

impl Eq for Game {}

pub fn day2_challenge1(file_path: &str) -> Result<u32, Box<dyn Error>> {
    let bag = CubeCollection {
        red: 12,
        green: 13,
        blue: 14,
    };
    let games = read_all_games(file_path)?;

    return Ok(games
        .iter()
        .filter(|g| g.is_possible_with_bag(&bag))
        .map(|g| g.id)
        .sum());
}

pub fn day2_challenge2(file_path: &str) -> Result<u32, Box<dyn Error>> {
    let games = read_all_games(file_path)?;

    let mut result: u32 = 0;
    for game in games {
        let required_bag = game.maximum_number_of_seen_cubes_per_color();
        let product = required_bag.red * required_bag.green * required_bag.blue;
        result += product;
    }

    return Ok(result);
}

fn read_all_games(file_path: &str) -> Result<Vec<Game>, Box<dyn Error>> {
    let text = fs::read_to_string(file_path)?;

    let mut games: Vec<Game> = Vec::new();
    for line in text.lines() {
        let maybe_game = Game::parse(line);
        if maybe_game.is_err() {
            return Err(Box::new(GenericError::new(
                "not all lines contain a valid game",
            )));
        }

        games.push(maybe_game.unwrap());
    }

    return Ok(games);
}

#[cfg(test)]
mod tests {
    use crate::day2::{CubeCollection, Game};

    #[test]
    fn parse_cube_reveal_one_blue() {
        let input = "1 blue";
        let actual = CubeCollection::parse(input).unwrap();

        assert_eq!(
            CubeCollection {
                red: 0,
                green: 0,
                blue: 1,
            },
            actual
        )
    }

    #[test]
    fn parse_cube_reveal_one_red() {
        let input = "1 red";
        let actual = CubeCollection::parse(input).unwrap();

        assert_eq!(
            CubeCollection {
                red: 1,
                green: 0,
                blue: 0,
            },
            actual
        )
    }

    #[test]
    fn parse_cube_reveal_one_green() {
        let input = "1 green";
        let actual = CubeCollection::parse(input).unwrap();

        assert_eq!(
            CubeCollection {
                red: 0,
                green: 1,
                blue: 0,
            },
            actual
        )
    }

    #[test]
    fn parse_cube_reveal_all_colors() {
        let input = "10 green, 11 red, 12 blue";
        let actual = CubeCollection::parse(input).unwrap();

        assert_eq!(
            CubeCollection {
                red: 11,
                green: 10,
                blue: 12,
            },
            actual
        )
    }

    #[test]
    fn parse_cube_reveal_trailing_spaces() {
        let input = "  10  green  ,     33  blue";
        let actual = CubeCollection::parse(input).unwrap();

        assert_eq!(
            CubeCollection {
                red: 0,
                green: 10,
                blue: 33,
            },
            actual
        )
    }

    #[test]
    fn parse_cube_reveal_wrong_order() {
        let input = "green 1";
        let actual = CubeCollection::parse(input);

        assert_eq!(actual.is_err(), true)
    }

    #[test]
    fn parse_cube_reveal_unknown_color() {
        let input = "10 orange";
        let actual = CubeCollection::parse(input);

        assert_eq!(actual.is_err(), true)
    }

    #[test]
    fn parse_cube_reveal_missing_amount() {
        let input = "green";
        let actual = CubeCollection::parse(input);

        assert_eq!(actual.is_err(), true)
    }

    #[test]
    fn parse_cube_reveal_missing_color() {
        let input = "10";
        let actual = CubeCollection::parse(input);

        assert_eq!(actual.is_err(), true)
    }

    #[test]
    fn parse_cube_reveal_amount_is_not_a_number() {
        let input = "ten green";
        let actual = CubeCollection::parse(input);

        assert_eq!(actual.is_err(), true)
    }

    #[test]
    fn parse_game_simple_case() {
        let input = "Game 1: 1 blue, 2 green, 3 red; 4 blue, 5 red";
        let actual = Game::parse(input).unwrap();

        assert_eq!(
            Game {
                id: 1,
                reveals: vec![
                    CubeCollection {
                        red: 3,
                        green: 2,
                        blue: 1,
                    },
                    CubeCollection {
                        red: 5,
                        green: 0,
                        blue: 4,
                    },
                ],
            },
            actual
        )
    }

    #[test]
    fn parse_game_real_case() {
        let input = "Game 5: 3 blue, 3 red, 8 green; 5 blue, 1 red; 1 green, 19 blue, 3 red; 1 red, 5 green, 3 blue; 4 green, 20 blue, 4 red; 20 blue, 4 green";
        let actual = Game::parse(input).unwrap();

        assert_eq!(
            Game {
                id: 5,
                reveals: vec![
                    CubeCollection {
                        red: 3,
                        green: 8,
                        blue: 3,
                    },
                    CubeCollection {
                        red: 1,
                        green: 0,
                        blue: 5,
                    },
                    CubeCollection {
                        red: 3,
                        green: 1,
                        blue: 19,
                    },
                    CubeCollection {
                        red: 1,
                        green: 5,
                        blue: 3,
                    },
                    CubeCollection {
                        red: 4,
                        green: 4,
                        blue: 20,
                    },
                    CubeCollection {
                        red: 0,
                        green: 4,
                        blue: 20,
                    },
                ],
            },
            actual
        );
    }

    #[test]
    fn game_is_impossible_with_bag() {
        let game = Game {
            id: 0,
            reveals: vec![
                CubeCollection {
                    red: 10,
                    green: 0,
                    blue: 0,
                },
                CubeCollection {
                    red: 0,
                    green: 11,
                    blue: 0,
                },
                CubeCollection {
                    red: 0,
                    green: 0,
                    blue: 12,
                },
            ],
        };
        let bag = CubeCollection {
            red: 9,
            green: 10,
            blue: 11,
        };

        assert_eq!(game.is_possible_with_bag(&bag), false)
    }

    #[test]
    fn game_is_possible_with_bag() {
        let game = Game {
            id: 0,
            reveals: vec![
                CubeCollection {
                    red: 10,
                    green: 0,
                    blue: 0,
                },
                CubeCollection {
                    red: 0,
                    green: 11,
                    blue: 0,
                },
                CubeCollection {
                    red: 0,
                    green: 0,
                    blue: 12,
                },
            ],
        };
        let bag = CubeCollection {
            red: 11,
            green: 12,
            blue: 13,
        };

        assert_eq!(game.is_possible_with_bag(&bag), true)
    }
}
