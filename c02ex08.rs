use std::io;
use std::io::prelude::*;

fn main() {
    let mut base = String::new();
    let mut indice = String::new();
    
    let bas: f64;
    let ind: i32;
    
    print!("Entre com o valor da base: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut base).expect("Erro de input: linha 13.");
    bas = base.trim().parse::<f64>().expect("Erro de conversão de tipos: linha 14.");

    print!("Entre com o valor do índice: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut indice).expect("Erro de input: linha 18");
    ind = indice.trim().parse::<i32>().expect("Erro de conversão de tipos: linha 19.");

    println!();
    println!("Exponenciação = {:8.2}", bas.powi(ind));

    println!();
    print!("Tecle <Enter> para encerrar...");
    io::stdout().flush().unwrap();
    io::stdin().read(&mut [0u8]).unwrap();

}
