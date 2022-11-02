use std::io;
use std::io::prelude::*;

fn main() {
    let mut dividendo = String::new();
    let mut divisor = String::new();

    let dvd: i64;
    let dvr: i64;
    let res: i64;
    let quo: i64;

    print!("Entre com o valor do dividendo: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut dividendo).expect("Erro na leitura do dividendo.");
    dvd = dividendo.trim().parse::<i64>().expect("Erro de conversão do dividendo.");

    print!("Entre com o valor do divisor: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut divisor).expect("Erro na leitura do divisor.");
    dvr = divisor.trim().parse::<i64>().expect("Erro de conversão do divisor.");

    quo = dvd/dvr;
    res = dvd%dvr;

    println!();
    println!("Quociente = {:8}", quo);
    println!("Resto     = {:8}",res);
    
    println!();
    print!("Tecle <Enter> para encerrar...");
    io::stdout().flush().unwrap();
    io::stdin().read(&mut [0u8]).unwrap();
}
