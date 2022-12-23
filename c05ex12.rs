use std::io;
use std::io::prelude::*;

fn main() {
    let mut valor = String::new();

    let sucessor = |x: i64| {x + 1};

    print!("Digite um número inteiro: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut valor).unwrap();
    let valor: i64 = match valor.trim().parse() {
        Ok(num) => num,
        Err(_) => 0
    };

    println!("O sucessor de {} é {}", valor, sucessor(valor));

    println!();
    print!("Tecle <Enter> para sair...");
    io::stdout().flush().unwrap();
    io::stdin().read(&mut [0u8]).unwrap();

}
