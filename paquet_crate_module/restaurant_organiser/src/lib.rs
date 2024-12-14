/*

PAQUET (1) --> contient CRATES (1 ou plusieurs)
  |
  |--> CRATE (1 binaire) --> contient MODULES (pour organiser le code)
  |
  |--> CRATE (1 ou plusieurs bibliothèques) --> contient MODULES



 */

//pub use crate::salle_a_manger::accueil;


fn servir_commande() {}

mod salle_a_manger;

mod cuisines {

    pub enum AmuseBouche {
        Soupe,
        Salade,
    }
    pub struct PetitDejeuner {
        pub tartine_grillee: String,
        fruit_de_saison: String,
    }

    impl PetitDejeuner {
        pub fn en_ete(tartine_grillee: &str) -> PetitDejeuner {
            PetitDejeuner {
                tartine_grillee: String::from(tartine_grillee),
                fruit_de_saison: String::from("pêches"),
            }
        }
    }

    fn corriger_commande_erronee() {
        cuisiner_commande();
        super::servir_commande();
    }

    fn cuisiner_commande() {}
}
pub use crate::salle_a_manger::accueil;
//use crate::salle_a_manger::accueil;
//use salle_a_manger::accueil;

pub fn manger_au_restaurant() {
    // Chemin absolu
    crate::salle_a_manger::accueil::ajouter_a_la_liste_attente();

    // Chemin relatif
    salle_a_manger::accueil::ajouter_a_la_liste_attente();

    accueil::ajouter_a_la_liste_attente();

    // On commande un petit-déjeuner en été avec tartine grillée au seigle
    let mut repas = cuisines::PetitDejeuner::en_ete("seigle");
    // On change d'avis sur le pain que nous souhaitons
    repas.tartine_grillee = String::from("blé");
    println!( "Je voudrais une tartine grillée au {}, s'il vous plaît.",
              repas.tartine_grillee);

    // La prochaine ligne ne va pas se compiler si nous ne la commentons pas,
    // car nous ne sommes pas autorisés à voir ou modifier le fruit de saison
    // qui accompagne le repas.

    // repas.fruit_de_saison = String::from("myrtilles");

    let commande1 = cuisines::AmuseBouche::Soupe;
    let commande2 = cuisines::AmuseBouche::Salade;
}