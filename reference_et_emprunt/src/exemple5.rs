fn main() {
    let mut s = String::from("hello");

    let r1 = &s; // sans problème
    let r2 = &s; // sans problème
    let r3 = &mut s; // GROS PROBLEME
    //Nous ne pouvons pas non plus avoir une référence mutable pendant que nous en avons une autre immuable vers la même valeur.

    println!("{}, {}, and {}", r1, r2, r3);
}
