fn main() {}

#[test]
fn testa_tudo() {
    assert_cmd::Command::new("cargo")
        .arg("-q")
        .arg("-Zscript")
        .arg("src/examples/capitulo_01/01-01/main.rs")
        .arg("run")
        .assert()
        .success()
        .stdout("Teste");
}

