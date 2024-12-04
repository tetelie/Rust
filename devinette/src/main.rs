use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Devinez le nombre !");

    let nombre_secret = rand::thread_rng().gen_range(1..101);

    //println!("Le nombre secret est : {}", nombre_secret);

    loop {
        println!("Veuillez entrer le nombre !");

        let mut supposition = String::new();

        io::stdin()
            .read_line(&mut supposition)
            .expect("Echec de la lecture de l'entrée utilisateur");

        println!("Votre nombre : {}", supposition);

        let supposition: u32 = match supposition.trim().parse()
        {
            Ok(nombre) => nombre,
            Err(_) => continue,
        };

        match supposition.cmp(&nombre_secret)
        {
            Ordering::Less => println!("C'est plus !"),
            Ordering::Greater => println!("C'est moins !"),
            Ordering::Equal => {
                println!("Vous avez gagné !");
                break;
            }
        }
    }

}