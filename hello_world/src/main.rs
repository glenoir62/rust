mod lifetime;
mod trait_test;

use trait_test::{Affichable, Rectangle, Cercle, afficher_aire};

fn main() {
    println!("Hello, world,test!");
    let x: i32 = 33;
    x.afficher();


    let rect = Rectangle { largeur: 5.0, hauteur: 3.0 };
    let cercle = Cercle { rayon: 2.0 };

    afficher_aire(&rect);    // Aire: 15.00
    afficher_aire(&cercle);  // Aire: 12.57

    lifetime::lifetime_test();
}
