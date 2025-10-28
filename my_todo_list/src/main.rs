use std::collections::HashMap;
use std::io;

fn main() {
    let mut tasks: HashMap<i32, (String, TaskStatus)> = HashMap::new();
    let mut next_task_id = 1;

    loop {
        println!("Gestionnaire de listes de tâches");
        println!("1. Ajouter une tâche");
        println!("2. Afficher la liste de tâches");
        println!("3. Marquer une tâche comme complète");
        println!("4. Supprimer une tâche");
        println!("5. Quitter");

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).expect("Échec de la lecture de l'entrée");

        match choice.trim() {
            "1" => {
                println!("Entrez la description de la tâche :");
                let mut description = String::new();
                io::stdin()
                    .read_line(&mut description)
                    .expect("Échec de la lecture de l'entrée");
                add_task(&mut tasks, description.trim().to_string(), &mut next_task_id);
            },
            "2" => {
                list_tasks(&tasks);
            },
            "3" => {
                println!("Entrez l'ID de la tâche à marquer comme complète :");
                let mut id_str = String::new();
                io::stdin().read_line(&mut id_str).expect("Échec de la lecture de l'entrée");
                if let Ok(id) = id_str.trim().parse::<i32>() {
                    complete_task(&mut tasks, id);
                } else {
                    println!("Veuillez entrer un ID valide.");
                }
            },
            "4" => {
                println!("Entrez l'ID de la tâche à supprimer :");
                let mut id_str = String::new();
                io::stdin().read_line(&mut id_str).expect("Échec de la lecture de l'entrée");
                if let Ok(id) = id_str.trim().parse::<i32>() {
                    remove_task(&mut tasks, id);
                } else {
                    println!("Veuillez entrer un ID valide.");
                }
            }
            "5" => {
                println!("Au revoir !");
                break;
            }
            _ => println!("Choix invalide."),
        }
    }
}

fn remove_task<T: Task>(tasks: &mut HashMap<i32, (T, TaskStatus)>, id: i32) {
    if tasks.remove(&id).is_some() {
        println!("Tâche supprimée.");
    } else {
        println!("ID de tâche invalide.");
    }
}
fn list_tasks<T: Task>(tasks: &HashMap<i32, (T, TaskStatus)>) {
    println!("Liste de tâches :");
    for (id, (description, status)) in tasks {
        println!("{}: {} (Statut : {:?})", id, description.description(), status);
    }
}

fn complete_task<T: Task>(tasks: &mut HashMap<i32, (T, TaskStatus)>, id: i32) {

    if let Some((_, status)) = tasks.get_mut(&id) {
        *status = TaskStatus::Complete;
        println!("Tâche marquée comme complète.");

    } else {
        println!("ID de tâche invalide.");
    }
}
fn add_task<T: Task>(
    tasks: &mut HashMap<i32, (T, TaskStatus)>,
    description: T,
    next_task_id: &mut i32,
) {
    tasks.insert(*next_task_id, (description, TaskStatus::Incomplete));
    *next_task_id += 1;
    println!("Tâche ajoutée.");
}

/*
#[derive(Debug)] est un attribut (annotation) qui demande au compilateur Rust
 de générer automatiquement l'implémentation du trait Debug pour votre type.
 Debug est un trait standard de Rust qui permet de formater un type pour le débogage.
 ``
╔═══════════════════════════════════════════════╗
║          #[derive(Debug)]                     ║
╠═══════════════════════════════════════════════╣
║                                               ║
║  Quoi?   Génère automatiquement le trait     ║
║          Debug pour votre type                ║
║                                               ║
║  Usage:  println!("{:?}", x)                  ║
║          println!("{:#?}", x)  (indenté)      ║
║          dbg!(x)               (avec infos)   ║
║                                               ║
║  Avantages:                                   ║
║   ✅ Code généré automatiquement              ║
║   ✅ Affichage lisible                        ║
║   ✅ Essentiel pour le débogage               ║
║   ✅ Compatible avec assertions               ║
║                                               ║
║  Combinable avec:                             ║
║   Clone, Copy, PartialEq, Eq, etc.           ║
║                                               ║
╚═══════════════════════════════════════════════╝
 */
#[derive(Debug)]
enum TaskStatus {
    Incomplete,
    Complete,
}

trait Task {
    fn description(&self) -> String;
    fn status(&self) -> TaskStatus;
}

impl Task for String {
    fn description(&self) -> String {
        self.clone()
    }

    fn status(&self) -> TaskStatus {
        TaskStatus::Incomplete
    }
}