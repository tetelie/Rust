enum PieceUs {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn valeur_en_centimes(piece: PieceUs) -> u8 {
    match piece {
        PieceUs::Penny => {
            println!("Un centime porte-bonheur !");
            1
        }
        PieceUs::Nickel => 5,
        PieceUs::Dime => 10,
        PieceUs::Quarter => 25,
    }
}

fn main() {}
