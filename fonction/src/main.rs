fn main() {
    println!("Hello, world!");
    une_autre_fonction(5, 'h');

    // instruction
    let y = 6;

    let y = {
        let x = 5;
        x + 1
    };

    println!("La valeur de y est {}", y);

    let y = cinq();
    println!("La valeur de y est {}", y);


    println!("La valeur de y+1 est {}", plus_un(y));

}

fn une_autre_fonction(valeur: i32, unite: char)
{
    println!("La mesure est: {}{}", valeur, unite);
}

fn cinq() -> i32 {
    5
}

fn plus_un(x: i32) -> i32
{
    x + 1
}
