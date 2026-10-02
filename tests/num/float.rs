use Language::num::float::{FloatLiteral, FloatSuffix};

#[test]
fn test_float_literal_creation_with_all_suffixes() {
    let suffixes = [
        (FloatSuffix::F32, 3.14159),
        (FloatSuffix::F64, 2.718281828),
        (FloatSuffix::None, 1.414),
    ];

    for (suffix, val) in suffixes {
        let lit = FloatLiteral {
            value: val,
            suffix,
        };
        assert_eq!(lit.value, val);
        assert_eq!(lit.suffix, suffix);
    }
}

#[test]
fn test_float_literal_equality() {
    let lit_a = FloatLiteral { value: 1.5, suffix: FloatSuffix::F32 };
    let lit_b = FloatLiteral { value: 1.5, suffix: FloatSuffix::F32 };
    let lit_diff_val = FloatLiteral { value: 2.5, suffix: FloatSuffix::F32 };
    let lit_diff_suffix = FloatLiteral { value: 1.5, suffix: FloatSuffix::F64 };

    assert_eq!(lit_a, lit_b);
    assert_ne!(lit_a, lit_diff_val);
    assert_ne!(lit_a, lit_diff_suffix);
}

#[test]
fn test_float_literal_clone_and_copy() {
    let lit = FloatLiteral { value: 42.0, suffix: FloatSuffix::F64 };
    let copied = lit;
    let cloned = lit.clone();

    assert_eq!(lit, copied);
    assert_eq!(lit, cloned);
}

#[test]
fn test_float_literal_debug_format() {
    let lit = FloatLiteral { value: 0.125, suffix: FloatSuffix::F32 };
    let formatted = format!("{:?}", lit);
    assert!(formatted.contains("FloatLiteral"));
    assert!(formatted.contains("0.125"));
    assert!(formatted.contains("F32"));
}

#[test]
fn test_float_literal_edge_cases() {
    let zero = FloatLiteral { value: 0.0, suffix: FloatSuffix::None };
    assert_eq!(zero.value, 0.0);

    let neg_zero = FloatLiteral { value: -0.0, suffix: FloatSuffix::None };
    assert_eq!(neg_zero.value, -0.0);

    let large_float = FloatLiteral { value: 1e308, suffix: FloatSuffix::F64 };
    assert_eq!(large_float.value, 1e308);

    let small_float = FloatLiteral { value: 1e-308, suffix: FloatSuffix::F64 };
    assert_eq!(small_float.value, 1e-308);
}
