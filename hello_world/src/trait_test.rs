// Définition du trait Affichable
/*
Les Traits en Rust 🦀Les traits sont l'un des concepts les plus puissants de Rust.
 Ils définissent un comportement partagé entre différents types.
 Définition simpleUn trait est comme un contrat ou une interface :
  il déclare un ensemble de méthodes qu'un type doit implémenter.
 */
pub(crate) trait Affichable {
    fn afficher(&self);
}

// Implémentation du trait Affichable pour le type i32
impl Affichable for i32 {
    fn afficher(&self) {
        println!("C'est un nombre : {}", self);
    }
}

// Un trait pour calculer l'aire
pub(crate)  trait Aire {
    fn aire(&self) -> f64;
}

pub(crate)  struct Rectangle {
    pub largeur: f64,
    pub hauteur: f64,
}

pub(crate)  struct Cercle {
    pub rayon: f64,
}

impl Aire for Rectangle {
    fn aire(&self) -> f64 {
        self.largeur * self.hauteur
    }
}

impl Aire for Cercle {
    fn aire(&self) -> f64 {
        std::f64::consts::PI * self.rayon * self.rayon
    }
}

// Fonction générique acceptant tout type implémentant Aire
pub(crate) fn afficher_aire<T: Aire>(forme: &T) {
    println!("Aire: {:.2}", forme.aire());
}
