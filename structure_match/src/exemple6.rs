fn main() {
    let jete_de_de = 9;
    match jete_de_de {
        3 => ajouter_chapeau_fantaisie(),
        7 => enleve_chapeau_fantaisie(),
        _ => (),
    }

    fn ajouter_chapeau_fantaisie() {}
    fn enleve_chapeau_fantaisie() {}
}
