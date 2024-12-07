mod exemple1;
mod exemple2;
mod exemple3;

fn main() {
    {                    // s n'est pas en vigueur ici, elle n'est pas encore déclarée
        let s = "hello"; // s est en vigueur à partir de ce point

        // on fait des choses avec s ici
    }                    // cette portée est maintenant terminée, et s n'est plus en vigueur



    let mut s = String::from("hello");

    s.push_str(", world!"); // push_str() ajoute un littéral de chaîne dans une String

    println!("{}", s); // Cela va afficher `hello, world!`


    {
        let s = String::from("hello"); // s est en vigueur à partir de ce point

        // on fait des choses avec s ici
    }                                  // cette portée est désormais terminée, et s
    // n'est plus en vigueur maintenant

    let x = 5;
    let y = x;

    let s1 = String::from("hello");
    let s2 = s1; // s1 n'est plus utilisable ici


    let s1 = String::from("hello");
    let s2 = s1.clone();

    println!("s1 = {}, s2 = {}", s1, s2);
}
