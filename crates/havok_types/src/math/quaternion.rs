use derive_new::new;
use parse_display::Display;

/// # XML
/// ```xml
/// <hkparam>(0.000000 0.000000 0.000000 1.000000)</hkparam>
/// ```
#[repr(C, align(16))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, Default, PartialEq, PartialOrd, Display, new)]
#[display("({x:.06} {y:.06} {z:.06} {scaler:.06})")]
pub struct Quaternion {
    x: f32,
    y: f32,
    z: f32,
    scaler: f32,
}
