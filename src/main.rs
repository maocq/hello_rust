use std::{io, thread};
use std::num::ParseIntError;
use std::time::Duration;

fn main() {
    _futures_join();
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

fn _result_match() {
    let result: Result<i32, &str> = Ok(5);

    let number = match result {
        Ok(x) => x,
        Err(_e) => 0,
    };
    assert_eq!(5, number)
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

fn _threads_join() {
    thread::spawn(|| {
        thread::sleep(Duration::from_millis(3000));
        println!("Hola desde 1");
    });
    let thread2 = thread::spawn(|| {
        thread::sleep(Duration::from_millis(3000));
        println!("Hola desde 2");
    });

    println!("end...");
    thread2.join().expect("Error");
    //thread::sleep(Duration::from_millis(4000));
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
    //unwrap() hará un pánico ( panic! ) si el Result es Err
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

fn _option_match() {
    let option = Some(5);

    let number = match option {
        Some(x) => x,
        None => 0,
    };
    assert_eq!(5, number);
}

fn _result_funciones() {
    enum Error {
        Tecnico
    }

    let f: fn(i32) -> Result<i32, Error> = |num: i32| match num {
        1 => Ok(num + 1),
        _ => Err(Error::Tecnico)
    };

    /*fn f(num: i32) -> Result<i32, Error> {
        match num {
            1 => Ok(num + 1),
            _ => Err(Error::Tecnico)
        }
    }*/

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

fn _panic_result() {
    let result: Result<i32, &str> = Ok(1);
    //let result: Result<i32, &str> = Err("Error =(");

    let valor = result.ok().expect("Error!");
    assert_eq!(1, valor)
}

fn _try() {
    fn _parser(num: &str) -> Result<i32, ParseIntError> {
        num.parse()
    }

    fn f(x: &str, y: &str) -> Result<i32, ParseIntError> {
        let num1 = _parser(x);
        let num2 = _parser(y);

        //let resultado = _parser(x) ? + _parser(y)?;
        let resultado = num1? + num2?;
        Ok(resultado)
    }

    assert!(f("1", "2").is_ok());
    assert!(f("1P", "2").is_err());

    match f("1P", "2") {
        Ok(n) => println!("Ok: {}", n),
        Err(e) => println!("Error: {}", e)
    }
}

fn _try_azucar_sintactico() {
    fn foo(n: i32) -> Result<i32, String> {
        if n % 2 == 0 {
            Ok(1)
        } else { Err(String::from("Error")) }
    }

    fn bar() -> Result<i32, String> {
        Ok(2)
    }

    fn foo_bar() -> Result<i32, String> {
        let res = foo(2)? + bar()?;
        Ok(res)
    }

    let fb = foo_bar();
    assert!(fb.is_ok());
}

fn _apuntadores_a_funcion() {
    fn mas_uno(i: i32) -> i32 {
        i + 1
    }

    let f: fn(i32) -> i32 = mas_uno;
    assert_eq!(2, f(1));
}

fn _primitivos() {
    let _a: bool = false;
    let _b: char = 'x';
    let _c: i32 = 42; //i8, i16, i32, i64, u8, u16, u32, u64, isize, usize, f32, f64
}

fn _arreglos() {
    let mut m: [i32; 3] = [1, 2, 3];
    m[2] = 5;
    assert_eq!(5, m[2]);
}

fn _slices() {
    let a: [i32; 5] = [0, 1, 2, 3, 4];
    let middle: &[i32] = &a[1..4];
    assert_eq!(1, middle[0]);
}

fn _tuplas() {
    let (x, y) = (1, "Hello");
    assert_eq!(1, x);
    assert_eq!("Hello", y);

    let z = (1, "Hello");
    assert_eq!(1, z.0);
}

fn _expresiones() {
    let x = 5;
    let y = if x == 5 { 10 } else { 15 };
    assert_eq!(10, y)
}

fn _while() {
    let mut x = 0;
    while x < 10 {
        x += 1;
    }
    assert_eq!(10, x)
}

fn _for() {
    for x in 0..10 {
        println!("{}", x);
    }
}

fn _loop() {
    let mut x = 0;
    loop {
        x += 1;
        if x >= 10 {
            break
        }
    }
    assert_eq!(10, x)
}

fn _etiquetas_loop() {
    'exterior: for x in 0..10 {
        'interior: for y in 0..10 {
            if x % 2 == 0 { continue 'exterior; } // continua el ciclo por encima de x
            if y % 2 == 0 { continue 'interior; } // continua el ciclo por encima de y
            println!("x: {}, y: {}", x, y);
        }
    }
}

fn _enumerate() {
    for (i,j) in (5..10).enumerate() {
        println!("i = {} y j = {}", i, j);
    }

    let lineas = "hola\nmundo".lines();
    for (numero_linea, linea) in lineas.enumerate() {
        println!("{}: {}", numero_linea, linea);
    }
}

fn _pertenencia() {
    let v = vec![1, 2, 3];
    let v2 = v;
    println!("v2[0] es: {}", v2[0]);
    //println!("v[0] es: {}", v[0]); // Error borrow of moved value: `v`
}

fn _pertenencia_funcion() {
    fn tomar(_v: Vec<i32>) {
        // Algo
    }

    let v = vec![1, 2, 3];
    tomar(v);
    //println!("v[0] es: {}", v[0]); // Error  borrow of moved value: `v`
}

fn _copy() {
    // i32 , Todos los tipos primitivos implementan el trait Copy
    // Se realiza una copia y su pertenencia no es movida
    let v: i32 = 1;
    let _v2 = v;
    println!("v es: {}", v); // =)
}

fn _devolver_pertenencia() {
    fn _foo(v: Vec<i32>) -> Vec<i32> {
        v
    }
    fn foo(v1: Vec<i32>, v2: Vec<i32>) -> (Vec<i32>, Vec<i32>, i32) {
        (v1, v2, 42)
    }
    let v1 = vec![1, 2, 3];
    let v2 = vec![1, 2, 3];
    let (v1, _v2, _r) = foo(v1, v2);
    assert_eq!(1, v1[0]);
}

fn _prestamo() {
    fn foo(_v1: &Vec<i32>, _v2: &Vec<i32>) -> i32 {
        42
    }
    let v1 = vec![1, 2, 3];
    let _v2 = vec![1, 2, 3];
    let _r = foo(&v1, &_v2);

    // podemos usar a v1 y v2 aqui
    assert_eq!(1, v1[0]);
}

fn _mutabilidad() {
    let mut x = 5;
    assert_eq!(5, x);
    x = 6;
    assert_eq!(6, x);
}

fn _estructuras() {
    struct Punto {
        x: i32,
        y: i32,
    }
    let origen = Punto { x: 0, y: 0 };
    assert_eq!(0, origen.x);
    assert_eq!(0, origen.y);
}

fn _sintaxis_de_actualizacion() {
    struct Punto3d {
        _x: i32,
        _y: i32,
        _z: i32,
    }
    let origen = Punto3d { _x: 1, _y: 2, _z: 3 };
    let punto = Punto3d { _y: 1, .. origen };
    assert_eq!(3, punto._z);
}

fn _estructuras_pertenencia() {
    struct Punto {
        x: i32,
        y: i32,
    }
    fn foo(punto: Punto) -> i32 {
        punto.x + punto.y
    }
    let origen = Punto { x: 1, y: 2 };
    let suma = foo(origen);
    println!("{}", suma);
    //println!("Punto x {}", origen.x); // Error borrow of moved value: `origen`
}

fn _estructuras_prestamo() {
    struct Punto {
        x: i32,
        y: i32,
    }
    fn foo(punto: &Punto) -> i32 {
        punto.x + punto.y
    }
    let origen = Punto { x: 1, y: 2 };
    let suma = foo(&origen);
    assert_eq!(3, suma);
    assert_eq!(1, origen.x);
}

fn _tupla_estructuras() {
    struct Color(i32, i32, i32);
    let azul = Color(0, 0, 255);
    assert_eq!(255, azul.2);
}

fn _estructuras_tipo_unitario() {
    struct Electron;
    let _e = Electron;
}

fn _enumeraciones() {
    enum Mensaje {
        Salir,
        CambiarColor(i32, i32, i32),
        Mover { _x: i32, _y: i32 },
        Escribir(String),
    }
    let _salir = Mensaje::Salir;
    let _cambiar_color = Mensaje::CambiarColor(0, 0, 255);

    use Mensaje::{Mover};
    let _mover = Mover {_x: 0, _y: 2};

    let _escribir = Mensaje::Escribir("Hello".to_string());
}

fn _match_en_enums() {
    enum _Mensaje {
        Salir,
        CambiarColor(i32, i32, i32),
        Mover { x: i32, _y: i32 },
        Escribir(String),
    }
    fn _salir() { /* ... */ }
    fn _cambiar_color(_r: i32, _g: i32, _b: i32) { /* ... */ }
    fn _mover_cursor(_x: i32, _y: i32) { /* ... */ }

    fn _procesar_mensaje(msj: _Mensaje) {
        match msj {
            _Mensaje::Salir => _salir(),
            _Mensaje::CambiarColor(r, g, b) => _cambiar_color(r, g, b),
            _Mensaje::Mover { x, _y: y } => _mover_cursor(x, y),
            _Mensaje::Escribir(s) => println!("{}", s),
        };
    }
}

fn _multiples_patrones() {
    let x = 2;
    let num = match x {
        1 | 2 => "1, 2",
        3 => "3",
        _ => "...",
    };
    assert_eq!("1, 2", num);
}

fn _match_rangos() {
    let x = 3;
    let resultado = match x {
        1 ..= 5 => "uno al cinco",
        _ => "cualquier cosa",
    };
    assert_eq!("uno al cinco", resultado);

    let y = 's';
    let letra = match y {
        'a' ..= 'j' => "letra temprana",
        'k' ..= 'z' => "letra tardia",
        _ => "algo mas"
    };
    assert_eq!("letra tardia", letra);
}

fn _destructuracion() {
    struct Punto {
        x: i32,
        y: i32,
    }
    let origen = Punto { x: 0, y: 2 };
    match origen {
        Punto { x, y } => println!("({},{})", x, y),
    }

    match origen {
        Punto { x, .. } => println!("x es {}", x)
    }
}

fn _enlaces_a_variable() {
    let x = 1;
    match x {
        e @ 1 ..= 5 => println!("valor de rango {} obtenido", e),
        _ => println!("lo que sea"),
    }
}

fn _guardias() {
    enum EnteroOpcional {
        Valor(i32),
        _Faltante,
    }
    let x = EnteroOpcional::Valor(5);
    match x {
        EnteroOpcional::Valor(i) if i > 5 => println!("Entero mayor a cinco obtenido!"),
        EnteroOpcional::Valor(..) => println!("Entero obtenido!"),
        EnteroOpcional::_Faltante => println!("Sin suerte."),
    }
}

fn _multiples_patrones_y_guardias() {
    let x = 4;
    let y = false;
    let resultado = match x {
        4 | 5 if y => "si",
        _ => "no"
    };
    assert_eq!("no", resultado);
}

fn _llamadas_a_metodos() {
    struct Circulo {
        _x: f64,
        _y: f64,
        radio: f64,
    }
    impl Circulo {
        fn area(&self) -> f64 {
            std::f64::consts::PI * (self.radio * self.radio)
        }
    }

    let c = Circulo { _x: 0.0, _y: 0.0, radio: 2.0 };
    println!("{}", c.area());
}

fn _metodos_en_cadena() {
    struct Circulo {
        x: f64,
        y: f64,
        radio: f64,
    }
    impl Circulo {
        fn agrandar(&self, incremento: f64) -> Circulo {
            Circulo { x: self.x, y: self.y, radio: self.radio + incremento }
        }
        fn area(&self) -> f64 {
            std::f64::consts::PI * (self.radio * self.radio)
        }
    }

    let c = Circulo { x: 0.0, y: 0.0, radio: 2.0 };
    println!("{}", c.area());
    let d = c.agrandar(2.0).area();
    println!("{}", d);
}


fn _funciones_asociadas() {
    struct Circulo { _x: f64, _y: f64, radio: f64, }

    impl Circulo {
        fn new(x: f64, y: f64, radio: f64) -> Circulo {
            Circulo { _x: x, _y: y, radio: radio, }
        }
    }

    let c = Circulo::new(0.0, 0.0, 2.0);
    assert_eq!(2.0, c.radio);
}

fn _builder() {
    struct Circulo { x: f64, y: f64, radio: f64 }
    impl Circulo {
        fn area(&self) -> f64 {
            std::f64::consts::PI * (self.radio * self.radio)
        }
    }
    struct CirculoBuilder { x: f64, y: f64, radio: f64 }

    impl CirculoBuilder {
        fn new() -> CirculoBuilder {
            CirculoBuilder { x: 0.0, y: 0.0, radio: 1.0, }
        }
        fn x(&mut self, coordenada: f64) -> &mut CirculoBuilder {
            self.x = coordenada;
            self
        }
        fn y(&mut self, coordenada: f64) -> &mut CirculoBuilder {
            self.y = coordenada;
            self
        }
        fn radio(&mut self, radio: f64) -> &mut CirculoBuilder {
            self.radio = radio;
            self
        }
        fn build(&self) -> Circulo {
            Circulo { x: self.x, y: self.y, radio: self.radio }
        }
    }

    let c = CirculoBuilder::new()
        .x(1.0)
        .y(2.0)
        .radio(2.0)
        .build();
    println!("area: {}", c.area());
    println!("x: {}", c.x);
    println!("y: {}", c.y);

    assert_eq!(2.0, c.y);
}

fn _cadenas_de_caracteres() {
    let _saludo: &str = "Hola.";

    let mut s: String = "Hola".to_string();
    s.push_str(", mundo.");

    assert_eq!("Hola, mundo.", s);
}

fn _genericos() {
    enum _Option<T> {
        _Some(T),
        _None,
    }
    let _x: _Option<i32> = _Option::_Some(5);
}

fn _funciones_genericas() {
    fn foo<T>(x: T) -> T {
        x
    }
    let num = foo(1);
    assert_eq!(1, num);
}

fn _structs_genericos() {
    struct Info<T1, T2> {
        x: T1,
        y: T2,
    }
    impl<T1, T2> Info<T1, T2> {
        fn foo(&self) {
            //
        }
    }

    let info = Info { x: 1, y: "=)" };
    info.foo();
    assert_eq!(1, info.x);
    assert_eq!("=)", info.y);
}

fn _traits() {
    trait Area {
        fn area(&self) -> f64;
    }

    struct Circulo { _x: f64, _y: f64, radio: f64 }
    impl Area for Circulo {
        fn area(&self) -> f64 {
            std::f64::consts::PI * (self.radio * self.radio)
        }
    }

    let c = Circulo{ _x:0.0, _y:0.0, radio: 2.0 };
    let a = c.area();
    println!("{}", a);

    //Genericos
    fn imrimir_area<T: Area>(figura: T) {
        println!("Esta figura tiene un area de {}", figura.area());
    }

    imrimir_area(c)
}

fn _multiples_limites_de_trait() {
    use std::fmt::Display;

    fn foo<T: Clone, K: Clone + Display>(x: T, y: K) -> String {
        let _x_clone = x.clone();
        let y_clone = y.clone();
        format!("{}", y_clone)
    }
    fn bar<T, K>(x: T, y: K) -> String where T: Clone, K: Clone + Display {
        let _x_clone = x.clone();
        let y_clone = y.clone();
        format!("{}", y_clone)
    }

    let r_foo = foo("Hola", "mundo");
    let r_bar = bar("Hola", "mundo");

    assert_eq!(r_foo, r_bar);
}

fn _metodos_por_defecto() {
    trait Foo {
        fn es_valido(&self) -> bool;
        fn es_invalido(&self) -> bool { !self.es_valido() }
    }

    struct Default;
    impl Foo for Default {
        fn es_valido(&self) -> bool { true }
    }

    let default = Default;
    assert!(default.es_valido());
    assert!(!default.es_invalido());
}

fn _metodos_por_defecto_bar() {
    trait Bar {
        fn plus_one(x: i32) -> i32 { x + 1}
    }

    struct ImplBar;
    impl Bar for ImplBar{};

    let sum = ImplBar::plus_one(2);
    assert_eq!(3, sum);
}

fn _herencia() {
    trait Foo {
        fn foo(&self);
    }
    trait FooBar : Foo {
        fn foobar(&self);
    }

    struct Baz;
    impl Foo for Baz {
        fn foo(&self) { println!("foo"); }
    }
    impl FooBar for Baz {
        fn foobar(&self) { println!("foobar"); }
    }

    let baz = Baz;
    baz.foo();
    baz.foobar();
}

fn _drop() {
    struct HasDrop;
    impl Drop for HasDrop {
        fn drop(&mut self) {
            println!("Dropeando!");
        }
    }
    let _x = HasDrop;
}

fn _if_let() {
    fn foo(x: i32) { println!("Number: {}", x); }

    let option = Some(5);
    if let Some(x) = option {
        foo(x);
    }
}

fn _closures() {
    let plus_one = |x: i32| x + 1;
    assert_eq!(2, plus_one(1));

    //let plus_one: fn(i32) -> i32 = |x: i32| x + 1;

    /*
    fn suma_uno_v1 (x: i32) -> i32 { x + 1 }
    let suma_uno_v2 = |x: i32| -> i32 { x + 1 };
    let suma_uno_v3 = |x: i32| x + 1 ;
     */
}

fn _closures_como_argumentos() {
    fn llamar_con_uno<F>(closure: F) -> i32 where F : Fn(i32) -> i32 {
        closure(1)
    }
    let respuesta = llamar_con_uno(|x| x + 2);
    assert_eq!(3, respuesta);
}

fn _retornando_closures() {
    fn factory() -> Box<dyn Fn(i32) -> i32> {
        let num = 5;
        Box::new(move |x| x + num)
    }

    let f = factory();
    let respuesta = f(1);
    assert_eq!(6, respuesta);
}

fn _futures() {
    use futures::executor::block_on;

    async fn hello_world() {
        println!("hello, world!");
    }

    let future = hello_world();
    block_on(future);
}

fn _await() {
    async fn first_function() -> u32 {
        thread::sleep(Duration::from_millis(2000));
        println!("1");
        1
    }
    async fn second_function() -> u32 {
        thread::sleep(Duration::from_millis(2000));
        println!("2");
        2
    }
    async fn another_function() {

        let first = first_function().await;
        let second = second_function().await;
        let sum = first + second;
        println!("{}", sum);
    }

    use futures::executor::block_on;
    block_on(another_function());
    println!("end...");
}

fn _futures_join() {
    use futures::join;
    use futures::executor::block_on;

    async fn get_book() -> u32 {
        thread::sleep(Duration::from_millis(3000));
        println!("get_book");
        1
    }

    async fn get_music() -> u32 {
        thread::sleep(Duration::from_millis(3000));
        println!("get_music");
        1
    }

    async fn get_book_and_music() -> (u32, u32) {
        let book_fut = get_book();
        let music_fut = get_music();
        join!(book_fut, music_fut)
    }

    block_on(get_book_and_music());
}

fn _futures_join_2() {
    use futures::executor::block_on;

    async fn learn_song() -> u32 {
        thread::sleep(Duration::from_millis(3000));
        println!("learn_song");
        1
    }

    async fn sing_song(_song: u32) -> u32 {
        thread::sleep(Duration::from_millis(3000));
        println!("sing_song");
        1
    }

    async fn dance() -> u32 {
        thread::sleep(Duration::from_millis(3000));
        println!("dance");
        1
    }

    async fn learn_and_sing() {
        let song = learn_song().await;
        sing_song(song).await;
    }

    async fn async_main() {
        let f1 = learn_and_sing();
        let f2 = dance();

        futures::join!(f1, f2);
    }

    block_on(async_main());
}
