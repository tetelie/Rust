fn main() {
    let nombre = 3;

    if nombre < 5 {
        println!("la condition est vérifiée");
    } else {
        println!("La condition n'est pas vérifiée");
    }

    if nombre != 0 {
        println!("Le nombre valait autre chose que zéro");
    }

    if nombre % 4 == 0 {
        println!("Le nombre est divisible par 4");
    } else if nombre % 3 == 0 {
        println!("Le nombre est divisible par 3");
    } else if nombre % 2 == 0 {
        println!("Le nombre est divisible par 2");
    } else {
        println!("Le nombre n'est pas divisible par 4, 3 ou 2");
    }

    let condition = true;
    let nombre = if condition { 5 } else { 6 };
    println!("La valeur du nombre est {}", nombre);
}