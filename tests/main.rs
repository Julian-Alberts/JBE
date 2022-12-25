#[test]
fn derive_builder() {
    let t = trybuild::TestCases::new();
    t.pass("tests/derive_builder/basic.rs");
    t.pass("tests/derive_builder/renamed_builder.rs");
    t.pass("tests/derive_builder/renamed_error.rs");
    t.pass("tests/derive_builder/optional_value.rs");
    t.pass("tests/derive_builder/default_values.rs");
}

#[test]
fn derive_consuming_builder() {
    let t = trybuild::TestCases::new();
    t.pass("tests/derive_consuming_builder/basic.rs");
    t.compile_fail("tests/derive_consuming_builder/call_multiple_times.rs");
    t.pass("tests/derive_consuming_builder/renamed_builder.rs");
    t.pass("tests/derive_consuming_builder/optional_value.rs");
    t.pass("tests/derive_consuming_builder/default_values.rs");
}
