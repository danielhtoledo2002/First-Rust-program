use std::io;
use std::ptr::null;
// Importamos try_read que devuelve un error si falla y el tipo de error que devuelve
use text_io::{try_read, Error};

struct Usuario {
    usuario : String,
    total: i32,
    cantidad: Vec<i32>,
    productos:Vec<String>,
    dinero: i32,
}
enum Alimentos {
    Jamon,
    Queso,
    Carne
}

impl Usuario {
    fn new(total: i32) -> Usuario { return Usuario { usuario: "".to_string(), total, cantidad: vec![0, 0, 0], productos: vec![], dinero:0 } }
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
fn main()  {
    /*
        let mut s = String::from("hello");

        change(&mut s);
        println!("{}", s);
    */
    let mut usuario: Usuario = Usuario::new(0);
    let mut cantidad:i32 = 0;
    // Intentamos leer un i32
    println!("Ingrese el nombre del usuario: ");

    let mut  persona:String = String::from("dad");
    loop {
        let persona: Result<String, Error> = try_read!();
        if persona.is_err() {
            println!("Error ingrese un string ");
            continue;
        } else {
            persona.unwrap()
        };
        break
    }
    usuario.usuario = persona;
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
                        usuario.cantidad[0] +=1;
                    }
                    2 =>{
                        usuario.add_value(Alimentos::Queso);
                        usuario.productos.push("Carne".to_string());
                        usuario.cantidad[1] +=1;
                    }
                    3 =>{
                        usuario.add_value(Alimentos::Carne);
                        usuario.productos.push("Carne".to_string());
                        usuario.cantidad[2] +=1;

                    }
                    _ => {}
                }
            }

            2=>{
                cantidad = show_carrito(cantidad.clone(), &mut usuario);
                println!("{} Jamón", usuario.cantidad[0]);
                println!("{} Queso", usuario.cantidad[1]);
                println!("{} Carne", usuario.cantidad[2]);
                println!("El total de productos es {}, y el preció a pagar es {}", cantidad, usuario.total);
            }

            3=>{
                usuario = clean_usuario();
                cantidad = 0;
                println!("carrito borrado exitosamente");
            }

            4=>{
                cantidad = show_carrito(cantidad.clone(), &mut usuario);
                if cantidad == 0{
                    println!("No hay productos que pagar");
                }
                else {
                    println!("Ingrese el dinero: ");
                    let mut dinero: Result<i32, Error> = try_read!();
                    // Si la lectura resultó en error le decimos que no y que vuelva a intentar
                    let dinero = if dinero.is_err() {
                        println!("Error: Ingresa una opción válida!!");
                        continue;
                    } else {
                        // Si no es error, desenvolvemos el valor en Ok(VALOR)
                        dinero.unwrap()
                    };
                    let cambio = if dinero >= usuario.total {
                        dinero - usuario.total
                    }else {
                        println!("No tienes el dinero suficiente para pagar");
                        continue

                    };
                    println!("El cambio es {}", cambio);
                    println!("Gracias por su compra");
                    usuario = clean_usuario();
                    cantidad = 0;
                }

            }


            5=>{break}
            _ => {}
        };
    }
}
fn menu (){
    println!("----------------Menu--------------");
    println!("------------Bienvenido----------");
    println!("--1. Menu de objetos que desea comprar--");
    println!("--2. Mostrar carrito--");
    println!("--3 Eliminar carrito ");
    println!("--4 Pago--");
    println!("--5. Salida--");
    println!("Ingrese la opción que desea: ");
}

fn submenu(){
    println!("------------Bienvenido----------");
    println!("-- 1. Jamón -- 300$ el kilo");
    println!("-- 2. Queso -- 180 $ el kilo");
    println!("-- 3. Carne -- 400 $ el kilo");
    println!("Ingrese la opción que desea: ");
}

fn clean_usuario() -> Usuario{
    let mut usuario = Usuario::new(0);
    return usuario
}
fn show_carrito(  mut cantidad: i32, usuario: &mut Usuario)-> i32{
    for i in 0..usuario.cantidad.len(){
        cantidad += usuario.cantidad[i];
    }
    cantidad
}
/* referencia ejemplo
fn change(some_string: &mut String) {
    some_string.push_str(", world");
}
usuario::default
*/