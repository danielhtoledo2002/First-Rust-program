use std::io;
// Importamos try_read que devuelve un error si falla y el tipo de error que devuelve
use text_io::{try_read, Error};
struct tienda {
    total: i64,
    cantidad_: i64,
}

fn main() {
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
                let sub_option: () = if sub_option.is_err() {
                    println!("Error: Ingresa una sub-opción válida!!");
                    continue;
                } else {
                    sub_option.unwrap();
                    if sub_option == 1{
                        println!("A elegido un jamón!");


                    }
                };

            }
            2=>{break}
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