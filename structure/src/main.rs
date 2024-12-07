mod exemple1;
mod exemple2;
mod exemple3;

struct Utilisateur {
    actif: bool,
    pseudo: String,
    email: String,
    nombre_de_connexions: u64,
}

fn main() {
    let utilisateur1 = Utilisateur {
        email: String::from("quelquun@example.com"),
        pseudo: String::from("pseudoquelconque123"),
        actif: true,
        nombre_de_connexions: 1,
    };

    let mut utilisateur2 = Utilisateur {
        email: String::from("quelquun@example.com"),
        pseudo: String::from("pseudoquelconque123"),
        actif: true,
        nombre_de_connexions: 1,
    };

    utilisateur2.email = String::from("unautremail@example.com");

    fn creer_utilisateur(email: String, pseudo: String) -> Utilisateur {
        Utilisateur {
            email: email,
            pseudo: pseudo,
            actif: true,
            nombre_de_connexions: 1,
        }
    }

    fn creer_utilisateur_plus_court(email: String, pseudo: String) -> Utilisateur {
        Utilisateur {
            email,
            pseudo,
            actif: true,
            nombre_de_connexions: 1,
        }
    }

    // copie dans utilisateur3 les données de utilisateur 1 en changant seulement l'email
    let utilisateur3 = Utilisateur {
        email: String::from("quelquundautre@example.com"),
        ..utilisateur1
    };

    // ici utilisateur n'est plus en vigeur car ses valeurs ont été déplacé dans utilisateur3

}
