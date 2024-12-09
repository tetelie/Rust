mod exemple1;
mod exemple2;
mod exemple3;
mod exemple4;
mod exemple5;
mod exemple6;

enum PieceUs {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn valeur_en_centimes(piece: PieceUs) -> u8 {
    match piece {
        PieceUs::Penny => 1,
        PieceUs::Nickel => 5,
        PieceUs::Dime => 10,
        PieceUs::Quarter => 25,
    }
}

fn main() {}
