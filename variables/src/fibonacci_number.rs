#[allow(dead_code)]
fn fibonacci_number(nth: u8) -> Result<u128, FibonacciError> {
    if nth < 2 {
        return Ok(nth as u128);
    }

    let mut previous: u128 = 0;
    let mut current: u128 = 1;

    for _ in 2..=nth {
        let next = previous
            .checked_add(current)
            .ok_or(FibonacciError::Overflow)?;
        previous = current;
        current = next;
    }

    Ok(current)
}

#[allow(dead_code)]
#[derive(Debug, PartialEq)]
enum FibonacciError {
    Overflow,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn returns_the_nth_fibonacci_number() {
        const CASES: [(u8, Result<u128, FibonacciError>); 10] = [
            (0, Ok(0)),
            (1, Ok(1)),
            (2, Ok(1)),
            (3, Ok(2)),
            (4, Ok(3)),
            (5, Ok(5)),
            (6, Ok(8)),
            (7, Ok(13)),
            (186, Ok(332825110087067562321196029789634457848)),
            (187, Err(FibonacciError::Overflow)),
        ];

        for (nth, expected) in CASES {
            assert_eq!(fibonacci_number(nth), expected);
        }
    }
}
