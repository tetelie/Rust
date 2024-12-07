#[derive(Debug)] // permet de rendre affichable la structure
struct Rectangle {

    largeur: u32,
    hauteur: u32,
}

fn main() {
    let rect1 = Rectangle { largeur: 30, hauteur: 50 };

    println!(
        "L'aire du rectangle est de {} pixels carrés.",
        aire(&rect1)
    );

    println!("rect1 est {:?}", rect1); // affichage de debug du rectangle
    println!("rect1 est {:#?}", rect1); // affichage de debug du rectangle sur plusieurs lignes

}

fn aire(rectangle: &Rectangle) -> u32 {
    rectangle.largeur * rectangle.hauteur
}
