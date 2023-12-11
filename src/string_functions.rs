use crate::GenericError;

pub fn split_and_clean<'a>(input: &'a str, separator: &str) -> Vec<&'a str> {
    let mut parts = input
        .split(separator)
        .map(|s| s.trim())
        .collect::<Vec<&str>>();
    parts.retain(|s| !s.is_empty());

    return parts;
}

pub fn to_u32(input: &str) -> Result<u32, GenericError> {
    let maybe_result = input.parse::<u32>();
    if maybe_result.is_err() {
        return Err(GenericError::new("unable to convert string to u32"));
    }

    return Ok(maybe_result.unwrap());
}

pub fn all_to_u32(input: &[&str]) -> Result<Vec<u32>, GenericError> {
    let mut result: Vec<u32> = Vec::new();
    for x in input {
        result.push(to_u32(x)?);
    }

    return Ok(result);
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

#[cfg(test)]
mod tests {
    use crate::string_functions::split_and_clean;
    use crate::string_functions::to_u32;

    #[test]
    fn split_and_clean_ok() {
        let input = "foo |  bar | b  a\tz \t| ";
        let actual = split_and_clean(input, "|");

        assert_eq!(actual, vec!["foo", "bar", "b  a\tz"]);
    }

    #[test]
    fn to_u32_ok() {
        assert_eq!(to_u32("1337").unwrap(), 1337);
    }

    #[test]
    fn to_u32_err() {
        assert_eq!(to_u32("foo").is_err(), true);
    }
}
