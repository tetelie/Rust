fn main() {
    let supposition: u32 = "42".parse().expect("Ce n'est pas un nombre");
    let x = 2.0;
    let y: f32 = 3.0;

    // addition
    let somme = 5 + 10;

    // soustraction
    let difference = 95.5 - 4.3;

    // multiplication
    let produit = 4 * 30;

    // division
    let quotient = 56.7 / 32.2;
    let arrondi = 2 / 3; // retournera 0

    // modulo
    let reste = 43 % 5;

    let t = true;

    let f: bool = false;

    let c = 'z';
    let z = 'Z';
    let emoji = 'e';

    let tup: (i32, f64, u8) = (500, 6.4, 1);

    let (x, y, z) = tup;
    println!("x: {}, y: {}, z: {}", x, y, z);

    let un = tup.2;

    let a = [1, 2, 3, 4, 5];

    let mois = ["Janvier", "Février", "Mars", "Avril", "Mai", "Juin", "Juillet",
        "Août", "Septembre", "Octobre", "Novembre", "Décembre"];

    let a: [i32; 5] = [1, 2, 3, 4, 5];

    let a = [3; 5];

    let premier = a[0];
    let second = a[1];
}
