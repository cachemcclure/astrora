//! OMM (Orbit Mean-Elements Message) Parsing
//!
//! This module handles parsing of OMM data in JSON format, which is the
//! modern successor to the TLE format.
//!
//! # OMM Format
//!
//! OMM (Orbit Mean-Elements Message) is a CCSDS standard format for
//! distributing orbital elements. It supports:
//! - Human-readable JSON format
//! - 9-digit catalog numbers (vs 5-digit TLE limit)
//! - Additional metadata and tracking information
//! - Multiple object classes (payload, rocket body, debris)
//!
//! # Example OMM JSON
//!
//! ```json
//! {
//!   "OBJECT_NAME": "ISS (ZARYA)",
//!   "OBJECT_ID": "1998-067A",
//!   "EPOCH": "2008-09-20T12:25:40.104",
//!   "MEAN_MOTION": 15.72125391,
//!   "ECCENTRICITY": 0.0006703,
//!   "INCLINATION": 51.6416,
//!   "RA_OF_ASC_NODE": 247.4627,
//!   "ARG_OF_PERICENTER": 130.5360,
//!   "MEAN_ANOMALY": 325.0288,
//!   "EPHEMERIS_TYPE": 0,
//!   "CLASSIFICATION_TYPE": "U",
//!   "NORAD_CAT_ID": 25544,
//!   "ELEMENT_SET_NO": 292,
//!   "REV_AT_EPOCH": 56353,
//!   "BSTAR": -0.000011606,
//!   "MEAN_MOTION_DOT": -0.00002182,
//!   "MEAN_MOTION_DDOT": 0.0
//! }
//! ```
//!
//! # OMM vs TLE
//!
//! | Feature | TLE | OMM |
//! |---------|-----|-----|
//! | Format | Fixed-width text | JSON/XML/KVN |
//! | Catalog # | 5 digits (max 99999) | 9+ digits |
//! | Human-readable | No | Yes |
//! | Extensible | No | Yes |
//! | Size | ~140 bytes | ~400+ bytes |
//! | Adoption | Universal (legacy) | Growing (Space-Track default) |
//!
//! # Transition Timeline
//!
//! - **2023**: Space-Track began supporting OMM format
//! - **2024**: TLE format capped at catalog #69999
//! - **2025+**: OMM becoming primary format for new satellites
//! - **Legacy**: TLE will remain for backward compatibility
//!
//! # References
//!
//! - CCSDS 502.0-B-2: Orbit Data Messages
//! - <https://public.ccsds.org/Pubs/502x0b2c1.pdf>
//! - <https://www.space-track.org/documentation#/OMM>

use sgp4::Elements;
use crate::satellite::sgp4_wrapper::Sgp4Error;
use serde_json;

/// Parse an OMM JSON string
///
/// # Arguments
///
/// * `json_string` - OMM data in JSON format
///
/// # Returns
///
/// Parsed orbital elements ready for SGP4 propagation
///
/// # Errors
///
/// - `OmmParsingFailed`: If JSON format is invalid or contains invalid data
///
/// # Example
///
/// ```rust,ignore
/// let omm_json = r#"{
///   "OBJECT_NAME": "ISS (ZARYA)",
///   "OBJECT_ID": "1998-067A",
///   "EPOCH": "2008-09-20T12:25:40.104",
///   "MEAN_MOTION": 15.72125391,
///   "ECCENTRICITY": 0.0006703,
///   "INCLINATION": 51.6416,
///   "RA_OF_ASC_NODE": 247.4627,
///   "ARG_OF_PERICENTER": 130.5360,
///   "MEAN_ANOMALY": 325.0288,
///   "NORAD_CAT_ID": 25544
/// }"#;
///
/// let elements = parse_omm(omm_json)?;
/// ```
pub fn parse_omm(json_string: &str) -> Result<Elements, Sgp4Error> {
    // Parse JSON into Elements using serde
    // The sgp4::Elements struct implements Deserialize when serde feature is enabled
    serde_json::from_str(json_string)
        .map_err(|e| Sgp4Error::OmmParsingFailed(e.to_string()))
}

/// Parse multiple OMM objects from a JSON array
///
/// Some Space-Track queries return arrays of OMM objects.
///
/// # Example
///
/// ```rust,ignore
/// let omm_array = r#"[
///   { "OBJECT_NAME": "SAT1", ... },
///   { "OBJECT_NAME": "SAT2", ... }
/// ]"#;
///
/// let elements_vec = parse_omm_batch(omm_array)?;
/// ```
pub fn parse_omm_batch(json_array: &str) -> Result<Vec<Elements>, Sgp4Error> {
    use serde_json::Value;

    // Parse as JSON array
    let array: Vec<Value> = serde_json::from_str(json_array)
        .map_err(|e| Sgp4Error::OmmParsingFailed(format!("Invalid JSON array: {e}")))?;

    // Parse each object
    array
        .iter()
        .enumerate()
        .map(|(i, obj)| {
            let obj_str = serde_json::to_string(obj)
                .map_err(|e| Sgp4Error::OmmParsingFailed(format!("Object {i} serialize error: {e}")))?;
            parse_omm(&obj_str)
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    const ISS_OMM: &str = r#"{
        "OBJECT_NAME": "ISS (ZARYA)",
        "OBJECT_ID": "1998-067A",
        "EPOCH": "2008-09-20T12:25:40.104",
        "MEAN_MOTION": 15.72125391,
        "ECCENTRICITY": 0.0006703,
        "INCLINATION": 51.6416,
        "RA_OF_ASC_NODE": 247.4627,
        "ARG_OF_PERICENTER": 130.5360,
        "MEAN_ANOMALY": 325.0288,
        "EPHEMERIS_TYPE": 0,
        "CLASSIFICATION_TYPE": "U",
        "NORAD_CAT_ID": 25544,
        "ELEMENT_SET_NO": 292,
        "REV_AT_EPOCH": 56353,
        "BSTAR": -0.000011606,
        "MEAN_MOTION_DOT": -0.00002182,
        "MEAN_MOTION_DDOT": 0.0
    }"#;

    #[test]
    fn test_parse_omm() {
        let elements = parse_omm(ISS_OMM).unwrap();

        assert_eq!(elements.norad_id, 25544);
        assert_eq!(elements.object_name, Some("ISS (ZARYA)".to_string()));
        assert_eq!(elements.international_designator, Some("1998-067A".to_string()));
        // Note: sgp4::Elements stores inclination in degrees (not radians)
        assert!((elements.inclination - 51.6416).abs() < 0.001);
        assert!((elements.eccentricity - 0.0006703).abs() < 0.0000001);
        assert!((elements.mean_motion - 15.72125391).abs() < 0.00001);
        assert_eq!(elements.element_set_number, 292);
        assert_eq!(elements.revolution_number, 56353);
    }

    #[test]
    fn test_parse_omm_batch() {
        let omm_array = format!("[{}, {}]", ISS_OMM, ISS_OMM);
        let elements_vec = parse_omm_batch(&omm_array).unwrap();

        assert_eq!(elements_vec.len(), 2);
        assert_eq!(elements_vec[0].norad_id, 25544);
        assert_eq!(elements_vec[1].norad_id, 25544);
    }

    #[test]
    fn test_invalid_json() {
        let result = parse_omm("{invalid json");
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), Sgp4Error::OmmParsingFailed(_)));
    }

    #[test]
    fn test_missing_required_field() {
        // Missing MEAN_MOTION
        let incomplete_omm = r#"{
            "OBJECT_NAME": "TEST",
            "EPOCH": "2008-09-20T12:25:40.104",
            "NORAD_CAT_ID": 12345
        }"#;

        let result = parse_omm(incomplete_omm);
        assert!(result.is_err());
    }

    #[test]
    fn test_required_fields_only() {
        // Test with all required fields (no optional object name or ID)
        // Note: sgp4 crate has many mandatory fields
        let required_fields_omm = r#"{
            "EPOCH": "2008-09-20T12:25:40.104",
            "MEAN_MOTION": 15.72125391,
            "ECCENTRICITY": 0.0006703,
            "INCLINATION": 51.6416,
            "RA_OF_ASC_NODE": 247.4627,
            "ARG_OF_PERICENTER": 130.5360,
            "MEAN_ANOMALY": 325.0288,
            "NORAD_CAT_ID": 25544,
            "CLASSIFICATION_TYPE": "U",
            "MEAN_MOTION_DOT": 0.0,
            "MEAN_MOTION_DDOT": 0.0,
            "BSTAR": 0.0,
            "ELEMENT_SET_NO": 1,
            "REV_AT_EPOCH": 1000,
            "EPHEMERIS_TYPE": 0
        }"#;

        let elements = parse_omm(required_fields_omm).unwrap();
        assert_eq!(elements.norad_id, 25544);
    }
}
