#[test]
fn fail() {
    let t = trybuild::TestCases::new();
    t.compile_fail("tests/fail/*.rs");
}

#[test]
fn pass() {
    macrotest::expand("tests/expand/*.rs");
}
