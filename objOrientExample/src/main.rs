#[path = "./structs/math_structs.rs"]
mod math_structs;

use crate::math_structs::{Divide, Multiply, Subtract, Sum};
use math_structs::calculadora::*;

fn main() {
    println!("Exemplo de stucts com polimorfismo e uso de traits!\n");
    mostra(&Sum { a: 2.0, b: 3.0 });
    mostra(&Subtract { a: 2.0, b: 3.0 });
    mostra(&Multiply { a: 2.0, b: 3.0 });
    mostra(&Divide { a: 2.0, b: 3.0 });
}

/*
MAIN REFS
-  https://codebr.net/artigo/programacao-orientada-a-objetos-em-rust#:~:text=Heran%C3%A7a%20em%20Rust&text=de%20%60Pessoa%60.-,A%20fun%C3%A7%C3%A3o%20%60nome%60%20%C3%A9%20p%C3%BAblica%20e%20pode%20ser%20usada%20para,um%20novo%20campo%20%60curso%60.
-  https://www.geeksforgeeks.org/rust-interface/
-  https://stackoverflow.com/questions/28655362/how-does-one-round-a-floating-point-number-to-a-specified-number-of-digits
-  https://hackernoon.com/including-files-and-deeply-directories-in-rust-q35o3yer

*/
