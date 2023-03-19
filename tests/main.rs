#[test]
fn derive_builder() {
    let t = trybuild::TestCases::new();
    t.pass("tests/derive_builder/basic.rs");
    t.pass("tests/derive_builder/renamed_builder.rs");
    t.pass("tests/derive_builder/renamed_error.rs");
    t.pass("tests/derive_builder/optional_value.rs");
    t.pass("tests/derive_builder/default_values.rs");
    t.pass("tests/derive_builder/generic.rs");
    t.pass("tests/derive_builder/generic_where.rs");
}
