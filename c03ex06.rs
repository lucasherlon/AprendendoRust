use std::io;
use std::io::prelude::*;

fn main() {
    let mut lado_a = String::new();
    let mut lado_b = String::new();
    let mut lado_c = String::new();

    let a: f32;
    let b: f32;
    let c: f32;

    print!("Entre com o lado <a>: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut lado_a).expect("input error!");
    a = lado_a.trim().parse::<f32>().expect("typer conversion error!");

    print!("Entre com o lado <b>: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut lado_b).expect("input error!");
    b = lado_b.trim().parse::<f32>().expect("typer conversion error!");

    print!("Entre com o lado <c>: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut lado_c).expect("input error!");
    c = lado_c.trim().parse::<f32>().expect("typer conversion error!");

    if a<b+c && b<a+c && c<a+b {
        if a==b && b==c {
            println!("O triângulo é equilátero.");
        } else {
            if a==b || b==c {
                println!("O triângulo é isóceles.");
            } else {
                println!("O triângulo é escaleno.")
            }
        }
    } else {
        println!("Os lados fornecidos não formam um triângulo.")
    }

    println!();
    print!("Tecle <Enter> para sair...");
    io::stdout().flush().unwrap();
    io::stdin().read(&mut [0u8]).unwrap();

}
