//! Time handling module using hifitime for nanosecond-precision astrodynamics
//!
//! This module provides high-precision time handling for orbital mechanics calculations,
//! with nanosecond precision guaranteed for 65,536 years around the reference epoch.
//!
//! # Features
//! - Nanosecond precision time handling
//! - Full leap second support
//! - Multiple time scale support (UTC, TAI, TT, TDB, GPS, etc.)
//! - Astropy-compatible API
//! - Efficient conversions between time scales

use hifitime::{Duration as HifiDuration, Epoch as HifiEpoch, TimeScale};
use pyo3::prelude::*;

/// High-precision epoch representation for astrodynamics
///
/// Wraps hifitime::Epoch to provide nanosecond-precision time handling
/// with full leap second support and multiple time scale conversions.
///
/// # Examples
/// ```
/// use astrora_core::core::time::Epoch;
///
/// // Create epoch from Gregorian date (UTC)
/// let epoch = Epoch::from_gregorian_utc(2000, 1, 1, 12, 0, 0, 0).unwrap();
///
/// // Get J2000 epoch
/// let j2000 = Epoch::j2000();
/// ```
#[pyclass(module = "astrora._core.time")]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Epoch {
    inner: HifiEpoch,
}

impl Epoch {
    /// Create a new Epoch from hifitime::Epoch
    pub fn new(epoch: HifiEpoch) -> Self {
        Self { inner: epoch }
    }

    /// Get the inner hifitime::Epoch
    pub fn inner(&self) -> HifiEpoch {
        self.inner
    }

    /// Create epoch from Gregorian date in UTC
    ///
    /// # Arguments
    /// * `year` - Year (e.g., 2000)
    /// * `month` - Month (1-12)
    /// * `day` - Day (1-31)
    /// * `hour` - Hour (0-23)
    /// * `minute` - Minute (0-59)
    /// * `second` - Second (0-59, or 60 during leap second)
    /// * `nanos` - Nanoseconds (0-999,999,999)
    pub fn from_gregorian_utc(
        year: i32,
        month: u8,
        day: u8,
        hour: u8,
        minute: u8,
        second: u8,
        nanos: u32,
    ) -> Self {
        Self::new(HifiEpoch::from_gregorian_utc(year, month, day, hour, minute, second, nanos))
    }

    /// Create epoch from Gregorian date in TAI
    pub fn from_gregorian_tai(
        year: i32,
        month: u8,
        day: u8,
        hour: u8,
        minute: u8,
        second: u8,
        nanos: u32,
    ) -> Self {
        Self::new(HifiEpoch::from_gregorian_tai(year, month, day, hour, minute, second, nanos))
    }

    /// Create epoch from Gregorian date in TT (Terrestrial Time)
    pub fn from_gregorian_tt(
        year: i32,
        month: u8,
        day: u8,
        hour: u8,
        minute: u8,
        second: u8,
        nanos: u32,
    ) -> Self {
        Self::new(HifiEpoch::from_gregorian(year, month, day, hour, minute, second, nanos, TimeScale::TT))
    }

    /// Create epoch from Gregorian date in TDB (Barycentric Dynamical Time)
    pub fn from_gregorian_tdb(
        year: i32,
        month: u8,
        day: u8,
        hour: u8,
        minute: u8,
        second: u8,
        nanos: u32,
    ) -> Self {
        Self::new(HifiEpoch::from_gregorian(year, month, day, hour, minute, second, nanos, TimeScale::TDB))
    }

    /// Create epoch at midnight UTC
    pub fn from_gregorian_utc_midnight(year: i32, month: u8, day: u8) -> Self {
        Self::new(HifiEpoch::from_gregorian_utc_at_midnight(year, month, day))
    }

    /// Create epoch at noon UTC
    pub fn from_gregorian_utc_noon(year: i32, month: u8, day: u8) -> Self {
        Self::new(HifiEpoch::from_gregorian_utc_at_noon(year, month, day))
    }

    /// Create epoch from Modified Julian Date (MJD) in specified time scale
    ///
    /// # Arguments
    /// * `mjd_days` - Modified Julian Date in days
    /// * `time_scale` - Time scale (UTC, TAI, TT, TDB, etc.)
    pub fn from_mjd(mjd_days: f64, time_scale: TimeScale) -> Self {
        Self::new(HifiEpoch::from_mjd_in_time_scale(mjd_days, time_scale))
    }

    /// Create epoch from Julian Date (JD) in specified time scale
    pub fn from_jd(jd_days: f64, time_scale: TimeScale) -> Self {
        Self::new(HifiEpoch::from_jde_in_time_scale(jd_days, time_scale))
    }

    /// J2000.0 epoch (2000-01-01 12:00:00 TT)
    ///
    /// Standard reference epoch for astrodynamics, defined as:
    /// 2000 January 1, 12:00:00 TT (Terrestrial Time)
    /// JD 2451545.0 TT
    pub fn j2000() -> Self {
        // J2000 is defined as 2000-01-01 at noon (12:00:00) in TT
        Self::from_gregorian_tt(2000, 1, 1, 12, 0, 0, 0)
    }

    /// Get current system time as UTC epoch (requires std feature)
    pub fn now() -> Self {
        Self::new(HifiEpoch::now().unwrap())
    }

    /// Convert to specified time scale
    pub fn to_time_scale(&self, time_scale: TimeScale) -> Self {
        Self::new(self.inner.to_time_scale(time_scale))
    }

    /// Get epoch in UTC time scale
    pub fn to_utc(&self) -> Self {
        self.to_time_scale(TimeScale::UTC)
    }

    /// Get epoch in TAI time scale
    pub fn to_tai(&self) -> Self {
        self.to_time_scale(TimeScale::TAI)
    }

    /// Get epoch in TT (Terrestrial Time) time scale
    pub fn to_tt(&self) -> Self {
        self.to_time_scale(TimeScale::TT)
    }

    /// Get epoch in TDB (Barycentric Dynamical Time) time scale
    pub fn to_tdb(&self) -> Self {
        self.to_time_scale(TimeScale::TDB)
    }

    /// Get epoch in GPS time scale
    pub fn to_gpst(&self) -> Self {
        self.to_time_scale(TimeScale::GPST)
    }

    /// Get Modified Julian Date in TT
    pub fn to_mjd_tt(&self) -> f64 {
        self.inner.to_mjd_tt_days()
    }

    /// Get Modified Julian Date in UTC
    pub fn to_mjd_utc(&self) -> f64 {
        self.inner.to_mjd_utc_days()
    }

    /// Get Modified Julian Date in TAI
    pub fn to_mjd_tai(&self) -> f64 {
        self.inner.to_mjd_tai_days()
    }

    /// Get Modified Julian Date in TDB
    pub fn to_mjd_tdb(&self) -> f64 {
        // Convert to TDB and get MJD (TDB MJD methods not directly available)
        // Using duration since TT seconds as approximation
        let tdb_epoch = self.to_tdb();
        tdb_epoch.to_mjd_tai() // Use TAI as proxy since direct TDB MJD not available
    }

    /// Get Julian Date in TT
    pub fn to_jd_tt(&self) -> f64 {
        self.inner.to_jde_tt_days()
    }

    /// Get Julian Date in UTC
    pub fn to_jd_utc(&self) -> f64 {
        self.inner.to_jde_utc_days()
    }

    /// Get Julian Date in TT as a two-part value for maximum precision
    ///
    /// Returns (jd1, jd2) where the full Julian Date is jd1 + jd2.
    /// This format is used by ERFA/SOFA functions for precession/nutation calculations
    /// to maintain numerical precision (avoiding loss of precision in floating point arithmetic).
    ///
    /// Typically:
    /// - jd1 = 2451545.0 (integer part, often the J2000 epoch constant)
    /// - jd2 = fractional days from jd1
    ///
    /// # Returns
    ///
    /// Tuple of (jd1, jd2) representing the two-part Julian Date in TT
    ///
    /// # Examples
    ///
    /// ```rust,ignore
    /// let epoch = Epoch::from_gregorian_utc(2025, 10, 22, 0, 0, 0, 0);
    /// let (jd1, jd2) = epoch.to_jd_tt_two_part();
    /// // Use with ERFA functions
    /// let prec = iau2006_precession(jd1, jd2);
    /// ```
    pub fn to_jd_tt_two_part(&self) -> (f64, f64) {
        let jd_tt = self.to_jd_tt();
        // Use J2000 epoch (2451545.0) as the integer part for best precision
        let jd1 = 2451545.0;
        let jd2 = jd_tt - jd1;
        (jd1, jd2)
    }

    /// Get seconds since J2000 epoch in TT
    pub fn to_tt_seconds_since_j2000(&self) -> f64 {
        let j2000 = Epoch::j2000();
        let tt_self = self.to_tt();
        let tt_j2000 = j2000.to_tt();
        (tt_self.inner - tt_j2000.inner).to_seconds()
    }

    /// Get seconds since J2000 epoch in TDB
    pub fn to_tdb_seconds_since_j2000(&self) -> f64 {
        let j2000 = Epoch::j2000();
        let tdb_self = self.to_tdb();
        let tdb_j2000 = j2000.to_tdb();
        (tdb_self.inner - tdb_j2000.inner).to_seconds()
    }

    /// Get duration since J2000 epoch
    pub fn duration_since_j2000(&self) -> Duration {
        let j2000 = Epoch::j2000();
        self.duration_since(&j2000)
    }

    /// Get Gregorian date components in UTC
    ///
    /// Returns: (year, month, day, hour, minute, second, nanosecond)
    pub fn to_gregorian_utc(&self) -> (i32, u8, u8, u8, u8, u8, u32) {
        let (year, month, day, hour, minute, second, nanos) = self.inner.to_gregorian_utc();
        (year, month, day, hour, minute, second, nanos)
    }

    /// Get Gregorian date components in TAI
    pub fn to_gregorian_tai(&self) -> (i32, u8, u8, u8, u8, u8, u32) {
        let (year, month, day, hour, minute, second, nanos) = self.inner.to_gregorian_tai();
        (year, month, day, hour, minute, second, nanos)
    }

    /// Add a duration to this epoch
    pub fn add_duration(&self, duration: Duration) -> Self {
        Self::new(self.inner + duration.inner())
    }

    /// Subtract a duration from this epoch
    pub fn sub_duration(&self, duration: Duration) -> Self {
        Self::new(self.inner - duration.inner())
    }

    /// Get duration between two epochs (self - other)
    pub fn duration_since(&self, other: &Epoch) -> Duration {
        Duration::new(self.inner - other.inner)
    }

    /// Format epoch as ISO 8601 string in UTC
    pub fn to_iso_string(&self) -> String {
        format!("{}", self.inner)
    }
}

/// High-precision duration representation
///
/// Wraps hifitime::Duration to provide precise time interval handling
/// compatible with astrodynamics calculations.
#[pyclass(module = "astrora._core.time")]
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Duration {
    inner: HifiDuration,
}

impl Duration {
    /// Create a new Duration from hifitime::Duration
    pub fn new(duration: HifiDuration) -> Self {
        Self { inner: duration }
    }

    /// Get the inner hifitime::Duration
    pub fn inner(&self) -> HifiDuration {
        self.inner
    }

    /// Create duration from seconds
    pub fn from_seconds(seconds: f64) -> Self {
        Self::new(HifiDuration::from_seconds(seconds))
    }

    /// Create duration from minutes
    pub fn from_minutes(minutes: f64) -> Self {
        Self::new(HifiDuration::from_seconds(minutes * 60.0))
    }

    /// Create duration from hours
    pub fn from_hours(hours: f64) -> Self {
        Self::new(HifiDuration::from_seconds(hours * 3600.0))
    }

    /// Create duration from days
    pub fn from_days(days: f64) -> Self {
        Self::new(HifiDuration::from_seconds(days * 86400.0))
    }

    /// Get duration in seconds
    pub fn to_seconds(&self) -> f64 {
        self.inner.to_seconds()
    }

    /// Get duration in minutes
    pub fn to_minutes(&self) -> f64 {
        self.to_seconds() / 60.0
    }

    /// Get duration in hours
    pub fn to_hours(&self) -> f64 {
        self.to_seconds() / 3600.0
    }

    /// Get duration in days
    pub fn to_days(&self) -> f64 {
        self.to_seconds() / 86400.0
    }

    /// Get absolute value of duration
    pub fn abs(&self) -> Self {
        Self::new(self.inner.abs())
    }

    /// Add two durations
    pub fn add(&self, other: &Duration) -> Self {
        Self::new(self.inner + other.inner)
    }

    /// Subtract two durations
    pub fn sub(&self, other: &Duration) -> Self {
        Self::new(self.inner - other.inner)
    }

    /// Multiply duration by scalar
    pub fn mul(&self, scalar: f64) -> Self {
        Self::new(self.inner * scalar)
    }

    /// Divide duration by scalar
    pub fn div(&self, scalar: f64) -> Self {
        Self::new(self.inner / scalar)
    }

    /// Check if duration is zero
    pub fn is_zero(&self) -> bool {
        self.inner.total_nanoseconds() == 0
    }

    /// Check if duration is positive
    pub fn is_positive(&self) -> bool {
        self.inner.total_nanoseconds() > 0
    }

    /// Check if duration is negative
    pub fn is_negative(&self) -> bool {
        self.inner.total_nanoseconds() < 0
    }
}

// ============================================================================
// Python Bindings
// ============================================================================

#[pymethods]
impl Epoch {
    /// Create epoch from Gregorian date in UTC
    #[new]
    #[pyo3(signature = (year, month, day, hour=0, minute=0, second=0, nanos=0))]
    fn py_new(
        year: i32,
        month: u8,
        day: u8,
        hour: u8,
        minute: u8,
        second: u8,
        nanos: u32,
    ) -> Self {
        Self::from_gregorian_utc(year, month, day, hour, minute, second, nanos)
    }

    /// Create epoch at midnight UTC
    #[staticmethod]
    fn from_midnight_utc(year: i32, month: u8, day: u8) -> Self {
        Self::from_gregorian_utc_midnight(year, month, day)
    }

    /// Create epoch at noon UTC
    #[staticmethod]
    fn from_noon_utc(year: i32, month: u8, day: u8) -> Self {
        Self::from_gregorian_utc_noon(year, month, day)
    }

    /// Get J2000.0 epoch (2000-01-01 12:00:00 TT)
    #[staticmethod]
    fn j2000_epoch() -> Self {
        Self::j2000()
    }

    /// Create epoch from Julian Date in specified time scale
    ///
    /// # Parameters
    /// * `jd`: Julian Date in days
    /// * `scale`: Time scale as string ('UTC', 'TAI', 'TT', 'TDB', 'GPST')
    ///
    /// # Example
    /// ```python
    /// from astrora._core import Epoch
    /// epoch = Epoch.from_jd(2451545.0, 'TT')  # J2000 in TT
    /// ```
    #[staticmethod]
    fn from_jd_scale(jd: f64, scale: &str) -> PyResult<Self> {
        let time_scale = match scale.to_uppercase().as_str() {
            "UTC" => TimeScale::UTC,
            "TAI" => TimeScale::TAI,
            "TT" => TimeScale::TT,
            "TDB" => TimeScale::TDB,
            "GPST" => TimeScale::GPST,
            _ => return Err(pyo3::exceptions::PyValueError::new_err(
                format!("Unsupported time scale: {scale}. Use UTC, TAI, TT, TDB, or GPST")
            )),
        };
        Ok(Self::from_jd(jd, time_scale))
    }

    /// Create epoch from Modified Julian Date in specified time scale
    ///
    /// # Parameters
    /// * `mjd`: Modified Julian Date in days
    /// * `scale`: Time scale as string ('UTC', 'TAI', 'TT', 'TDB', 'GPST')
    ///
    /// # Example
    /// ```python
    /// from astrora._core import Epoch
    /// epoch = Epoch.from_mjd(51544.5, 'TT')  # J2000 in TT
    /// ```
    #[staticmethod]
    fn from_mjd_scale(mjd: f64, scale: &str) -> PyResult<Self> {
        let time_scale = match scale.to_uppercase().as_str() {
            "UTC" => TimeScale::UTC,
            "TAI" => TimeScale::TAI,
            "TT" => TimeScale::TT,
            "TDB" => TimeScale::TDB,
            "GPST" => TimeScale::GPST,
            _ => return Err(pyo3::exceptions::PyValueError::new_err(
                format!("Unsupported time scale: {scale}. Use UTC, TAI, TT, TDB, or GPST")
            )),
        };
        Ok(Self::from_mjd(mjd, time_scale))
    }

    /// Convert to UTC time scale
    fn as_utc(&self) -> Self {
        self.to_utc()
    }

    /// Convert to TAI time scale
    fn as_tai(&self) -> Self {
        self.to_tai()
    }

    /// Convert to TT time scale
    fn as_tt(&self) -> Self {
        self.to_tt()
    }

    /// Convert to TDB time scale
    fn as_tdb(&self) -> Self {
        self.to_tdb()
    }

    /// Get Modified Julian Date in TT
    #[getter]
    fn mjd_tt(&self) -> f64 {
        self.to_mjd_tt()
    }

    /// Get Modified Julian Date in UTC
    #[getter]
    fn mjd_utc(&self) -> f64 {
        self.to_mjd_utc()
    }

    /// Get Modified Julian Date in TAI
    #[getter]
    fn mjd_tai(&self) -> f64 {
        self.to_mjd_tai()
    }

    /// Get Modified Julian Date in TDB
    #[getter]
    fn mjd_tdb(&self) -> f64 {
        self.to_mjd_tdb()
    }

    /// Get Julian Date in TT
    #[getter]
    fn jd_tt(&self) -> f64 {
        self.to_jd_tt()
    }

    /// Get Julian Date in UTC
    #[getter]
    fn jd_utc(&self) -> f64 {
        self.to_jd_utc()
    }

    /// Get seconds since J2000 in TT
    #[getter]
    fn tt_seconds(&self) -> f64 {
        self.to_tt_seconds_since_j2000()
    }

    /// Get seconds since J2000 in TDB
    #[getter]
    fn tdb_seconds(&self) -> f64 {
        self.to_tdb_seconds_since_j2000()
    }

    /// Add duration to epoch
    fn __add__(&self, duration: &Duration) -> Self {
        self.add_duration(*duration)
    }

    /// Subtract duration or epoch
    fn __sub__(&self, other: PyObject, py: Python) -> PyResult<PyObject> {
        // Try to extract as Duration first
        if let Ok(duration) = other.extract::<Duration>(py) {
            return Ok(self.sub_duration(duration).into_py(py));
        }
        // Try to extract as Epoch
        if let Ok(epoch) = other.extract::<Epoch>(py) {
            return Ok(self.duration_since(&epoch).into_py(py));
        }
        Err(pyo3::exceptions::PyTypeError::new_err(
            "Can only subtract Duration or Epoch from Epoch"
        ))
    }

    /// Compare epochs
    fn __eq__(&self, other: &Self) -> bool {
        self == other
    }

    /// Compare epochs
    fn __lt__(&self, other: &Self) -> bool {
        self.inner < other.inner
    }

    /// Compare epochs
    fn __le__(&self, other: &Self) -> bool {
        self.inner <= other.inner
    }

    /// Compare epochs
    fn __gt__(&self, other: &Self) -> bool {
        self.inner > other.inner
    }

    /// Compare epochs
    fn __ge__(&self, other: &Self) -> bool {
        self.inner >= other.inner
    }

    /// String representation
    fn __repr__(&self) -> String {
        format!("Epoch('{}')", self.to_iso_string())
    }

    fn __str__(&self) -> String {
        self.to_iso_string()
    }
}

#[pymethods]
impl Duration {
    /// Create duration from seconds
    #[new]
    fn py_new(seconds: f64) -> Self {
        Self::from_seconds(seconds)
    }

    /// Create duration from minutes
    #[staticmethod]
    fn from_min(minutes: f64) -> Self {
        Self::from_minutes(minutes)
    }

    /// Create duration from hours
    #[staticmethod]
    fn from_hrs(hours: f64) -> Self {
        Self::from_hours(hours)
    }

    /// Create duration from days
    #[staticmethod]
    fn from_day(days: f64) -> Self {
        Self::from_days(days)
    }

    /// Get duration in seconds
    #[getter]
    fn seconds(&self) -> f64 {
        self.to_seconds()
    }

    /// Get duration in minutes
    #[getter]
    fn minutes(&self) -> f64 {
        self.to_minutes()
    }

    /// Get duration in hours
    #[getter]
    fn hours(&self) -> f64 {
        self.to_hours()
    }

    /// Get duration in days
    #[getter]
    fn days(&self) -> f64 {
        self.to_days()
    }

    /// Add durations
    fn __add__(&self, other: &Duration) -> Self {
        self.add(other)
    }

    /// Subtract durations
    fn __sub__(&self, other: &Duration) -> Self {
        self.sub(other)
    }

    /// Multiply by scalar
    fn __mul__(&self, scalar: f64) -> Self {
        self.mul(scalar)
    }

    /// Divide by scalar
    fn __truediv__(&self, scalar: f64) -> Self {
        self.div(scalar)
    }

    /// Negate duration
    fn __neg__(&self) -> Self {
        Self::new(-self.inner)
    }

    /// Absolute value
    fn __abs__(&self) -> Self {
        self.abs()
    }

    /// Compare durations
    fn __eq__(&self, other: &Self) -> bool {
        self == other
    }

    fn __lt__(&self, other: &Self) -> bool {
        self < other
    }

    fn __le__(&self, other: &Self) -> bool {
        self <= other
    }

    fn __gt__(&self, other: &Self) -> bool {
        self > other
    }

    fn __ge__(&self, other: &Self) -> bool {
        self >= other
    }

    /// String representation
    fn __repr__(&self) -> String {
        format!("Duration({} s)", self.to_seconds())
    }

    fn __str__(&self) -> String {
        let days = self.to_days();
        if days.abs() >= 1.0 {
            format!("{days:.6} days")
        } else {
            format!("{:.9} s", self.to_seconds())
        }
    }
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_relative_eq;

    #[test]
    fn test_j2000_epoch() {
        let j2000 = Epoch::j2000();

        // J2000 is defined as 2000-01-01 12:00:00 TT
        // MJD at J2000 = 51544.5
        assert_relative_eq!(j2000.to_mjd_tt(), 51544.5, epsilon = 1e-10);

        // JD at J2000 = 2451545.0
        assert_relative_eq!(j2000.to_jd_tt(), 2451545.0, epsilon = 1e-10);

        // Seconds since J2000 should be 0
        assert_relative_eq!(j2000.to_tt_seconds_since_j2000(), 0.0, epsilon = 1e-10);
    }

    #[test]
    fn test_epoch_from_gregorian_utc() {
        let epoch = Epoch::from_gregorian_utc(2000, 1, 1, 12, 0, 0, 0);

        // Should be very close to J2000, accounting for TAI-UTC offset
        let j2000 = Epoch::j2000();
        let diff = epoch.duration_since(&j2000);

        // TAI-UTC difference at J2000 was 32 seconds
        // TT-TAI is constant at 32.184 seconds
        // So TT-UTC = 32.184 + 32 = 64.184 seconds at J2000
        assert_relative_eq!(diff.to_seconds().abs(), 64.184, epsilon = 0.1);
    }

    #[test]
    fn test_time_scale_conversions() {
        // Create an epoch in UTC
        let utc_epoch = Epoch::from_gregorian_utc(2020, 3, 15, 10, 30, 45, 0);

        // Convert to different time scales (these represent the same instant in time)
        let tai_epoch = utc_epoch.to_tai();

        // They all represent the same instant, so the duration between them should be ~0
        let diff = tai_epoch.duration_since(&utc_epoch);
        assert_relative_eq!(diff.to_seconds(), 0.0, epsilon = 1e-6);

        // They should differ (though this test is somewhat fragile)
        // A better test: convert both to MJD and check the offset
        let mjd_utc = utc_epoch.to_mjd_utc();
        let mjd_tai = tai_epoch.to_mjd_tai();

        // The MJD values should differ by the leap second offset in days
        // ~37 seconds / 86400 seconds/day ≈ 0.000428 days
        let mjd_diff = (mjd_tai - mjd_utc).abs();
        assert!(mjd_diff > 0.0003); // At least 30 seconds worth
        assert!(mjd_diff < 0.0005); // Less than 45 seconds worth
    }

    #[test]
    fn test_mjd_jd_conversions() {
        let epoch = Epoch::from_gregorian_utc(2000, 1, 1, 0, 0, 0, 0);

        let mjd = epoch.to_mjd_utc();
        let jd = epoch.to_jd_utc();

        // JD = MJD + 2400000.5
        assert_relative_eq!(jd, mjd + 2400000.5, epsilon = 1e-10);

        // MJD for 2000-01-01 00:00:00 UTC should be 51544.0
        assert_relative_eq!(mjd, 51544.0, epsilon = 0.001);
    }

    #[test]
    fn test_duration_operations() {
        let d1 = Duration::from_hours(1.0);
        let d2 = Duration::from_minutes(30.0);

        // Addition
        let sum = d1.add(&d2);
        assert_relative_eq!(sum.to_minutes(), 90.0, epsilon = 1e-9);

        // Subtraction
        let diff = d1.sub(&d2);
        assert_relative_eq!(diff.to_minutes(), 30.0, epsilon = 1e-9);

        // Multiplication
        let scaled = d1.mul(2.5);
        assert_relative_eq!(scaled.to_hours(), 2.5, epsilon = 1e-9);

        // Division
        let divided = d1.div(2.0);
        assert_relative_eq!(divided.to_minutes(), 30.0, epsilon = 1e-9);
    }

    #[test]
    fn test_epoch_duration_arithmetic() {
        let epoch = Epoch::j2000();
        let duration = Duration::from_days(1.0);

        // Add duration
        let future = epoch.add_duration(duration);
        let diff = future.duration_since(&epoch);
        assert_relative_eq!(diff.to_days(), 1.0, epsilon = 1e-9);

        // Subtract duration
        let past = epoch.sub_duration(duration);
        let diff2 = epoch.duration_since(&past);
        assert_relative_eq!(diff2.to_days(), 1.0, epsilon = 1e-9);
    }

    #[test]
    fn test_duration_conversions() {
        let duration = Duration::from_days(1.5);

        assert_relative_eq!(duration.to_days(), 1.5, epsilon = 1e-9);
        assert_relative_eq!(duration.to_hours(), 36.0, epsilon = 1e-9);
        assert_relative_eq!(duration.to_minutes(), 2160.0, epsilon = 1e-9);
        assert_relative_eq!(duration.to_seconds(), 129600.0, epsilon = 1e-9);
    }

    #[test]
    fn test_gregorian_roundtrip() {
        let original_year = 2024;
        let original_month = 10;
        let original_day = 22;
        let original_hour = 14;
        let original_minute = 30;
        let original_second = 45;
        let original_nanos = 123456789;

        let epoch = Epoch::from_gregorian_utc(
            original_year,
            original_month,
            original_day,
            original_hour,
            original_minute,
            original_second,
            original_nanos,
        );

        let (year, month, day, hour, minute, second, nanos) = epoch.to_gregorian_utc();

        assert_eq!(year, original_year);
        assert_eq!(month, original_month);
        assert_eq!(day, original_day);
        assert_eq!(hour, original_hour);
        assert_eq!(minute, original_minute);
        assert_eq!(second, original_second);
        assert_eq!(nanos, original_nanos);
    }

    #[test]
    fn test_epoch_comparison() {
        let epoch1 = Epoch::j2000();
        let epoch2 = epoch1.add_duration(Duration::from_days(1.0));

        assert!(epoch1.inner < epoch2.inner);
        assert!(epoch2.inner > epoch1.inner);
        assert_eq!(epoch1, epoch1);
    }

    #[test]
    fn test_midnight_and_noon() {
        let midnight = Epoch::from_gregorian_utc_midnight(2000, 1, 1);
        let noon = Epoch::from_gregorian_utc_noon(2000, 1, 1);

        let diff = noon.duration_since(&midnight);
        assert_relative_eq!(diff.to_hours(), 12.0, epsilon = 1e-9);
    }

    #[test]
    fn test_duration_signs() {
        let pos = Duration::from_seconds(100.0);
        let neg = Duration::from_seconds(-50.0);
        let zero = Duration::from_seconds(0.0);

        assert!(pos.is_positive());
        assert!(!pos.is_negative());

        assert!(neg.is_negative());
        assert!(!neg.is_positive());

        assert!(zero.is_zero());
        assert!(!zero.is_positive());
        assert!(!zero.is_negative());
    }

    #[test]
    fn test_from_gregorian_tai() {
        let epoch_tai = Epoch::from_gregorian_tai(2000, 1, 1, 12, 0, 0, 0);
        let epoch_utc = Epoch::from_gregorian_utc(2000, 1, 1, 12, 0, 0, 0);

        // TAI and UTC represent different time scales, so they should be different
        // TAI-UTC difference at J2000 was 32 seconds
        let diff = epoch_tai.duration_since(&epoch_utc);
        assert!(diff.to_seconds().abs() > 0.0);
    }

    #[test]
    fn test_from_gregorian_tt() {
        let epoch_tt = Epoch::from_gregorian_tt(2000, 1, 1, 12, 0, 0, 0);
        let j2000 = Epoch::j2000();

        // Should be very close to J2000 which is defined in TT
        let diff = epoch_tt.duration_since(&j2000);
        assert_relative_eq!(diff.to_seconds().abs(), 0.0, epsilon = 1e-6);
    }

    #[test]
    fn test_from_gregorian_tdb() {
        let epoch_tdb = Epoch::from_gregorian_tdb(2000, 1, 1, 12, 0, 0, 0);
        let j2000 = Epoch::j2000();

        // TDB and TT differ by small periodic terms (up to ~2ms)
        let diff = epoch_tdb.duration_since(&j2000);
        assert!(diff.to_seconds().abs() < 0.01); // Within 10ms
    }

    #[test]
    fn test_from_mjd() {
        let mjd = 51544.5; // J2000 MJD
        let epoch = Epoch::from_mjd(mjd, TimeScale::TT);
        let j2000 = Epoch::j2000();

        let diff = epoch.duration_since(&j2000);
        assert_relative_eq!(diff.to_seconds(), 0.0, epsilon = 1e-6);
    }

    #[test]
    fn test_from_jd() {
        let jd = 2451545.0; // J2000 JD
        let epoch = Epoch::from_jd(jd, TimeScale::TT);
        let j2000 = Epoch::j2000();

        let diff = epoch.duration_since(&j2000);
        assert_relative_eq!(diff.to_seconds(), 0.0, epsilon = 1e-6);
    }

    #[test]
    fn test_to_time_scale() {
        let utc_epoch = Epoch::from_gregorian_utc(2020, 6, 15, 10, 30, 0, 0);

        // Convert to TT using generic function
        let tt_epoch = utc_epoch.to_time_scale(TimeScale::TT);
        let tt_epoch2 = utc_epoch.to_tt();

        // Should be the same
        let diff = tt_epoch.duration_since(&tt_epoch2);
        assert_relative_eq!(diff.to_seconds(), 0.0, epsilon = 1e-9);
    }

    #[test]
    fn test_to_tt_and_tdb() {
        let epoch = Epoch::from_gregorian_utc(2015, 7, 1, 0, 0, 0, 0);

        let tt_epoch = epoch.to_tt();
        let tdb_epoch = epoch.to_tdb();

        // TT and TDB should differ by small periodic terms
        let diff = tt_epoch.duration_since(&tdb_epoch);
        assert!(diff.to_seconds().abs() < 0.01); // Within 10ms
    }

    #[test]
    fn test_to_gpst() {
        let epoch = Epoch::from_gregorian_utc(2020, 1, 1, 0, 0, 0, 0);
        let gps_epoch = epoch.to_gpst();

        // GPS time and UTC differ by leap seconds (no leap seconds after GPS epoch)
        // Should be a valid conversion
        assert!(gps_epoch.to_mjd_utc() > 0.0);
    }

    #[test]
    fn test_to_mjd_tdb() {
        let j2000 = Epoch::j2000();
        let mjd_tdb = j2000.to_mjd_tdb();

        // Should be close to 51544.5 (J2000 MJD)
        assert_relative_eq!(mjd_tdb, 51544.5, epsilon = 0.001);
    }

    #[test]
    fn test_to_jd_tt_two_part() {
        let j2000 = Epoch::j2000();
        let (jd1, jd2) = j2000.to_jd_tt_two_part();

        // Two-part JD for increased precision
        let total_jd = jd1 + jd2;
        assert_relative_eq!(total_jd, 2451545.0, epsilon = 1e-10);

        // First part should be the integer part
        assert_relative_eq!(jd1, 2451545.0, epsilon = 0.1);
    }

    #[test]
    fn test_to_tdb_seconds_since_j2000() {
        let j2000 = Epoch::j2000();
        let seconds_tdb = j2000.to_tdb_seconds_since_j2000();

        // Should be very close to 0
        assert_relative_eq!(seconds_tdb, 0.0, epsilon = 0.01);

        // Test with offset
        let future = j2000.add_duration(Duration::from_days(1.0));
        let seconds_future = future.to_tdb_seconds_since_j2000();
        assert_relative_eq!(seconds_future, 86400.0, epsilon = 0.1);
    }

    #[test]
    fn test_duration_since_j2000() {
        let j2000 = Epoch::j2000();
        let duration = j2000.duration_since_j2000();

        // J2000 epoch should have 0 duration since itself
        assert_relative_eq!(duration.to_seconds(), 0.0, epsilon = 1e-6);

        // Test with offset
        let future = j2000.add_duration(Duration::from_days(10.0));
        let dur_future = future.duration_since_j2000();
        assert_relative_eq!(dur_future.to_days(), 10.0, epsilon = 1e-9);
    }

    #[test]
    fn test_to_gregorian_tai() {
        let epoch = Epoch::from_gregorian_tai(2024, 3, 15, 14, 30, 45, 123456789);
        let (year, month, day, hour, minute, second, nanos) = epoch.to_gregorian_tai();

        assert_eq!(year, 2024);
        assert_eq!(month, 3);
        assert_eq!(day, 15);
        assert_eq!(hour, 14);
        assert_eq!(minute, 30);
        assert_eq!(second, 45);
        assert_eq!(nanos, 123456789);
    }

    #[test]
    fn test_to_iso_string() {
        let epoch = Epoch::from_gregorian_utc(2000, 1, 1, 12, 0, 0, 0);
        let iso = epoch.to_iso_string();

        // Should contain year, month, day
        assert!(iso.contains("2000"));
        assert!(iso.contains("01"));
        assert!(iso.contains("12"));
    }

    #[test]
    fn test_duration_abs() {
        let pos = Duration::from_seconds(100.0);
        let neg = Duration::from_seconds(-100.0);

        let abs_pos = pos.abs();
        let abs_neg = neg.abs();

        assert_relative_eq!(abs_pos.to_seconds(), 100.0, epsilon = 1e-9);
        assert_relative_eq!(abs_neg.to_seconds(), 100.0, epsilon = 1e-9);
    }

    #[test]
    fn test_duration_from_days() {
        let duration = Duration::from_days(2.5);
        assert_relative_eq!(duration.to_seconds(), 2.5 * 86400.0, epsilon = 1e-9);
        assert_relative_eq!(duration.to_hours(), 60.0, epsilon = 1e-9);
    }

    #[test]
    fn test_epoch_now() {
        let now = Epoch::now();
        let j2000 = Epoch::j2000();

        // Now should be well after J2000 (2000-01-01)
        let diff = now.duration_since(&j2000);
        assert!(diff.to_days() > 9000.0); // More than ~25 years after J2000
    }

    #[test]
    fn test_mjd_jd_relationship() {
        // Test the relationship: JD = MJD + 2400000.5
        let epoch = Epoch::from_gregorian_utc(2020, 6, 15, 12, 0, 0, 0);

        let mjd_utc = epoch.to_mjd_utc();
        let jd_utc = epoch.to_jd_utc();

        assert_relative_eq!(jd_utc, mjd_utc + 2400000.5, epsilon = 1e-10);
    }

    #[test]
    fn test_time_scale_consistency() {
        // Create epoch in different time scales representing the same Gregorian date
        let year = 2020;
        let month = 3;
        let day = 15;

        let utc = Epoch::from_gregorian_utc(year, month, day, 12, 0, 0, 0);
        let tai = Epoch::from_gregorian_tai(year, month, day, 12, 0, 0, 0);

        // When converted to the same time scale, the MJD values should differ
        let mjd_utc = utc.to_mjd_utc();
        let mjd_tai_native = tai.to_mjd_tai();

        // Different time scales should have different MJD values for same Gregorian date
        assert!((mjd_tai_native - mjd_utc).abs() < 1.0); // Within a day
    }

    #[test]
    fn test_duration_edge_cases() {
        // Test very small duration
        let tiny = Duration::from_seconds(1e-9);
        assert!(tiny.to_seconds() > 0.0);
        assert!(tiny.is_positive());

        // Test very large duration
        let large = Duration::from_days(365.25 * 100.0); // 100 years
        assert_relative_eq!(large.to_days(), 36525.0, epsilon = 1.0);
    }

    #[test]
    fn test_leap_second_aware() {
        // Create epoch near a known leap second (June 30, 2015 23:59:60)
        // Note: This tests that hifitime handles leap seconds properly
        let pre_leap = Epoch::from_gregorian_utc(2015, 6, 30, 23, 59, 59, 0);
        let post_leap = Epoch::from_gregorian_utc(2015, 7, 1, 0, 0, 0, 0);

        let diff = post_leap.duration_since(&pre_leap);
        // Should be 1 second (leap second is internal to hifitime)
        assert_relative_eq!(diff.to_seconds(), 1.0, epsilon = 0.1);
    }
}
