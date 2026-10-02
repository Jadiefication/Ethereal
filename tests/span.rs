use Language::span::Span;

#[test]
fn test_span_creation_and_field_access() {
    let span = Span { start: 10, end: 25 };
    assert_eq!(span.start, 10);
    assert_eq!(span.end, 25);
}

#[test]
fn test_span_equality_and_inequality() {
    let span_a = Span { start: 5, end: 15 };
    let span_b = Span { start: 5, end: 15 };
    let span_c = Span { start: 5, end: 20 };
    let span_d = Span { start: 0, end: 15 };

    assert_eq!(span_a, span_b);
    assert_ne!(span_a, span_c);
    assert_ne!(span_a, span_d);
}

#[test]
fn test_span_clone_and_copy_semantics() {
    let span = Span {
        start: 42,
        end: 100,
    };
    let copied = span;
    let cloned = span.clone();

    assert_eq!(span, copied);
    assert_eq!(span, cloned);
}

#[test]
fn test_span_debug_representation() {
    let span = Span { start: 1, end: 2 };
    let debug_str = format!("{:?}", span);
    assert!(debug_str.contains("Span"));
    assert!(debug_str.contains("start: 1"));
    assert!(debug_str.contains("end: 2"));
}

#[test]
fn test_span_boundary_values() {
    let zero_span = Span { start: 0, end: 0 };
    assert_eq!(zero_span.start, 0);
    assert_eq!(zero_span.end, 0);

    let max_span = Span {
        start: u32::MAX - 1,
        end: u32::MAX,
    };
    assert_eq!(max_span.start, u32::MAX - 1);
    assert_eq!(max_span.end, u32::MAX);
}
