mod exemple1;
mod exemple2;

fn main() {
    let s = String::from("hello world");

    let hello = &s[0..5];
    let world = &s[6..11];

    let slice = &s[0..2]; // fait la même chose
    let slice = &s[..2];  // fait la même chose

    let taille = s.len();

    let slice = &s[0..taille]; // même chose
    let slice = &s[..];           // même chose
}
