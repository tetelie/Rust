enum Message {
    Quitter,
    Deplacer { x: i32, y: i32 },
    Ecrire(String),
    ChangerCouleur(i32, i32, i32),
}
impl Message {
    fn appeler(&self) {
        // le corps de la méthode sera défini ici
    }
}
fn main() {
    let m = Message::Ecrire(String::from("hello"));
    m.appeler();
}
