fn main() {
    println!("Bienvenue dans la calculatrice simple !");
    loop {
        println!("Veuillez choisir une opération: +, -, *, / ou quit pour quitter.");

        /*
        mut (devant operation) = "Cette variable peut changer"
        &mut (devant operation) = "Je prête cette variable ET j'autorise qu'on la modifie"
         */
        let mut operation = String::new(); //mut : la variable PEUT être modifiée

        /*
        Imaginez que operation est votre cahier :

        Sans & : vous donnez le cahier → vous ne l'avez plus
        Avec & : vous prêtez le cahier en lecture seule
        Avec &mut : vous prêtez le cahier et autorisez qu'on écrive dedans ✅
         */
        std::io::stdin()
            .read_line(&mut operation) // &mut : je PRÊTE ma variable,
            // en autorisant sa modification
            .expect("Erreur lors de la lecture de la ligne");

        let operation = operation.trim();
        // (suite de la boucle...)
        if operation == "quit" {
            break;
        }

        let valid_operations = ["+", "-", "*", "/"];
        let mut is_valid_operation = false;

        for &valid_op in &valid_operations {
            if operation == valid_op {
                is_valid_operation = true;
                break;
            }
        }

        if !is_valid_operation {
            println!("Opération non reconnue.");
            continue;
        }


        // Dans la fonction main
        println!("Entrez le premier nombre:");
        let num1 = read_number();

        println!("Entrez le deuxième nombre:");
        let num2 = read_number();
        // ... (à continuer dans les prochaines étapes)

        let result: f64;

        if operation == "+" {
            result = num1 + num2;
        } else if operation == "-" {
            result = num1 - num2;
        } else if operation == "*" {
            result = num1 * num2;
        } else {
            result = num1 / num2;
        }
        println!("Résultat: {} {} {} = {}", num1, operation, num2, result);

    }
}

fn read_number() -> f64 {
    let mut res: f64 = 0.0;

    while res == 0.0 {
        let mut input = String::new();
        std::io::stdin()
            .read_line(&mut input)
            .expect("Erreur lors de la lecture de la ligne");

        // Ici, le code est un peu nouveau
        // Nous expliquerons en détail ce code dans le chapitre sur la gestion des erreurs
        // Pour le moment, comprenez juste que, si le parse réussit, nous attribuons la value à res
        // Sinon, nous affichons un texte d'erreur, et la boucle while reste valide donc recommence
        match input.trim().parse::<f64>() {
            Ok(value) => {
                res = value;
            },
            Err(_) => {
                println!("Ce n'est pas un nombre valide")
            }
        };
    }
    res
}
