struct Utilisateur {
    actif: bool,
    pseudo: &str, // pas possible car besoin d'une durée de vie
    email: &str,  // pas possible car besoin d'une durée de vie
    nombre_de_connexions: u64,
}

fn main() {
    let utilisateur1 = Utilisateur {
        email: "quelquun@example.com",
        pseudo: "pseudoquelconque123",
        actif: true,
        nombre_de_connexions: 1,
    };
}
