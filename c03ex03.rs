use std::io;
use std::io::prelude::*;

fn main() {
    let mut valor = String::new();

    let numero: i32;

    print!("Digite um número inteiro: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut valor).expect("input error!");
    numero = valor.trim().parse::<i32>().expect("type conversion error!");

    if numero >= 20 && numero <= 90 {
        println!("O número {} está entre 20 e 90", numero);
    } else {
        println!("O número {} não está entre 20 e 90", numero);
    }

    println!();
    print!("Tecle <Enter> para encerrar...");
    io::stdout().flush().unwrap();
    io::stdin().read(&mut [0u8]).unwrap();
}
