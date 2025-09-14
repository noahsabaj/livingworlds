//! Utility functions for name generation
//!
//! Helper functions that support the name generation process but aren't
//! specific to any particular culture or name type.

/// Convert a number to Roman numerals
///
/// Used for adding numeric designations to names (e.g., "Alexandria III")
/// Supports numbers 1-50 for practical name generation purposes.
pub fn to_roman_numeral(n: u32) -> &'static str {
    match n {
        1 => "I",
        2 => "II",
        3 => "III",
        4 => "IV",
        5 => "V",
        6 => "VI",
        7 => "VII",
        8 => "VIII",
        9 => "IX",
        10 => "X",
        11 => "XI",
        12 => "XII",
        13 => "XIII",
        14 => "XIV",
        15 => "XV",
        16 => "XVI",
        17 => "XVII",
        18 => "XVIII",
        19 => "XIX",
        20 => "XX",
        21 => "XXI",
        22 => "XXII",
        23 => "XXIII",
        24 => "XXIV",
        25 => "XXV",
        26 => "XXVI",
        27 => "XXVII",
        28 => "XXVIII",
        29 => "XXIX",
        30 => "XXX",
        31 => "XXXI",
        32 => "XXXII",
        33 => "XXXIII",
        34 => "XXXIV",
        35 => "XXXV",
        36 => "XXXVI",
        37 => "XXXVII",
        38 => "XXXVIII",
        39 => "XXXIX",
        40 => "XL",
        41 => "XLI",
        42 => "XLII",
        43 => "XLIII",
        44 => "XLIV",
        45 => "XLV",
        46 => "XLVI",
        47 => "XLVII",
        48 => "XLVIII",
        49 => "XLIX",
        50 => "L",
        _ => "", // Return empty string for unsupported numbers
    }
}

/// Capitalize the first letter of a string
///
/// Useful for ensuring proper capitalization of generated names
pub fn capitalize_first(s: &str) -> String {
    let mut chars = s.chars();
    match chars.next() {
        None => String::new(),
        Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
    }
}

/// Join name parts with proper spacing
///
/// Handles empty strings and extra whitespace gracefully
pub fn join_name_parts(parts: &[&str]) -> String {
    parts
        .iter()
        .filter(|s| !s.is_empty())
        .map(|s| s.trim())
        .collect::<Vec<_>>()
        .join(" ")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_roman_numerals() {
        assert_eq!(to_roman_numeral(1), "I");
        assert_eq!(to_roman_numeral(4), "IV");
        assert_eq!(to_roman_numeral(9), "IX");
        assert_eq!(to_roman_numeral(14), "XIV");
        assert_eq!(to_roman_numeral(19), "XIX");
        assert_eq!(to_roman_numeral(27), "XXVII");
        assert_eq!(to_roman_numeral(40), "XL");
        assert_eq!(to_roman_numeral(49), "XLIX");
        assert_eq!(to_roman_numeral(50), "L");
        assert_eq!(to_roman_numeral(100), ""); // Unsupported
    }

    #[test]
    fn test_capitalize() {
        assert_eq!(capitalize_first("hello"), "Hello");
        assert_eq!(capitalize_first("WORLD"), "WORLD");
        assert_eq!(capitalize_first(""), "");
        assert_eq!(capitalize_first("élite"), "Élite");
    }

    #[test]
    fn test_join_name_parts() {
        assert_eq!(join_name_parts(&["New", "York"]), "New York");
        assert_eq!(join_name_parts(&["", "London", ""]), "London");
        assert_eq!(
            join_name_parts(&["  San  ", "  Francisco  "]),
            "San Francisco"
        );
        assert_eq!(join_name_parts(&[]), "");
    }
}
