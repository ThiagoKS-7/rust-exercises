#[path = "../traits/math_traits.rs"]
mod math_traits;

// importou o módulo das trait pra cá e dai chama ele numa caixa
use crate::math_structs::math_traits::expressions_traits::ShowResult;

pub struct Sum {
    pub a: f32,
    pub b: f32,
}
pub struct Subtract {
    pub a: f32,
    pub b: f32,
}
pub struct Multiply {
    pub a: f32,
    pub b: f32,
}
pub struct Divide {
    pub a: f32,
    pub b: f32,
}
impl ShowResult for Sum {
    fn show_result(&self) -> String {
        format!("Resultado da soma: {}", (self.a + self.b))
    }
}
impl ShowResult for Subtract {
    fn show_result(&self) -> String {
        format!("Resultado da subtracao: {}", (self.a - self.b))
    }
}
impl ShowResult for Multiply {
    fn show_result(&self) -> String {
        format!("Resultado da multiplicacao: {}", (self.a * self.b))
    }
}
impl ShowResult for Divide {
    fn show_result(&self) -> String {
        format!("Resultado da divisao: {:.2}", (self.a / self.b))
    }
}

pub mod calculadora {
    use crate::math_structs::math_traits::expressions_traits::ShowResult;

    pub fn mostra(exp: &impl ShowResult) {
        println!("{}", exp.show_result());
    }
}
