use std::io;

fn main() {
    println!("Entrez une phrase :");
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Erreur lors de la lecture de l'entrée");

    let trimmed_input = input.trim();

    let vowel_count = trimmed_input
        .chars()
        .filter(|c| "aeiouyAEIOUY".contains(*c))
        .count();
    println!("Nombre de voyelles dans la phrase : {}", vowel_count);

    let clean_input: String = trimmed_input
        .chars()
        .filter(|c| c.is_alphanumeric())
        .collect();
    let is_palindrome = clean_input
        .to_lowercase()
        .chars()
        .eq(clean_input.to_lowercase().chars().rev());

    if is_palindrome {
        println!("La phrase est un palindrome !");
    } else {
        println!("La phrase n'est pas un palindrome.");
    }

    let mut words: Vec<&str> = trimmed_input.split_whitespace().collect();
    words.sort_by_key(|&word| word.len());
    println!("Mots triés par longueur : {:?}", words);

    let acronym: String = trimmed_input
        .split_whitespace()
        .map(|word| word.chars().next().unwrap_or(' '))
        .collect();
    println!("Acronyme de la phrase : {}", acronym);

}
