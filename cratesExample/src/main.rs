#[path = "./tools/math_funcs.rs"]
mod math_funcs;

use crate::math_funcs::expressions::*; //importa tudo q tem no modulo expressions

// https://hackernoon.com/including-files-and-deeply-directories-in-rust-q35o3yer
fn main() {
    println!("Exemplo de como usar crates no rust");
    println!(
        "Resultado da add_numbers para 2 e 3: {}",
        add_numbers(2.0, 3.0)
    );
    println!(
        "Resultado da subtract_numbers para 2 e 3: {}",
        subtract_numbers(2.0, 3.0)
    );
    println!(
        "Resultado da multiply_numbers para 2 e 3: {}",
        multiply_numbers(2.0, 3.0)
    );
    println!(
        "Resultado da divide_numbers para 2 e 3: {:.2}",
        divide_numbers(2.0, 3.0)
    );
}
