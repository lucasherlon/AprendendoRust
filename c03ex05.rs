use std::io;
use std::io::prelude::*;

fn main() {
    let mut valor = String::new();

    let vlr: i64;

    print!("Digite um número inteiro: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut valor).expect("input error!");
    vlr = valor.trim().parse::<i64>().expect("type conversion error!");

    if !(vlr >= 10) {
        println!("Valor informado: {}", vlr);
    } else {
        println!("Valor inválido!");
    }

    println!();
    print!("Tecle <Enter> para sair...");
    io::stdout().flush().unwrap();
    io::stdin().read(&mut [0u8]).unwrap();
}
