use std::{io, thread};
use std::num::ParseIntError;
use std::time::Duration;

fn main() {
    _result_funciones();
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
    assert_eq!(0, number);
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
    assert_eq!("5", end);
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

fn _iteradores() {
    let mut rango = 0..10;
    loop {
        match rango.next() {
            Some(x) => {
                println!("{}", x);
            },
            None => { break }
        }
    }

    let nums = vec![1, 2, 3];
    for num in nums.iter() {
        println!("{}", num);
    }
}

fn _consumidores() {
    let _uno_hasta_cien = (1..101).collect::<Vec<i32>>();
    let _uno_hasta_cien = (1..101).collect::<Vec<_>>();

    let mayores_a_cuarenta_y_dos = (0..100)
        .find(|x| *x > 42);
    match mayores_a_cuarenta_y_dos {
        Some(_) => println!("Tenemos algunos números!"),
        None => println!("No se encontraron números :("),
    }

    let suma = (1..4).fold(0, |suma, x| suma + x); //6
    assert_eq!(6, suma);
}

fn _adaptadores_de_iterador() {
    let _nums = (1..100).map(|x| x + 1).collect::<Vec<_>>();

    let _nums =  (1..30)
        .filter(|&x| x % 2 == 0)
        .filter(|&x| x % 3 == 0)
        .take(5)
        .collect::<Vec<i32>>();

    for x in (1..11).map(|x| x + 1).collect::<Vec<_>>() {
        println!("{}", x);
    }
}

fn _hilos() {
    thread::spawn(|| {
        println!("Hola desde un hilo!");
    });
    thread::sleep(Duration::from_millis(10));
}

fn _thread_handle() {
    let handle = thread::spawn(|| {
        "Hola desde un hilo!"
    });
    assert_eq!("Hola desde un hilo!", handle.join().unwrap());
}

fn _panico_hilo() {
    let valor = 1;
    let result = thread::spawn(move || {
        if valor % 2 == 0 { panic!("ups!"); }
        1
    }).join();

    let resultado = match result {
        Ok(n) => n,
        Err(_e) => 0
    };
    assert_eq!(1, resultado);
}

fn _panico_unreachable() {
    enum Estado {
        _Activo,
        _Inactivo,
        Desconocido
    }

    use Estado::{_Activo, _Inactivo, Desconocido};
    let estado = Desconocido;
    let _numero = match estado {
        _Activo => 1,
        _Inactivo => 0,
        _ => unreachable!()
    };
    println!("Linea no alcanzable")
}

fn _option() {
    let s = "foo";

    assert_eq!(s.find('f'), Some(0));
    assert_eq!(s.find('z'), None);

    assert_eq!(s.find('f').map(|p| p + 1), Some(1));
    assert_eq!(s.find('z').map(|p| p + 1), None);
}

fn _result_funciones() {
    enum Error {
        Tecnico
    }

    fn f(num: i32) -> Result<i32, Error> {
        match num {
            1 => Ok(num + 1),
            _ => Err(Error::Tecnico)
        }
    }

    assert!(f(1).is_ok());
    assert!(f(2).is_err());


    let result: Result<i32, &str> = f(2)
        .map(|ok| ok)
        .map_err(|_err| "Error =(");

    match result {
        Ok(n) => println!("{}", n),
        Err(e) => println!("{}", e)
    };
}