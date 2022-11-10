/// main.rs

/*
projeto: c02ex04
Autor: Lucas Herlon
Data: 10/11/2022
Versão: 1.65
*/

use std::*;
use std::io::prelude::*;

fn main() {
    
    // Trecho com apresentação de faixa mínima e máxima de valores

    println!("i8 de {} até {}", i8::MIN, i8::MAX);
    println!("u8 de {} até {}", u8::MIN, u8::MAX);
    println!();

    println!("i16 de {} até {}", i16::MIN, i16::MAX);
    println!("u16 de {} até {}", u16::MIN, u16::MAX);
    println!();

    println!("i32 de {} até {}", i32::MIN, i32::MAX);
    println!("u32 de {} até {}", u32::MIN, u32::MAX);
    println!();

    println!("i64 de {} até {}", i64::MIN, i64::MAX);
    println!("u64 de {} até {}", u64::MIN, u64::MAX);
    println!();

    println!("isize de {} até {}", isize::MIN, isize::MAX);
    println!("usize de {} até {}", usize::MIN, usize::MAX);

    println!();
    print!("Tecle <Enter> para encerrar...");
    io::stdout().flush().unwrap();
    io::stdin().read(&mut [0u8]).unwrap();
}
