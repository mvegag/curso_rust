
/// Usamos la palabra reservada loop para evitar definir la condición del loop
/// es como en python while True:
/// con loop se corre indefinidamente, por eso usamos break

fn main() {
    let mut x = 1;
    loop {
        println!("x es {}", x);
        x += 1;
        if x > 5 {
            break;
        }
    }
}
