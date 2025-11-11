use rand::Rng;
use std::cmp::Ordering;
use std::collections::HashMap;
use std::io::{self, Write};

// Constantes du jeu - évite les "magic numbers" dans le code
const MAX_TENTATIVES: u32 = 10;
const POINTS_PAR_ESSAI: u32 = 10;

fn main() {
    // HashMap pour stocker les scores de tous les joueurs
    // Clé : nom du joueur (String)
    // Valeur : vecteur de tous ses scores (Vec<u32>)
    let mut scores: HashMap<String, Vec<u32>> = HashMap::new();

    // Boucle principale du jeu - permet de rejouer plusieurs parties
    loop {
        jouer_une_partie(&mut scores);

        // Si le joueur ne veut pas rejouer, on affiche le classement et on quitte
        if !rejouer() {
            afficher_classement(&scores);
            println!("Merci d'avoir joué !");
            break;
        }
    }
}

/// Fonction principale qui gère une partie complète
///
/// # Arguments
/// * `scores` - Référence mutable vers la HashMap des scores pour enregistrer le résultat
fn jouer_une_partie(scores: &mut HashMap<String, Vec<u32>>) {
    // Initialisation du générateur de nombres aléatoires
    let mut rng = rand::rng();

    // Génération du nombre secret entre 1 et 100 (inclusif)
    let nombre_secret = rng.random_range(1..=100);

    // Vecteur pour stocker toutes les tentatives du joueur
    // with_capacity() pré-alloue la mémoire pour éviter les réallocations
    let mut tentatives: Vec<u32> = Vec::with_capacity(MAX_TENTATIVES as usize);

    // Compteur d'essais restants
    let mut essais_restants = MAX_TENTATIVES;

    // Affichage du message de bienvenue
    println!("\n🎯 Bienvenue dans Devine le nombre !");
    println!("Vous avez {} essais pour trouver un nombre entre 1 et 100\n", MAX_TENTATIVES);

    // Boucle de jeu principale
    loop {
        // Affichage des tentatives précédentes (uniquement s'il y en a)
        if !tentatives.is_empty() {
            println!("📊 Vos tentatives : {:?}", tentatives);
        }

        println!("💭 Essais restants : {}", essais_restants);

        // Lecture et validation du nombre saisi par l'utilisateur
        // match avec guards (if) pour vérifier que le nombre est dans la plage valide
        let devine = match lire_nombre() {
            Some(n) if (1..=100).contains(&n) => n,  // Nombre valide dans la plage
            Some(_) => {  // Nombre hors plage
                println!("⚠️  Le nombre doit être entre 1 et 100 !");
                continue;  // Recommence la boucle sans décrémenter les essais
            }
            None => continue,  // Erreur de lecture, recommence
        };

        // Ajout de la tentative dans l'historique
        tentatives.push(devine);

        // Décrément des essais APRÈS avoir ajouté la tentative
        // Important : cela garantit un score correct basé sur le nombre de tentatives
        essais_restants -= 1;

        // Comparaison avec le nombre secret
        // En Rust, u32 implémente le trait Ord, qui permet de comparer une valeur de ce type avec une autre,
        // renvoyant un Ordering, qui est une Enum à 3 variantes (Less, Greater, Equal).
        // Pattern matching pour gérer tous les cas de figure :
        match devine.cmp(&nombre_secret) {
            Ordering::Less => println!("📉 Trop petit !"),
            Ordering::Greater => println!("📈 Trop grand !"),
            Ordering::Equal => {
                // Victoire ! Calcul et affichage du score
                println!("\n🎉 Bravo ! Vous avez trouvé le nombre {} !", nombre_secret);

                // Score = essais restants + 1 (car on a décrémenté avant) × points par essai
                let score_final = (essais_restants + 1) * POINTS_PAR_ESSAI;
                println!("⭐ Score : {} points ({} essais)", score_final, tentatives.len());

                // Enregistrement du score dans la HashMap
                enregistrer_score(scores, score_final);
                return;  // Fin de la partie
            }
        }

        // Vérification de la défaite (plus d'essais)
        if essais_restants == 0 {
            println!("\n💥 Perdu ! Le nombre était : {}", nombre_secret);
            return;  // Fin de la partie sans score
        }
    }
}

/// Lit un nombre saisi par l'utilisateur avec validation
///
/// # Returns
/// * `Some(u32)` - Le nombre saisi si valide
/// * `None` - Si l'entrée n'est pas un nombre valide
fn lire_nombre() -> Option<u32> {
    // Affichage du prompt
    print!("➤ Votre nombre : ");

    // flush() force l'affichage immédiat (sinon le print! attend un \n)
    io::stdout().flush().unwrap();

    // Lecture de l'entrée utilisateur
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Erreur de lecture");

    // Tentative de conversion String -> u32
    // parse() renvoie un Result<u32, ParseIntError>
    match input.trim().parse() {
        Ok(num) => Some(num),  // Conversion réussie
        Err(_) => {  // Erreur de parsing (pas un nombre)
            println!("⚠️  Veuillez entrer un nombre valide !");
            None
        }
    }
}

/// Enregistre le score d'un joueur dans la HashMap
///
/// # Arguments
/// * `scores` - Référence mutable vers la HashMap des scores
/// * `score` - Score à enregistrer
fn enregistrer_score(scores: &mut HashMap<String, Vec<u32>>, score: u32) {
    println!("\n📝 Entrez votre nom :");

    // Lecture du nom du joueur
    let mut nom = String::new();
    io::stdin()
        .read_line(&mut nom)
        .expect("Erreur de lecture");

    // trim() supprime les espaces et retours à la ligne
    let nom = nom.trim().to_string();

    // Validation : on n'enregistre pas si le nom est vide
    if nom.is_empty() {
        println!("Score non enregistré (nom vide)");
        return;
    }

    // entry() évite de faire deux lookups (get puis insert)
    // or_insert_with() crée un nouveau Vec seulement si nécessaire (lazy evaluation)
    scores
        .entry(nom.clone())
        .or_insert_with(Vec::new)
        .push(score);

    println!("✅ Score enregistré pour {} !", nom);
}

/// Demande au joueur s'il veut rejouer
///
/// # Returns
/// * `true` - Si le joueur veut rejouer
/// * `false` - Sinon
fn rejouer() -> bool {
    println!("\n🔄 Voulez-vous rejouer ? (o/n)");

    let mut reponse = String::new();
    io::stdin()
        .read_line(&mut reponse)
        .expect("Erreur de lecture");

    // matches!() est une macro qui teste si une valeur correspond à un pattern
    // Plus concis que match avec true/false
    matches!(reponse.trim().to_lowercase().as_str(), "o" | "oui" | "y" | "yes")
}

/// Affiche le classement final de tous les joueurs avec leurs statistiques
///
/// # Arguments
/// * `scores` - Référence vers la HashMap contenant tous les scores
fn afficher_classement(scores: &HashMap<String, Vec<u32>>) {
    // Si aucun score, on ne fait rien
    if scores.is_empty() {
        return;
    }

    println!("\n🏆 === CLASSEMENT FINAL ===");

    // Construction d'un vecteur de tuples pour le classement
    // On calcule : meilleur score, total des points, nombre de parties
    let mut classement: Vec<_> = scores
        .iter()
        .map(|(nom, scores_joueur)| {
            // iter().max() renvoie une Option<&u32>, unwrap_or pour gérer le cas vide
            let meilleur = *scores_joueur.iter().max().unwrap_or(&0);

            // sum() additionne tous les scores
            let total: u32 = scores_joueur.iter().sum();

            // len() donne le nombre de parties jouées
            let parties = scores_joueur.len();

            // Tuple avec toutes les infos
            (nom, meilleur, total, parties)
        })
        .collect();

    // Tri par meilleur score décroissant
    // b.1.cmp(&a.1) inverse l'ordre (décroissant)
    // .1 accède au deuxième élément du tuple (meilleur score)
    classement.sort_by(|a, b| b.1.cmp(&a.1));

    // Affichage du classement avec numérotation
    for (i, (nom, meilleur, total, parties)) in classement.iter().enumerate() {
        println!(
            "{}. {} - Meilleur: {} pts | Total: {} pts | Parties: {}",
            i + 1,  // enumerate() commence à 0, donc +1 pour l'affichage
            nom,
            meilleur,
            total,
            parties
        );
    }
}