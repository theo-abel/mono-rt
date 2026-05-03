/// The type discriminant returned by `mono_type_get_type`, corresponding to `MonoTypeEnum` in the
/// Mono source (`mono/metadata/metadata.h`).
///
/// Annotated with `#[non_exhaustive]` so that adding named variants in the future is not a
/// breaking change. The `Other` variant covers any discriminant not yet named here.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum TypeKind {
    /// `MONO_TYPE_END` (0x00) — internal sentinel.
    End,
    /// `MONO_TYPE_VOID` (0x01)
    Void,
    /// `MONO_TYPE_BOOLEAN` (0x02)
    Boolean,
    /// `MONO_TYPE_CHAR` (0x03)
    Char,
    /// `MONO_TYPE_I1` (0x04) — `sbyte`
    I1,
    /// `MONO_TYPE_U1` (0x05) — `byte`
    U1,
    /// `MONO_TYPE_I2` (0x06) — `short`
    I2,
    /// `MONO_TYPE_U2` (0x07) — `ushort`
    U2,
    /// `MONO_TYPE_I4` (0x08) — `int`
    I4,
    /// `MONO_TYPE_U4` (0x09) — `uint`
    U4,
    /// `MONO_TYPE_I8` (0x0a) — `long`
    I8,
    /// `MONO_TYPE_U8` (0x0b) — `ulong`
    U8,
    /// `MONO_TYPE_R4` (0x0c) — `float`
    R4,
    /// `MONO_TYPE_R8` (0x0d) — `double`
    R8,
    /// `MONO_TYPE_STRING` (0x0e)
    String,
    /// `MONO_TYPE_PTR` (0x0f) — unmanaged pointer
    Ptr,
    /// `MONO_TYPE_BYREF` (0x10) — by-reference parameter
    ByRef,
    /// `MONO_TYPE_VALUETYPE` (0x11) — value type / struct
    ValueType,
    /// `MONO_TYPE_CLASS` (0x12) — reference type
    Class,
    /// `MONO_TYPE_VAR` (0x13) — generic type parameter (`T`)
    Var,
    /// `MONO_TYPE_ARRAY` (0x14) — multi-dimensional array
    Array,
    /// `MONO_TYPE_GENERICINST` (0x15) — instantiated generic type
    GenericInst,
    /// `MONO_TYPE_TYPEDBYREF` (0x16) — `TypedReference`
    TypedByRef,
    /// `MONO_TYPE_I` (0x18) — `nint`
    I,
    /// `MONO_TYPE_U` (0x19) — `nuint`
    U,
    /// `MONO_TYPE_FNPTR` (0x1b) — function pointer
    FnPtr,
    /// `MONO_TYPE_OBJECT` (0x1c) — `System.Object`
    Object,
    /// `MONO_TYPE_SZARRAY` (0x1d) — single-dimension zero-based array (most C# arrays)
    SzArray,
    /// `MONO_TYPE_MVAR` (0x1e) — generic method parameter (`M`)
    MVar,
    /// Any discriminant not covered by the named variants above.
    Other(u32),
}

#[allow(clippy::too_many_lines)]
impl From<u32> for TypeKind {
    fn from(v: u32) -> Self {
        match v {
            0x00 => Self::End,
            0x01 => Self::Void,
            0x02 => Self::Boolean,
            0x03 => Self::Char,
            0x04 => Self::I1,
            0x05 => Self::U1,
            0x06 => Self::I2,
            0x07 => Self::U2,
            0x08 => Self::I4,
            0x09 => Self::U4,
            0x0a => Self::I8,
            0x0b => Self::U8,
            0x0c => Self::R4,
            0x0d => Self::R8,
            0x0e => Self::String,
            0x0f => Self::Ptr,
            0x10 => Self::ByRef,
            0x11 => Self::ValueType,
            0x12 => Self::Class,
            0x13 => Self::Var,
            0x14 => Self::Array,
            0x15 => Self::GenericInst,
            0x16 => Self::TypedByRef,
            0x18 => Self::I,
            0x19 => Self::U,
            0x1b => Self::FnPtr,
            0x1c => Self::Object,
            0x1d => Self::SzArray,
            0x1e => Self::MVar,
            other => Self::Other(other),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use super::TypeKind;

    fn check(cases: &[(u32, TypeKind)]) {
        for &(raw, ref expected) in cases {
            assert_eq!(TypeKind::from(raw), *expected, "discriminant 0x{raw:02x}");
        }
    }

    #[test]
    fn primitive_variants_map_correctly() {
        check(&[
            (0x00, TypeKind::End),
            (0x01, TypeKind::Void),
            (0x02, TypeKind::Boolean),
            (0x03, TypeKind::Char),
            (0x04, TypeKind::I1),
            (0x05, TypeKind::U1),
            (0x06, TypeKind::I2),
            (0x07, TypeKind::U2),
            (0x08, TypeKind::I4),
            (0x09, TypeKind::U4),
            (0x0a, TypeKind::I8),
            (0x0b, TypeKind::U8),
            (0x0c, TypeKind::R4),
            (0x0d, TypeKind::R8),
        ]);
    }

    #[test]
    fn reference_and_generic_variants_map_correctly() {
        check(&[
            (0x0e, TypeKind::String),
            (0x0f, TypeKind::Ptr),
            (0x10, TypeKind::ByRef),
            (0x11, TypeKind::ValueType),
            (0x12, TypeKind::Class),
            (0x13, TypeKind::Var),
            (0x14, TypeKind::Array),
            (0x15, TypeKind::GenericInst),
            (0x16, TypeKind::TypedByRef),
            (0x18, TypeKind::I),
            (0x19, TypeKind::U),
            (0x1b, TypeKind::FnPtr),
            (0x1c, TypeKind::Object),
            (0x1d, TypeKind::SzArray),
            (0x1e, TypeKind::MVar),
        ]);
    }

    #[test]
    fn gap_discriminants_produce_other() {
        // 0x17 and 0x1a are intentional gaps in the MONO_TYPE_* numbering
        assert_eq!(TypeKind::from(0x17), TypeKind::Other(0x17));
        assert_eq!(TypeKind::from(0x1a), TypeKind::Other(0x1a));
    }

    #[test]
    fn high_discriminant_produces_other() {
        assert_eq!(TypeKind::from(0xFF), TypeKind::Other(0xFF));
        assert_eq!(TypeKind::from(100), TypeKind::Other(100));
    }

    #[test]
    fn other_variants_equality_uses_inner_value() {
        assert_eq!(TypeKind::Other(7), TypeKind::Other(7));
        assert_ne!(TypeKind::Other(7), TypeKind::Other(8));
    }

    #[test]
    fn type_kind_is_hashable() {
        let mut set = HashSet::new();
        set.insert(TypeKind::I4);
        set.insert(TypeKind::I4);
        set.insert(TypeKind::U4);
        assert_eq!(set.len(), 2);
    }

    #[test]
    fn type_kind_is_copy() {
        let a = TypeKind::Boolean;
        let b = a;
        // both are usable after copy
        assert_eq!(a, b);
    }

    #[test]
    fn type_kind_debug_contains_variant_name() {
        assert!(format!("{:?}", TypeKind::I4).contains("I4"));
        assert!(format!("{:?}", TypeKind::Other(99)).contains("Other"));
    }
}
