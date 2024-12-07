fn main()
{
    let largeur1 = 30;
    let hauteur1 = 50;

    println!("L'aire du rectangle est {}", aire(largeur1, hauteur1));
}

fn aire(largeur: u32, hauteur: u32) -> u32{
    largeur * hauteur
}