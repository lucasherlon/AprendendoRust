use std::io;
use std::io::prelude::*;

fn main() {
    let mut celsius = String::new();

    println!("Digite a temperatura em celsius: ");
    io::stdin().read_line(&mut celsius).unwrap();

    let c: f64 = celsius.trim().parse::<f64>().unwrap();
    let f: f64 = (9.*c + 160.)/9.;

    println!("{} grau(s) celsius equivale(m) a {:.2} grau(s) fahrenheiht", c, f);

    print!("Pressione <Enter> para sair...");
    io::stdout().flush().unwrap();
    io::stdin().read(&mut [0u8]).unwrap();
}
