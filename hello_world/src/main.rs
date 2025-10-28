mod lifetime;
mod trait_test;
mod generic_test;
mod closure_function_high_test;
mod option_test;

use closure_function_high_test::{doubler, appliquer, closure_test};
use trait_test::{Affichable, Rectangle, Cercle, afficher_aire};
use crate::generic_test::{afficher_contenu, plus_grand};
use crate::option_test::option_test;

fn main() {
    println!("Hello, world,test!");

    //trait
    let x: i32 = 33;
    x.afficher();


    let rect = Rectangle { largeur: 5.0, hauteur: 3.0 };
    let cercle = Cercle { rayon: 2.0 };

    afficher_aire(&rect);    // Aire: 15.00
    afficher_aire(&cercle);  // Aire: 12.57

    //lifetime
    lifetime::lifetime_test();

    //generic
    let vecteur_entiers = vec![1, 2, 3];
    let vecteur_mots = vec!["Hello", "Rust", "World"];

    afficher_contenu(vecteur_entiers);
    afficher_contenu(vecteur_mots);

    let a = 5;
    let b = 10;
    let max = plus_grand(a, b);
    println!("Le plus grand nombre est : {}", max);

    let c = 3.5;
    let d = 2.7;
    let max_float = plus_grand(c, d);
    println!("Le plus grand nombre flottant est : {}", max_float);

    //high level function
    let ma_fonction = doubler;
    let resultat = ma_fonction(5);
    println!("Résultat : {}", resultat); // Résultat : 10


    let resultat = appliquer(doubler, 5);
    println!("Résultat : {}", resultat); // Résultat : 10

    //closure
    let x = 5;
    let doubler = |y| y * x;

    let resultat = doubler(10);
    println!("Résultat closure : {}", resultat); // Résultat : 50

    closure_test();

    // Option
    option_test();

}
