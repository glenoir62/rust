mod lifetime;

fn main() {
    println!("Hello, world,test!");
    let x: i32 = 33;
    x.afficher();


    let rect = Rectangle { largeur: 5.0, hauteur: 3.0 };
    let cercle = Cercle { rayon: 2.0 };

    afficher_aire(&rect);    // Aire: 15.00
    afficher_aire(&cercle);  // Aire: 12.57
}

// Définition du trait Affichable
/*
Les Traits en Rust 🦀Les traits sont l'un des concepts les plus puissants de Rust.
 Ils définissent un comportement partagé entre différents types.
 Définition simpleUn trait est comme un contrat ou une interface :
  il déclare un ensemble de méthodes qu'un type doit implémenter.
 */
trait Affichable {
    fn afficher(&self);
}

// Implémentation du trait Affichable pour le type i32
impl Affichable for i32 {
    fn afficher(&self) {
        println!("C'est un nombre : {}", self);
    }
}

// Un trait pour calculer l'aire
trait Aire {
    fn aire(&self) -> f64;
}

struct Rectangle {
    largeur: f64,
    hauteur: f64,
}

struct Cercle {
    rayon: f64,
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
fn afficher_aire<T: Aire>(forme: &T) {
    println!("Aire: {:.2}", forme.aire());
}

