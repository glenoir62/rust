mod closure_function_high_test;
mod generic_test;
mod lifetime;
mod option_test;
mod trait_struct;
mod trait_test;
use std::sync::mpsc;

use closure_function_high_test::{appliquer, closure_test, doubler};
use std::cmp::Ordering;
use trait_struct::rectangle::Rectangle as OtherRectangle;
use trait_test::{Affichable, Cercle, Rectangle, afficher_aire};
use std::thread;
use std::time::Duration;

use crate::generic_test::{afficher_contenu, plus_grand};
use crate::option_test::option_test;

fn main() {
    println!("Hello, world,test!");

    //trait
    let x: i32 = 33;
    x.afficher();

    let rect = Rectangle {
        largeur: 5.0,
        hauteur: 3.0,
    };
    let cercle = Cercle { rayon: 2.0 };

    afficher_aire(&rect); // Aire: 15.00
    afficher_aire(&cercle); // Aire: 12.57

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

    //struct trait
    let rect1 = OtherRectangle::new(10, 20);

    let area1 = rect1.area();
    println!("Superficie du rectangle : {}", area1);
    println!("Struct traits ->");

    #[derive(PartialOrd, PartialEq)]
    struct Temperature {
        value: f64,
    }

    let hot = Temperature { value: 30.0 };
    let cold = Temperature { value: 10.0 };

    match hot.partial_cmp(&cold) {
        Some(Ordering::Greater) => println!("It's hot!"),
        Some(Ordering::Less) => println!("It's cold!"),
        Some(Ordering::Equal) => println!("It's the same temperature."),
        None => println!("Temperature comparison is not defined."),
    }

    //Thread

    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("Je suis le thread spawné numéro {}", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..5 {
        println!("Je suis le thread principal numéro {}", i);
        thread::sleep(Duration::from_millis(1));
    }

    handle.join().unwrap();


    let (tx, rx) = mpsc::channel();

    // Producteur
    let producteur = thread::spawn(move || {
        for i in 0..10 {
            println!("Production: {}", i);
            tx.send(i).unwrap();
            thread::sleep(Duration::from_millis(100));
        }
    });

    // Consommateur
    let consommateur = thread::spawn(move || {
        while let Ok(valeur) = rx.recv() {
            println!("  Consommation: {}", valeur);
            thread::sleep(Duration::from_millis(200));
        }
    });

    producteur.join().unwrap();
    consommateur.join().unwrap();
}
