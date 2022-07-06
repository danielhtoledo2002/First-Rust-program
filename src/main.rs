use std::io;
use std::ptr::null;
// Importamos try_read que devuelve un error si falla y el tipo de error que devuelve
use text_io::{try_read, Error};

struct Tienda {
    usuario : String,
    total: i32,
    cantidad: Vec<i32>,
    productos:Vec<String>,
}
enum Alimentos {
    Jamon,
    Queso,
    Carne
}

impl  Tienda {
    fn new(total: i32) -> Tienda { return Tienda { usuario: "".to_string(), total, cantidad: vec![0,0,0], productos: vec![] } }
    fn add_value(&mut self, alimento: Alimentos) {
        self.total += match alimento {
            Alimentos::Jamon => {
                println!("Jamón agregado al carrito");
                300
            },
            Alimentos::Queso => {
                println!("Queso agregado al carrito");
                180
            },
            Alimentos::Carne => {
                println!("Carne agregada al carrito");
                400
            },
        };
    }
}
fn main() {
/*
    let mut s = String::from("hello");

    change(&mut s);
    println!("{}", s);
*/
    let mut cantidad:i32 = 0;


    let mut usuario:Tienda = Tienda::new(0);

    loop {
        menu();
        // Intentamos leer un i32
        let option: Result<i32, Error> = try_read!();
        // Si la lectura resultó en error le decimos que no y que vuelva a intentar
        let option = if option.is_err() {
            println!("Error: Ingresa una opción válida!!");
            continue;
        } else {
            // Si no es error, desenvolvemos el valor en Ok(VALOR)
            option.unwrap()
        };

        match option {
            1 => {
                submenu();
                // Intentamos leer un i32
                let sub_option: Result<i32, Error>  = try_read!();
                // Si la lectura resultó en error le decimos que no y que vuelva a intentar
                let sub_option = if sub_option.is_err() {
                    println!("Error: Ingresa una opción válida!!");
                    continue;
                } else {
                    // Si no es error, desenvolvemos el valor en Ok(VALOR)
                    sub_option.unwrap()
                };
                println!("{:?}", usuario.cantidad);
                match sub_option {
                    1 => {
                        usuario.add_value(Alimentos::Jamon);
                        usuario.productos.push("Jamón".to_string());
                        usuario.usuario = "Daniel".to_owned();
                        usuario.cantidad[0] +=1;
                        println!("{:?}", usuario.cantidad);
                    }
                    2 =>{
                        usuario.add_value(Alimentos::Queso);
                        usuario.productos.push("Carne".to_string());
                        usuario.usuario = "Daniel".to_owned();
                        usuario.cantidad[1] +=1;

                        println!("{:?}", usuario.cantidad);

                    }
                    3 =>{
                        usuario.add_value(Alimentos::Carne);
                        usuario.productos.push("Carne".to_string());
                        usuario.usuario = "Daniel".to_owned();
                        usuario.cantidad[2] +=1;
                        println!("{:?}", usuario.cantidad);

                    }
                    _ => {}
                }



            }
            2=>{
                cantidad = 0;
                println!("{} Jamón", usuario.cantidad[0]);
                println!("{} Queso", usuario.cantidad[1]);
                println!("{} Carne", usuario.cantidad[2]);
                for i in 0..usuario.cantidad.len(){
                    println!("{}, {}", usuario.cantidad[i], cantidad);
                    cantidad += usuario.cantidad[i];
                }
                println!("El total de productos es {}, y el preció a pagar es {}", cantidad, usuario.total);
            }
            _ => {}
        };
    }
}




fn menu (){
    println!("----------------Menu--------------");
    println!("------------Bienvenido----------");
    println!("--1. Menu de objetos que desea comprar--");
    println!("--2. Mostrar carrito--");
    println!("--3 Pago--");
    println!("--4. Salida--");
    println!("Ingrese la opción que desea: ");
}

fn submenu(){
    println!("------------Bienvenido----------");
    println!("-- 1. Jamón -- 300$ el kilo");
    println!("-- 2. Queso -- 180 $ el kilo");
    println!("-- 3. Carne -- 400 $ el kilo");
    println!("Ingrese la opción que desea: ");
}



/*
fn change(some_string: &mut String) {
    some_string.push_str(", world");
}
*/