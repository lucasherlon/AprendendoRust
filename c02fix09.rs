use std::io;
use std::io::prelude::*;

fn main() {
    let mut _sa = String::new();
    let mut _pr = String::new();

    print!("Digite o salário atual: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut _sa).expect("Input error: line 10");
    let sa: f32 = _sa.trim().parse::<f32>().expect("Type conversion error: line 11");

    print!("Digite o percentual de reajuste: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut _pr).expect("Input error: line 15");
    let pr: f32 = _pr.trim().parse::<f32>().expect("Type conversion error: line 16");

    let ns: f32 = sa*((pr/100.)+1.);

    println!("O novo salário é: R$ {:.2}", ns);

    println!();
    print!("Tecle <Enter> para sair...");
    io::stdout().flush().unwrap();
    io::stdin().read(&mut [0u8]).expect("Input error: line 25");
}
