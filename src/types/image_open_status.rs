use std::fmt;

/// Mirrors `MonoImageOpenStatus` from `mono/metadata/image.h`.
///
/// Used as an output parameter by [`crate::MonoImage::open_from_data`] and
/// [`crate::MonoAssembly::load_from_image`]. Any variant other than `Ok` means the load failed.
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MonoImageOpenStatus {
    Ok = 0,
    ErrorErrno = 1,
    MissingAssemblyRef = 2,
    ImageInvalid = 3,
}

impl MonoImageOpenStatus {
    /// Converts a raw C enum integer to the corresponding variant.
    ///
    /// Unknown values map to `ImageInvalid` rather than panicking.
    #[must_use]
    pub fn from_raw(v: i32) -> Self {
        match v {
            0 => Self::Ok,
            1 => Self::ErrorErrno,
            2 => Self::MissingAssemblyRef,
            _ => Self::ImageInvalid,
        }
    }

    /// Returns `true` when the status indicates success.
    #[must_use]
    pub fn is_ok(self) -> bool {
        matches!(self, Self::Ok)
    }
}

impl fmt::Display for MonoImageOpenStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let msg = match self {
            Self::Ok => "ok",
            Self::ErrorErrno => "errno error",
            Self::MissingAssemblyRef => "missing assembly reference",
            Self::ImageInvalid => "invalid image",
        };
        f.write_str(msg)
    }
}

#[cfg(test)]
mod tests {
    use super::MonoImageOpenStatus;

    #[test]
    fn from_raw_known_values() {
        assert_eq!(MonoImageOpenStatus::from_raw(0), MonoImageOpenStatus::Ok);
        assert_eq!(
            MonoImageOpenStatus::from_raw(1),
            MonoImageOpenStatus::ErrorErrno
        );
        assert_eq!(
            MonoImageOpenStatus::from_raw(2),
            MonoImageOpenStatus::MissingAssemblyRef
        );
        assert_eq!(
            MonoImageOpenStatus::from_raw(3),
            MonoImageOpenStatus::ImageInvalid
        );
    }

    #[test]
    fn from_raw_unknown_maps_to_invalid() {
        assert_eq!(
            MonoImageOpenStatus::from_raw(99),
            MonoImageOpenStatus::ImageInvalid
        );
        assert_eq!(
            MonoImageOpenStatus::from_raw(-1),
            MonoImageOpenStatus::ImageInvalid
        );
    }

    #[test]
    fn is_ok_only_for_ok() {
        assert!(MonoImageOpenStatus::Ok.is_ok());
        assert!(!MonoImageOpenStatus::ErrorErrno.is_ok());
        assert!(!MonoImageOpenStatus::MissingAssemblyRef.is_ok());
        assert!(!MonoImageOpenStatus::ImageInvalid.is_ok());
    }

    #[test]
    fn display_messages() {
        assert_eq!(MonoImageOpenStatus::Ok.to_string(), "ok");
        assert_eq!(MonoImageOpenStatus::ErrorErrno.to_string(), "errno error");
        assert_eq!(
            MonoImageOpenStatus::MissingAssemblyRef.to_string(),
            "missing assembly reference"
        );
        assert_eq!(
            MonoImageOpenStatus::ImageInvalid.to_string(),
            "invalid image"
        );
    }
}
