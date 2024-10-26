use colored::Colorize;
fn main() {
    println! (
        "\n {}\n {}\n {}\n {}\n {}\n {}\n {}\n {}\n\n",
        "Exemplos do livro \"O Livrão de Rust\"",
        "Disponível em:",
        " - xxx".green(),
        " - xxx".green(),
        " - xxx".green(),
        "Comandos de teste:",
        "rustup nightly".yellow(),
        "cargo test".yellow(),
    );
}

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

