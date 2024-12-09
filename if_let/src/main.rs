fn main() {


    let une_valeur_u8 = Some(3u8);
    match une_valeur_u8 {
        Some(max) => println!("Le maximum est réglé sur {}", max),
        _ => (), // code inutile
    }

    // solution
    let une_valeur_u8 = Some(3u8);
    if let Some(max) = une_valeur_u8 {
        println!("Le maximum est réglé sur {}", max);
    }
}
