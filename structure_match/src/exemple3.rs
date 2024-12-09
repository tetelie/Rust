fn main() {
    fn plus_un(x: Option<i32>) -> Option<i32> {
        match x {
            None => None,
            Some(i) => Some(i + 1),
        }
    }

    let cinq = Some(5);
    let six = plus_un(cinq);
    let none = plus_un(None);
}
