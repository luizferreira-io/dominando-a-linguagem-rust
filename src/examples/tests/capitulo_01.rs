
use crate::examples::tests::test_functions::{test_success, test_failure};

#[cfg(test)]

#[test]
fn listagem_01_01() {
    test_success("src/examples/capitulo_01/listagem_01_01/main.rs",
                 "Meu número é 42.\n"
    );
}

#[test]
fn listagem_01_02() {
    test_failure("src/examples/capitulo_01/listagem_01_02/main.rs");
}

#[test]
fn listagem_01_03() {
    test_success(
        "src/examples/capitulo_01/listagem_01_03/main.rs",
        "Meu número é 42.\nMeu número é 7.\n"
    );
}



