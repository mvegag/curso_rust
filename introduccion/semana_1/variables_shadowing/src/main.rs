fn main() {
    // shadowing es crear una variable y actualizar su valor después
    let mut altura = 190;
    altura = altura - 20;
    let resultado = if altura > 180 {
        "Alto"
    } else if altura > 170 {
        "Promedio"
    } else {
        "Bajo"
    };

    println!("Resultados: {}", resultado);

    let salud = if altura < 180 {"buena"} else {"desconocida"};
    println!("Salud: {}", salud);

    // Shadowing a un tipo diferente
    let salud = if altura < 180 {true} else {false};
}


