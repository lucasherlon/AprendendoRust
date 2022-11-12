use std::io;
use std::io::prelude::*;

fn main() {
    let mut fahr = String::new();

    println!("Digite a temperatura em fahrenheit: ");
    io::stdin().read_line(&mut fahr).unwrap();

    let f: f64 = fahr.trim().parse::<f64>().unwrap();
    let c: f64 = ((f-32.)*5.)/9.;

    println!("{} grau(s) fahrenheit equivale(m) a {:.2} grau(s) celsius", f, c);

    print!("Pressione <Enter> para sair...");
    io::stdout().flush().unwrap();
    io::stdin().read(&mut [0u8]).unwrap();
}