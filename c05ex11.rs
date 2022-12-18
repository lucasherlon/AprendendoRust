use std::io;
use std::io::prelude::*;

fn pausa() {
    println!();
    print!("Tecle <Enter> para sair...");
    io::stdout().flush().unwrap();
    io::stdin().read(&mut [0u8]).unwrap();
}

fn fatorial(n:u64) -> u64 {
    if n == 0 {
        return 1;
    }
    return n * fatorial(n-1);
}

fn escrevefat(vlr: u64, subrotina: fn(u64) -> u64) -> u64 {
    println!("Resultado = {}", subrotina(vlr));
    return vlr;
}

fn main() {
    let mut valor = String::new();

    print!("Digite o número que você quer calcular o fatorial: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut valor).expect("Input error!");
    
    let valor: u64 = match valor.trim().parse() {
        Ok(num) => num,
        Err(_) => panic!("Parsing error!")
    };

    escrevefat(valor, fatorial);
    pausa();

}
