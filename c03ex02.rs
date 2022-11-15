use std::io;
use std::io::prelude::*;

fn main() {
    let mut valor_a = String::new();
    let mut valor_b = String::new();

    let a: i32;
    let b: i32;
    let r: i32;

    print!("Digite o valor de <a>: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut valor_a).expect("input error!");
    a = valor_a.trim().parse::<i32>().expect("type conversion error!");

    print!("Digite o valor de <b>: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut valor_b).expect("input error!");
    b = valor_b.trim().parse::<i32>().expect("type conversion error!");

    r = a * b;

    println!();

    if r >= 20 {
        println!("Resultado = {}", r+5);
    } else {
        println!("Resultado = {}", r-7);
    }

    println!();
    print!("Tecle <Enter> para sair...");
    io::stdout().flush().unwrap();
    io::stdin().read(&mut [0u8]).unwrap();

}
