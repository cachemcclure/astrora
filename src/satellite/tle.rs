//! TLE (Two-Line Element) Parsing
//!
//! This module handles parsing of TLE data in both 2-line and 3-line formats.
//!
//! # TLE Format
//!
//! TLEs are the standard format for distributing satellite orbital elements.
//! They come in two variants:
//!
//! ## 2-Line Format
//! ```text
//! 1 25544U 98067A   08264.51782528 -.00002182  00000-0 -11606-4 0  2927
//! 2 25544  51.6416 247.4627 0006703 130.5360 325.0288 15.72125391563537
//! ```
//!
//! ## 3-Line Format (with satellite name)
//! ```text
//! ISS (ZARYA)
//! 1 25544U 98067A   08264.51782528 -.00002182  00000-0 -11606-4 0  2927
//! 2 25544  51.6416 247.4627 0006703 130.5360 325.0288 15.72125391563537
//! ```
//!
//! # Line 1 Fields
//!
//! - Column 01: Line number (1)
//! - Column 03-07: Satellite catalog number (NORAD ID)
//! - Column 08: Classification (U=Unclassified, C=Classified, S=Secret)
//! - Column 10-17: International designator (launch year, launch number, piece)
//! - Column 19-32: Epoch (year and day fraction)
//! - Column 34-43: First derivative of mean motion (ballistic coefficient)
//! - Column 45-52: Second derivative of mean motion (not used)
//! - Column 54-61: Drag term (B* drag coefficient)
//! - Column 63: Ephemeris type (always 0)
//! - Column 65-68: Element set number
//! - Column 69: Checksum
//!
//! # Line 2 Fields
//!
//! - Column 01: Line number (2)
//! - Column 03-07: Satellite catalog number (same as line 1)
//! - Column 09-16: Inclination \[degrees\]
//! - Column 18-25: Right ascension of ascending node \[degrees\]
//! - Column 27-33: Eccentricity (decimal point assumed, e.g., 0006703 = 0.0006703)
//! - Column 35-42: Argument of perigee \[degrees\]
//! - Column 44-51: Mean anomaly \[degrees\]
//! - Column 53-63: Mean motion \[revolutions/day\]
//! - Column 64-68: Revolution number at epoch
//! - Column 69: Checksum
//!
//! # References
//!
//! - <https://celestrak.org/NORAD/documentation/tle-fmt.php>
//! - Spacetrack Report #3 (Hoots & Roehrich, 1980)
//! - CelesTrak TLE format specification

use sgp4::Elements;
use crate::satellite::sgp4_wrapper::Sgp4Error;

/// Parse a TLE string (2-line or 3-line format)
///
/// Automatically detects whether the input is 2-line or 3-line format.
///
/// # Arguments
///
/// * `tle_string` - TLE data as a string (with or without satellite name)
///
/// # Returns
///
/// Parsed orbital elements ready for SGP4 propagation
///
/// # Errors
///
/// - `TleParsingFailed`: If TLE format is invalid or contains invalid data
///
/// # Example
///
/// ```rust,ignore
/// let tle = "ISS (ZARYA)
/// 1 25544U 98067A   08264.51782528 -.00002182  00000-0 -11606-4 0  2927
/// 2 25544  51.6416 247.4627 0006703 130.5360 325.0288 15.72125391563537";
///
/// let elements = parse_tle(tle)?;
/// println!("Satellite: {}", elements.object_name.unwrap_or("Unknown".to_string()));
/// println!("NORAD ID: {}", elements.norad_id);
/// ```
pub fn parse_tle(tle_string: &str) -> Result<Elements, Sgp4Error> {
    // Count non-empty lines
    let lines: Vec<&str> = tle_string
        .lines()
        .map(|l| l.trim())
        .filter(|l| !l.is_empty())
        .collect();

    match lines.len() {
        2 => {
            // 2-line format
            parse_2line_tle(lines[0], lines[1])
        }
        3 => {
            // 3-line format (name + 2 TLE lines)
            parse_3line_tle(lines[0], lines[1], lines[2])
        }
        _ => Err(Sgp4Error::TleParsingFailed(format!(
            "Expected 2 or 3 lines, got {}",
            lines.len()
        ))),
    }
}

/// Parse 2-line TLE format (no satellite name)
fn parse_2line_tle(line1: &str, line2: &str) -> Result<Elements, Sgp4Error> {
    // Use sgp4 crate's built-in parser (returns Vec<Elements>)
    let tle_string = format!("{line1}\n{line2}");
    let elements_vec = sgp4::parse_2les(&tle_string)
        .map_err(|e| Sgp4Error::TleParsingFailed(e.to_string()))?;

    // Extract first element (should only be one for single TLE)
    elements_vec.into_iter().next()
        .ok_or_else(|| Sgp4Error::TleParsingFailed("No elements found in TLE".to_string()))
}

/// Parse 3-line TLE format (with satellite name)
fn parse_3line_tle(name: &str, line1: &str, line2: &str) -> Result<Elements, Sgp4Error> {
    // Use sgp4 crate's built-in parser (returns Vec<Elements>)
    let tle_string = format!("{name}\n{line1}\n{line2}");
    let elements_vec = sgp4::parse_3les(&tle_string)
        .map_err(|e| Sgp4Error::TleParsingFailed(e.to_string()))?;

    // Extract first element (should only be one for single TLE)
    elements_vec.into_iter().next()
        .ok_or_else(|| Sgp4Error::TleParsingFailed("No elements found in TLE".to_string()))
}

/// Validate TLE checksum
///
/// Each TLE line ends with a checksum digit (modulo-10 sum of all digits in line).
/// Letters, blanks, periods, and plus signs are counted as 0.
/// Minus signs are counted as 1.
///
/// # Note
///
/// The sgp4 crate already validates checksums internally, but this function
/// is provided for educational purposes and debugging.
pub fn validate_checksum(line: &str) -> bool {
    if line.len() < 69 {
        return false;
    }

    let checksum_char = line.chars().nth(68).unwrap_or('0');
    let expected_checksum = checksum_char.to_digit(10).unwrap_or(0);

    // Calculate actual checksum
    let mut sum = 0;
    for c in line[..68].chars() {
        if c.is_ascii_digit() {
            sum += c.to_digit(10).unwrap();
        } else if c == '-' {
            sum += 1;
        }
        // Letters, blanks, periods, plus signs count as 0
    }

    (sum % 10) == expected_checksum
}

#[cfg(test)]
mod tests {
    use super::*;

    const ISS_TLE_2LINE: &str = "1 25544U 98067A   08264.51782528 -.00002182  00000-0 -11606-4 0  2927
2 25544  51.6416 247.4627 0006703 130.5360 325.0288 15.72125391563537";

    const ISS_TLE_3LINE: &str = "ISS (ZARYA)
1 25544U 98067A   08264.51782528 -.00002182  00000-0 -11606-4 0  2927
2 25544  51.6416 247.4627 0006703 130.5360 325.0288 15.72125391563537";

    #[test]
    fn test_parse_2line_tle() {
        let elements = parse_tle(ISS_TLE_2LINE).unwrap();

        assert_eq!(elements.norad_id, 25544);
        // sgp4 crate returns expanded format (1998-067A, not 98067A)
        assert_eq!(elements.international_designator, Some("1998-067A".to_string()));
        // Note: sgp4::Elements stores inclination in degrees (not radians)
        assert!((elements.inclination - 51.6416).abs() < 0.001);
        assert!((elements.eccentricity - 0.0006703).abs() < 0.0000001);
        assert!((elements.mean_motion - 15.72125391).abs() < 0.00001);
    }

    #[test]
    fn test_parse_3line_tle() {
        let elements = parse_tle(ISS_TLE_3LINE).unwrap();

        assert_eq!(elements.norad_id, 25544);
        assert_eq!(elements.object_name, Some("ISS (ZARYA)".to_string()));
        // sgp4 crate returns expanded format (1998-067A, not 98067A)
        assert_eq!(elements.international_designator, Some("1998-067A".to_string()));
        // Note: sgp4::Elements stores inclination in degrees (not radians)
        assert!((elements.inclination - 51.6416).abs() < 0.001);
    }

    #[test]
    fn test_validate_checksum() {
        let line1 = "1 25544U 98067A   08264.51782528 -.00002182  00000-0 -11606-4 0  2927";
        let line2 = "2 25544  51.6416 247.4627 0006703 130.5360 325.0288 15.72125391563537";

        assert!(validate_checksum(line1));
        assert!(validate_checksum(line2));
    }

    #[test]
    fn test_invalid_checksum() {
        // Modify last digit to make checksum invalid
        let bad_line = "1 25544U 98067A   08264.51782528 -.00002182  00000-0 -11606-4 0  2920";
        assert!(!validate_checksum(bad_line));
    }

    #[test]
    fn test_invalid_format() {
        // Only one line
        let result = parse_tle("1 25544U 98067A   08264.51782528 -.00002182  00000-0 -11606-4 0  2927");
        assert!(result.is_err());

        // Too many lines
        let result = parse_tle("Line1\nLine2\nLine3\nLine4");
        assert!(result.is_err());
    }

    #[test]
    fn test_different_satellite_name() {
        // Test with different satellite name to ensure 3-line format works properly
        // Using ISS orbital data but with a custom name
        let satellite_tle = "CUSTOM SATELLITE NAME
1 25544U 98067A   08264.51782528 -.00002182  00000-0 -11606-4 0  2927
2 25544  51.6416 247.4627 0006703 130.5360 325.0288 15.72125391563537";

        let elements = parse_tle(satellite_tle).unwrap();
        assert_eq!(elements.norad_id, 25544);
        assert_eq!(elements.object_name, Some("CUSTOM SATELLITE NAME".to_string()));
        // Note: sgp4::Elements stores inclination in degrees (not radians)
        assert!((elements.inclination - 51.6416).abs() < 0.001);
        assert!((elements.eccentricity - 0.0006703).abs() < 0.0000001);
    }
}
