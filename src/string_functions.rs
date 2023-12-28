use crate::GenericError;

pub fn split_and_clean<'a>(input: &'a str, separator: &str) -> Vec<&'a str> {
    let mut parts = input
        .split(separator)
        .map(|s| s.trim())
        .collect::<Vec<&str>>();
    parts.retain(|s| !s.is_empty());

    return parts;
}

pub fn to_u128(input: &str) -> Result<u128, GenericError> {
    let maybe_result = input.parse::<u128>();
    if maybe_result.is_err() {
        return Err(GenericError::new("unable to convert string to u128"));
    }

    return Ok(maybe_result.unwrap());
}

pub fn all_to_u128(input: &[&str]) -> Result<Vec<u128>, GenericError> {
    let mut result: Vec<u128> = Vec::new();
    for x in input {
        result.push(to_u128(x)?);
    }

    return Ok(result);
}

pub fn to_i128(input: &str) -> Result<i128, GenericError> {
    let maybe_result = input.parse::<i128>();
    if maybe_result.is_err() {
        return Err(GenericError::new("unable to convert string to i128"));
    }

    return Ok(maybe_result.unwrap());
}

pub fn all_to_i128(input: &[&str]) -> Result<Vec<i128>, GenericError> {
    let mut result: Vec<i128> = Vec::new();
    for x in input {
        result.push(to_i128(x)?);
    }

    return Ok(result);
}

pub fn to_usize(input: &str) -> Result<usize, GenericError> {
    let maybe_result = input.parse::<usize>();
    if maybe_result.is_err() {
        return Err(GenericError::new("unable to convert string to usize"));
    }

    return Ok(maybe_result.unwrap());
}

pub fn all_to_usize(input: &[&str]) -> Result<Vec<usize>, GenericError> {
    let mut result = Vec::new();
    for x in input {
        result.push(to_usize(x)?);
    }

    return Ok(result);
}
