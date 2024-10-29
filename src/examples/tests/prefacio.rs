use crate::examples::tests::test_functions::test_failure;
#[cfg(test)]

#[test]
fn exemplo_01() {
    /*
    assert_cmd::Command::new("cargo")
        .arg("-q")
        .arg("-Zscript")
        .arg("src/examples/prefacio/exemplo_01/main.rs")
        .arg("run")
        .assert()
        .failure();
        //.stdout("Teste");
     */
    test_failure("src/examples/prefacio/exemplo_01/main.rs");
}