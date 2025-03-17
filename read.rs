use std::io;
use std::str::FromStr;

pub fn input<T: FromStr>(msg: &str) -> T {
    loop {
        let mut new_str = String::new();
        println!("{}", msg);
        
        io::stdin().read_line(&mut new_str).expect("Erro ao ler a linha");

        match new_str.trim().parse::<T>() {
            Ok(value) => return value,
            Err(_) => println!("Entrada inválida! Tente novamente."),
        }
    }
}