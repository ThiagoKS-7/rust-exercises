fn main() {
    // '{}' são trocados automaticamente por qualquer argumento
    // que vier depois
    println!("{} days", 31);

    // Argumentos posicionais também podem ser usados. Especificar um inteiro dentro dos `{}`
    // determina qual deles vai ser usado. Argumentos começam em
    // 0 imediatamente depois da string de formato
    println!("{0}, this is {1}. {1}, this is {0}", "Alice", "Bob");

    // Também suporta argumentos nomeados
    println!(
        "{subject} {verb} {object}",
        object = "the lazy dog",
        subject = "the quick brown fox",
        verb = "jumps over"
    );

    // Tem como invocar formatação em tipos, usando um ':'
    println!("Base 10:               {}", 69420); // 69420
    println!("Base 2 (binary):       {:b}", 69420); // 10000111100101100
    println!("Base 8 (octal):        {:o}", 69420); // 207454
    println!("Base 16 (hexadecimal): {:x}", 69420); // 10f2c
    println!("Base 16 maiúsculo (hexadecimal): {:X}", 69420); // 10F2C

    // Tem como justificar o texto na direita com uma largura específica
    println!("{number:>5}", number = 1); // output "    1".

    // Tem como colocar outras coisas ao invé de espaços,
    println!("{number:0>5}", number = 1); // 00001
                                          // Se faz o ajuste para o lado oposto invertendo o sinal
    println!("{number:0<5}", number = 1); // 10000

    // Tem como passar a largura por parâmetro, usando `$`
    // nessa caso daí precisa usar parâmetros nomeados
    println!("{number:0>width$}", number = 1, width = 5);

    // Rust consegue checar tbm a quantidade de argumentos
    // println!("My name is {0}, {1} {0}", "Bond");
    // FIXME ^ Add the missing argument: "James"
    println!("My name is {0}, {1} {0}", "Bond", "James");

    // somente tipos que implementarem a interface fmt:display podem usar `{}`.
    // Tipos criados pelo user não implementam fmt::Display por padrão.

    // #[allow(dead_code)] // disable `dead_code` which warn against unused module
    //struct Structure(i32);

    // This will not compile because `Structure` does not implement
    // fmt::Display.
    // println!("This struct `{:#?}` won't print...", Structure(3));
    // TODO ^ Try uncommenting this line

    // For Rust 1.58 and above, you can directly capture the argument from a
    // surrounding variable. Just like the above, this will output
    // "    1", 4 white spaces and a "1".
    let number: f64 = 1.0;
    let width: usize = 5;
    println!("{number:>width$}");
}
