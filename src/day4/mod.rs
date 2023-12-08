use crate::day4::game::Game;
use std::error::Error;
use std::fs;

mod game;

pub fn day4_challenge1(file_path: &str) -> Result<u32, Box<dyn Error>> {
    let text = fs::read_to_string(file_path)?;
    let games = Game::parse_all(&text)?;

    return Ok(games.iter().map(|g| g.points()).sum());
}
