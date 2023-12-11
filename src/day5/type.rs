use phf::phf_map;

use crate::GenericError;

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Type {
    SEED,
    SOIL,
    FERTILIZER,
    WATER,
    LIGHT,
    TEMPERATURE,
    HUMIDITY,
    LOCATION,
}

static TYPES: phf::Map<&'static str, Type> = phf_map! {
    "seed" => Type::SEED,
    "soil" => Type::SOIL,
    "fertilizer" => Type::FERTILIZER,
    "water" => Type::WATER,
    "light" => Type::LIGHT,
    "temperature" => Type::TEMPERATURE,
    "humidity" => Type::HUMIDITY,
    "location" => Type::LOCATION
};

impl Type {
    pub fn from_string(input: &str) -> Result<Type, GenericError> {
        let maybe_type = TYPES.get(input);
        if maybe_type.is_none() {
            return Err(GenericError::new("unknown type"));
        }

        return Ok(maybe_type.unwrap().clone());
    }
}
