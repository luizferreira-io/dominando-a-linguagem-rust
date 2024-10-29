use colored::Colorize;

mod examples;
fn main() {
    println! (
        "\n {}\n {}\n {}\n {}\n {}\n {}\n {}\n {}\n\n",
        "Exemplos do livro \"O Livrão de Rust\"",
        "Disponível em:",
        " - xxx".green(),
        " - xxx".green(),
        " - xxx".green(),
        "Comandos para teste automatizado:",
        "rustup default nightly".yellow(),
        "cargo test".yellow(),
    );
}

