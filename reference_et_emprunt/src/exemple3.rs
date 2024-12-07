fn main() {
    let mut s = String::from("hello");

    let r1 = &mut s;
    let r2 = &mut s;
    // on ne peut pas emprunté s plus d'une fois de manière mutable en même temps
    println!("{}, {}", r1, r2);
}
