fn main() {
    // tenemos que definir la variable con let y definir que tipo de variables son
    // todas las variables son inmutables por defecto, usamos mut para definirlas mutables
    let mut mensaje = String::from("Nombre: Mateo, Peso: ");
    mensaje.clear();
    mensaje = String::from("Nombre: Mateo, Peso: ");
    let mut kilos = 180.0;
    // Una vez que se define la variable con mut, no hay que volver a hacerlo
    kilos = 200.0;
    // no se puede dividir entero y decimal
    let peso = kilos / 2.2;
    println!("{}{}", mensaje, peso)
}
