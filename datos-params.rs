use std::io;

fn main() {
    // Solicitar nombre
    println!("Ingrese su nombre:");
    let mut name = String::new();
    io::stdin()
        .read_line(&mut name)
        .expect("Error al leer el nombre");

    // Solicitar edad
    println!("Ingrese su edad:");
    let mut age = String::new();
    io::stdin()
        .read_line(&mut age)
        .expect("Error al leer la edad");
    let age: u32 = age.trim().parse().expect("Edad inválida");

    // Mostrar opciones
    println!("Seleccione una opción:");
    println!("1. Opción 1");
    println!("2. Opción 2");
    println!("3. Opción 3");

    // Leer selección de usuario
    let mut selection = String::new();
    io::stdin()
        .read_line(&mut selection)
        .expect("Error al leer la selección");
    let selection: u32 = selection.trim().parse().expect("Selección inválida");

    // Mostrar resultados
    println!("Nombre: {}", name.trim());
    println!("Edad: {}", age);
    match selection {
        1 => println!("Seleccionó la opción 1"),
        2 => println!("Seleccionó la opción 2"),
        3 => println!("Seleccionó la opción 3"),
        _ => println!("Selección inválida"),
    }
}

