use rand::Rng;
use std::io;

pub fn play_game() -> i64 {
    ui::clear_terminal();
    // Choisis un mot cible aléatoire
    let words = vec!["rust", "cargo", "programming", "cli", "training", "game"];

    // Génération d'un nombre aléatoire entre 1 et 100
    let mut rng = rand::thread_rng();
    let nombre_secret = rng.gen_range(1..words.len());
    let target_word = words.get(nombre_secret).unwrap();

    // Conversion du mot cible en vecteur de caractères pour faciliter la manipulation
    let target_chars: Vec<char> = target_word.chars().collect();

    // Vecteur pour stocker les lettres correctes devinées
    let mut correct_guesses: Vec<char> = vec![' '; target_word.len()];
    let mut misplaced_guesses: Vec<char>;
    let mut nombre_essai = 10;
    let score_final: i64;

    println!("Bienvenue dans le jeu de Motus en Rust!");
    println!("Vous avez au total {} essais", nombre_essai);

    loop {
        // Demande à l'utilisateur de proposer un mot
        println!("Devinez le mot ({} lettres) :", target_word.len());
        let mut user_input = String::new();
        io::stdin()
            .read_line(&mut user_input)
            .expect("Erreur de lecture de la ligne");
        let user_input = user_input.trim();

        if !user_input.len().eq(&target_word.len()) {
            println!("Attention, la longueur attendue n'est pas respectée !");
            if nombre_essai > 0 {
                nombre_essai -= 1;
            }
            println!("Il vous reste {} essais !", nombre_essai);
            continue;
        }

        // Conversion de l'entrée de l'utilisateur en vecteur de caractères
        let user_chars: Vec<char> = user_input.trim().chars().collect();

        // Vérifie les lettres correctes et mal placées
        misplaced_guesses = Vec::new();
        for (index, &user_char) in user_chars.iter().enumerate() {
            if let Some(char) = target_chars.get(index) {
                if char.eq(&user_char) {
                    correct_guesses[index] = user_char;
                } else {
                    correct_guesses[index] = ' ';
                    if target_chars.contains(&user_char) {
                        misplaced_guesses.push(user_char);
                    }
                }
            }
        }

        // Vérifie si l'utilisateur a deviné le mot correctement
        if correct_guesses.iter().all(|&x| x != ' ') {
            println!("Félicitations! Vous avez deviné le mot : {}", target_word);
            score_final = nombre_essai * 10;
            break;
        }

        println!("\nLettres correctes et bien placées: {:?}", correct_guesses);
        println!(
            "\nLettres correctes mais mal placées: {:?}",
            misplaced_guesses
        );
        println!("Réessayez!\n");
        if nombre_essai > 0 {
            nombre_essai -= 1;
        }
        println!("Il vous reste {} essais !", nombre_essai);
    }
    score_final
}