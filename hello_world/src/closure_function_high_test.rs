/*

Résumé 🎯
Closures :

Fonctions anonymes : |x| x + 1
Capturent l'environnement
Trois traits : Fn, FnMut, FnOnce
Mot-clé move pour prendre possession

Fonctions de haut niveau :

Prennent des closures en paramètre : fn apply<F: Fn(i32)>
Retournent des closures : -> impl Fn(i32)
Essentielles avec les itérateurs : map, filter, fold

Les closures rendent le code Rust expressif, concis et puissant ! 🚀
 */
pub fn doubler(x: i32) -> i32 {
    x * 2
}

pub fn appliquer<F>(f: F, x: i32) -> i32
where
    F: Fn(i32) -> i32,
{
    f(x)
}

struct Produit {
    nom: String,
    prix: f64,
    en_stock: bool,
}

pub fn closure_test() {
    let produits = vec![
        Produit { nom: "Laptop".to_string(), prix: 999.99, en_stock: true },
        Produit { nom: "Souris".to_string(), prix: 29.99, en_stock: true },
        Produit { nom: "Clavier".to_string(), prix: 79.99, en_stock: false },
        Produit { nom: "Écran".to_string(), prix: 299.99, en_stock: true },
    ];

    // Filtrer, transformer et calculer
    let total_disponible: f64 = produits.iter()
        .filter(|p| p.en_stock)                    // Produits en stock
        .filter(|p| p.prix < 500.0)                // Prix < 500€
        .map(|p| p.prix)                           // Extraire les prix
        .sum();                                    // Sommer

    println!("Total des produits disponibles < 500€: {:.2}€", total_disponible);

    // Trouver le produit le moins cher en stock
    let moins_cher = produits.iter()
        .filter(|p| p.en_stock)
        .min_by(|a, b| a.prix.partial_cmp(&b.prix).unwrap());

    if let Some(produit) = moins_cher {
        println!("Produit le moins cher: {} à {:.2}€", produit.nom, produit.prix);
    }

    // Créer une fonction de réduction personnalisée
    let appliquer_reduction = |pourcentage: f64| {
        move |prix: f64| prix * (1.0 - pourcentage / 100.0)
    };

    let reduction_20 = appliquer_reduction(20.0);
    let prix_reduit = reduction_20(100.0);
    println!("Prix après réduction: {:.2}€", prix_reduit);  // 80.00€
}