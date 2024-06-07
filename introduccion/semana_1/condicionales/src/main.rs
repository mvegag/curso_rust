/// Aquí usamos la condicional `if let`

fn main() {
    // Tenemos que usar Option<Option<T>> para decirle a rust que tipo de datos vamos a tener
    let algun_numero: Option<Option<()>> = Some(None);
    // let algun_numero = Some(42);
    if let Some(numero) = algun_numero {
        println!("El número es {:?}", numero);
    } else {
        println!("No hay numero!")
    }
}
