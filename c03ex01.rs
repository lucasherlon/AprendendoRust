use std::io;
use std::io::prelude::*;

fn main() {
    let mut valor_a = String::new();
    let mut valor_b = String::new();

    let a: u8;
    let b: u8;
    let r: u8;

    print!("Digite o valor de <a>: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut valor_a).unwrap();
    a = valor_a.trim().parse::<u8>().expect("Erro na conversão de tipo");

    print!("Digite o valor de <b>: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut valor_b).unwrap();
    b = valor_b.trim().parse::<u8>().expect("Erro na conversão de tipo");

    r = a + b;

    if r > 10 {
        println!("Resultado = {}", r);
    }

    println!();
    print!("Tecle <Enter> para encerrar...");
    io::stdout().flush().unwrap();
    io::stdin().read(&mut [0u8]).unwrap();
}
