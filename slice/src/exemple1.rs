fn premier_mot(s: &String) -> &str {
    let octets = s.as_bytes();

    for (i, &element) in octets.iter().enumerate() {
        if element == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

fn premier_mot_mieux(s: &str) -> &str {
    let octets = s.as_bytes();

    for (i, &element) in octets.iter().enumerate() {
        if element == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

fn main() {
    let mut s = String::from("hello world");

    let mot = premier_mot(&s);

    s.clear(); // Erreur !

    println!("Le premier mot est : {}", mot);

    /*
    Ici, le type de s est un &str : c'est une slice qui pointe vers un endroit précis du binaire.
    C'est aussi la raison pour laquelle les littéraux de chaîne sont immuables ; &str est une référence immuable.
     */
    let s = "Hello, world!";

    let ma_string = String::from("hello world");

    // `premier_mot_mieux` fonctionne avec les slices de `String`, que ce soit sur
    // une partie ou sur sur son intégralité
    let mot = premier_mot_mieux(&ma_string[0..6]);
    let mot = premier_mot_mieux(&ma_string[..]);

    // `premier_mot_mieux` fonctionne également sur des références vers des `String`,
    // qui sont équivalentes à des slices de toute la `String`
    let mot = premier_mot_mieux(&ma_string);

    let mon_litteral_de_chaine = "hello world";

    // `premier_mot_mieux` fonctionne avec les slices de littéraux de chaîne, qu'elles
    // soient partielles ou intégrales
    let mot = premier_mot_mieux(&mon_litteral_de_chaine[0..6]);
    let mot = premier_mot_mieux(&mon_litteral_de_chaine[..]);

    // Comme les littéraux de chaîne *sont* déjà des slices de chaînes,
    // cela fonctionne aussi, sans la syntaxe de slice !
    let mot = premier_mot_mieux(mon_litteral_de_chaine);
}

