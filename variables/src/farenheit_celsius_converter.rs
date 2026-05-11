#[allow(dead_code)]
pub fn fahrenheit_to_celsius(farenheit: f64) -> f64 {
    (farenheit - 32.) * 5. / 9.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fahrenheit_to_celsius_scenarios() {
        let cases = [
            (32., 0.),
            (212., 100.),
            (-40., -40.),
            (98.6, 37.),
            (-459.67, -273.15),
        ];

        for (input, expected) in cases {
            let actual = fahrenheit_to_celsius(input);

            assert_eq!(
                actual, expected,
                "Failed at {}°F: expected {}, got {}",
                input, expected, actual
            );
        }
    }
}
