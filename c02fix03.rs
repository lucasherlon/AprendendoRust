use std::io;
use std::io::prelude::*;

fn main() {
    let mut _a = String::new();
    let mut _b = String::new();
    let aux: i32;
    
    print!("Digite o número A: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut _a).expect("Erro de input: linha 11.");
    let mut a: i32 = _a.trim().parse::<i32>().expect("Erro de conversão: linha 12.");

    print!("Digite o número B: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut _b).expect("Erro de input: linha 16.");
    let mut b: i32 = _b.trim().parse::<i32>().expect("Erro de conversão: linha 17.");

    aux = a;
    a = b;
    b = aux;

    println!("Novo valor de A: {}", a);
    println!("Novo valor de B: {}", b);
}
