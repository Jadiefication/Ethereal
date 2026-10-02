use Language::num::int::{IntLiteral, IntSuffix};

#[test]
fn test_int_literal_creation_with_all_suffixes() {
    let suffixes = [
        (IntSuffix::U8, 255),
        (IntSuffix::U16, 65535),
        (IntSuffix::U32, 4294967295),
        (IntSuffix::U64, 9223372036854775807),
        (IntSuffix::I8, 127),
        (IntSuffix::I16, 32767),
        (IntSuffix::I32, 2147483647),
        (IntSuffix::I64, 9223372036854775807),
        (IntSuffix::None, 12345),
    ];

    for (suffix, val) in suffixes {
        let lit = IntLiteral { value: val, suffix };
        assert_eq!(lit.value, val);
        assert_eq!(lit.suffix, suffix);
    }
}

#[test]
fn test_int_literal_equality() {
    let lit_a = IntLiteral {
        value: 42,
        suffix: IntSuffix::U32,
    };
    let lit_b = IntLiteral {
        value: 42,
        suffix: IntSuffix::U32,
    };
    let lit_diff_val = IntLiteral {
        value: 43,
        suffix: IntSuffix::U32,
    };
    let lit_diff_suffix = IntLiteral {
        value: 42,
        suffix: IntSuffix::I32,
    };

    assert_eq!(lit_a, lit_b);
    assert_ne!(lit_a, lit_diff_val);
    assert_ne!(lit_a, lit_diff_suffix);
}

#[test]
fn test_int_literal_clone_and_copy() {
    let lit = IntLiteral {
        value: 100,
        suffix: IntSuffix::I64,
    };
    let copied = lit;
    let cloned = lit.clone();

    assert_eq!(lit, copied);
    assert_eq!(lit, cloned);
}

#[test]
fn test_int_literal_debug_format() {
    let lit = IntLiteral {
        value: 50,
        suffix: IntSuffix::U8,
    };
    let formatted = format!("{:?}", lit);
    assert!(formatted.contains("IntLiteral"));
    assert!(formatted.contains("value: 50"));
    assert!(formatted.contains("U8"));
}

#[test]
fn test_int_literal_boundary_values() {
    let zero = IntLiteral {
        value: 0,
        suffix: IntSuffix::None,
    };
    assert_eq!(zero.value, 0);

    let max_val = IntLiteral {
        value: i64::MAX,
        suffix: IntSuffix::I64,
    };
    assert_eq!(max_val.value, i64::MAX);

    let min_val = IntLiteral {
        value: i64::MIN,
        suffix: IntSuffix::I64,
    };
    assert_eq!(min_val.value, i64::MIN);
}
