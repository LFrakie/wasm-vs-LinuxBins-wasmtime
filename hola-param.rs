use std::env;

fn main() {
    // Obtener el primer argumento pasado en la línea de comandos
    let args: Vec<String> = env::args().collect();
    let name = &args[1];

    // Imprimir un saludo personalizado
    println!("Hola, {}!", name);
}

