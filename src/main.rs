use std::{io, thread};
use std::num::ParseIntError;

fn main() {
    _threads();
}

fn _vectores() {
    let x = vec!["Hola", "mundo"];
    let _y = x[0];
}

fn _input() {
    println!("Introduce data ...");
    let mut data = String::new();
    io::stdin().read_line(&mut data)
        .ok()
        .expect("Fallo al leer linea");
    print!("Data: {}", data);
}

fn _parseo() {
    let num = "3p";
    let result = num.parse();

    let number = match  result {
        Ok(n) => n,
        Err(_) => 0
    };
    println!("{}", number);
}

fn _result() {
    let num1: Result<i32, ParseIntError> = "2".parse();
    let num2: Result<i32, ParseIntError> = "3".parse();

    let result = num1
        .and_then(|x| num2
            .map(|y| x + y));

    let end = match result {
        Ok(n) => n.to_string(),
        Err(e) => e.to_string()
    };
    println!("{}", end); //5
}

fn _threads() {
    let handles: Vec<_> = (0..10).map(|x| {
        thread::spawn(move|| {
            println!("{}", x)
        })
    }).collect();

    for h in handles {
        h.join().ok().expect("No se pudo unir un hilo!");
    }
}