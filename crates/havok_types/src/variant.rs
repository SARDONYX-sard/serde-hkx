//! Variant: object pointer + class meta pointer
use crate::Pointer;

/// C++ info
/// - byte size: 8(x86)/ 16(x86_64)
///
/// Only used for `value` of `hkCustomAttributesAttribute`.
#[derive(Debug, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Variant {
    pub object: Pointer,
    /// `hkClass*` is a class that holds meta-information (Flags, vtable, etc.) for each C++ Havok class and is stored in its own static field.
    pub class: Pointer,
}

impl Variant {
    /// Creates a new `Variant`
    pub const fn new(object: Pointer, class: Pointer) -> Self {
        Self { object, class }
    }
}
