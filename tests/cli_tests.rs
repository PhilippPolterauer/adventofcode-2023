#[test]
fn cli_tests() {
    let t = trycmd::TestCases::new();
    t.case("tests/cmd/*.trycmd").case("tests/cmd/*.toml");
}
#[test]
fn trycmd() {
    let t = trycmd::TestCases::new().case("README.md");

}
