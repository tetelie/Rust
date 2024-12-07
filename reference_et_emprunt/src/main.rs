mod exemple1;
mod exemple2;
mod exemple3;
mod exemple4;
mod exemple5;
mod exemple6;
mod exemple7;

fn main() {
    let s1 = String::from("hello");

    let long = calculer_taille(&s1); // ici on utilise un référence

    println!("La taille de '{}' est {}.", s1, long); // si on utilisait pas de référence précédement, la variable s1 ne serait plus en vigueur ici
}

fn calculer_taille(s: &String) -> usize { // s est une référence à une String
    s.len()
} // Ici, s sort de la portée. Mais comme elle ne prend pas possession de ce
// à quoi elle fait référence, il ne se passe rien.

