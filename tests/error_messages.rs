#[test]
fn error_messages() {
    let t = trybuild::TestCases::new();
    t.compile_fail("tests/error_messages/*.rs");
}
