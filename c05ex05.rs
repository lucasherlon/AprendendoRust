use std::io;
use std::io::prelude::*;

fn pause() {
    println!();
    print!("Tecle <Enter> para sair...");
    io::stdout().flush().unwrap();
    io::stdin().read(&mut [0u8]).unwrap();
}

fn fatorial (n: u64, f: &mut u64) {
    for i in 1..n+1 {
        *f *= i;
    }
}

fn main() {
    let mut valor = String::new();
    let mut resultado: u64 = 1;

    print!("Digite o valor do fatorial: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut valor).expect("input error!");
    let valor: u64 = match valor.trim().parse() {
        Ok(num) => num,
        Err(_) => 0,
    };

    fatorial(valor, &mut resultado);
    println!("O fatorial de {} é {}", valor, resultado);
    pause();

}
