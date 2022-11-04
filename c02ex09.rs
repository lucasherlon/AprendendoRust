use std::io;
use std::io::prelude::*;

fn main() {
    let valor = 10;

    println!("Binário: {:b}", valor);
    println!("Octal: {:o}", valor);
    println!("Hexadecimal: {:x}", valor);

    println!();
    println!("ou");
    println!("BIN: {:b} OCT: {:o} HEX: {:X}", valor, valor, valor);

    println!();
    print!("Tecle <Enter> para encerrar...");
    io::stdout().flush().unwrap();
    io::stdin().read(&mut [0u8]).unwrap();
}
