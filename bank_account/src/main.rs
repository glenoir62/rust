fn main() {
    println!("Bienvenue sur votre compte bancaire !");
    let mut balance = 1000.0;
    println!("Votre solde initial: {}€", balance);

    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("Erreur lors de la lecture de la ligne");
    let deposit: f64 = input.trim().parse().expect("Veuillez entrer un nombre valide");
    balance += deposit;
    println!("Après un dépôt de {}: {}", deposit, balance);

    println!("Combien voulez-vous retirer?");
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("Erreur lors de la lecture de la ligne");
    let withdrawal: f64 = input.trim().parse().expect("Veuillez entrer un nombre valide");
    if balance - withdrawal < 0.0 {
        println!("Retrait impossible, solde insuffisant !");
    } else {
        balance -= withdrawal;
        println!("Après un retrait de {}: {}", withdrawal, balance);
    }
    let borrowed_balance = &balance;
    println!("Solde emprunté (sans modification): {}€", borrowed_balance);

    let withdrawal_fee = 10.0;
    let balance = balance - withdrawal_fee;  // shadowing
    println!("Solde après shadowing: {}€", balance);
}